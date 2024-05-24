#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "INPUTMUX"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Inputmux Register for SCT0 Input"]
    pub SCT0_INMUX: [crate::RWRegister<u32>; 8usize],
    #[doc = "Cluster CTIMER%s, containing CTIMER[012]CAP0, CTIMER[012]CAP1, CTIMER[012]CAP2, CTIMER[012]CAP3, TIMER[012]TRIG"]
    pub CTIMER: [ctimer::RegisterBlock; 3usize],
    _reserved0: [u8; 0x20],
    #[doc = "Inputmux Register for SMARTDMA Arch B Inputs"]
    pub SMARTDMAARCHB_INMUX: [crate::RWRegister<u32>; 8usize],
    #[doc = "Pin Interrupt Select"]
    pub PINTSEL: [crate::RWRegister<u32>; 8usize],
    _reserved1: [u8; 0xa0],
    #[doc = "Selection for Frequency Measurement Reference Clock"]
    pub FREQMEAS_REF: crate::RWRegister<u32>,
    #[doc = "Selection for Frequency Measurement Target Clock"]
    pub FREQMEAS_TAR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x18],
    #[doc = "Capture Select Register for CTIMER Inputs"]
    pub CTIMER3CAP0: crate::RWRegister<u32>,
    #[doc = "Capture Select Register for CTIMER Inputs"]
    pub CTIMER3CAP1: crate::RWRegister<u32>,
    #[doc = "Capture Select Register for CTIMER Inputs"]
    pub CTIMER3CAP2: crate::RWRegister<u32>,
    #[doc = "Capture Select Register for CTIMER Inputs"]
    pub CTIMER3CAP3: crate::RWRegister<u32>,
    #[doc = "Trigger Register for TIMER"]
    pub TIMER3TRIG: crate::RWRegister<u32>,
    _reserved3: [u8; 0xc],
    #[doc = "Capture Select Register for CTIMER Inputs"]
    pub CTIMER4CAP0: crate::RWRegister<u32>,
    #[doc = "Capture Select Register for CTIMER Inputs"]
    pub CTIMER4CAP1: crate::RWRegister<u32>,
    #[doc = "Capture Select Register for CTIMER Inputs"]
    pub CTIMER4CAP2: crate::RWRegister<u32>,
    #[doc = "Capture Select Register for CTIMER Inputs"]
    pub CTIMER4CAP3: crate::RWRegister<u32>,
    #[doc = "Trigger Register for TIMER"]
    pub TIMER4TRIG: crate::RWRegister<u32>,
    _reserved4: [u8; 0x8c],
    #[doc = "CMP0 Input Connections"]
    pub CMP0_TRIG: crate::RWRegister<u32>,
    _reserved5: [u8; 0x1c],
    #[doc = "ADC Trigger Input Connections"]
    pub ADC0_TRIG: [crate::RWRegister<u32>; 4usize],
    _reserved6: [u8; 0x30],
    #[doc = "ADC Trigger Input Connections"]
    pub ADC1_TRIG: [crate::RWRegister<u32>; 4usize],
    _reserved7: [u8; 0x30],
    #[doc = "DAC%s Trigger Inputs"]
    pub DAC__TRIG: [crate::RWRegister<u32>; 3usize],
    #[doc = "Cluster ENC%s, containing ENC?_TRIG, ENC?_HOME, ENC?_INDEX, ENC?_PHASEB, ENC?_PHASEA"]
    pub ENC: [enc::RegisterBlock; 2usize],
    #[doc = "PWM0 External Synchronization"]
    pub FlexPWM0_SM_EXTSYNC: [crate::RWRegister<u32>; 4usize],
    #[doc = "PWM0 Input Trigger Connections"]
    pub FlexPWM0_SM_EXTA: [crate::RWRegister<u32>; 4usize],
    #[doc = "PWM0 External Force Trigger Connections"]
    pub FlexPWM0_EXTFORCE: crate::RWRegister<u32>,
    #[doc = "PWM0 Fault Input Trigger Connections"]
    pub FlexPWM0_FAULT: [crate::RWRegister<u32>; 4usize],
    _reserved8: [u8; 0xc],
    #[doc = "PWM1 External Synchronization"]
    pub FlexPWM1_SM_EXTSYNC: [crate::RWRegister<u32>; 4usize],
    #[doc = "PWM1 Input Trigger Connections"]
    pub FlexPWM1_SM_EXTA: [crate::RWRegister<u32>; 4usize],
    #[doc = "PWM1 External Force Trigger Connections"]
    pub FlexPWM1_EXTFORCE: crate::RWRegister<u32>,
    #[doc = "PWM1 Fault Input Trigger Connections"]
    pub FlexPWM1_FAULT: [crate::RWRegister<u32>; 4usize],
    _reserved9: [u8; 0xc],
    #[doc = "PWM0 External Clock Trigger"]
    pub PWM0_EXT_CLK: crate::RWRegister<u32>,
    #[doc = "PWM1 External Clock Trigger"]
    pub PWM1_EXT_CLK: crate::RWRegister<u32>,
    _reserved10: [u8; 0x18],
    #[doc = "EVTG Trigger Input Connections %s"]
    pub EVTG_TRIG: [crate::RWRegister<u32>; 16usize],
    #[doc = "USB-FS Trigger Input Connections"]
    pub USBFS_TRIG: crate::RWRegister<u32>,
    _reserved11: [u8; 0x1c],
    #[doc = "TSI Trigger Input Connections"]
    pub TSI_TRIG: crate::RWRegister<u32>,
    _reserved12: [u8; 0x1c],
    #[doc = "EXT Trigger Connections %s"]
    pub EXT_TRIG: [crate::RWRegister<u32>; 8usize],
    #[doc = "CMP1 Input Connections"]
    pub CMP1_TRIG: crate::RWRegister<u32>,
    _reserved13: [u8; 0x1c],
    #[doc = "CMP2 Input Connections"]
    pub CMP2_TRIG: crate::RWRegister<u32>,
    _reserved14: [u8; 0x1c],
    #[doc = "SINC Filter Channel%s Trigger Input Connections"]
    pub SINC_FILTER_CH: [crate::RWRegister<u32>; 5usize],
    _reserved15: [u8; 0x4c],
    #[doc = "OPAMP%s Trigger Input Connections"]
    pub OPAMP__TRIG: [crate::RWRegister<u32>; 3usize],
    _reserved16: [u8; 0x14],
    #[doc = "LP_FLEXCOMM%s Trigger Input Connections"]
    pub FLEXCOMM__TRIG: [crate::RWRegister<u32>; 10usize],
    #[doc = "FlexIO Trigger Input Connections %s"]
    pub FLEXIO_TRIG: [crate::RWRegister<u32>; 8usize],
    #[doc = "Cluster DMA%s, containing DMA?_REQ_ENABLE0, DMA?_REQ_ENABLE0_SET, DMA?_REQ_ENABLE0_CLR, DMA?_REQ_ENABLE0_TOG, DMA?_REQ_ENABLE1, DMA?_REQ_ENABLE1_SET, DMA?_REQ_ENABLE1_CLR, DMA?_REQ_ENABLE1_TOG, DMA?_REQ_ENABLE2, DMA?_REQ_ENABLE2_SET, DMA?_REQ_ENABLE2_CLR, DMA?_REQ_ENABLE2_TOG, DMA?_REQ_ENABLE3, DMA?_REQ_ENABLE3_SET, DMA?_REQ_ENABLE3_CLR"]
    pub DMA: [dma::RegisterBlock; 2usize],
}
#[doc = "Inputmux Register for SCT0 Input"]
pub mod SCT0_INMUX {
    #[doc = "Input number to SCT0 inputs."]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SCT0_IN0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "SCT0_IN1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT0_IN2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT0_IN3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT0_IN4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "SCT0_IN5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "SCT0_IN6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "SCT0_IN7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER0_MAT0 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER1_MAT0 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CTIMER2_MAT0 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CTIMER3_MAT0 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "CTIMER4_MAT0 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0 ADC0_IRQ input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "usb0 start of frame input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "usb1 start of frame input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "SINC Filter CH0 Conversion Complete input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "SINC Filter CH1 Conversion Complete input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "SINC Filter CH2 Conversion Complete input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "SINC Filter CH3 Conversion Complete input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "SINC Filter CH4 Conversion Complete input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "ARM_TXEV (CM33 event output) input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "DEBUG_HALTED input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "ADC1_IRQ input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "FC3_P0 (SDO, RXD, SDA) input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "FC3_P1 (SCK, TXD, SCL)) input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "FC3_P2 (SDI, RTS, SCLS, TXD) input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "FC3_P3 (PCS[0], CTS, SDAS, RXD) input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "LP_FLEXCOMM0 trig 0 (lpuart_trg_txword) input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "LP_FLEXCOMM0 trig 1 (lpuart_trg_rxword) input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "LP_FLEXCOMM0 trig 2 (lpuart_trg_rxidle) input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
            pub const VAL64: u32 = 64;
            #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
            pub const VAL65: u32 = 65;
            #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
            pub const VAL66: u32 = 66;
            #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
            pub const VAL67: u32 = 67;
            #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
            pub const VAL68: u32 = 68;
            #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
            pub const VAL69: u32 = 69;
            #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
            pub const VAL70: u32 = 70;
            #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
            pub const VAL71: u32 = 71;
            #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
            pub const VAL72: u32 = 72;
            #[doc = "SAI0 TX BCLK input is selected"]
            pub const VAL73: u32 = 73;
            #[doc = "SAI0 RX BCLK input is selected"]
            pub const VAL74: u32 = 74;
            #[doc = "SAI1 TX BCLK input is selected"]
            pub const VAL75: u32 = 75;
            #[doc = "SAI1 RX BCLK input is selected"]
            pub const VAL76: u32 = 76;
        }
    }
}
pub mod ctimer {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "Cluster CTIMER%s, containing CTIMER[012]CAP0, CTIMER[012]CAP1, CTIMER[012]CAP2, CTIMER[012]CAP3, TIMER[012]TRIG"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Capture Select Register for CTIMER Inputs"]
        pub CTIMERCAP0: crate::RWRegister<u32>,
        #[doc = "Capture Select Register for CTIMER Inputs"]
        pub CTIMERCAP1: crate::RWRegister<u32>,
        #[doc = "Capture Select Register for CTIMER Inputs"]
        pub CTIMERCAP2: crate::RWRegister<u32>,
        #[doc = "Capture Select Register for CTIMER Inputs"]
        pub CTIMERCAP3: crate::RWRegister<u32>,
        #[doc = "Trigger Register for TIMER"]
        pub TIMERTRIG: crate::RWRegister<u32>,
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    pub mod CTIMERCAP0 {
        #[doc = "Input number for CTIMER0"]
        pub mod INP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "CT_INP0 input is selected"]
                pub const VAL0: u32 = 0;
                #[doc = "CT_INP1 input is selected"]
                pub const VAL1: u32 = 1;
                #[doc = "CT_INP2 input is selected"]
                pub const VAL2: u32 = 2;
                #[doc = "CT_INP3 input is selected"]
                pub const VAL3: u32 = 3;
                #[doc = "CT_INP4 input is selected"]
                pub const VAL4: u32 = 4;
                #[doc = "CT_INP5 input is selected"]
                pub const VAL5: u32 = 5;
                #[doc = "CT_INP6 input is selected"]
                pub const VAL6: u32 = 6;
                #[doc = "CT_INP7 input is selected"]
                pub const VAL7: u32 = 7;
                #[doc = "CT_INP8 input is selected"]
                pub const VAL8: u32 = 8;
                #[doc = "CT_INP9 input is selected"]
                pub const VAL9: u32 = 9;
                #[doc = "CT_INP10 input is selected"]
                pub const VAL10: u32 = 10;
                #[doc = "CT_INP11 input is selected"]
                pub const VAL11: u32 = 11;
                #[doc = "CT_INP12 input is selected"]
                pub const VAL12: u32 = 12;
                #[doc = "CT_INP13 input is selected"]
                pub const VAL13: u32 = 13;
                #[doc = "CT_INP14 input is selected"]
                pub const VAL14: u32 = 14;
                #[doc = "CT_INP15 input is selected"]
                pub const VAL15: u32 = 15;
                #[doc = "CT_INP16 input is selected"]
                pub const VAL16: u32 = 16;
                #[doc = "CT_INP17 input is selected"]
                pub const VAL17: u32 = 17;
                #[doc = "CT_INP18 input is selected"]
                pub const VAL18: u32 = 18;
                #[doc = "CT_INP19 input is selected"]
                pub const VAL19: u32 = 19;
                #[doc = "usb0 start of frame input is selected"]
                pub const VAL20: u32 = 20;
                #[doc = "usb1 start of frame input is selected"]
                pub const VAL21: u32 = 21;
                #[doc = "DCDC_BURST_ACTIVE input is selected"]
                pub const VAL22: u32 = 22;
                #[doc = "sai0_tx_sync_out input is selected"]
                pub const VAL23: u32 = 23;
                #[doc = "sai0_rx_sync_out input is selected"]
                pub const VAL24: u32 = 24;
                #[doc = "ADC0_IRQ input is selected"]
                pub const VAL25: u32 = 25;
                #[doc = "ADC1_IRQ input is selected"]
                pub const VAL26: u32 = 26;
                #[doc = "CMP0_OUT input is selected"]
                pub const VAL27: u32 = 27;
                #[doc = "CMP1_OUT input is selected"]
                pub const VAL28: u32 = 28;
                #[doc = "CMP2_OUT input is selected"]
                pub const VAL29: u32 = 29;
                #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
                pub const VAL30: u32 = 30;
                #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
                pub const VAL31: u32 = 31;
                #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
                pub const VAL32: u32 = 32;
                #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
                pub const VAL33: u32 = 33;
                #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
                pub const VAL34: u32 = 34;
                #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
                pub const VAL35: u32 = 35;
                #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
                pub const VAL36: u32 = 36;
                #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
                pub const VAL37: u32 = 37;
                #[doc = "ENC0_CMP/POS_MATCH input is selected"]
                pub const VAL38: u32 = 38;
                #[doc = "ENC1_CMP/POS_MATCH input is selected"]
                pub const VAL39: u32 = 39;
                #[doc = "EVTG_OUT0A input is selected"]
                pub const VAL40: u32 = 40;
                #[doc = "EVTG_OUT0B input is selected"]
                pub const VAL41: u32 = 41;
                #[doc = "EVTG_OUT1A input is selected"]
                pub const VAL42: u32 = 42;
                #[doc = "EVTG_OUT1B input is selected"]
                pub const VAL43: u32 = 43;
                #[doc = "EVTG_OUT2A input is selected"]
                pub const VAL44: u32 = 44;
                #[doc = "EVTG_OUT2B input is selected"]
                pub const VAL45: u32 = 45;
                #[doc = "EVTG_OUT3A input is selected"]
                pub const VAL46: u32 = 46;
                #[doc = "EVTG_OUT3B input is selected"]
                pub const VAL47: u32 = 47;
                #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
                pub const VAL50: u32 = 50;
                #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
                pub const VAL51: u32 = 51;
                #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
                pub const VAL52: u32 = 52;
                #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
                pub const VAL53: u32 = 53;
                #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
                pub const VAL54: u32 = 54;
                #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
                pub const VAL55: u32 = 55;
                #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
                pub const VAL56: u32 = 56;
                #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
                pub const VAL57: u32 = 57;
                #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
                pub const VAL58: u32 = 58;
                #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
                pub const VAL59: u32 = 59;
                #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
                pub const VAL60: u32 = 60;
                #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
                pub const VAL61: u32 = 61;
                #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
                pub const VAL62: u32 = 62;
                #[doc = "sai1_tx_sync_out input is selected"]
                pub const VAL63: u32 = 63;
                #[doc = "sai1_rx_sync_out input is selected"]
                pub const VAL64: u32 = 64;
            }
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    pub mod CTIMERCAP1 {
        #[doc = "Input number for CTIMER0"]
        pub mod INP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "CT_INP0 input is selected"]
                pub const VAL0: u32 = 0;
                #[doc = "CT_INP1 input is selected"]
                pub const VAL1: u32 = 1;
                #[doc = "CT_INP2 input is selected"]
                pub const VAL2: u32 = 2;
                #[doc = "CT_INP3 input is selected"]
                pub const VAL3: u32 = 3;
                #[doc = "CT_INP4 input is selected"]
                pub const VAL4: u32 = 4;
                #[doc = "CT_INP5 input is selected"]
                pub const VAL5: u32 = 5;
                #[doc = "CT_INP6 input is selected"]
                pub const VAL6: u32 = 6;
                #[doc = "CT_INP7 input is selected"]
                pub const VAL7: u32 = 7;
                #[doc = "CT_INP8 input is selected"]
                pub const VAL8: u32 = 8;
                #[doc = "CT_INP9 input is selected"]
                pub const VAL9: u32 = 9;
                #[doc = "CT_INP10 input is selected"]
                pub const VAL10: u32 = 10;
                #[doc = "CT_INP11 input is selected"]
                pub const VAL11: u32 = 11;
                #[doc = "CT_INP12 input is selected"]
                pub const VAL12: u32 = 12;
                #[doc = "CT_INP13 input is selected"]
                pub const VAL13: u32 = 13;
                #[doc = "CT_INP14 input is selected"]
                pub const VAL14: u32 = 14;
                #[doc = "CT_INP15 input is selected"]
                pub const VAL15: u32 = 15;
                #[doc = "CT_INP16 input is selected"]
                pub const VAL16: u32 = 16;
                #[doc = "CT_INP17 input is selected"]
                pub const VAL17: u32 = 17;
                #[doc = "CT_INP18 input is selected"]
                pub const VAL18: u32 = 18;
                #[doc = "CT_INP19 input is selected"]
                pub const VAL19: u32 = 19;
                #[doc = "usb0 start of frame input is selected"]
                pub const VAL20: u32 = 20;
                #[doc = "usb1 start of frame input is selected"]
                pub const VAL21: u32 = 21;
                #[doc = "DCDC_BURST_ACTIVE input is selected"]
                pub const VAL22: u32 = 22;
                #[doc = "sai0_tx_sync_out input is selected"]
                pub const VAL23: u32 = 23;
                #[doc = "sai0_rx_sync_out input is selected"]
                pub const VAL24: u32 = 24;
                #[doc = "ADC0_IRQ input is selected"]
                pub const VAL25: u32 = 25;
                #[doc = "ADC1_IRQ input is selected"]
                pub const VAL26: u32 = 26;
                #[doc = "CMP0_OUT input is selected"]
                pub const VAL27: u32 = 27;
                #[doc = "CMP1_OUT input is selected"]
                pub const VAL28: u32 = 28;
                #[doc = "CMP2_OUT input is selected"]
                pub const VAL29: u32 = 29;
                #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
                pub const VAL30: u32 = 30;
                #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
                pub const VAL31: u32 = 31;
                #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
                pub const VAL32: u32 = 32;
                #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
                pub const VAL33: u32 = 33;
                #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
                pub const VAL34: u32 = 34;
                #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
                pub const VAL35: u32 = 35;
                #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
                pub const VAL36: u32 = 36;
                #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
                pub const VAL37: u32 = 37;
                #[doc = "ENC0_CMP/POS_MATCH input is selected"]
                pub const VAL38: u32 = 38;
                #[doc = "ENC1_CMP/POS_MATCH input is selected"]
                pub const VAL39: u32 = 39;
                #[doc = "EVTG_OUT0A input is selected"]
                pub const VAL40: u32 = 40;
                #[doc = "EVTG_OUT0B input is selected"]
                pub const VAL41: u32 = 41;
                #[doc = "EVTG_OUT1A input is selected"]
                pub const VAL42: u32 = 42;
                #[doc = "EVTG_OUT1B input is selected"]
                pub const VAL43: u32 = 43;
                #[doc = "EVTG_OUT2A input is selected"]
                pub const VAL44: u32 = 44;
                #[doc = "EVTG_OUT2B input is selected"]
                pub const VAL45: u32 = 45;
                #[doc = "EVTG_OUT3A input is selected"]
                pub const VAL46: u32 = 46;
                #[doc = "EVTG_OUT3B input is selected"]
                pub const VAL47: u32 = 47;
                #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
                pub const VAL50: u32 = 50;
                #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
                pub const VAL51: u32 = 51;
                #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
                pub const VAL52: u32 = 52;
                #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
                pub const VAL53: u32 = 53;
                #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
                pub const VAL54: u32 = 54;
                #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
                pub const VAL55: u32 = 55;
                #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
                pub const VAL56: u32 = 56;
                #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
                pub const VAL57: u32 = 57;
                #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
                pub const VAL58: u32 = 58;
                #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
                pub const VAL59: u32 = 59;
                #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
                pub const VAL60: u32 = 60;
                #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
                pub const VAL61: u32 = 61;
                #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
                pub const VAL62: u32 = 62;
                #[doc = "sai1_tx_sync_out input is selected"]
                pub const VAL63: u32 = 63;
                #[doc = "sai1_rx_sync_out input is selected"]
                pub const VAL64: u32 = 64;
            }
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    pub mod CTIMERCAP2 {
        #[doc = "Input number for CTIMER0"]
        pub mod INP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "CT_INP0 input is selected"]
                pub const VAL0: u32 = 0;
                #[doc = "CT_INP1 input is selected"]
                pub const VAL1: u32 = 1;
                #[doc = "CT_INP2 input is selected"]
                pub const VAL2: u32 = 2;
                #[doc = "CT_INP3 input is selected"]
                pub const VAL3: u32 = 3;
                #[doc = "CT_INP4 input is selected"]
                pub const VAL4: u32 = 4;
                #[doc = "CT_INP5 input is selected"]
                pub const VAL5: u32 = 5;
                #[doc = "CT_INP6 input is selected"]
                pub const VAL6: u32 = 6;
                #[doc = "CT_INP7 input is selected"]
                pub const VAL7: u32 = 7;
                #[doc = "CT_INP8 input is selected"]
                pub const VAL8: u32 = 8;
                #[doc = "CT_INP9 input is selected"]
                pub const VAL9: u32 = 9;
                #[doc = "CT_INP10 input is selected"]
                pub const VAL10: u32 = 10;
                #[doc = "CT_INP11 input is selected"]
                pub const VAL11: u32 = 11;
                #[doc = "CT_INP12 input is selected"]
                pub const VAL12: u32 = 12;
                #[doc = "CT_INP13 input is selected"]
                pub const VAL13: u32 = 13;
                #[doc = "CT_INP14 input is selected"]
                pub const VAL14: u32 = 14;
                #[doc = "CT_INP15 input is selected"]
                pub const VAL15: u32 = 15;
                #[doc = "CT_INP16 input is selected"]
                pub const VAL16: u32 = 16;
                #[doc = "CT_INP17 input is selected"]
                pub const VAL17: u32 = 17;
                #[doc = "CT_INP18 input is selected"]
                pub const VAL18: u32 = 18;
                #[doc = "CT_INP19 input is selected"]
                pub const VAL19: u32 = 19;
                #[doc = "usb0 start of frame input is selected"]
                pub const VAL20: u32 = 20;
                #[doc = "usb1 start of frame input is selected"]
                pub const VAL21: u32 = 21;
                #[doc = "DCDC_BURST_ACTIVE input is selected"]
                pub const VAL22: u32 = 22;
                #[doc = "sai0_tx_sync_out input is selected"]
                pub const VAL23: u32 = 23;
                #[doc = "sai0_rx_sync_out input is selected"]
                pub const VAL24: u32 = 24;
                #[doc = "ADC0_IRQ input is selected"]
                pub const VAL25: u32 = 25;
                #[doc = "ADC1_IRQ input is selected"]
                pub const VAL26: u32 = 26;
                #[doc = "CMP0_OUT input is selected"]
                pub const VAL27: u32 = 27;
                #[doc = "CMP1_OUT input is selected"]
                pub const VAL28: u32 = 28;
                #[doc = "CMP2_OUT input is selected"]
                pub const VAL29: u32 = 29;
                #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
                pub const VAL30: u32 = 30;
                #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
                pub const VAL31: u32 = 31;
                #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
                pub const VAL32: u32 = 32;
                #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
                pub const VAL33: u32 = 33;
                #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
                pub const VAL34: u32 = 34;
                #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
                pub const VAL35: u32 = 35;
                #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
                pub const VAL36: u32 = 36;
                #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
                pub const VAL37: u32 = 37;
                #[doc = "ENC0_CMP/POS_MATCH input is selected"]
                pub const VAL38: u32 = 38;
                #[doc = "ENC1_CMP/POS_MATCH input is selected"]
                pub const VAL39: u32 = 39;
                #[doc = "EVTG_OUT0A input is selected"]
                pub const VAL40: u32 = 40;
                #[doc = "EVTG_OUT0B input is selected"]
                pub const VAL41: u32 = 41;
                #[doc = "EVTG_OUT1A input is selected"]
                pub const VAL42: u32 = 42;
                #[doc = "EVTG_OUT1B input is selected"]
                pub const VAL43: u32 = 43;
                #[doc = "EVTG_OUT2A input is selected"]
                pub const VAL44: u32 = 44;
                #[doc = "EVTG_OUT2B input is selected"]
                pub const VAL45: u32 = 45;
                #[doc = "EVTG_OUT3A input is selected"]
                pub const VAL46: u32 = 46;
                #[doc = "EVTG_OUT3B input is selected"]
                pub const VAL47: u32 = 47;
                #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
                pub const VAL50: u32 = 50;
                #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
                pub const VAL51: u32 = 51;
                #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
                pub const VAL52: u32 = 52;
                #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
                pub const VAL53: u32 = 53;
                #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
                pub const VAL54: u32 = 54;
                #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
                pub const VAL55: u32 = 55;
                #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
                pub const VAL56: u32 = 56;
                #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
                pub const VAL57: u32 = 57;
                #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
                pub const VAL58: u32 = 58;
                #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
                pub const VAL59: u32 = 59;
                #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
                pub const VAL60: u32 = 60;
                #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
                pub const VAL61: u32 = 61;
                #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
                pub const VAL62: u32 = 62;
                #[doc = "sai1_tx_sync_out input is selected"]
                pub const VAL63: u32 = 63;
                #[doc = "sai1_rx_sync_out input is selected"]
                pub const VAL64: u32 = 64;
            }
        }
    }
    #[doc = "Capture Select Register for CTIMER Inputs"]
    pub mod CTIMERCAP3 {
        #[doc = "Input number for CTIMER0"]
        pub mod INP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "CT_INP0 input is selected"]
                pub const VAL0: u32 = 0;
                #[doc = "CT_INP1 input is selected"]
                pub const VAL1: u32 = 1;
                #[doc = "CT_INP2 input is selected"]
                pub const VAL2: u32 = 2;
                #[doc = "CT_INP3 input is selected"]
                pub const VAL3: u32 = 3;
                #[doc = "CT_INP4 input is selected"]
                pub const VAL4: u32 = 4;
                #[doc = "CT_INP5 input is selected"]
                pub const VAL5: u32 = 5;
                #[doc = "CT_INP6 input is selected"]
                pub const VAL6: u32 = 6;
                #[doc = "CT_INP7 input is selected"]
                pub const VAL7: u32 = 7;
                #[doc = "CT_INP8 input is selected"]
                pub const VAL8: u32 = 8;
                #[doc = "CT_INP9 input is selected"]
                pub const VAL9: u32 = 9;
                #[doc = "CT_INP10 input is selected"]
                pub const VAL10: u32 = 10;
                #[doc = "CT_INP11 input is selected"]
                pub const VAL11: u32 = 11;
                #[doc = "CT_INP12 input is selected"]
                pub const VAL12: u32 = 12;
                #[doc = "CT_INP13 input is selected"]
                pub const VAL13: u32 = 13;
                #[doc = "CT_INP14 input is selected"]
                pub const VAL14: u32 = 14;
                #[doc = "CT_INP15 input is selected"]
                pub const VAL15: u32 = 15;
                #[doc = "CT_INP16 input is selected"]
                pub const VAL16: u32 = 16;
                #[doc = "CT_INP17 input is selected"]
                pub const VAL17: u32 = 17;
                #[doc = "CT_INP18 input is selected"]
                pub const VAL18: u32 = 18;
                #[doc = "CT_INP19 input is selected"]
                pub const VAL19: u32 = 19;
                #[doc = "usb0 start of frame input is selected"]
                pub const VAL20: u32 = 20;
                #[doc = "usb1 start of frame input is selected"]
                pub const VAL21: u32 = 21;
                #[doc = "DCDC_BURST_ACTIVE input is selected"]
                pub const VAL22: u32 = 22;
                #[doc = "sai0_tx_sync_out input is selected"]
                pub const VAL23: u32 = 23;
                #[doc = "sai0_rx_sync_out input is selected"]
                pub const VAL24: u32 = 24;
                #[doc = "ADC0_IRQ input is selected"]
                pub const VAL25: u32 = 25;
                #[doc = "ADC1_IRQ input is selected"]
                pub const VAL26: u32 = 26;
                #[doc = "CMP0_OUT input is selected"]
                pub const VAL27: u32 = 27;
                #[doc = "CMP1_OUT input is selected"]
                pub const VAL28: u32 = 28;
                #[doc = "CMP2_OUT input is selected"]
                pub const VAL29: u32 = 29;
                #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
                pub const VAL30: u32 = 30;
                #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
                pub const VAL31: u32 = 31;
                #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
                pub const VAL32: u32 = 32;
                #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
                pub const VAL33: u32 = 33;
                #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
                pub const VAL34: u32 = 34;
                #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
                pub const VAL35: u32 = 35;
                #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
                pub const VAL36: u32 = 36;
                #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
                pub const VAL37: u32 = 37;
                #[doc = "ENC0_CMP/POS_MATCH input is selected"]
                pub const VAL38: u32 = 38;
                #[doc = "ENC1_CMP/POS_MATCH input is selected"]
                pub const VAL39: u32 = 39;
                #[doc = "EVTG_OUT0A input is selected"]
                pub const VAL40: u32 = 40;
                #[doc = "EVTG_OUT0B input is selected"]
                pub const VAL41: u32 = 41;
                #[doc = "EVTG_OUT1A input is selected"]
                pub const VAL42: u32 = 42;
                #[doc = "EVTG_OUT1B input is selected"]
                pub const VAL43: u32 = 43;
                #[doc = "EVTG_OUT2A input is selected"]
                pub const VAL44: u32 = 44;
                #[doc = "EVTG_OUT2B input is selected"]
                pub const VAL45: u32 = 45;
                #[doc = "EVTG_OUT3A input is selected"]
                pub const VAL46: u32 = 46;
                #[doc = "EVTG_OUT3B input is selected"]
                pub const VAL47: u32 = 47;
                #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
                pub const VAL50: u32 = 50;
                #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
                pub const VAL51: u32 = 51;
                #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
                pub const VAL52: u32 = 52;
                #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
                pub const VAL53: u32 = 53;
                #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
                pub const VAL54: u32 = 54;
                #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
                pub const VAL55: u32 = 55;
                #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
                pub const VAL56: u32 = 56;
                #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
                pub const VAL57: u32 = 57;
                #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
                pub const VAL58: u32 = 58;
                #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
                pub const VAL59: u32 = 59;
                #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
                pub const VAL60: u32 = 60;
                #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
                pub const VAL61: u32 = 61;
                #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
                pub const VAL62: u32 = 62;
                #[doc = "sai1_tx_sync_out input is selected"]
                pub const VAL63: u32 = 63;
                #[doc = "sai1_rx_sync_out input is selected"]
                pub const VAL64: u32 = 64;
            }
        }
    }
    #[doc = "Trigger Register for TIMER"]
    pub mod TIMERTRIG {
        #[doc = "Input number for CTIMER0"]
        pub mod INP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "CT_INP0 input is selected"]
                pub const VAL0: u32 = 0;
                #[doc = "CT_INP1 input is selected"]
                pub const VAL1: u32 = 1;
                #[doc = "CT_INP2 input is selected"]
                pub const VAL2: u32 = 2;
                #[doc = "CT_INP3 input is selected"]
                pub const VAL3: u32 = 3;
                #[doc = "CT_INP4 input is selected"]
                pub const VAL4: u32 = 4;
                #[doc = "CT_INP5 input is selected"]
                pub const VAL5: u32 = 5;
                #[doc = "CT_INP6 input is selected"]
                pub const VAL6: u32 = 6;
                #[doc = "CT_INP7 input is selected"]
                pub const VAL7: u32 = 7;
                #[doc = "CT_INP8 input is selected"]
                pub const VAL8: u32 = 8;
                #[doc = "CT_INP9 input is selected"]
                pub const VAL9: u32 = 9;
                #[doc = "CT_INP10 input is selected"]
                pub const VAL10: u32 = 10;
                #[doc = "CT_INP11 input is selected"]
                pub const VAL11: u32 = 11;
                #[doc = "CT_INP12 input is selected"]
                pub const VAL12: u32 = 12;
                #[doc = "CT_INP13 input is selected"]
                pub const VAL13: u32 = 13;
                #[doc = "CT_INP14 input is selected"]
                pub const VAL14: u32 = 14;
                #[doc = "CT_INP15 input is selected"]
                pub const VAL15: u32 = 15;
                #[doc = "CT_INP16 input is selected"]
                pub const VAL16: u32 = 16;
                #[doc = "CT_INP17 input is selected"]
                pub const VAL17: u32 = 17;
                #[doc = "CT_INP18 input is selected"]
                pub const VAL18: u32 = 18;
                #[doc = "CT_INP19 input is selected"]
                pub const VAL19: u32 = 19;
                #[doc = "usb0 start of frame input is selected"]
                pub const VAL20: u32 = 20;
                #[doc = "usb1 start of frame input is selected"]
                pub const VAL21: u32 = 21;
                #[doc = "DCDC_BURST_ACTIVE input is selected"]
                pub const VAL22: u32 = 22;
                #[doc = "sai0_tx_sync_out input is selected"]
                pub const VAL23: u32 = 23;
                #[doc = "sai0_rx_sync_out input is selected"]
                pub const VAL24: u32 = 24;
                #[doc = "ADC0_IRQ input is selected"]
                pub const VAL25: u32 = 25;
                #[doc = "ADC1_IRQ input is selected"]
                pub const VAL26: u32 = 26;
                #[doc = "CMP0_OUT input is selected"]
                pub const VAL27: u32 = 27;
                #[doc = "CMP1_OUT input is selected"]
                pub const VAL28: u32 = 28;
                #[doc = "CMP2_OUT input is selected"]
                pub const VAL29: u32 = 29;
                #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
                pub const VAL30: u32 = 30;
                #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
                pub const VAL31: u32 = 31;
                #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
                pub const VAL32: u32 = 32;
                #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
                pub const VAL33: u32 = 33;
                #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
                pub const VAL34: u32 = 34;
                #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
                pub const VAL35: u32 = 35;
                #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
                pub const VAL36: u32 = 36;
                #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
                pub const VAL37: u32 = 37;
                #[doc = "ENC0_CMP/POS_MATCH input is selected"]
                pub const VAL38: u32 = 38;
                #[doc = "ENC1_CMP/POS_MATCH input is selected"]
                pub const VAL39: u32 = 39;
                #[doc = "EVTG_OUT0A input is selected"]
                pub const VAL40: u32 = 40;
                #[doc = "EVTG_OUT0B input is selected"]
                pub const VAL41: u32 = 41;
                #[doc = "EVTG_OUT1A input is selected"]
                pub const VAL42: u32 = 42;
                #[doc = "EVTG_OUT1B input is selected"]
                pub const VAL43: u32 = 43;
                #[doc = "EVTG_OUT2A input is selected"]
                pub const VAL44: u32 = 44;
                #[doc = "EVTG_OUT2B input is selected"]
                pub const VAL45: u32 = 45;
                #[doc = "EVTG_OUT3A input is selected"]
                pub const VAL46: u32 = 46;
                #[doc = "EVTG_OUT3B input is selected"]
                pub const VAL47: u32 = 47;
                #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
                pub const VAL50: u32 = 50;
                #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
                pub const VAL51: u32 = 51;
                #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
                pub const VAL52: u32 = 52;
                #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
                pub const VAL53: u32 = 53;
                #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
                pub const VAL54: u32 = 54;
                #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
                pub const VAL55: u32 = 55;
                #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
                pub const VAL56: u32 = 56;
                #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
                pub const VAL57: u32 = 57;
                #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
                pub const VAL58: u32 = 58;
                #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
                pub const VAL59: u32 = 59;
                #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
                pub const VAL60: u32 = 60;
                #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
                pub const VAL61: u32 = 61;
                #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
                pub const VAL62: u32 = 62;
                #[doc = "sai1_tx_sync_out input is selected"]
                pub const VAL63: u32 = 63;
                #[doc = "sai1_rx_sync_out input is selected"]
                pub const VAL64: u32 = 64;
            }
        }
    }
}
#[doc = "Inputmux Register for SMARTDMA Arch B Inputs"]
pub mod SMARTDMAARCHB_INMUX {
    #[doc = "Input number select to SmartDMA ARCHB input"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPIO P0_0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "GPIO P0_1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "GPIO P0_2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "GPIO P0_3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "GPIO P0_4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "GPIO P0_5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "GPIO P0_6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "GPIO P0_7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "GPIO P0_8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "GPIO P0_9 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "GPIO P0_10 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "GPIO P0_11 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "GPIO P0_12 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "GPIO P0_13 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "GPIO P0_14 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "GPIO P0_15 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "SCT0 SCT_OUT8 input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "SCT0 SCT_OUT9 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "MRT0 MRT_CH0_IRQ input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "MRT0 MRT_CH1_IRQ input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "CTIMER4_MAT3 input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "CTIMER4_MAT2 input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "CTIMER3_MAT3 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "CTIMER3_MAT2 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "CTIMER1_MAT2 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "UTICK0 UTICK_IRQ input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "WWDT0 WDT0_IRQ input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "ADC0 ADC0_IRQ input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "CMP0_IRQ input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "LPSPI LSPI_HS_IRQ input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "LP_FLEXCOMM7_IRQ input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "LP_FLEXCOMM6_IRQ input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "LP_FLEXCOMM5_IRQ input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "LP_FLEXCOMM4_IRQ input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "LP_FLEXCOMM3_IRQ input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "LP_FLEXCOMM2_IRQ input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "LP_FLEXCOMM1_IRQ input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "LP_FLEXCOMM0_IRQ input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "DMA0_IRQ input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "DMA1_IRQ input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "SYS_IRQ input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "RTC_COMBO_IRQ input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "PINT GPIO_INT_BMATCH input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "usb0 start of frame input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "usb1 start of frame input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "OSTIMER0 OS_EVENT_TIMER_IRQ input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "ADC1_IRQ input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "CMP0_IRQ/CMP1_IRQ/CMP2_IRQ input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "DAC0_IRQ input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "DAC1_IRQ/DAC2_IRQ input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "PWM0_IRQ input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "PWM1_IRQ input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "ENC0_IRQ input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "ENC1_IRQ input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "GPIO1_alias0 GPIO1 Pin Event Trig 0 input is selected"]
            pub const VAL65: u32 = 65;
            #[doc = "GPIO1_alias1 GPIO1 Pin Event Trig 1 input is selected"]
            pub const VAL66: u32 = 66;
            #[doc = "GPIO2_alias0 GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL67: u32 = 67;
            #[doc = "GPIO2_alias1 GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL68: u32 = 68;
            #[doc = "GPIO3_alias0 GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL69: u32 = 69;
            #[doc = "GPIO3_alias1 GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL70: u32 = 70;
        }
    }
}
#[doc = "Pin Interrupt Select"]
pub mod PINTSEL {
    #[doc = "Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INP = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPIO P0_0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "GPIO P0_1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "GPIO P0_2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "GPIO P0_3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "GPIO P0_4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "GPIO P0_5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "GPIO P0_6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "GPIO P0_7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "GPIO P0_8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "GPIO P0_9 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "GPIO P0_10 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "GPIO P0_11 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "GPIO P0_12 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "GPIO P0_13 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "GPIO P0_14 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "GPIO P0_15 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "GPIO P0_16 input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "GPIO P0_17 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "GPIO P0_18 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "GPIO P0_19 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "GPIO P0_20 input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "GPIO P0_21 input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "GPIO P0_22 input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "GPIO P0_23 input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "GPIO P0_24 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "GPIO P0_25 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "GPIO P0_26 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "GPIO P0_27 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "GPIO P0_28 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "GPIO P0_29 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "GPIO P0_30 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "GPIO P0_31 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "GPIO P1_0 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "GPIO P1_1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "GPIO P1_2 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "GPIO P1_3 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "GPIO P1_4 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "GPIO P1_5 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "GPIO P1_6 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "GPIO P1_7 input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "GPIO P1_8 input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "GPIO P1_9 input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "GPIO P1_10 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "GPIO P1_11 input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "GPIO P1_12 input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "GPIO P1_13 input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "GPIO P1_14 input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "GPIO P1_15 input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "GPIO P1_16 input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "GPIO P1_17 input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "GPIO P1_18 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "GPIO P1_19 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "GPIO P1_20 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "GPIO P1_21 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "GPIO P1_22 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "GPIO P1_23 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "GPIO P1_30 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "GPIO P1_31 input is selected"]
            pub const VAL63: u32 = 63;
        }
    }
}
#[doc = "Selection for Frequency Measurement Reference Clock"]
pub mod FREQMEAS_REF {
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "clk_in (output of clk_in or XTAL mux in Clockgen) input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "FRO_12M input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "FRO_144M input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "OSC_32K input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CPU/system_clk input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "FREQME_CLK_IN0 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "FREQME_CLK_IN1 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL9: u32 = 9;
        }
    }
}
#[doc = "Selection for Frequency Measurement Target Clock"]
pub mod FREQMEAS_TAR {
    #[doc = "Clock source number (binary value) for frequency measure function target clock."]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "clk_in (output of clk_in or XTAL mux in Clockgen) input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "FRO_12M input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "FRO_144M input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "OSC_32K input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CPU/system_clk input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "FREQME_CLK_IN0 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "FREQME_CLK_IN1 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL9: u32 = 9;
        }
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
pub mod CTIMER3CAP0 {
    #[doc = "Input number for CTIMER0"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CT_INP0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "CT_INP1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "CT_INP2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "CT_INP3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "CT_INP4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CT_INP5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CT_INP6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CT_INP7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CT_INP8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CT_INP9 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CT_INP10 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CT_INP11 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "CT_INP12 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "CT_INP13 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "CT_INP14 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "CT_INP15 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "CT_INP16 input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "CT_INP17 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "CT_INP18 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "CT_INP19 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "usb0 start of frame input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "usb1 start of frame input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "DCDC_BURST_ACTIVE input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "sai0_tx_sync_out input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "sai0_rx_sync_out input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ADC0 ADC0_IRQ input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ADC0 ADC1_IRQ input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "sai1_tx_sync_out input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "sai1_rx_sync_out input is selected"]
            pub const VAL64: u32 = 64;
        }
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
pub mod CTIMER3CAP1 {
    #[doc = "Input number for CTIMER0"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CT_INP0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "CT_INP1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "CT_INP2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "CT_INP3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "CT_INP4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CT_INP5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CT_INP6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CT_INP7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CT_INP8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CT_INP9 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CT_INP10 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CT_INP11 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "CT_INP12 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "CT_INP13 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "CT_INP14 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "CT_INP15 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "CT_INP16 input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "CT_INP17 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "CT_INP18 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "CT_INP19 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "usb0 start of frame input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "usb1 start of frame input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "DCDC_BURST_ACTIVE input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "sai0_tx_sync_out input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "sai0_rx_sync_out input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ADC0 ADC0_IRQ input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ADC0 ADC1_IRQ input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "sai1_tx_sync_out input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "sai1_rx_sync_out input is selected"]
            pub const VAL64: u32 = 64;
        }
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
pub mod CTIMER3CAP2 {
    #[doc = "Input number for CTIMER0"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CT_INP0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "CT_INP1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "CT_INP2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "CT_INP3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "CT_INP4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CT_INP5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CT_INP6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CT_INP7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CT_INP8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CT_INP9 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CT_INP10 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CT_INP11 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "CT_INP12 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "CT_INP13 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "CT_INP14 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "CT_INP15 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "CT_INP16 input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "CT_INP17 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "CT_INP18 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "CT_INP19 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "usb0 start of frame input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "usb1 start of frame input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "DCDC_BURST_ACTIVE input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "sai0_tx_sync_out input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "sai0_rx_sync_out input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ADC0 ADC0_IRQ input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ADC0 ADC1_IRQ input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "sai1_tx_sync_out input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "sai1_rx_sync_out input is selected"]
            pub const VAL64: u32 = 64;
        }
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
pub mod CTIMER3CAP3 {
    #[doc = "Input number for CTIMER0"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CT_INP0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "CT_INP1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "CT_INP2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "CT_INP3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "CT_INP4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CT_INP5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CT_INP6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CT_INP7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CT_INP8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CT_INP9 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CT_INP10 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CT_INP11 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "CT_INP12 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "CT_INP13 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "CT_INP14 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "CT_INP15 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "CT_INP16 input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "CT_INP17 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "CT_INP18 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "CT_INP19 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "usb0 start of frame input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "usb1 start of frame input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "DCDC_BURST_ACTIVE input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "sai0_tx_sync_out input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "sai0_rx_sync_out input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ADC0 ADC0_IRQ input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ADC0 ADC1_IRQ input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "sai1_tx_sync_out input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "sai1_rx_sync_out input is selected"]
            pub const VAL64: u32 = 64;
        }
    }
}
#[doc = "Trigger Register for TIMER"]
pub mod TIMER3TRIG {
    #[doc = "Input number for CTIMER0"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CT_INP0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "CT_INP1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "CT_INP2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "CT_INP3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "CT_INP4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CT_INP5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CT_INP6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CT_INP7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CT_INP8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CT_INP9 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CT_INP10 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CT_INP11 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "CT_INP12 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "CT_INP13 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "CT_INP14 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "CT_INP15 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "CT_INP16 input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "CT_INP17 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "CT_INP18 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "CT_INP19 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "usb0 start of frame input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "usb1 start of frame input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "DCDC_BURST_ACTIVE input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "sai0_tx_sync_out input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "sai0_rx_sync_out input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ADC0 ADC0_IRQ input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ADC0 ADC1_IRQ input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "sai1_tx_sync_out input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "sai1_rx_sync_out input is selected"]
            pub const VAL64: u32 = 64;
        }
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
pub mod CTIMER4CAP0 {
    #[doc = "Input number for CTIMER0"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CT_INP0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "CT_INP1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "CT_INP2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "CT_INP3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "CT_INP4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CT_INP5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CT_INP6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CT_INP7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CT_INP8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CT_INP9 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CT_INP10 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CT_INP11 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "CT_INP12 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "CT_INP13 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "CT_INP14 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "CT_INP15 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "CT_INP16 input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "CT_INP17 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "CT_INP18 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "CT_INP19 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "usb0 start of frame input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "usb1 start of frame input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "DCDC_BURST_ACTIVE input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "sai0_tx_sync_out input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "sai0_rx_sync_out input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ADC0 ADC0_IRQ input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ADC0 ADC1_IRQ input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "sai1_tx_sync_out input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "sai1_rx_sync_out input is selected"]
            pub const VAL64: u32 = 64;
        }
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
pub mod CTIMER4CAP1 {
    #[doc = "Input number for CTIMER0"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CT_INP0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "CT_INP1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "CT_INP2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "CT_INP3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "CT_INP4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CT_INP5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CT_INP6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CT_INP7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CT_INP8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CT_INP9 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CT_INP10 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CT_INP11 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "CT_INP12 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "CT_INP13 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "CT_INP14 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "CT_INP15 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "CT_INP16 input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "CT_INP17 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "CT_INP18 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "CT_INP19 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "usb0 start of frame input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "usb1 start of frame input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "DCDC_BURST_ACTIVE input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "sai0_tx_sync_out input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "sai0_rx_sync_out input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ADC0 ADC0_IRQ input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ADC0 ADC1_IRQ input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "sai1_tx_sync_out input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "sai1_rx_sync_out input is selected"]
            pub const VAL64: u32 = 64;
        }
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
pub mod CTIMER4CAP2 {
    #[doc = "Input number for CTIMER0"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CT_INP0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "CT_INP1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "CT_INP2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "CT_INP3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "CT_INP4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CT_INP5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CT_INP6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CT_INP7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CT_INP8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CT_INP9 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CT_INP10 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CT_INP11 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "CT_INP12 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "CT_INP13 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "CT_INP14 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "CT_INP15 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "CT_INP16 input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "CT_INP17 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "CT_INP18 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "CT_INP19 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "usb0 start of frame input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "usb1 start of frame input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "DCDC_BURST_ACTIVE input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "sai0_tx_sync_out input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "sai0_rx_sync_out input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ADC0 ADC0_IRQ input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ADC0 ADC1_IRQ input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "sai1_tx_sync_out input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "sai1_rx_sync_out input is selected"]
            pub const VAL64: u32 = 64;
        }
    }
}
#[doc = "Capture Select Register for CTIMER Inputs"]
pub mod CTIMER4CAP3 {
    #[doc = "Input number for CTIMER0"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CT_INP0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "CT_INP1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "CT_INP2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "CT_INP3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "CT_INP4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CT_INP5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CT_INP6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CT_INP7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CT_INP8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CT_INP9 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CT_INP10 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CT_INP11 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "CT_INP12 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "CT_INP13 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "CT_INP14 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "CT_INP15 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "CT_INP16 input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "CT_INP17 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "CT_INP18 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "CT_INP19 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "usb0 start of frame input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "usb1 start of frame input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "DCDC_BURST_ACTIVE input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "sai0_tx_sync_out input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "sai0_rx_sync_out input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ADC0 ADC0_IRQ input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ADC0 ADC1_IRQ input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "sai1_tx_sync_out input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "sai1_rx_sync_out input is selected"]
            pub const VAL64: u32 = 64;
        }
    }
}
#[doc = "Trigger Register for TIMER"]
pub mod TIMER4TRIG {
    #[doc = "Input number for CTIMER0"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CT_INP0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "CT_INP1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "CT_INP2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "CT_INP3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "CT_INP4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CT_INP5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CT_INP6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CT_INP7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CT_INP8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CT_INP9 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CT_INP10 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CT_INP11 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "CT_INP12 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "CT_INP13 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "CT_INP14 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "CT_INP15 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "CT_INP16 input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "CT_INP17 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "CT_INP18 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "CT_INP19 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "usb0 start of frame input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "usb1 start of frame input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "DCDC_BURST_ACTIVE input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "sai0_tx_sync_out input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "sai0_rx_sync_out input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ADC0 ADC0_IRQ input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ADC0 ADC1_IRQ input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "LP_FLEXCOMM0 trig 0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LP_FLEXCOMM0 trig 1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "LP_FLEXCOMM0 trig 2 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "sai1_tx_sync_out input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "sai1_rx_sync_out input is selected"]
            pub const VAL64: u32 = 64;
        }
    }
}
#[doc = "CMP0 Input Connections"]
pub mod CMP0_TRIG {
    #[doc = "CMP0 input trigger"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT6 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT6 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER0_MAT0 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT0 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "LPTMR0 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "LPTMR1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL40: u32 = 40;
        }
    }
}
#[doc = "ADC Trigger Input Connections"]
pub mod ADC0_TRIG {
    #[doc = "ADC0 trigger inputs"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT0 SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT0 SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT0 SCT_OUT9 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER3_MAT3 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT3 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "DCDC_Burst_Done_Trig input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM0_A0_TRIG0 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "PWM0_A0_TRIG1 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "PWM0_A1_TRIG0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM0_A1_TRIG1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM0_A2_TRIG0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM0_A2_TRIG1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A3_TRIG0 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A3_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM1_A0_TRIG0 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM1_A0_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A1_TRIG0 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A2_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "PWM1_A3_TRIG0 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "PWM1_A3_TRIG1 input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "LPTMR0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LPTMR1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "FlexIO CH0 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "FlexIO CH1 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "FlexIO CH2 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "FlexIO CH3 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "SINC Filter CH0 Conversion Complete input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "SINC Filter CH1 Conversion Complete input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "SINC Filter CH2 Conversion Complete input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "SINC Filter CH3 Conversion Complete input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "SINC Filter CH4 Conversion Complete input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL64: u32 = 64;
            #[doc = "WUU input is selected"]
            pub const VAL65: u32 = 65;
        }
    }
}
#[doc = "ADC Trigger Input Connections"]
pub mod ADC1_TRIG {
    #[doc = "ADC0 trigger inputs"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT2 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT0 SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT0 SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT0 SCT_OUT3 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER3_MAT2 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT1 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "DCDC_Burst_Done_Trig input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM0_A0_TRIG0 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "PWM0_A0_TRIG1 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "PWM0_A1_TRIG0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM0_A1_TRIG1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM0_A2_TRIG0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM0_A2_TRIG1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A3_TRIG0 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A3_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM1_A0_TRIG0 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM1_A0_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A1_TRIG0 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A2_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "PWM1_A3_TRIG0 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "PWM1_A3_TRIG1 input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "LPTMR0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LPTMR1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "FlexIO CH0 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "FlexIO CH1 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "FlexIO CH2 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "FlexIO CH3 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "SINC Filter CH0 Conversion Complete input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "SINC Filter CH1 Conversion Complete input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "SINC Filter CH2 Conversion Complete input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "SINC Filter CH3 Conversion Complete input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "SINC Filter CH4 Conversion Complete input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL64: u32 = 64;
            #[doc = "WUU input is selected"]
            pub const VAL65: u32 = 65;
        }
    }
}
#[doc = "DAC%s Trigger Inputs"]
pub mod DAC__TRIG {
    #[doc = "DAC0 trigger input"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT3 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT0 SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT0 SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT0 SCT_OUT0 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER2_MAT0 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER3_MAT0 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "LPTMR0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "LPTMR1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL31: u32 = 31;
        }
    }
}
pub mod enc {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "Cluster ENC%s, containing ENC?_TRIG, ENC?_HOME, ENC?_INDEX, ENC?_PHASEB, ENC?_PHASEA"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "ENC0 Trigger Input Connections"]
        pub ENC_TRIG: crate::RWRegister<u32>,
        #[doc = "ENC0 Input Connections"]
        pub ENC_HOME: crate::RWRegister<u32>,
        #[doc = "ENC0 Input Connections"]
        pub ENC_INDEX: crate::RWRegister<u32>,
        #[doc = "ENC0 Input Connections"]
        pub ENC_PHASEB: crate::RWRegister<u32>,
        #[doc = "ENC0 Input Connections"]
        pub ENC_PHASEA: crate::RWRegister<u32>,
    }
    #[doc = "ENC0 Trigger Input Connections"]
    pub mod ENC_TRIG {
        #[doc = "ENC0 input connections"]
        pub mod INP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PINT PIN_INT0 input is selected"]
                pub const VAL0: u32 = 0;
                #[doc = "PINT PIN_INT4 input is selected"]
                pub const VAL1: u32 = 1;
                #[doc = "SCT_OUT4 input is selected"]
                pub const VAL2: u32 = 2;
                #[doc = "SCT_OUT5 input is selected"]
                pub const VAL3: u32 = 3;
                #[doc = "SCT_OUT1 input is selected"]
                pub const VAL4: u32 = 4;
                #[doc = "CTIMER0_MAT3 input is selected"]
                pub const VAL5: u32 = 5;
                #[doc = "CTIMER1_MAT3 input is selected"]
                pub const VAL6: u32 = 6;
                #[doc = "CTIMER2_MAT3 input is selected"]
                pub const VAL7: u32 = 7;
                #[doc = "CTIMER1_MAT0 input is selected"]
                pub const VAL8: u32 = 8;
                #[doc = "CTIMER3_MAT0 input is selected"]
                pub const VAL9: u32 = 9;
                #[doc = "ARM_TXEV input is selected"]
                pub const VAL11: u32 = 11;
                #[doc = "PINT GPIO_INT_BMAT input is selected"]
                pub const VAL12: u32 = 12;
                #[doc = "ADC0_tcomp[0] input is selected"]
                pub const VAL13: u32 = 13;
                #[doc = "ADC0_tcomp[1] input is selected"]
                pub const VAL14: u32 = 14;
                #[doc = "ADC0_tcomp[2] input is selected"]
                pub const VAL15: u32 = 15;
                #[doc = "ADC0_tcomp[3] input is selected"]
                pub const VAL16: u32 = 16;
                #[doc = "ADC1_tcomp[0] input is selected"]
                pub const VAL17: u32 = 17;
                #[doc = "ADC1_tcomp[1] input is selected"]
                pub const VAL18: u32 = 18;
                #[doc = "ADC1_tcomp[2] input is selected"]
                pub const VAL19: u32 = 19;
                #[doc = "ADC1_tcomp[3] input is selected"]
                pub const VAL20: u32 = 20;
                #[doc = "CMP0_OUT input is selected"]
                pub const VAL21: u32 = 21;
                #[doc = "CMP1_OUT input is selected"]
                pub const VAL22: u32 = 22;
                #[doc = "CMP2_OUT input is selected"]
                pub const VAL23: u32 = 23;
                #[doc = "PWM1_A0_TRIG0 input is selected"]
                pub const VAL24: u32 = 24;
                #[doc = "PWM1_A0_TRIG1 input is selected"]
                pub const VAL25: u32 = 25;
                #[doc = "PWM1_A1_TRIG0 input is selected"]
                pub const VAL26: u32 = 26;
                #[doc = "PWM1_A1_TRIG1 input is selected"]
                pub const VAL27: u32 = 27;
                #[doc = "PWM1_A2_TRIG0 input is selected"]
                pub const VAL28: u32 = 28;
                #[doc = "PWM1_A2_TRIG1 input is selected"]
                pub const VAL29: u32 = 29;
                #[doc = "PWM1_A3_TRIG0 input is selected"]
                pub const VAL30: u32 = 30;
                #[doc = "PWM1_A3_TRIG1 input is selected"]
                pub const VAL31: u32 = 31;
                #[doc = "ENC0_CMP/POS_MATCH input is selected"]
                pub const VAL32: u32 = 32;
                #[doc = "ENC1_CMP/POS_MATCH input is selected"]
                pub const VAL33: u32 = 33;
                #[doc = "EVTG_OUT0A input is selected"]
                pub const VAL34: u32 = 34;
                #[doc = "EVTG_OUT0B input is selected"]
                pub const VAL35: u32 = 35;
                #[doc = "EVTG_OUT1A input is selected"]
                pub const VAL36: u32 = 36;
                #[doc = "EVTG_OUT1B input is selected"]
                pub const VAL37: u32 = 37;
                #[doc = "EVTG_OUT2A input is selected"]
                pub const VAL38: u32 = 38;
                #[doc = "EVTG_OUT2B input is selected"]
                pub const VAL39: u32 = 39;
                #[doc = "EVTG_OUT3A input is selected"]
                pub const VAL40: u32 = 40;
                #[doc = "EVTG_OUT3B input is selected"]
                pub const VAL41: u32 = 41;
                #[doc = "TRIG_IN0 input is selected"]
                pub const VAL42: u32 = 42;
                #[doc = "TRIG_IN1 input is selected"]
                pub const VAL43: u32 = 43;
                #[doc = "TRIG_IN2 input is selected"]
                pub const VAL44: u32 = 44;
                #[doc = "TRIG_IN3 input is selected"]
                pub const VAL45: u32 = 45;
                #[doc = "TRIG_IN4 input is selected"]
                pub const VAL46: u32 = 46;
                #[doc = "TRIG_IN5 input is selected"]
                pub const VAL47: u32 = 47;
                #[doc = "TRIG_IN6 input is selected"]
                pub const VAL48: u32 = 48;
                #[doc = "TRIG_IN7 input is selected"]
                pub const VAL49: u32 = 49;
                #[doc = "TRIG_IN8 input is selected"]
                pub const VAL50: u32 = 50;
                #[doc = "TRIG_IN9 input is selected"]
                pub const VAL51: u32 = 51;
            }
        }
    }
    #[doc = "ENC0 Input Connections"]
    pub mod ENC_HOME {
        #[doc = "ENC0 input connections"]
        pub mod INP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PINT PIN_INT0 input is selected"]
                pub const VAL0: u32 = 0;
                #[doc = "PINT PIN_INT4 input is selected"]
                pub const VAL1: u32 = 1;
                #[doc = "SCT0 SCT_OUT4 input is selected"]
                pub const VAL2: u32 = 2;
                #[doc = "SCT0 SCT_OUT5 input is selected"]
                pub const VAL3: u32 = 3;
                #[doc = "SCT0 SCT_OUT1 input is selected"]
                pub const VAL4: u32 = 4;
                #[doc = "CTIMER0_MAT3 input is selected"]
                pub const VAL5: u32 = 5;
                #[doc = "CTIMER1_MAT3 input is selected"]
                pub const VAL6: u32 = 6;
                #[doc = "CTIMER2_MAT3 input is selected"]
                pub const VAL7: u32 = 7;
                #[doc = "CTIMER1_MAT0 input is selected"]
                pub const VAL8: u32 = 8;
                #[doc = "CTIMER3_MAT0 input is selected"]
                pub const VAL9: u32 = 9;
                #[doc = "ARM_TXEV input is selected"]
                pub const VAL11: u32 = 11;
                #[doc = "PINT GPIO_INT_BMAT input is selected"]
                pub const VAL12: u32 = 12;
                #[doc = "ADC0_tcomp[0] input is selected"]
                pub const VAL13: u32 = 13;
                #[doc = "ADC0_tcomp[1] input is selected"]
                pub const VAL14: u32 = 14;
                #[doc = "ADC0_tcomp[2] input is selected"]
                pub const VAL15: u32 = 15;
                #[doc = "ADC0_tcomp[3] input is selected"]
                pub const VAL16: u32 = 16;
                #[doc = "ADC1_tcomp[0] input is selected"]
                pub const VAL17: u32 = 17;
                #[doc = "ADC1_tcomp[1] input is selected"]
                pub const VAL18: u32 = 18;
                #[doc = "ADC1_tcomp[2] input is selected"]
                pub const VAL19: u32 = 19;
                #[doc = "ADC1_tcomp[3] input is selected"]
                pub const VAL20: u32 = 20;
                #[doc = "CMP0_OUT input is selected"]
                pub const VAL21: u32 = 21;
                #[doc = "CMP1_OUT input is selected"]
                pub const VAL22: u32 = 22;
                #[doc = "CMP2_OUT input is selected"]
                pub const VAL23: u32 = 23;
                #[doc = "PWM1_A0_TRIG0 input is selected"]
                pub const VAL24: u32 = 24;
                #[doc = "PWM1_A0_TRIG1 input is selected"]
                pub const VAL25: u32 = 25;
                #[doc = "PWM1_A1_TRIG0 input is selected"]
                pub const VAL26: u32 = 26;
                #[doc = "PWM1_A1_TRIG1 input is selected"]
                pub const VAL27: u32 = 27;
                #[doc = "PWM1_A2_TRIG0 input is selected"]
                pub const VAL28: u32 = 28;
                #[doc = "PWM1_A2_TRIG1 input is selected"]
                pub const VAL29: u32 = 29;
                #[doc = "PWM1_A3_TRIG0 input is selected"]
                pub const VAL30: u32 = 30;
                #[doc = "PWM1_A3_TRIG1 input is selected"]
                pub const VAL31: u32 = 31;
                #[doc = "ENC0_CMP/POS_MATCH input is selected"]
                pub const VAL32: u32 = 32;
                #[doc = "ENC1_CMP/POS_MATCH input is selected"]
                pub const VAL33: u32 = 33;
                #[doc = "EVTG_OUT0A input is selected"]
                pub const VAL34: u32 = 34;
                #[doc = "EVTG_OUT0B input is selected"]
                pub const VAL35: u32 = 35;
                #[doc = "EVTG_OUT1A input is selected"]
                pub const VAL36: u32 = 36;
                #[doc = "EVTG_OUT1B input is selected"]
                pub const VAL37: u32 = 37;
                #[doc = "EVTG_OUT2A input is selected"]
                pub const VAL38: u32 = 38;
                #[doc = "EVTG_OUT2B input is selected"]
                pub const VAL39: u32 = 39;
                #[doc = "EVTG_OUT3A input is selected"]
                pub const VAL40: u32 = 40;
                #[doc = "EVTG_OUT3B input is selected"]
                pub const VAL41: u32 = 41;
                #[doc = "TRIG_IN0 input is selected"]
                pub const VAL42: u32 = 42;
                #[doc = "TRIG_IN1 input is selected"]
                pub const VAL43: u32 = 43;
                #[doc = "TRIG_IN2 input is selected"]
                pub const VAL44: u32 = 44;
                #[doc = "TRIG_IN3 input is selected"]
                pub const VAL45: u32 = 45;
                #[doc = "TRIG_IN4 input is selected"]
                pub const VAL46: u32 = 46;
                #[doc = "TRIG_IN5 input is selected"]
                pub const VAL47: u32 = 47;
                #[doc = "TRIG_IN6 input is selected"]
                pub const VAL48: u32 = 48;
                #[doc = "TRIG_IN7 input is selected"]
                pub const VAL49: u32 = 49;
                #[doc = "TRIG_IN8 input is selected"]
                pub const VAL50: u32 = 50;
                #[doc = "TRIG_IN9 input is selected"]
                pub const VAL51: u32 = 51;
            }
        }
    }
    #[doc = "ENC0 Input Connections"]
    pub mod ENC_INDEX {
        #[doc = "ENC0 input connections"]
        pub mod INP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PINT PIN_INT0 input is selected"]
                pub const VAL0: u32 = 0;
                #[doc = "PINT PIN_INT4 input is selected"]
                pub const VAL1: u32 = 1;
                #[doc = "SCT_OUT4 input is selected"]
                pub const VAL2: u32 = 2;
                #[doc = "SCT_OUT5 input is selected"]
                pub const VAL3: u32 = 3;
                #[doc = "SCT_OUT1 input is selected"]
                pub const VAL4: u32 = 4;
                #[doc = "CTIMER0_MAT3 input is selected"]
                pub const VAL5: u32 = 5;
                #[doc = "CTIMER1_MAT3 input is selected"]
                pub const VAL6: u32 = 6;
                #[doc = "CTIMER2_MAT3 input is selected"]
                pub const VAL7: u32 = 7;
                #[doc = "CTIMER1_MAT0 input is selected"]
                pub const VAL8: u32 = 8;
                #[doc = "CTIMER3_MAT0 input is selected"]
                pub const VAL9: u32 = 9;
                #[doc = "ARM_TXEV input is selected"]
                pub const VAL11: u32 = 11;
                #[doc = "PINT GPIO_INT_BMAT input is selected"]
                pub const VAL12: u32 = 12;
                #[doc = "ADC0_tcomp[0] input is selected"]
                pub const VAL13: u32 = 13;
                #[doc = "ADC0_tcomp[1] input is selected"]
                pub const VAL14: u32 = 14;
                #[doc = "ADC0_tcomp[2] input is selected"]
                pub const VAL15: u32 = 15;
                #[doc = "ADC0_tcomp[3] input is selected"]
                pub const VAL16: u32 = 16;
                #[doc = "ADC1_tcomp[0] input is selected"]
                pub const VAL17: u32 = 17;
                #[doc = "ADC1_tcomp[1] input is selected"]
                pub const VAL18: u32 = 18;
                #[doc = "ADC1_tcomp[2] input is selected"]
                pub const VAL19: u32 = 19;
                #[doc = "ADC1_tcomp[3] input is selected"]
                pub const VAL20: u32 = 20;
                #[doc = "CMP0_OUT input is selected"]
                pub const VAL21: u32 = 21;
                #[doc = "CMP1_OUT input is selected"]
                pub const VAL22: u32 = 22;
                #[doc = "CMP2_OUT input is selected"]
                pub const VAL23: u32 = 23;
                #[doc = "PWM1_A0_TRIG0 input is selected"]
                pub const VAL24: u32 = 24;
                #[doc = "PWM1_A0_TRIG1 input is selected"]
                pub const VAL25: u32 = 25;
                #[doc = "PWM1_A1_TRIG0 input is selected"]
                pub const VAL26: u32 = 26;
                #[doc = "PWM1_A1_TRIG1 input is selected"]
                pub const VAL27: u32 = 27;
                #[doc = "PWM1_A2_TRIG0 input is selected"]
                pub const VAL28: u32 = 28;
                #[doc = "PWM1_A2_TRIG1 input is selected"]
                pub const VAL29: u32 = 29;
                #[doc = "PWM1_A3_TRIG0 input is selected"]
                pub const VAL30: u32 = 30;
                #[doc = "PWM1_A3_TRIG1 input is selected"]
                pub const VAL31: u32 = 31;
                #[doc = "ENC0_CMP/POS_MATCH input is selected"]
                pub const VAL32: u32 = 32;
                #[doc = "ENC1_CMP/POS_MATCH input is selected"]
                pub const VAL33: u32 = 33;
                #[doc = "EVTG_OUT0A input is selected"]
                pub const VAL34: u32 = 34;
                #[doc = "EVTG_OUT0B input is selected"]
                pub const VAL35: u32 = 35;
                #[doc = "EVTG_OUT1A input is selected"]
                pub const VAL36: u32 = 36;
                #[doc = "EVTG_OUT1B input is selected"]
                pub const VAL37: u32 = 37;
                #[doc = "EVTG_OUT2A input is selected"]
                pub const VAL38: u32 = 38;
                #[doc = "EVTG_OUT2B input is selected"]
                pub const VAL39: u32 = 39;
                #[doc = "EVTG_OUT3A input is selected"]
                pub const VAL40: u32 = 40;
                #[doc = "EVTG_OUT3B input is selected"]
                pub const VAL41: u32 = 41;
                #[doc = "TRIG_IN0 input is selected"]
                pub const VAL42: u32 = 42;
                #[doc = "TRIG_IN1 input is selected"]
                pub const VAL43: u32 = 43;
                #[doc = "TRIG_IN2 input is selected"]
                pub const VAL44: u32 = 44;
                #[doc = "TRIG_IN3 input is selected"]
                pub const VAL45: u32 = 45;
                #[doc = "TRIG_IN4 input is selected"]
                pub const VAL46: u32 = 46;
                #[doc = "TRIG_IN5 input is selected"]
                pub const VAL47: u32 = 47;
                #[doc = "TRIG_IN6 input is selected"]
                pub const VAL48: u32 = 48;
                #[doc = "TRIG_IN7 input is selected"]
                pub const VAL49: u32 = 49;
                #[doc = "TRIG_IN8 input is selected"]
                pub const VAL50: u32 = 50;
                #[doc = "TRIG_IN9 input is selected"]
                pub const VAL51: u32 = 51;
            }
        }
    }
    #[doc = "ENC0 Input Connections"]
    pub mod ENC_PHASEB {
        #[doc = "ENC0 input connections"]
        pub mod INP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PINT PIN_INT0 input is selected"]
                pub const VAL0: u32 = 0;
                #[doc = "PINT PIN_INT4 input is selected"]
                pub const VAL1: u32 = 1;
                #[doc = "SCT_OUT4 input is selected"]
                pub const VAL2: u32 = 2;
                #[doc = "SCT_OUT5 input is selected"]
                pub const VAL3: u32 = 3;
                #[doc = "SCT_OUT1 input is selected"]
                pub const VAL4: u32 = 4;
                #[doc = "CTIMER0_MAT3 input is selected"]
                pub const VAL5: u32 = 5;
                #[doc = "CTIMER1_MAT3 input is selected"]
                pub const VAL6: u32 = 6;
                #[doc = "CTIMER2_MAT3 input is selected"]
                pub const VAL7: u32 = 7;
                #[doc = "CTIMER1_MAT0 input is selected"]
                pub const VAL8: u32 = 8;
                #[doc = "CTIMER3_MAT0 input is selected"]
                pub const VAL9: u32 = 9;
                #[doc = "ARM_TXEV input is selected"]
                pub const VAL11: u32 = 11;
                #[doc = "PINT GPIO_INT_BMAT input is selected"]
                pub const VAL12: u32 = 12;
                #[doc = "ADC0_tcomp[0] input is selected"]
                pub const VAL13: u32 = 13;
                #[doc = "ADC0_tcomp[1] input is selected"]
                pub const VAL14: u32 = 14;
                #[doc = "ADC0_tcomp[2] input is selected"]
                pub const VAL15: u32 = 15;
                #[doc = "ADC0_tcomp[3] input is selected"]
                pub const VAL16: u32 = 16;
                #[doc = "ADC1_tcomp[0] input is selected"]
                pub const VAL17: u32 = 17;
                #[doc = "ADC1_tcomp[1] input is selected"]
                pub const VAL18: u32 = 18;
                #[doc = "ADC1_tcomp[2] input is selected"]
                pub const VAL19: u32 = 19;
                #[doc = "ADC1_tcomp[3] input is selected"]
                pub const VAL20: u32 = 20;
                #[doc = "CMP0_OUT input is selected"]
                pub const VAL21: u32 = 21;
                #[doc = "CMP1_OUT input is selected"]
                pub const VAL22: u32 = 22;
                #[doc = "CMP2_OUT input is selected"]
                pub const VAL23: u32 = 23;
                #[doc = "PWM1_A0_TRIG0 input is selected"]
                pub const VAL24: u32 = 24;
                #[doc = "PWM1_A0_TRIG1 input is selected"]
                pub const VAL25: u32 = 25;
                #[doc = "PWM1_A1_TRIG0 input is selected"]
                pub const VAL26: u32 = 26;
                #[doc = "PWM1_A1_TRIG1 input is selected"]
                pub const VAL27: u32 = 27;
                #[doc = "PWM1_A2_TRIG0 input is selected"]
                pub const VAL28: u32 = 28;
                #[doc = "PWM1_A2_TRIG1 input is selected"]
                pub const VAL29: u32 = 29;
                #[doc = "PWM1_A3_TRIG0 input is selected"]
                pub const VAL30: u32 = 30;
                #[doc = "PWM1_A3_TRIG1 input is selected"]
                pub const VAL31: u32 = 31;
                #[doc = "ENC0_CMP/POS_MATCH input is selected"]
                pub const VAL32: u32 = 32;
                #[doc = "ENC1_CMP/POS_MATCH input is selected"]
                pub const VAL33: u32 = 33;
                #[doc = "EVTG_OUT0A input is selected"]
                pub const VAL34: u32 = 34;
                #[doc = "EVTG_OUT0B input is selected"]
                pub const VAL35: u32 = 35;
                #[doc = "EVTG_OUT1A input is selected"]
                pub const VAL36: u32 = 36;
                #[doc = "EVTG_OUT1B input is selected"]
                pub const VAL37: u32 = 37;
                #[doc = "EVTG_OUT2A input is selected"]
                pub const VAL38: u32 = 38;
                #[doc = "EVTG_OUT2B input is selected"]
                pub const VAL39: u32 = 39;
                #[doc = "EVTG_OUT3A input is selected"]
                pub const VAL40: u32 = 40;
                #[doc = "EVTG_OUT3B input is selected"]
                pub const VAL41: u32 = 41;
                #[doc = "TRIG_IN0 input is selected"]
                pub const VAL42: u32 = 42;
                #[doc = "TRIG_IN1 input is selected"]
                pub const VAL43: u32 = 43;
                #[doc = "TRIG_IN2 input is selected"]
                pub const VAL44: u32 = 44;
                #[doc = "TRIG_IN3 input is selected"]
                pub const VAL45: u32 = 45;
                #[doc = "TRIG_IN4 input is selected"]
                pub const VAL46: u32 = 46;
                #[doc = "TRIG_IN5 input is selected"]
                pub const VAL47: u32 = 47;
                #[doc = "TRIG_IN6 input is selected"]
                pub const VAL48: u32 = 48;
                #[doc = "TRIG_IN7 input is selected"]
                pub const VAL49: u32 = 49;
                #[doc = "TRIG_IN8 input is selected"]
                pub const VAL50: u32 = 50;
                #[doc = "TRIG_IN9 input is selected"]
                pub const VAL51: u32 = 51;
            }
        }
    }
    #[doc = "ENC0 Input Connections"]
    pub mod ENC_PHASEA {
        #[doc = "ENC0 input connections"]
        pub mod INP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PINT PIN_INT0 input is selected"]
                pub const VAL0: u32 = 0;
                #[doc = "PINT PIN_INT4 input is selected"]
                pub const VAL1: u32 = 1;
                #[doc = "SCT_OUT4 input is selected"]
                pub const VAL2: u32 = 2;
                #[doc = "SCT_OUT5 input is selected"]
                pub const VAL3: u32 = 3;
                #[doc = "SCT_OUT1 input is selected"]
                pub const VAL4: u32 = 4;
                #[doc = "CTIMER0_MAT3 input is selected"]
                pub const VAL5: u32 = 5;
                #[doc = "CTIMER1_MAT3 input is selected"]
                pub const VAL6: u32 = 6;
                #[doc = "CTIMER2_MAT3 input is selected"]
                pub const VAL7: u32 = 7;
                #[doc = "CTIMER1_MAT0 input is selected"]
                pub const VAL8: u32 = 8;
                #[doc = "CTIMER3_MAT0 input is selected"]
                pub const VAL9: u32 = 9;
                #[doc = "ARM_TXEV input is selected"]
                pub const VAL11: u32 = 11;
                #[doc = "PINT GPIO_INT_BMAT input is selected"]
                pub const VAL12: u32 = 12;
                #[doc = "ADC0_tcomp[0] input is selected"]
                pub const VAL13: u32 = 13;
                #[doc = "ADC0_tcomp[1] input is selected"]
                pub const VAL14: u32 = 14;
                #[doc = "ADC0_tcomp[2] input is selected"]
                pub const VAL15: u32 = 15;
                #[doc = "ADC0_tcomp[3] input is selected"]
                pub const VAL16: u32 = 16;
                #[doc = "ADC1_tcomp[0] input is selected"]
                pub const VAL17: u32 = 17;
                #[doc = "ADC1_tcomp[1] input is selected"]
                pub const VAL18: u32 = 18;
                #[doc = "ADC1_tcomp[2] input is selected"]
                pub const VAL19: u32 = 19;
                #[doc = "ADC1_tcomp[3] input is selected"]
                pub const VAL20: u32 = 20;
                #[doc = "CMP0_OUT input is selected"]
                pub const VAL21: u32 = 21;
                #[doc = "CMP1_OUT input is selected"]
                pub const VAL22: u32 = 22;
                #[doc = "CMP2_OUT input is selected"]
                pub const VAL23: u32 = 23;
                #[doc = "PWM1_A0_TRIG0 input is selected"]
                pub const VAL24: u32 = 24;
                #[doc = "PWM1_A0_TRIG1 input is selected"]
                pub const VAL25: u32 = 25;
                #[doc = "PWM1_A1_TRIG0 input is selected"]
                pub const VAL26: u32 = 26;
                #[doc = "PWM1_A1_TRIG1 input is selected"]
                pub const VAL27: u32 = 27;
                #[doc = "PWM1_A2_TRIG0 input is selected"]
                pub const VAL28: u32 = 28;
                #[doc = "PWM1_A2_TRIG1 input is selected"]
                pub const VAL29: u32 = 29;
                #[doc = "PWM1_A3_TRIG0 input is selected"]
                pub const VAL30: u32 = 30;
                #[doc = "PWM1_A3_TRIG1 input is selected"]
                pub const VAL31: u32 = 31;
                #[doc = "ENC0_CMP/POS_MATCH input is selected"]
                pub const VAL32: u32 = 32;
                #[doc = "ENC1_CMP/POS_MATCH input is selected"]
                pub const VAL33: u32 = 33;
                #[doc = "EVTG_OUT0A input is selected"]
                pub const VAL34: u32 = 34;
                #[doc = "EVTG_OUT0B input is selected"]
                pub const VAL35: u32 = 35;
                #[doc = "EVTG_OUT1A input is selected"]
                pub const VAL36: u32 = 36;
                #[doc = "EVTG_OUT1B input is selected"]
                pub const VAL37: u32 = 37;
                #[doc = "EVTG_OUT2A input is selected"]
                pub const VAL38: u32 = 38;
                #[doc = "EVTG_OUT2B input is selected"]
                pub const VAL39: u32 = 39;
                #[doc = "EVTG_OUT3A input is selected"]
                pub const VAL40: u32 = 40;
                #[doc = "EVTG_OUT3B input is selected"]
                pub const VAL41: u32 = 41;
                #[doc = "TRIG_IN0 input is selected"]
                pub const VAL42: u32 = 42;
                #[doc = "TRIG_IN1 input is selected"]
                pub const VAL43: u32 = 43;
                #[doc = "TRIG_IN2 input is selected"]
                pub const VAL44: u32 = 44;
                #[doc = "TRIG_IN3 input is selected"]
                pub const VAL45: u32 = 45;
                #[doc = "TRIG_IN4 input is selected"]
                pub const VAL46: u32 = 46;
                #[doc = "TRIG_IN5 input is selected"]
                pub const VAL47: u32 = 47;
                #[doc = "TRIG_IN6 input is selected"]
                pub const VAL48: u32 = 48;
                #[doc = "TRIG_IN7 input is selected"]
                pub const VAL49: u32 = 49;
                #[doc = "TRIG_IN8 input is selected"]
                pub const VAL50: u32 = 50;
                #[doc = "TRIG_IN9 input is selected"]
                pub const VAL51: u32 = 51;
            }
        }
    }
}
#[doc = "PWM0 External Synchronization"]
pub mod FlexPWM0_SM_EXTSYNC {
    #[doc = "Trigger input connections for PWM0"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT5 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT2 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER2_MAT0 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT0 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM1_A0_TRIG0 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "PWM1_A0_TRIG1 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "PWM1_A1_TRIG0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM1_A1_TRIG1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM1_A2_TRIG0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM1_A2_TRIG1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM1_A3_TRIG0 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM1_A3_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "TRIG_IN1 input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "TRIG_IN2 input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "TRIG_IN3 input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "TRIG_IN4 input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "TRIG_IN5 input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "TRIG_IN6 input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "TRIG_IN7 input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "TRIG_IN8 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "TRIG_IN9 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "SINC Filter CH0 sync Break input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "SINC Filter CH1 sync Break input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "SINC Filter CH2 sync Break input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "SINC Filter CH3 sync Break input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "SINC Filter CH4 sync Break input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL60: u32 = 60;
        }
    }
}
#[doc = "PWM0 Input Trigger Connections"]
pub mod FlexPWM0_SM_EXTA {
    #[doc = "Trigger input connections for PWM0"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT5 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT2 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER2_MAT0 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT0 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM1_A0_TRIG0 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "PWM1_A0_TRIG1 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "PWM1_A1_TRIG0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM1_A1_TRIG1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM1_A2_TRIG0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM1_A2_TRIG1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM1_A3_TRIG0 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM1_A3_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "TRIG_IN1 input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "TRIG_IN2 input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "TRIG_IN3 input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "TRIG_IN4 input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "TRIG_IN5 input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "TRIG_IN6 input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "TRIG_IN7 input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "TRIG_IN8 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "TRIG_IN9 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "SINC Filter CH0 sync Break input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "SINC Filter CH1 sync Break input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "SINC Filter CH2 sync Break input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "SINC Filter CH3 sync Break input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "SINC Filter CH4 sync Break input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL60: u32 = 60;
        }
    }
}
#[doc = "PWM0 External Force Trigger Connections"]
pub mod FlexPWM0_EXTFORCE {
    #[doc = "Trigger input connections for PWM0"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT5 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT2 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER2_MAT0 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT0 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM1_A0_TRIG0 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "PWM1_A0_TRIG1 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "PWM1_A1_TRIG0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM1_A1_TRIG1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM1_A2_TRIG0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM1_A2_TRIG1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM1_A3_TRIG0 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM1_A3_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "TRIG_IN1 input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "TRIG_IN2 input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "TRIG_IN3 input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "TRIG_IN4 input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "TRIG_IN5 input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "TRIG_IN6 input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "TRIG_IN7 input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "TRIG_IN8 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "TRIG_IN9 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "SINC Filter CH0 sync Break input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "SINC Filter CH1 sync Break input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "SINC Filter CH2 sync Break input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "SINC Filter CH3 sync Break input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "SINC Filter CH4 sync Break input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL60: u32 = 60;
        }
    }
}
#[doc = "PWM0 Fault Input Trigger Connections"]
pub mod FlexPWM0_FAULT {
    #[doc = "Trigger input connections for PWM0"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT5 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT2 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER2_MAT0 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT0 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM1_A0_TRIG0 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "PWM1_A0_TRIG1 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "PWM1_A1_TRIG0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM1_A1_TRIG1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM1_A2_TRIG0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM1_A2_TRIG1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM1_A3_TRIG0 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM1_A3_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "TRIG_IN1 input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "TRIG_IN2 input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "TRIG_IN3 input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "TRIG_IN4 input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "TRIG_IN5 input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "TRIG_IN6 input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "TRIG_IN7 input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "TRIG_IN8 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "TRIG_IN9 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "SINC Filter CH0 sync Break input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "SINC Filter CH1 sync Break input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "SINC Filter CH2 sync Break input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "SINC Filter CH3 sync Break input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "SINC Filter CH4 sync Break input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL60: u32 = 60;
        }
    }
}
#[doc = "PWM1 External Synchronization"]
pub mod FlexPWM1_SM_EXTSYNC {
    #[doc = "Trigger input connections for PWM0"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT2 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT3 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER2_MAT1 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT1 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM0_A0_TRIG0 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "PWM0_A0_TRIG1 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "PWM0_A1_TRIG0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM0_A1_TRIG1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM0_A2_TRIG0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM0_A2_TRIG1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A3_TRIG0 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A3_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "TRIG_IN1 input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "TRIG_IN2 input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "TRIG_IN3 input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "TRIG_IN4 input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "TRIG_IN5 input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "TRIG_IN6 input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "TRIG_IN7 input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "TRIG_IN8 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "TRIG_IN9 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "SINC Filter CH0 sync Break input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "SINC Filter CH1 sync Break input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "SINC Filter CH2 sync Break input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "SINC Filter CH3 sync Break input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "SINC Filter CH4 sync Break input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL60: u32 = 60;
        }
    }
}
#[doc = "PWM1 Input Trigger Connections"]
pub mod FlexPWM1_SM_EXTA {
    #[doc = "Trigger input connections for PWM0"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT2 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT3 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER2_MAT1 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT1 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM0_A0_TRIG0 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "PWM0_A0_TRIG1 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "PWM0_A1_TRIG0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM0_A1_TRIG1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM0_A2_TRIG0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM0_A2_TRIG1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A3_TRIG0 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A3_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "TRIG_IN1 input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "TRIG_IN2 input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "TRIG_IN3 input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "TRIG_IN4 input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "TRIG_IN5 input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "TRIG_IN6 input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "TRIG_IN7 input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "TRIG_IN8 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "TRIG_IN9 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "SINC Filter CH0 sync Break input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "SINC Filter CH1 sync Break input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "SINC Filter CH2 sync Break input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "SINC Filter CH3 sync Break input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "SINC Filter CH4 sync Break input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL60: u32 = 60;
        }
    }
}
#[doc = "PWM1 External Force Trigger Connections"]
pub mod FlexPWM1_EXTFORCE {
    #[doc = "Trigger input connections for PWM1"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT2 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT3 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER2_MAT1 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT1 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM0_A0_TRIG0 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "PWM0_A0_TRIG1 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "PWM0_A1_TRIG0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM0_A1_TRIG1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM0_A2_TRIG0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM0_A2_TRIG1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A3_TRIG0 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A3_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "TRIG_IN1 input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "TRIG_IN2 input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "TRIG_IN3 input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "TRIG_IN4 input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "TRIG_IN5 input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "TRIG_IN6 input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "TRIG_IN7 input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "TRIG_IN8 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "TRIG_IN9 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "SINC Filter CH0 sync Break input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "SINC Filter CH1 sync Break input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "SINC Filter CH2 sync Break input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "SINC Filter CH3 sync Break input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "SINC Filter CH4 sync Break input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL60: u32 = 60;
        }
    }
}
#[doc = "PWM1 Fault Input Trigger Connections"]
pub mod FlexPWM1_FAULT {
    #[doc = "Trigger input connections for PWM1"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT2 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT3 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER2_MAT1 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT1 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM0_A0_TRIG0 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "PWM0_A0_TRIG1 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "PWM0_A1_TRIG0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM0_A1_TRIG1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM0_A2_TRIG0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM0_A2_TRIG1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A3_TRIG0 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A3_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "TRIG_IN1 input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "TRIG_IN2 input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "TRIG_IN3 input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "TRIG_IN4 input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "TRIG_IN5 input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "TRIG_IN6 input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "TRIG_IN7 input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "TRIG_IN8 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "TRIG_IN9 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "SINC Filter CH0 sync Break input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "SINC Filter CH1 sync Break input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "SINC Filter CH2 sync Break input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "SINC Filter CH3 sync Break input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "SINC Filter CH4 sync Break input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL60: u32 = 60;
        }
    }
}
#[doc = "PWM0 External Clock Trigger"]
pub mod PWM0_EXT_CLK {
    #[doc = "Trigger input connections for PWM"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRO16K input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "OSC_32k input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "TRIG_IN7 input is selected"]
            pub const VAL5: u32 = 5;
        }
    }
}
#[doc = "PWM1 External Clock Trigger"]
pub mod PWM1_EXT_CLK {
    #[doc = "Trigger input connections for PWM"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRO16K input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "OSC_32k input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "TRIG_IN7 input is selected"]
            pub const VAL5: u32 = 5;
        }
    }
}
#[doc = "EVTG Trigger Input Connections %s"]
pub mod EVTG_TRIG {
    #[doc = "EVTG trigger input connections"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT_OUT0 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT_OUT1 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT2 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "SCT_OUT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER2_MAT2 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CTIMER3_MAT2 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CTIMER4_MAT2 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC0_IRQ input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC1_IRQ input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM0_A0_TRIG0 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM0_A0_TRIG1 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM0_A1_TRIG0 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A1_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A2_TRIG0 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM0_A2_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM0_A3_TRIG0 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM0_A3_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A0_TRIG0 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A0_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A1_TRIG0 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "PWM1_A1_TRIG1 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "PWM1_A2_TRIG0 input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "PWM1_A2_TRIG1 input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "PWM1_A3_TRIG0 input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "PWM1_A3_TRIG1 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "TRIG_IN1 input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "TRIG_IN2 input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "TRIG_IN3 input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "LPTMR0 input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "LPTMR1 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "SINC Filter CH0 Break input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "SINC Filter CH1 Break input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "SINC Filter CH2 Break input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "SINC Filter CH3 Break input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "SINC Filter CH4 Break input is selected"]
            pub const VAL55: u32 = 55;
        }
    }
}
#[doc = "USB-FS Trigger Input Connections"]
pub mod USBFS_TRIG {
    #[doc = "USB-FS trigger input connections. The trigger output of LP_FLEXCOMM is an input of peripheral INPUTMUX."]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LP_FLEXCOMM 0 trigger out [3] input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "LP_FLEXCOMM 1 trigger out [3] input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "LP_FLEXCOMM 2 trigger out [3] input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "LP_FLEXCOMM 3 trigger out [3] input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "LP_FLEXCOMM 4 trigger out [3] input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "LP_FLEXCOMM 5 trigger out [3] input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "LP_FLEXCOMM 6 trigger out [3] input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "LP_FLEXCOMM 7 trigger out [3] input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "LP_FLEXCOMM 8 trigger out [3] input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "LP_FLEXCOMM 9 trigger out [3] input is selected"]
            pub const VAL9: u32 = 9;
        }
    }
}
#[doc = "TSI Trigger Input Connections"]
pub mod TSI_TRIG {
    #[doc = "TSI trigger input connections"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPTMR0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "LPTMR1 input is selected"]
            pub const VAL1: u32 = 1;
        }
    }
}
#[doc = "EXT Trigger Connections %s"]
pub mod EXT_TRIG {
    #[doc = "EXT trigger input connections"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "ADC0_IRQ input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "ADC1_IRQ input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "LPTMR0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "LPTMR1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "SCT Out0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "SCT Out1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "SCT Out2 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "SCT Out3 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "SCT Out4 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "SCT Out5 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "Flexcomm0 trigger output 3 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "Flexcomm1 trigger output 3 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "Flexcomm2 trigger output 3 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "Flexcomm3 trigger output 3 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "Flexcomm4 trigger output 3 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "Flexcomm5 trigger output 3 input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "Flexcomm6 trigger output 3 input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "Flexcomm7 trigger output 3 input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "Flexcomm8 trigger output 3 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "Flexcomm9 trigger output 3 input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "ENET_PPS_OUT_0 input is selected"]
            pub const VAL47: u32 = 47;
        }
    }
}
#[doc = "CMP1 Input Connections"]
pub mod CMP1_TRIG {
    #[doc = "CMP1 input trigger"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT7 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT0 SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT0 SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT0 SCT_OUT7 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER3_MAT1 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT1 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "LPTMR0 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "LPTMR1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL40: u32 = 40;
        }
    }
}
#[doc = "CMP2 Input Connections"]
pub mod CMP2_TRIG {
    #[doc = "CMP2 input trigger"]
    pub mod TRIGIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT4 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT0 SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT0 SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT0 SCT_OUT8 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER3_MAT2 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT2 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "PWM0_A0_TRIG0/PWM0_A0_TRIG1 input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "PWM0_A1_TRIG0/PWM0_A1_TRIG1 input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "PWM0_A2_TRIG0/PWM0_A2_TRIG1 input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "PWM0_A3_TRIG0/PWM0_A3_TRIG1 input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "PWM1_A0_TRIG0/PWM1_A0_TRIG1 input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "PWM1_A1_TRIG0/PWM1_A1_TRIG1 input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "PWM1_A2_TRIG0/PWM1_A2_TRIG1 input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM1_A3_TRIG0/PWM1_A3_TRIG1 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "LPTMR0 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "LPTMR1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL40: u32 = 40;
        }
    }
}
#[doc = "SINC Filter Channel%s Trigger Input Connections"]
pub mod SINC_FILTER_CH {
    #[doc = "SINC FILTER trigger input connections"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "SCT_OUT4 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT9 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER3_MAT3 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER4_MAT3 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM0_A0_TRIG0 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "PWM0_A0_TRIG1 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "PWM0_A1_TRIG0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM0_A1_TRIG1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM0_A2_TRIG0 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM0_A2_TRIG1 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A3_TRIG0 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A3_TRIG1 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM1_A0_TRIG0 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM1_A0_TRIG1 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A1_TRIG0 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A1_TRIG1 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG0 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A2_TRIG1 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "PWM1_A3_TRIG0 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "PWM1_A3_TRIG1 input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "ENC0_CMP/POS_MATCH input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "ENC1_CMP/POS_MATCH input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "LPTMR0 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "LPTMR1 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "FlexIO CH0 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "FlexIO CH1 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "FlexIO CH2 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "FlexIO CH3 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "WUU input is selected"]
            pub const VAL56: u32 = 56;
        }
    }
}
#[doc = "OPAMP%s Trigger Input Connections"]
pub mod OPAMP__TRIG {
    #[doc = "OPAMP trigger input connections"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT0 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT1 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "PINT PIN_INT2 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "PINT PIN_INT3 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT4 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "SCT_OUT6 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "SCT_OUT7 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "SCT_OUT8 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER0_MAT3 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CTIMER1_MAT3 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "CTIMER2_MAT3 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "CTIMER3_MAT3 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "CTIMER4_MAT3 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "PWM0_A0_TRIG0 input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "PWM0_A0_TRIG1 input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "PWM0_A1_TRIG0 input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "PWM0_A1_TRIG1 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "PWM0_A2_TRIG0 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "PWM0_A2_TRIG1 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM0_A3_TRIG0 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A3_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM1_A0_TRIG0 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM1_A0_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM1_A1_TRIG0 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM1_A1_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM1_A2_TRIG0 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM1_A2_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A3_TRIG0 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "PWM1_A3_TRIG1 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "TRIG_IN1 input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "TRIG_IN2 input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "TRIG_IN3 input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "FlexIO CH4 input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "FlexIO CH5 input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "FlexIO CH6 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "FlexIO CH7 input is selected"]
            pub const VAL54: u32 = 54;
        }
    }
}
#[doc = "LP_FLEXCOMM%s Trigger Input Connections"]
pub mod FLEXCOMM__TRIG {
    #[doc = "FLEXCOMM0 trigger input connections"]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT4 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT5 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "PINT PIN_INT6 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT6 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "SCT_OUT7 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "CTIMER0_MAT1 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "CTIMER1_MAT1 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "CTIMER2_MAT0 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "CTIMER3_MAT0 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "CTIMER4_MAT0 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "LPTMR0 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "LPTMR1 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "TRIG_IN1 input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "TRIG_IN2 input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "TRIG_IN3 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "TRIG_IN4 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "TRIG_IN10 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "TRIG_IN11 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "FlexIO CH4 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "FlexIO CH5 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "FlexIO CH6 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "FlexIO CH7 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "USB0 ipp_ind_uart_rxd_usbmux input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "GPIO2 Pin Event Trig 0 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "GPIO2 Pin Event Trig 1 input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "GPIO3 Pin Event Trig 0 input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "GPIO3 Pin Event Trig 1 input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "WUU input is selected"]
            pub const VAL42: u32 = 42;
        }
    }
}
#[doc = "FlexIO Trigger Input Connections %s"]
pub mod FLEXIO_TRIG {
    #[doc = "Input number for FlexIO0."]
    pub mod INP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PINT PIN_INT4 input is selected"]
            pub const VAL0: u32 = 0;
            #[doc = "PINT PIN_INT5 input is selected"]
            pub const VAL1: u32 = 1;
            #[doc = "PINT PIN_INT6 input is selected"]
            pub const VAL2: u32 = 2;
            #[doc = "PINT PIN_INT7 input is selected"]
            pub const VAL3: u32 = 3;
            #[doc = "SCT_OUT5 input is selected"]
            pub const VAL4: u32 = 4;
            #[doc = "SCT_OUT6 input is selected"]
            pub const VAL5: u32 = 5;
            #[doc = "SCT_OUT7 input is selected"]
            pub const VAL6: u32 = 6;
            #[doc = "SCT_OUT8 input is selected"]
            pub const VAL7: u32 = 7;
            #[doc = "SCT_OUT9 input is selected"]
            pub const VAL8: u32 = 8;
            #[doc = "T0_MAT1 input is selected"]
            pub const VAL9: u32 = 9;
            #[doc = "T1_MAT1 input is selected"]
            pub const VAL10: u32 = 10;
            #[doc = "T2_MAT1 input is selected"]
            pub const VAL11: u32 = 11;
            #[doc = "T3_MAT1 input is selected"]
            pub const VAL12: u32 = 12;
            #[doc = "T4_MAT1 input is selected"]
            pub const VAL13: u32 = 13;
            #[doc = "LPTMR0 input is selected"]
            pub const VAL14: u32 = 14;
            #[doc = "LPTMR1 input is selected"]
            pub const VAL15: u32 = 15;
            #[doc = "ARM_TXEV input is selected"]
            pub const VAL16: u32 = 16;
            #[doc = "PINT GPIO_INT_BMAT input is selected"]
            pub const VAL17: u32 = 17;
            #[doc = "ADC0_tcomp[0] input is selected"]
            pub const VAL18: u32 = 18;
            #[doc = "ADC0_tcomp[1] input is selected"]
            pub const VAL19: u32 = 19;
            #[doc = "ADC0_tcomp[2] input is selected"]
            pub const VAL20: u32 = 20;
            #[doc = "ADC0_tcomp[3] input is selected"]
            pub const VAL21: u32 = 21;
            #[doc = "ADC1_tcomp[0] input is selected"]
            pub const VAL22: u32 = 22;
            #[doc = "ADC1_tcomp[1] input is selected"]
            pub const VAL23: u32 = 23;
            #[doc = "ADC1_tcomp[2] input is selected"]
            pub const VAL24: u32 = 24;
            #[doc = "ADC1_tcomp[3] input is selected"]
            pub const VAL25: u32 = 25;
            #[doc = "CMP0_OUT input is selected"]
            pub const VAL26: u32 = 26;
            #[doc = "CMP1_OUT input is selected"]
            pub const VAL27: u32 = 27;
            #[doc = "CMP2_OUT input is selected"]
            pub const VAL28: u32 = 28;
            #[doc = "PWM0_A0_TRIG0 input is selected"]
            pub const VAL29: u32 = 29;
            #[doc = "PWM0_A0_TRIG1 input is selected"]
            pub const VAL30: u32 = 30;
            #[doc = "PWM0_A1_TRIG0 input is selected"]
            pub const VAL31: u32 = 31;
            #[doc = "PWM0_A1_TRIG1 input is selected"]
            pub const VAL32: u32 = 32;
            #[doc = "PWM0_A2_TRIG0 input is selected"]
            pub const VAL33: u32 = 33;
            #[doc = "PWM0_A2_TRIG1 input is selected"]
            pub const VAL34: u32 = 34;
            #[doc = "PWM0_A3_TRIG0 input is selected"]
            pub const VAL35: u32 = 35;
            #[doc = "PWM0_A3_TRIG1 input is selected"]
            pub const VAL36: u32 = 36;
            #[doc = "PWM1_A0_TRIG0 input is selected"]
            pub const VAL37: u32 = 37;
            #[doc = "PWM1_A0_TRIG1 input is selected"]
            pub const VAL38: u32 = 38;
            #[doc = "PWM1_A1_TRIG0 input is selected"]
            pub const VAL39: u32 = 39;
            #[doc = "PWM1_A1_TRIG1 input is selected"]
            pub const VAL40: u32 = 40;
            #[doc = "PWM1_A2_TRIG0 input is selected"]
            pub const VAL41: u32 = 41;
            #[doc = "PWM1_A2_TRIG1 input is selected"]
            pub const VAL42: u32 = 42;
            #[doc = "PWM1_A3_TRIG0 input is selected"]
            pub const VAL43: u32 = 43;
            #[doc = "PWM1_A3_TRIG1 input is selected"]
            pub const VAL44: u32 = 44;
            #[doc = "EVTG_OUT0A input is selected"]
            pub const VAL45: u32 = 45;
            #[doc = "EVTG_OUT0B input is selected"]
            pub const VAL46: u32 = 46;
            #[doc = "EVTG_OUT1A input is selected"]
            pub const VAL47: u32 = 47;
            #[doc = "EVTG_OUT1B input is selected"]
            pub const VAL48: u32 = 48;
            #[doc = "EVTG_OUT2A input is selected"]
            pub const VAL49: u32 = 49;
            #[doc = "EVTG_OUT2B input is selected"]
            pub const VAL50: u32 = 50;
            #[doc = "EVTG_OUT3A input is selected"]
            pub const VAL51: u32 = 51;
            #[doc = "EVTG_OUT3B input is selected"]
            pub const VAL52: u32 = 52;
            #[doc = "TRIG_IN0 input is selected"]
            pub const VAL53: u32 = 53;
            #[doc = "TRIG_IN1 input is selected"]
            pub const VAL54: u32 = 54;
            #[doc = "TRIG_IN2 input is selected"]
            pub const VAL55: u32 = 55;
            #[doc = "TRIG_IN3 input is selected"]
            pub const VAL56: u32 = 56;
            #[doc = "TRIG_IN4 input is selected"]
            pub const VAL57: u32 = 57;
            #[doc = "SINC Filter CH0 Conversion Complete input is selected"]
            pub const VAL58: u32 = 58;
            #[doc = "SINC Filter CH1 Conversion Complete input is selected"]
            pub const VAL59: u32 = 59;
            #[doc = "SINC Filter CH2 Conversion Complete input is selected"]
            pub const VAL60: u32 = 60;
            #[doc = "SINC Filter CH3 Conversion Complete input is selected"]
            pub const VAL61: u32 = 61;
            #[doc = "SINC Filter CH4 Conversion Complete input is selected"]
            pub const VAL62: u32 = 62;
            #[doc = "LP_FLEXCOMM0 trig 0 (lpuart_trg_txword) input is selected"]
            pub const VAL63: u32 = 63;
            #[doc = "LP_FLEXCOMM0 trig 1 (lpuart_trg_rxword) input is selected"]
            pub const VAL64: u32 = 64;
            #[doc = "LP_FLEXCOMM0 trig 2 (lpuart_trg_rxidle) input is selected"]
            pub const VAL65: u32 = 65;
            #[doc = "LP_FLEXCOMM1 trig 0 input is selected"]
            pub const VAL66: u32 = 66;
            #[doc = "LP_FLEXCOMM1 trig 1 input is selected"]
            pub const VAL67: u32 = 67;
            #[doc = "LP_FLEXCOMM1 trig 2 input is selected"]
            pub const VAL68: u32 = 68;
            #[doc = "LP_FLEXCOMM2 trig 0 input is selected"]
            pub const VAL69: u32 = 69;
            #[doc = "LP_FLEXCOMM2 trig 1 input is selected"]
            pub const VAL70: u32 = 70;
            #[doc = "LP_FLEXCOMM2 trig 2 input is selected"]
            pub const VAL71: u32 = 71;
            #[doc = "LP_FLEXCOMM3 trig 0 input is selected"]
            pub const VAL72: u32 = 72;
            #[doc = "LP_FLEXCOMM3 trig 1 input is selected"]
            pub const VAL73: u32 = 73;
            #[doc = "LP_FLEXCOMM3 trig 2 input is selected"]
            pub const VAL74: u32 = 74;
            #[doc = "LP_FLEXCOMM3 trig 3 input is selected"]
            pub const VAL75: u32 = 75;
            #[doc = "WUU input is selected"]
            pub const VAL76: u32 = 76;
        }
    }
}
pub mod dma {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "Cluster DMA%s, containing DMA?_REQ_ENABLE0, DMA?_REQ_ENABLE0_SET, DMA?_REQ_ENABLE0_CLR, DMA?_REQ_ENABLE0_TOG, DMA?_REQ_ENABLE1, DMA?_REQ_ENABLE1_SET, DMA?_REQ_ENABLE1_CLR, DMA?_REQ_ENABLE1_TOG, DMA?_REQ_ENABLE2, DMA?_REQ_ENABLE2_SET, DMA?_REQ_ENABLE2_CLR, DMA?_REQ_ENABLE2_TOG, DMA?_REQ_ENABLE3, DMA?_REQ_ENABLE3_SET, DMA?_REQ_ENABLE3_CLR"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "DMA0 Request Enable0"]
        pub DMA_REQ_ENABLE0: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable0"]
        pub DMA_REQ_ENABLE0_SET: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable0"]
        pub DMA_REQ_ENABLE0_CLR: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable0"]
        pub DMA_REQ_ENABLE0_TOG: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable1"]
        pub DMA_REQ_ENABLE1: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable1"]
        pub DMA_REQ_ENABLE1_SET: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable1"]
        pub DMA_REQ_ENABLE1_CLR: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable1"]
        pub DMA_REQ_ENABLE1_TOG: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable2"]
        pub DMA_REQ_ENABLE2: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable2"]
        pub DMA_REQ_ENABLE2_SET: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable2"]
        pub DMA_REQ_ENABLE2_CLR: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable2"]
        pub DMA_REQ_ENABLE2_TOG: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable3"]
        pub DMA_REQ_ENABLE3: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable3"]
        pub DMA_REQ_ENABLE3_SET: crate::RWRegister<u32>,
        #[doc = "DMA0 Request Enable3"]
        pub DMA_REQ_ENABLE3_CLR: crate::RWRegister<u32>,
    }
    #[doc = "DMA0 Request Enable0"]
    pub mod DMA_REQ_ENABLE0 {
        #[doc = "This register is used to enable and disable FLEXSPI0 receive event request."]
        pub mod REQ1_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE0: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE1: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable FLEXSPI0 transmit event request."]
        pub mod REQ2_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE2: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE3: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable INT0 request."]
        pub mod REQ3_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE4: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE5: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable INT1 request."]
        pub mod REQ4_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE6: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE7: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable INT2 request."]
        pub mod REQ5_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE8: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE9: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable INT3 request."]
        pub mod REQ6_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE10: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE11: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMAREQ_M0 request."]
        pub mod REQ7_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE12: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE13: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMAREQ_M1 request."]
        pub mod REQ8_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE14: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE15: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMAREQ_M0 request."]
        pub mod REQ9_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE16: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE17: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMAREQ_M1 request."]
        pub mod REQ10_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE18: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE19: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMAREQ_M0 request."]
        pub mod REQ11_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE20: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE21: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMAREQ_M1 request."]
        pub mod REQ12_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE22: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE23: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMAREQ_M0 request."]
        pub mod REQ13_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE24: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE25: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMAREQ_M1 request."]
        pub mod REQ14_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE26: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE27: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMAREQ_M0 request."]
        pub mod REQ15_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE28: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE29: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMAREQ_M1 request."]
        pub mod REQ16_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE30: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE31: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable wake up event request."]
        pub mod REQ17_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE32: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE33: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable FIFO_request."]
        pub mod REQ18_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE34: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE35: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMA0 request."]
        pub mod REQ19_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE36: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE37: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMA1 request."]
        pub mod REQ20_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE38: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE39: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable FIFO A request."]
        pub mod REQ21_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE40: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE41: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable FIFO B request."]
        pub mod REQ22_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE42: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE43: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable FIFO A request."]
        pub mod REQ23_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE44: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE45: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable FIFO B request."]
        pub mod REQ24_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE46: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE47: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable FIFO_request."]
        pub mod REQ25_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE48: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE49: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable FIFO_request."]
        pub mod REQ26_EN0 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE50: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE51: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable FIFO_request."]
        pub mod REQ27_EN0 {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE52: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE53: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMA_request."]
        pub mod REQ28_EN0 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE54: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE55: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMA_request."]
        pub mod REQ29_EN0 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE56: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE57: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMA_request."]
        pub mod REQ30_EN0 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE58: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE59: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable OUT0A request."]
        pub mod REQ31_EN0 {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE60: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE61: u32 = 1;
            }
        }
    }
    #[doc = "DMA0 Request Enable0"]
    pub mod DMA_REQ_ENABLE0_SET {
        #[doc = "Writing a 1 to REQ1_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ1_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ2_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ2_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ3_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ3_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ4_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ4_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ5_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ5_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ6_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ6_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ7_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ7_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ8_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ8_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ9_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ9_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ10_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ10_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ11_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ11_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ12_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ12_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ13_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ13_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ14_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ14_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ15_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ15_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ16_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ16_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ17_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ17_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ18_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ18_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ19_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ19_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ20_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ20_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ21_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ21_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ22_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ22_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ23_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ23_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ24_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ24_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ25_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ25_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ26_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ26_EN0 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ27_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ27_EN0 {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ28_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ28_EN0 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ29_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ29_EN0 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ30_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ30_EN0 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ31_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ31_EN0 {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "DMA0 Request Enable0"]
    pub mod DMA_REQ_ENABLE0_CLR {
        #[doc = "Writing a 1 to REQ1_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ1_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ2_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ2_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ3_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ3_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ4_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ4_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ5_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ5_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ6_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ6_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ7_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ7_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ8_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ8_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ9_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ9_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ10_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ10_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ11_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ11_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ12_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ12_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ13_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ13_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ14_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ14_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ15_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ15_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ16_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ16_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ17_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ17_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ18_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ18_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ19_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ19_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ20_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ20_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ21_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ21_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ22_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ22_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ23_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ23_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ24_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ24_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ25_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ25_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ26_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ26_EN0 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ27_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ27_EN0 {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ28_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ28_EN0 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ29_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ29_EN0 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ30_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ30_EN0 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ31_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ31_EN0 {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "DMA0 Request Enable0"]
    pub mod DMA_REQ_ENABLE0_TOG {
        #[doc = "Writing a 1 to REQ1_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ1_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ2_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ2_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ3_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ3_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ4_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ4_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ5_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ5_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ6_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ6_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ7_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ7_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ8_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ8_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to RE9_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ9_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ10_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ10_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ11_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ11_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ12_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ12_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ13_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ13_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ14_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ14_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ15_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ15_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ16_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ16_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ17_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ17_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ18_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ18_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ19_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ19_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ20_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ20_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ21_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ21_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ22_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ22_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ23_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ23_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ24_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ24_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ25_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ25_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ26_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ26_EN0 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ27_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ27_EN0 {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ28_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ28_EN0 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ29_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ29_EN0 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ30_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ30_EN0 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ31_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE0."]
        pub mod REQ31_EN0 {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "DMA0 Request Enable1"]
    pub mod DMA_REQ_ENABLE1 {
        #[doc = "This register is used to enable and disable OUT0B request."]
        pub mod REQ32_EN0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE62: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE63: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable OUT1A request."]
        pub mod REQ33_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE64: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE65: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable OUT1B request."]
        pub mod REQ34_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE66: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE67: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable OUT2A request."]
        pub mod REQ35_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE68: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE69: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable OUT2B request."]
        pub mod REQ36_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE70: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE71: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable OUT3A request."]
        pub mod REQ37_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE72: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE73: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable OUT3B request."]
        pub mod REQ38_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE74: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE75: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_capt0 request."]
        pub mod REQ39_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE76: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE77: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_capt1 request."]
        pub mod REQ40_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE78: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE79: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_capt2 request."]
        pub mod REQ41_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE80: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE81: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_capt3 request."]
        pub mod REQ42_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE82: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE83: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_val0 request."]
        pub mod REQ43_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE84: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE85: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_val1 request."]
        pub mod REQ44_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE86: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE87: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_val2 request."]
        pub mod REQ45_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE88: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE89: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_val3 request."]
        pub mod REQ46_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE90: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE91: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_capt0 request."]
        pub mod REQ47_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE92: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE93: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_capt1 request."]
        pub mod REQ48_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE94: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE95: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_capt2 request."]
        pub mod REQ49_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE96: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE97: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_capt3 request."]
        pub mod REQ50_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE98: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE99: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_val0 request."]
        pub mod REQ51_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE100: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE101: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_val1 request."]
        pub mod REQ52_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE102: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE103: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_val2 request."]
        pub mod REQ53_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE104: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE105: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable Req_val3 request."]
        pub mod REQ54_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE106: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE107: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable counter match event request."]
        pub mod REQ57_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE112: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE113: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable counter match event request."]
        pub mod REQ58_EN0 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE114: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE115: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMA request."]
        pub mod REQ59_EN0 {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE116: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE117: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable DMA request."]
        pub mod REQ60_EN0 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE118: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE119: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable shift register 0 request."]
        pub mod REQ61_EN0 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE120: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE121: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable shift register 1 request."]
        pub mod REQ62_EN0 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE122: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE123: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable shift register 2 request."]
        pub mod REQ63_EN0 {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE124: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE125: u32 = 1;
            }
        }
    }
    #[doc = "DMA0 Request Enable1"]
    pub mod DMA_REQ_ENABLE1_SET {
        #[doc = "Writing a 1 to REQ32_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ32_EN0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ33_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ33_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ34_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ34_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ35_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ35_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ36_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ36_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ37_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ37_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ38_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ38_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ39_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ39_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ40_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ40_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ41_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ41_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ42_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ42_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ43_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ43_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ44_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ44_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ45_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ45_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ46_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ46_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ47_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ47_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ48_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ48_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ49_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ49_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ50_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ50_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ51_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ51_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ52_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ52_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ53_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ53_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ54_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ54_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ55_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ55_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ56_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ56_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ57_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ57_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ58_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ58_EN0 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ59_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ59_EN0 {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ60_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ60_EN0 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ61_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ61_EN0 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ62_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ62_EN0 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ63_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ63_EN0 {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "DMA0 Request Enable1"]
    pub mod DMA_REQ_ENABLE1_CLR {
        #[doc = "Writing a 1 to REQ32_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ32_EN0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ33_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ33_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ34_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ34_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ35_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ35_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ36_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ36_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ37_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ37_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ38_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ38_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ39_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ39_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ40_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ40_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ41_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ41_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ42_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ42_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ43_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ43_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ44_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ44_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ45_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ45_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ46_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ46_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ47_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ47_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ48_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ48_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ49_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ49_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ50_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ50_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ51_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ51_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ52_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ52_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ53_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ53_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ54_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ54_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ55_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ55_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ56_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ56_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ57_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ57_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ58_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ58_EN0 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ59_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ59_EN0 {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ60_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ60_EN0 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ61_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ61_EN0 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ62_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ62_EN0 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ63_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ63_EN0 {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "DMA0 Request Enable1"]
    pub mod DMA_REQ_ENABLE1_TOG {
        #[doc = "Writing a 1 to REQ32_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ32_EN0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ33_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ33_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ34_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ34_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ35_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ35_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ36_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ36_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ37_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ37_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ38_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ38_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ39_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ39_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ40_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ40_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ41_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ41_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ42_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ42_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ43_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ43_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ44_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ44_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ55_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ45_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ46_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ46_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ47_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ47_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ48_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ48_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ49_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ49_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ50_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ50_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ51_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ51_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ52_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ52_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ53_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ53_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ54_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ54_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ55_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ55_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ56_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ56_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ57_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ57_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ58_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ58_EN0 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ59_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ59_EN0 {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ60_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ60_EN0 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ61_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ61_EN0 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ62_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ62_EN0 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ63_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE1."]
        pub mod REQ63_EN0 {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "DMA0 Request Enable2"]
    pub mod DMA_REQ_ENABLE2 {
        #[doc = "This register is used to enable and disable shift register 3 request."]
        pub mod REQ64_EN0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE0: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE1: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable shift register 4 request."]
        pub mod REQ65_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE0: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE1: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable shift register 5 request."]
        pub mod REQ66_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE2: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE3: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable shift register 6 request."]
        pub mod REQ67_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE4: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE5: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable shift register 7 request."]
        pub mod REQ68_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE6: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE7: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ69_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE8: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE9: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ70_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE10: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE11: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ71_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE12: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE13: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ72_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE14: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE15: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ73_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE16: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE17: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ74_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE18: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE19: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ75_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE20: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE21: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ76_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE22: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE23: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ77_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE24: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE25: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ78_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE26: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE27: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ79_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE28: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE29: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ80_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE30: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE31: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ81_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE32: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE33: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ82_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE34: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE35: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ83_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE36: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE37: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ84_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE38: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE39: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ85_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE40: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE41: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ86_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE42: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE43: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ87_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE44: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE45: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ88_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE46: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE47: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable channel 0 request."]
        pub mod REQ89_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE48: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE49: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable channel 1 request."]
        pub mod REQ90_EN0 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE50: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE51: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ91_EN0 {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE52: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE53: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ92_EN0 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE54: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE55: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ93_EN0 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE56: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE57: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ94_EN0 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE58: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE59: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ95_EN0 {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE60: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE61: u32 = 1;
            }
        }
    }
    #[doc = "DMA0 Request Enable2"]
    pub mod DMA_REQ_ENABLE2_SET {
        #[doc = "Writing a 1 to REQ64_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ64_EN0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ65_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ65_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ66_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ66_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ67_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ67_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ68_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ68_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ69_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ69_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ70_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ70_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ71_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ71_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ72_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ72_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ73_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ73_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ74_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ74_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ75_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ75_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ876_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ76_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ77_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ77_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ78_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ78_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ79_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ79_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ80_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ80_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ81_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ81_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ82_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ82_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ83_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ83_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ84_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ84_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ85_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ85_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ86_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ86_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ87_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ87_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ88_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ88_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ89_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ89_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ90_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ90_EN0 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ91_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ91_EN0 {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ92_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ92_EN0 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ93_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ93_EN0 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ94_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ94_EN0 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ95_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ95_EN0 {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "DMA0 Request Enable2"]
    pub mod DMA_REQ_ENABLE2_CLR {
        #[doc = "Writing a 1 to REQ64_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ64_EN0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ65_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ65_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ66_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ66_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ67_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ67_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ68_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ68_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ69_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ69_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ70_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ70_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ71_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ71_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ72_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ72_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ73_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ73_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ74_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ74_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ75_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ75_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ76_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ76_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ77_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ77_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ78_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ78_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ79_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ79_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ80_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ80_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ81_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ81_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ82_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ82_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ83_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ83_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ84_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ84_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ85_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ85_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ86_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ86_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ87_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ87_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ88_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ88_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ89_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ89_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ90_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ90_EN0 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ91_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ91_EN0 {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ92_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ92_EN0 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ93_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ93_EN0 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ94_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ94_EN0 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ95_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ95_EN0 {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "DMA0 Request Enable2"]
    pub mod DMA_REQ_ENABLE2_TOG {
        #[doc = "Writing a 1 to REQ64_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ64_EN0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ65_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ65_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ66_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ66_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ67_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ67_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ68_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ68_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ69_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ69_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ70_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ70_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ71_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ71_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ72_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ72_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ73_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ73_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ74_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ74_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ75_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ75_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ76_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ76_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ77_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ77_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ78_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ78_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ79_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ79_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ80_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ80_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ81_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ81_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ82_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ82_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ83_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ83_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ84_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ84_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ85_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ85_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ86_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ86_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ87_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ87_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ88_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ88_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ89_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ89_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ90_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ90_EN0 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ91_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ91_EN0 {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ92_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ92_EN0 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ93_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ93_EN0 {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ94_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ94_EN0 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ95_EN0 in this register toggles the corresponding bit in DMA0_REQ_ENABLE2."]
        pub mod REQ95_EN0 {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "DMA0 Request Enable3"]
    pub mod DMA_REQ_ENABLE3 {
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ96_EN0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE0: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE1: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable shift register 4 request."]
        pub mod REQ97_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE0: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE1: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ98_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE2: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE3: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable shift register 6 request."]
        pub mod REQ99_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE4: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE5: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ100_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE6: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE7: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable receive request."]
        pub mod REQ101_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE8: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE9: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable transmit request."]
        pub mod REQ102_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE10: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE11: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable ipd_req_sinc[0] or ipd_req_alt [0] request."]
        pub mod REQ103_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE12: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE13: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable ipd_req_sinc[1] or ipd_req_alt [1] request."]
        pub mod REQ104_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE14: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE15: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable ipd_req_sinc[2] or ipd_req_alt [2] request."]
        pub mod REQ105_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE16: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE17: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable ipd_req_sinc[3] or ipd_req_alt [3] request."]
        pub mod REQ106_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE18: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE19: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable ipd_req_sinc[4] or ipd_req_alt [4] request."]
        pub mod REQ107_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE20: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE21: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable pin event request 0."]
        pub mod REQ108_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE22: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE23: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable pin event request 1."]
        pub mod REQ109_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE24: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE25: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable pin event request 0."]
        pub mod REQ110_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE26: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE27: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable pin event request 1."]
        pub mod REQ111_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE28: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE29: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable pin event request 0."]
        pub mod REQ112_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE30: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE31: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable pin event request 1."]
        pub mod REQ113_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE32: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE33: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable pin event request 0."]
        pub mod REQ114_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE34: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE35: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable pin event request 1."]
        pub mod REQ115_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE36: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE37: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable pin event request 0."]
        pub mod REQ116_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE38: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE39: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable pin event request 1."]
        pub mod REQ117_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE40: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE41: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable pin event request 0."]
        pub mod REQ118_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE42: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE43: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable pin event request 1."]
        pub mod REQ119_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE44: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE45: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable end of scan request."]
        pub mod REQ120_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE46: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE47: u32 = 1;
            }
        }
        #[doc = "This register is used to enable and disable out of range request."]
        pub mod REQ121_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable"]
                pub const VALUE48: u32 = 0;
                #[doc = "Enable"]
                pub const VALUE49: u32 = 1;
            }
        }
    }
    #[doc = "DMA0 Request Enable3"]
    pub mod DMA_REQ_ENABLE3_SET {
        #[doc = "Writing a 1 to REQ96_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ96_EN0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ97_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ97_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ98_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ98_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ99_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ99_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ100_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ100_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ101_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ101_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ102_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ102_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ103_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ103_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ104_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ104_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ105_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ105_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ106_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ106_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ107_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ107_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ108_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ108_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ109_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ109_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ110_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ110_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ111_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ111_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ112_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ112_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ113_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ113_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ114_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ114_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ115_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ115_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ116_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ116_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ117_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ117_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ118_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ118_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ119_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ119_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ120_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ120_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ121_EN0 in this register sets the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ121_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "DMA0 Request Enable3"]
    pub mod DMA_REQ_ENABLE3_CLR {
        #[doc = "Writing a 1 to REQ96_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ96_EN0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ97_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ97_EN0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ98_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ98_EN0 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ99_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ99_EN0 {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ100_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ100_EN0 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ101_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ101_EN0 {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ102_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ102_EN0 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ103_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ103_EN0 {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ104_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ104_EN0 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ105_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ105_EN0 {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ106_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ106_EN0 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ107_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ107_EN0 {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ108_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ108_EN0 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ109_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ109_EN0 {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ110_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ110_EN0 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ111_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ111_EN0 {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ112_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ112_EN0 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ113_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ113_EN0 {
            pub const offset: u32 = 17;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ114_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ114_EN0 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ115_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ115_EN0 {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ116_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ116_EN0 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ117_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ117_EN0 {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ118_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ118_EN0 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ119_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ119_EN0 {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ120_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ120_EN0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Writing a 1 to REQ121_EN0 in this register clears the corresponding bit in DMA0_REQ_ENABLE3"]
        pub mod REQ121_EN0 {
            pub const offset: u32 = 25;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
