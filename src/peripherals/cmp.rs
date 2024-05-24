#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "LPCMP"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RWRegister<u32>,
    #[doc = "Comparator Control Register 0"]
    pub CCR0: crate::RWRegister<u32>,
    #[doc = "Comparator Control Register 1"]
    pub CCR1: crate::RWRegister<u32>,
    #[doc = "Comparator Control Register 2"]
    pub CCR2: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "DAC Control"]
    pub DCR: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub IER: crate::RWRegister<u32>,
    #[doc = "Comparator Status"]
    pub CSR: crate::RWRegister<u32>,
    #[doc = "Round Robin Control Register 0"]
    pub RRCR0: crate::RWRegister<u32>,
    #[doc = "Round Robin Control Register 1"]
    pub RRCR1: crate::RWRegister<u32>,
    #[doc = "Round Robin Control and Status"]
    pub RRCSR: crate::RWRegister<u32>,
    #[doc = "Round Robin Status"]
    pub RRSR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "Round Robin Control Register 2"]
    pub RRCR2: crate::RWRegister<u32>,
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {
            #[doc = "Round robin feature"]
            pub const ROUND_ROBIN: u32 = 1;
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
#[doc = "Parameter"]
pub mod PARAM {
    #[doc = "DAC Resolution"]
    pub mod DAC_RES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "4-bit DAC"]
            pub const RESO_4: u32 = 0;
            #[doc = "6-bit DAC"]
            pub const RESO_6: u32 = 1;
            #[doc = "8-bit DAC"]
            pub const RESO_8: u32 = 2;
            #[doc = "10-bit DAC"]
            pub const RESO_10: u32 = 3;
            #[doc = "12-bit DAC"]
            pub const RESO_12: u32 = 4;
            #[doc = "14-bit DAC"]
            pub const RESO_14: u32 = 5;
            #[doc = "16-bit DAC"]
            pub const RESO_16: u32 = 6;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator Control Register 0"]
pub mod CCR0 {
    #[doc = "Comparator Enable"]
    pub mod CMP_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables (The analog logic remains off and consumes no power.)"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Comparator Deep sleep Mode Enable"]
    pub mod CMP_STOP_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable the analog comparator regardless of CMP_EN."]
            pub const DISABLE: u32 = 0;
            #[doc = "Allows CMP_EN to enable the analog comparator."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Comparator Control Register 1"]
pub mod CCR1 {
    #[doc = "Windowing Enable"]
    pub mod WINDOW_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Sampling Enable"]
    pub mod SAMPLE_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "DMA Enable"]
    pub mod DMA_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Comparator Invert"]
    pub mod COUT_INV {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not invert"]
            pub const NO_INVERT: u32 = 0;
            #[doc = "Invert"]
            pub const INVERT: u32 = 1;
        }
    }
    #[doc = "Comparator Output Select"]
    pub mod COUT_SEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use COUT (filtered)"]
            pub const COUT: u32 = 0;
            #[doc = "Use COUTA (unfiltered)"]
            pub const COUTA: u32 = 1;
        }
    }
    #[doc = "Comparator Output Pin Enable"]
    pub mod COUT_PEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not available"]
            pub const UNAVAILABLE: u32 = 0;
            #[doc = "Available"]
            pub const AVAILABLE: u32 = 1;
        }
    }
    #[doc = "COUTA_OW Enable"]
    pub mod COUTA_OWEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "COUTA holds the last sampled value."]
            pub const SAMPLED: u32 = 0;
            #[doc = "Enables the COUTA signal value to be defined by COUTA_OW."]
            pub const COUTA_OW: u32 = 1;
        }
    }
    #[doc = "COUTA Output Level for Closed Window"]
    pub mod COUTA_OW {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "COUTA is 0"]
            pub const COUTA_0: u32 = 0;
            #[doc = "COUTA is 1"]
            pub const COUTA_1: u32 = 1;
        }
    }
    #[doc = "WINDOW/SAMPLE Signal Invert"]
    pub mod WINDOW_INV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not invert"]
            pub const NO_INVERT: u32 = 0;
            #[doc = "Invert"]
            pub const INVERT: u32 = 1;
        }
    }
    #[doc = "COUT Event Window Close"]
    pub mod WINDOW_CLS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "COUT event cannot close the window"]
            pub const NO_CLOSE: u32 = 0;
            #[doc = "COUT event can close the window"]
            pub const CLOSE: u32 = 1;
        }
    }
    #[doc = "COUT Event Select"]
    pub mod EVT_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rising edge"]
            pub const RISING: u32 = 0;
            #[doc = "Falling edge"]
            pub const FALLING: u32 = 1;
            #[doc = "Both edges"]
            pub const BOTH: u32 = 2;
        }
    }
    #[doc = "Functional Clock Source Select"]
    pub mod FUNC_CLK_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select functional clock source 0"]
            pub const FUNC0: u32 = 0;
            #[doc = "Select functional clock source 1"]
            pub const FUNC1: u32 = 1;
            #[doc = "Select functional clock source 2"]
            pub const FUNC2: u32 = 2;
            #[doc = "Select functional clock source 3"]
            pub const FUNC3: u32 = 3;
        }
    }
    #[doc = "Filter Sample Count"]
    pub mod FILT_CNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Filter is bypassed: COUT = COUTA"]
            pub const BYPASSED: u32 = 0;
            #[doc = "1 consecutive sample (Comparator output is simply sampled.)"]
            pub const SAMPLE_1: u32 = 1;
            #[doc = "2 consecutive samples"]
            pub const SAMPLE_2: u32 = 2;
            #[doc = "3 consecutive samples"]
            pub const SAMPLE_3: u32 = 3;
            #[doc = "4 consecutive samples"]
            pub const SAMPLE_4: u32 = 4;
            #[doc = "5 consecutive samples"]
            pub const SAMPLE_5: u32 = 5;
            #[doc = "6 consecutive samples"]
            pub const SAMPLE_6: u32 = 6;
            #[doc = "7 consecutive samples"]
            pub const SAMPLE_7: u32 = 7;
        }
    }
    #[doc = "Filter Sample Period"]
    pub mod FILT_PER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Comparator Control Register 2"]
pub mod CCR2 {
    #[doc = "CMP High Power Mode Select"]
    pub mod CMP_HPMD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low power (speed) comparison mode"]
            pub const LOW: u32 = 0;
            #[doc = "High power (speed) comparison mode"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "CMP Nano Power Mode Select"]
    pub mod CMP_NPMD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables CMP Nano power mode. CCR2[CMP_HPMD] determines the mode for the comparator."]
            pub const NO_NANO: u32 = 0;
            #[doc = "Enables CMP Nano power mode."]
            pub const NANO: u32 = 1;
        }
    }
    #[doc = "Comparator Hysteresis Control"]
    pub mod HYSTCTR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Level 0"]
            pub const LEVEL_0: u32 = 0;
            #[doc = "Level 1"]
            pub const LEVEL_1: u32 = 1;
            #[doc = "Level 2"]
            pub const LEVEL_2: u32 = 2;
            #[doc = "Level 3"]
            pub const LEVEL_3: u32 = 3;
        }
    }
    #[doc = "Plus Input MUX Select"]
    pub mod PSEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input 0p"]
            pub const INPUT_0: u32 = 0;
            #[doc = "Input 1p"]
            pub const INPUT_1: u32 = 1;
            #[doc = "Input 2p"]
            pub const INPUT_2: u32 = 2;
            #[doc = "Input 3p"]
            pub const INPUT_3: u32 = 3;
            #[doc = "Input 4p"]
            pub const INPUT_4: u32 = 4;
            #[doc = "Input 5p"]
            pub const INPUT_5: u32 = 5;
            #[doc = "Internal DAC output"]
            pub const INPUT_7: u32 = 7;
        }
    }
    #[doc = "Minus Input MUX Select"]
    pub mod MSEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input 0m"]
            pub const INPUT_0: u32 = 0;
            #[doc = "Input 1m"]
            pub const INPUT_1: u32 = 1;
            #[doc = "Input 2m"]
            pub const INPUT_2: u32 = 2;
            #[doc = "Input 3m"]
            pub const INPUT_3: u32 = 3;
            #[doc = "Input 4m"]
            pub const INPUT_4: u32 = 4;
            #[doc = "Input 5m"]
            pub const INPUT_5: u32 = 5;
            #[doc = "Internal DAC output"]
            pub const INPUT_7: u32 = 7;
        }
    }
}
#[doc = "DAC Control"]
pub mod DCR {
    #[doc = "DAC Enable"]
    pub mod DAC_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "DAC High Power Mode Select"]
    pub mod DAC_HPMD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "DAC Reference High Voltage Source Select"]
    pub mod VRSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "vrefh0"]
            pub const VREF0: u32 = 0;
            #[doc = "vrefh1"]
            pub const VREF1: u32 = 1;
        }
    }
    #[doc = "DAC Output Voltage Select"]
    pub mod DAC_DATA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable"]
pub mod IER {
    #[doc = "Comparator Flag Rising Interrupt Enable"]
    pub mod CFR_IE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the comparator flag rising interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the comparator flag rising interrupt when CFR is set."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Comparator Flag Falling Interrupt Enable"]
    pub mod CFF_IE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the comparator flag falling interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the comparator flag falling interrupt when CFF is set."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Round-Robin Flag Interrupt Enable"]
    pub mod RRF_IE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the round-robin flag interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the round-robin flag interrupt when the comparison result changes for a given channel."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Comparator Status"]
pub mod CSR {
    #[doc = "Analog Comparator Flag Rising"]
    pub mod CFR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const NOT_DETECTED: u32 = 0;
            #[doc = "Detected"]
            pub const DETECTED: u32 = 1;
        }
    }
    #[doc = "Analog Comparator Flag Falling"]
    pub mod CFF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const NOT_DETECTED: u32 = 0;
            #[doc = "Detected"]
            pub const DETECTED: u32 = 1;
        }
    }
    #[doc = "Round-Robin Flag"]
    pub mod RRF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const NOT_DETECTED: u32 = 0;
            #[doc = "Detected"]
            pub const DETECTED: u32 = 1;
        }
    }
    #[doc = "Analog Comparator Output"]
    pub mod COUT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Round Robin Control Register 0"]
pub mod RRCR0 {
    #[doc = "Round-Robin Enable"]
    pub mod RR_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Round-Robin Trigger Select"]
    pub mod RR_TRG_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External trigger"]
            pub const ENABLE: u32 = 0;
            #[doc = "Internal trigger"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Number of Sample Clocks"]
    pub mod RR_NSAM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0 clock"]
            pub const WAIT_0: u32 = 0;
            #[doc = "1 clock"]
            pub const WAIT_1: u32 = 1;
            #[doc = "2 clocks"]
            pub const WAIT_2: u32 = 2;
            #[doc = "3 clocks"]
            pub const WAIT_3: u32 = 3;
        }
    }
    #[doc = "Round Robin Clock Source Select"]
    pub mod RR_CLK_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select Round Robin clock Source 0"]
            pub const RR0: u32 = 0;
            #[doc = "Select Round Robin clock Source 1"]
            pub const RR1: u32 = 1;
            #[doc = "Select Round Robin clock Source 2"]
            pub const RR2: u32 = 2;
            #[doc = "Select Round Robin clock Source 3"]
            pub const RR3: u32 = 3;
        }
    }
    #[doc = "Initialization Delay Modulus"]
    pub mod RR_INITMOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "63 cycles (same as 111111b)"]
            pub const MOD_63: u32 = 0;
            #[doc = "1 to 63 cycles"]
            pub const MOD_1_63_1: u32 = 1;
            #[doc = "1 to 63 cycles"]
            pub const MOD_1_63_2: u32 = 2;
            #[doc = "1 to 63 cycles"]
            pub const MOD_1_63_3: u32 = 3;
            #[doc = "1 to 63 cycles"]
            pub const MOD_1_63_4: u32 = 4;
            #[doc = "1 to 63 cycles"]
            pub const MOD_1_63_5: u32 = 5;
            #[doc = "1 to 63 cycles"]
            pub const MOD_1_63_6: u32 = 6;
            #[doc = "1 to 63 cycles"]
            pub const MOD_1_63_7: u32 = 7;
            #[doc = "1 to 63 cycles"]
            pub const MOD_1_63_8: u32 = 8;
            #[doc = "1 to 63 cycles"]
            pub const MOD_1_63_9: u32 = 9;
        }
    }
    #[doc = "Number of Sample for One Channel"]
    pub mod RR_SAMPLE_CNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 samples"]
            pub const SAMPLE_0: u32 = 0;
            #[doc = "2 samples"]
            pub const SAMPLE_1: u32 = 1;
            #[doc = "3 samples"]
            pub const SAMPLE_2: u32 = 2;
            #[doc = "4 samples"]
            pub const SAMPLE_3: u32 = 3;
            #[doc = "5 samples"]
            pub const SAMPLE_4: u32 = 4;
            #[doc = "6 samples"]
            pub const SAMPLE_5: u32 = 5;
            #[doc = "7 samples"]
            pub const SAMPLE_6: u32 = 6;
            #[doc = "8 samples"]
            pub const SAMPLE_7: u32 = 7;
            #[doc = "9 samples"]
            pub const SAMPLE_8: u32 = 8;
            #[doc = "10 samples"]
            pub const SAMPLE_9: u32 = 9;
            #[doc = "11 samples"]
            pub const SAMPLE_10: u32 = 10;
            #[doc = "12 samples"]
            pub const SAMPLE_11: u32 = 11;
            #[doc = "13 samples"]
            pub const SAMPLE_12: u32 = 12;
            #[doc = "14 samples"]
            pub const SAMPLE_13: u32 = 13;
            #[doc = "15 samples"]
            pub const SAMPLE_14: u32 = 14;
            #[doc = "16 samples"]
            pub const SAMPLE_15: u32 = 15;
        }
    }
    #[doc = "Sample Time Threshold"]
    pub mod RR_SAMPLE_THRESHOLD {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "At least 1 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_0: u32 = 0;
            #[doc = "At least 2 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_1: u32 = 1;
            #[doc = "At least 3 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_2: u32 = 2;
            #[doc = "At least 4 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_3: u32 = 3;
            #[doc = "At least 5 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_4: u32 = 4;
            #[doc = "At least 6 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_5: u32 = 5;
            #[doc = "At least 7 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_6: u32 = 6;
            #[doc = "At least 8 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_7: u32 = 7;
            #[doc = "At least 9 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_8: u32 = 8;
            #[doc = "At least 10 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_9: u32 = 9;
            #[doc = "At least 11 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_10: u32 = 10;
            #[doc = "At least 12 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_11: u32 = 11;
            #[doc = "At least 13 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_12: u32 = 12;
            #[doc = "At least 14 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_13: u32 = 13;
            #[doc = "At least 15 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_14: u32 = 14;
            #[doc = "At least 16 sampled \"1\", the final result is \"1\""]
            pub const SAMPLE_15: u32 = 15;
        }
    }
}
#[doc = "Round Robin Control Register 1"]
pub mod RRCR1 {
    #[doc = "Channel 0 Input Enable in Trigger Mode"]
    pub mod RR_CH0EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Channel 1 Input Enable in Trigger Mode"]
    pub mod RR_CH1EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Channel 2 Input Enable in Trigger Mode"]
    pub mod RR_CH2EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Channel 3 Input Enable in Trigger Mode"]
    pub mod RR_CH3EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Channel 4 Input Enable in Trigger Mode"]
    pub mod RR_CH4EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Channel 5 Input Enable in Trigger Mode"]
    pub mod RR_CH5EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Channel 6 Input Enable in Trigger Mode"]
    pub mod RR_CH6EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Channel 7 Input Enable in Trigger Mode"]
    pub mod RR_CH7EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Fixed Port"]
    pub mod FIXP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fix the plus port. Sweep only the inputs to the minus port."]
            pub const FIX_PLUS: u32 = 0;
            #[doc = "Fix the minus port. Sweep only the inputs to the plus port."]
            pub const FIX_MINUS: u32 = 1;
        }
    }
    #[doc = "Fixed Channel Select"]
    pub mod FIXCH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel 0"]
            pub const FIX_CH0: u32 = 0;
            #[doc = "Channel 1"]
            pub const FIX_CH1: u32 = 1;
            #[doc = "Channel 2"]
            pub const FIX_CH2: u32 = 2;
            #[doc = "Channel 3"]
            pub const FIX_CH3: u32 = 3;
            #[doc = "Channel 4"]
            pub const FIX_CH4: u32 = 4;
            #[doc = "Channel 5"]
            pub const FIX_CH5: u32 = 5;
            #[doc = "Channel 6"]
            pub const FIX_CH6: u32 = 6;
            #[doc = "Channel 7"]
            pub const FIX_CH7: u32 = 7;
        }
    }
}
#[doc = "Round Robin Control and Status"]
pub mod RRCSR {
    #[doc = "Comparison Result for Channel 0"]
    pub mod RR_CH0OUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparison Result for Channel 1"]
    pub mod RR_CH1OUT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparison Result for Channel 2"]
    pub mod RR_CH2OUT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparison Result for Channel 3"]
    pub mod RR_CH3OUT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparison Result for Channel 4"]
    pub mod RR_CH4OUT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparison Result for Channel 5"]
    pub mod RR_CH5OUT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparison Result for Channel 6"]
    pub mod RR_CH6OUT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparison Result for Channel 7"]
    pub mod RR_CH7OUT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Round Robin Status"]
pub mod RRSR {
    #[doc = "Channel 0 Input Changed Flag"]
    pub mod RR_CH0F {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not different"]
            pub const NOT_DIFFERENT: u32 = 0;
            #[doc = "Different"]
            pub const DIFFERENT: u32 = 1;
        }
    }
    #[doc = "Channel 1 Input Changed Flag"]
    pub mod RR_CH1F {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not different"]
            pub const NOT_DIFFERENT: u32 = 0;
            #[doc = "Different"]
            pub const DIFFERENT: u32 = 1;
        }
    }
    #[doc = "Channel 2 Input Changed Flag"]
    pub mod RR_CH2F {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not different"]
            pub const NOT_DIFFERENT: u32 = 0;
            #[doc = "Different"]
            pub const DIFFERENT: u32 = 1;
        }
    }
    #[doc = "Channel 3 Input Changed Flag"]
    pub mod RR_CH3F {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not different"]
            pub const NOT_DIFFERENT: u32 = 0;
            #[doc = "Different"]
            pub const DIFFERENT: u32 = 1;
        }
    }
    #[doc = "Channel 4 Input Changed Flag"]
    pub mod RR_CH4F {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not different"]
            pub const NOT_DIFFERENT: u32 = 0;
            #[doc = "Different"]
            pub const DIFFERENT: u32 = 1;
        }
    }
    #[doc = "Channel 5 Input Changed Flag"]
    pub mod RR_CH5F {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not different"]
            pub const NOT_DIFFERENT: u32 = 0;
            #[doc = "Different"]
            pub const DIFFERENT: u32 = 1;
        }
    }
    #[doc = "Channel 6 Input Changed Flag"]
    pub mod RR_CH6F {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not different"]
            pub const NOT_DIFFERENT: u32 = 0;
            #[doc = "Different"]
            pub const DIFFERENT: u32 = 1;
        }
    }
    #[doc = "Channel 7 Input Changed Flag"]
    pub mod RR_CH7F {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not different"]
            pub const NOT_DIFFERENT: u32 = 0;
            #[doc = "Different"]
            pub const DIFFERENT: u32 = 1;
        }
    }
}
#[doc = "Round Robin Control Register 2"]
pub mod RRCR2 {
    #[doc = "Number of Sample Clocks"]
    pub mod RR_TIMER_RELOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Round-Robin Internal Timer Enable"]
    pub mod RR_TIMER_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
}
