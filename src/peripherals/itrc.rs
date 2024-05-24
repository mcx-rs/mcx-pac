#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "Intrusion and Tamper Response Controller"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "ITRC outputs and IN0 to IN15 Status"]
    pub STATUS: crate::RWRegister<u32>,
    #[doc = "ITRC IN16 to IN47 Status"]
    pub STATUS1: crate::RWRegister<u32>,
    #[doc = "no description available"]
    pub OUTx_SEL: [outx_sel::RegisterBlock; 7usize],
    _reserved0: [u8; 0x8],
    #[doc = "no description available"]
    pub OUTx_SEL_1: [outx_sel_1::RegisterBlock; 7usize],
    _reserved1: [u8; 0x8],
    #[doc = "no description available"]
    pub OUTx_SEL_2: [outx_sel_2::RegisterBlock; 7usize],
    _reserved2: [u8; 0x30],
    #[doc = "Software event 0"]
    pub SW_EVENT0: crate::RWRegister<u32>,
    #[doc = "Software event 1"]
    pub SW_EVENT1: crate::RWRegister<u32>,
}
#[doc = "ITRC outputs and IN0 to IN15 Status"]
pub mod STATUS {
    #[doc = "GDET0 & 1 interrupt."]
    pub mod IN0_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "TDET tamper output."]
    pub mod IN1_STATUS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Code Watchdog 0 interrupt."]
    pub mod IN2_STATUS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "VDD_MAIN volt tamper output."]
    pub mod IN3_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SPC VDD_CORE_LVD detect."]
    pub mod IN4_STATUS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Watch Dog timer event occurred."]
    pub mod IN5_STATUS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Flash ECC mismatch event occurred."]
    pub mod IN6_STATUS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "AHB secure bus checkers detected illegal access."]
    pub mod IN7_STATUS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ELS error event occurred."]
    pub mod IN8_STATUS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SPC VDD_CORE glitch detect event occurred."]
    pub mod IN9_STATUS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "PKC module detected an error event."]
    pub mod IN10_STATUS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Code Watchdog 1 interrupt."]
    pub mod IN11_STATUS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Watchdog 1 timer event interrupt."]
    pub mod IN112_STATUS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FREQME out of range status output."]
    pub mod IN113_STATUS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Software event 0 occurred."]
    pub mod IN14_STATUS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Software event 1 occurred."]
    pub mod IN15_STATUS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ITRC triggered ITRC_IRQ output."]
    pub mod OUT0_STATUS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ITRC triggered ELS_RESET to clear ELS key store."]
    pub mod OUT1_STATUS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ITRC triggered PUF_ZEROIZE to clear PUF key store and RAM."]
    pub mod OUT2_STATUS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ITRC triggered RAM_ZEROIZE."]
    pub mod OUT3_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ITRC triggered CHIP_RESET to reset the chip after all other response process finished."]
    pub mod OUT4_STATUS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ITRC triggered TMPR_OUT0 internal signal connected to various on-chip multiplexers."]
    pub mod OUT5_STATUS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ITRC triggered TMPR_OUT1 internal signal connected to various on-chip multiplexers."]
    pub mod OUT6_STATUS {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "ITRC IN16 to IN47 Status"]
pub mod STATUS1 {
    #[doc = "SSPC VDD_SYS_LVD detect event occurred."]
    pub mod IN16_STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SPC VDD_IO_LVD detect event occurred."]
    pub mod IN17_STATUS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Reserved"]
    pub mod IN18_STATUS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Reserved"]
    pub mod IN19_STATUS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "VDD_MAIN clock tamper output event occurred."]
    pub mod IN20_STATUS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "INTM interrupt monitor error 3~0 event occurred."]
    pub mod IN24_21_STATUS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "MSF SOCTRIM 7~0 ECC error event occurred."]
    pub mod IN32_25_STATUS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "GDET0/1 SFR error event occurred."]
    pub mod IN33_STATUS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SPC VDD_CORE high voltage detect event occurred."]
    pub mod IN34_STATUS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SPC VDD_SYS_HVD high voltage detect event occurred."]
    pub mod IN35_STATUS {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SPC VDD_IO high voltage detect event occurred."]
    pub mod IN36_STATUS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FLEXSPI GCM error event occurred."]
    pub mod IN37_STATUS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SM3 SGI error event occurred."]
    pub mod IN46_STATUS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "TRNG HW error event occurred."]
    pub mod IN47_STATUS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output not triggered."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output has been triggered."]
            pub const ENABLE: u32 = 1;
        }
    }
}
pub mod outx_sel {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Trigger Source IN0 to IN15 selector"]
        pub OUT_SEL: [crate::RWRegister<u32>; 2usize],
    }
    #[doc = "Trigger Source IN0 to IN15 selector"]
    pub mod OUT_SEL {
        #[doc = "Selects digital glitch detector as a trigger source."]
        pub mod IN0_SELn {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects TDET event as a trigger source."]
        pub mod IN1_SELn {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects Code Watchdog 0 event as a trigger source."]
        pub mod IN2_SELn {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects VDD_MAIN voltage tamper event as a trigger source."]
        pub mod IN3_SELn {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects low-voltage event on VDD_CORE rail as a trigger source."]
        pub mod IN4_SELn {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects Watchdog 0 timer event as a trigger source."]
        pub mod IN5_SELn {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects Flash ECC mismatch event as a trigger source."]
        pub mod IN6_SELn {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects AHB secure bus or MBC bus illegal access event as a trigger source."]
        pub mod IN7_SELn {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects ELS error event as a trigger source."]
        pub mod IN8_SELn {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects SPC VDD_CORE glitch detector as a trigger source."]
        pub mod IN9_SELn {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects PKC error event as a trigger source."]
        pub mod IN10_SELn {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects Code Watchdog 1 event as a trigger source."]
        pub mod IN11_SELn {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects Watchdog 1 timer event as a trigger source."]
        pub mod IN12_SELn {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects FREQME out of range status output as a trigger source."]
        pub mod IN13_SELn {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects software event 0 as a trigger source."]
        pub mod IN14_SELn {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects software event 1 as a trigger source."]
        pub mod IN15_SELn {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
pub mod outx_sel_1 {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Trigger Source IN16 to IN31 selector"]
        pub OUT_SEL_1: [crate::RWRegister<u32>; 2usize],
    }
    #[doc = "Trigger Source IN16 to IN31 selector"]
    pub mod OUT_SEL_1 {
        #[doc = "Selects SPC VDD_SYS_LVD detect as a trigger source."]
        pub mod IN16_SELn {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects SPC VDD_IO_LVD detect as a trigger source."]
        pub mod IN17_SELn {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Reserved."]
        pub mod IN18_SELn {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects VDD_MAIN temperature tamper output event as a trigger source."]
        pub mod IN19_SELn {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects VDD_MAIN clock tamper output event as a trigger source."]
        pub mod IN20_SELn {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects INTM interrupt monitor error 0 event as a trigger source."]
        pub mod IN21_SELn {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects INTM interrupt monitor error 1 event as a trigger source."]
        pub mod IN22_SELn {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects INTM interrupt monitor error 2 event as a trigger source."]
        pub mod IN23_SELn {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects INTM interrupt monitor error 3 event as a trigger source."]
        pub mod IN24_SELn {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects MSF SOCTRIM 0 ECC error event as a trigger source."]
        pub mod IN25_SELn {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects MSF SOCTRIM 1 ECC error event as a trigger source."]
        pub mod IN26_SELn {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects MSF SOCTRIM 2 ECC error event as a trigger source."]
        pub mod IN27_SELn {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects MSF SOCTRIM 3 ECC error event as a trigger source."]
        pub mod IN28_SELn {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects MSF SOCTRIM 4 ECC error event as a trigger source."]
        pub mod IN29_SELn {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects MSF SOCTRIM 5 ECC error event as a trigger source."]
        pub mod IN30_SELn {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects MSF SOCTRIM 6 ECC error event as a trigger source."]
        pub mod IN31_SELn {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
pub mod outx_sel_2 {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Trigger source IN32 to IN47 selector"]
        pub OUT_SEL_2: [crate::RWRegister<u32>; 2usize],
    }
    #[doc = "Trigger source IN32 to IN47 selector"]
    pub mod OUT_SEL_2 {
        #[doc = "Selects MSF SOCTRIM 7 ECC error event as a trigger source."]
        pub mod IN32_SELn {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects GDET0 & 1 SFR error detect as a trigger source."]
        pub mod IN33_SELn {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects SPC VDD_CORE_HVD as a trigger source."]
        pub mod IN34_SELn {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects VDD_SYS_HVD as a trigger source."]
        pub mod IN35_SELn {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects VDD_IO_HVD as a trigger source."]
        pub mod IN36_SELn {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects FLEXSPI GCM error as a trigger source."]
        pub mod IN37_SELn {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects SM3 SGI error as a trigger source."]
        pub mod IN46_SELn {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Selects TRNG HW Error as a trigger source."]
        pub mod IN47_SELn {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
#[doc = "Software event 0"]
pub mod SW_EVENT0 {
    #[doc = "Trigger software event 0."]
    pub mod TRIGGER_SW_EVENT_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Software event 1"]
pub mod SW_EVENT1 {
    #[doc = "Trigger software event 1."]
    pub mod TRIGGER_SW_EVENT_1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
