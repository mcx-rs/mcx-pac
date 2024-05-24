#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "LPSPI"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RWRegister<u32>,
    _reserved0: [u8; 0x8],
    #[doc = "Control"]
    pub CR: crate::RWRegister<u32>,
    #[doc = "Status"]
    pub SR: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub IER: crate::RWRegister<u32>,
    #[doc = "DMA Enable"]
    pub DER: crate::RWRegister<u32>,
    #[doc = "Configuration 0"]
    pub CFGR0: crate::RWRegister<u32>,
    #[doc = "Configuration 1"]
    pub CFGR1: crate::RWRegister<u32>,
    _reserved1: [u8; 0x8],
    #[doc = "Data Match 0"]
    pub DMR0: crate::RWRegister<u32>,
    #[doc = "Data Match 1"]
    pub DMR1: crate::RWRegister<u32>,
    _reserved2: [u8; 0x8],
    #[doc = "Clock Configuration"]
    pub CCR: crate::RWRegister<u32>,
    #[doc = "Clock Configuration 1"]
    pub CCR1: crate::RWRegister<u32>,
    _reserved3: [u8; 0x10],
    #[doc = "FIFO Control"]
    pub FCR: crate::RWRegister<u32>,
    #[doc = "FIFO Status"]
    pub FSR: crate::RWRegister<u32>,
    #[doc = "Transmit Command"]
    pub TCR: crate::RWRegister<u32>,
    #[doc = "Transmit Data"]
    pub TDR: crate::RWRegister<u32>,
    _reserved4: [u8; 0x8],
    #[doc = "Receive Status"]
    pub RSR: crate::RWRegister<u32>,
    #[doc = "Receive Data"]
    pub RDR: crate::RWRegister<u32>,
    #[doc = "Receive Data Read Only"]
    pub RDROR: crate::RWRegister<u32>,
    _reserved5: [u8; 0x380],
    #[doc = "Transmit Command Burst"]
    pub TCBR: crate::RWRegister<u32>,
    #[doc = "Transmit Data Burst"]
    pub TDBR: [crate::RWRegister<u32>; 128usize],
    #[doc = "Receive Data Burst"]
    pub RDBR: [crate::RWRegister<u32>; 128usize],
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Module Identification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {
            #[doc = "Standard feature set supporting a 32-bit shift register."]
            pub const STANDARD: u32 = 4;
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
    #[doc = "Transmit FIFO Size"]
    pub mod TXFIFO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Size"]
    pub mod RXFIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCS Number"]
    pub mod PCSNUM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control"]
pub mod CR {
    #[doc = "Module Enable"]
    pub mod MEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Software Reset"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not reset"]
            pub const NOT_RESET: u32 = 0;
            #[doc = "Reset"]
            pub const RESET: u32 = 1;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBGEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Reset Transmit FIFO"]
    pub mod RTF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Reset"]
            pub const TXFIFO_RST: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Reset Receive FIFO"]
    pub mod RRF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Reset"]
            pub const RXFIFO_RST: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Status"]
pub mod SR {
    #[doc = "Transmit Data Flag"]
    pub mod TDF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit data not requested"]
            pub const TXDATA_NOT_REQST: u32 = 0;
            #[doc = "Transmit data is requested"]
            pub const TXDATA_REQST: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Data Flag"]
    pub mod RDF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive data not ready"]
            pub const NOTREADY: u32 = 0;
            #[doc = "Receive data is ready"]
            pub const READY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word Complete Flag"]
    pub mod WCF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not complete"]
            pub const NOT_COMPLETED: u32 = 0;
            #[doc = "Complete"]
            pub const COMPLETED: u32 = 1;
        }
    }
    #[doc = "Frame Complete Flag"]
    pub mod FCF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not complete"]
            pub const NOT_COMPLETED: u32 = 0;
            #[doc = "Complete"]
            pub const COMPLETED: u32 = 1;
        }
    }
    #[doc = "Transfer Complete Flag"]
    pub mod TCF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not complete"]
            pub const NOT_COMPLETED: u32 = 0;
            #[doc = "Complete"]
            pub const COMPLETED: u32 = 1;
        }
    }
    #[doc = "Transmit Error Flag"]
    pub mod TEF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No underrun"]
            pub const NO_UNDERRUN: u32 = 0;
            #[doc = "Underrun"]
            pub const UNDERRUN: u32 = 1;
        }
    }
    #[doc = "Receive Error Flag"]
    pub mod REF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overflow"]
            pub const NOT_OVERFLOWED: u32 = 0;
            #[doc = "Overflow"]
            pub const OVERFLOWED: u32 = 1;
        }
    }
    #[doc = "Data Match Flag"]
    pub mod DMF {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No match"]
            pub const NO_MATCH: u32 = 0;
            #[doc = "Match"]
            pub const MATCH: u32 = 1;
        }
    }
    #[doc = "Module Busy Flag"]
    pub mod MBF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "LPSPI is idle"]
            pub const IDLE: u32 = 0;
            #[doc = "LPSPI is busy"]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable"]
pub mod IER {
    #[doc = "Transmit Data Interrupt Enable"]
    pub mod TDIE {
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
    #[doc = "Receive Data Interrupt Enable"]
    pub mod RDIE {
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
    #[doc = "Word Complete Interrupt Enable"]
    pub mod WCIE {
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
    #[doc = "Frame Complete Interrupt Enable"]
    pub mod FCIE {
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
    #[doc = "Transfer Complete Interrupt Enable"]
    pub mod TCIE {
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
    #[doc = "Transmit Error Interrupt Enable"]
    pub mod TEIE {
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
    #[doc = "Receive Error Interrupt Enable"]
    pub mod REIE {
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
    #[doc = "Data Match Interrupt Enable"]
    pub mod DMIE {
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
}
#[doc = "DMA Enable"]
pub mod DER {
    #[doc = "Transmit Data DMA Enable"]
    pub mod TDDE {
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
    #[doc = "Receive Data DMA Enable"]
    pub mod RDDE {
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
    #[doc = "Frame Complete DMA Enable"]
    pub mod FCDE {
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
}
#[doc = "Configuration 0"]
pub mod CFGR0 {
    #[doc = "Host Request Enable"]
    pub mod HREN {
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
    #[doc = "Host Request Polarity"]
    pub mod HRPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active high"]
            pub const DISABLED: u32 = 0;
            #[doc = "Active low"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Host Request Select"]
    pub mod HRSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HREQ pin"]
            pub const HREQPIN: u32 = 0;
            #[doc = "Input trigger"]
            pub const INPUT_TRIGGER: u32 = 1;
        }
    }
    #[doc = "Host Request Direction"]
    pub mod HRDIR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input"]
            pub const INPUT: u32 = 0;
            #[doc = "Output"]
            pub const OUTPUT: u32 = 1;
        }
    }
    #[doc = "Circular FIFO Enable"]
    pub mod CIRFIFO {
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
    #[doc = "Receive Data Match Only"]
    pub mod RDMO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const STORED: u32 = 0;
            #[doc = "Enable"]
            pub const DISCARDED: u32 = 1;
        }
    }
}
#[doc = "Configuration 1"]
pub mod CFGR1 {
    #[doc = "Master Mode"]
    pub mod MASTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave mode"]
            pub const SLAVE_MODE: u32 = 0;
            #[doc = "Master mode"]
            pub const MASTER_MODE: u32 = 1;
        }
    }
    #[doc = "Sample Point"]
    pub mod SAMPLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SCK edge"]
            pub const ON_SCK_EDGE: u32 = 0;
            #[doc = "Delayed SCK edge"]
            pub const ON_DELAYED_SCK_EDGE: u32 = 1;
        }
    }
    #[doc = "Automatic PCS"]
    pub mod AUTOPCS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "No Stall"]
    pub mod NOSTALL {
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
    #[doc = "Partial Enable"]
    pub mod PARTIAL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Discard"]
            pub const DISCARDED: u32 = 0;
            #[doc = "Store"]
            pub const STORED: u32 = 1;
        }
    }
    #[doc = "Peripheral Chip Select Polarity"]
    pub mod PCSPOL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Match Configuration"]
    pub mod MATCFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Match first data word with compare word"]
            pub const ENABLED_FIRSTDATAMATCH: u32 = 2;
            #[doc = "Match any data word with compare word"]
            pub const ENABLED_ANYDATAMATCH: u32 = 3;
            #[doc = "Sequential match, first data word"]
            pub const ENABLED_DATAMATCH_100: u32 = 4;
            #[doc = "Sequential match, any data word"]
            pub const ENABLED_DATAMATCH_101: u32 = 5;
            #[doc = "Match first data word (masked) with compare word (masked)"]
            pub const ENABLED_DATAMATCH_110: u32 = 6;
            #[doc = "Match any data word (masked) with compare word (masked)"]
            pub const ENABLED_DATAMATCH_111: u32 = 7;
        }
    }
    #[doc = "Pin Configuration"]
    pub mod PINCFG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SIN is used for input data; SOUT is used for output data."]
            pub const SIN_IN_SOUT_OUT: u32 = 0;
            #[doc = "SIN is used for both input and output data. Only half-duplex serial transfers are supported."]
            pub const SIN_BOTH_IN_OUT: u32 = 1;
            #[doc = "SOUT is used for both input and output data. Only half-duplex serial transfers are supported."]
            pub const SOUT_BOTH_IN_OUT: u32 = 2;
            #[doc = "SOUT is used for input data; SIN is used for output data."]
            pub const SOUT_IN_SIN_OUT: u32 = 3;
        }
    }
    #[doc = "Output Configuration"]
    pub mod OUTCFG {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output data retains last value."]
            pub const RETAIN_LASTVALUE: u32 = 0;
            #[doc = "Output data is 3-stated."]
            pub const TRISTATED: u32 = 1;
        }
    }
    #[doc = "Peripheral Chip Select Configuration"]
    pub mod PCSCFG {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PCS[3:2] are configured for chip select function"]
            pub const CHIP_SELECT: u32 = 0;
            #[doc = "PCS[3:2] are configured for half-duplex 4-bit transfers (PCS[3:2] = DATA[3:2])"]
            pub const HALFDUPLEX4BIT: u32 = 1;
        }
    }
}
#[doc = "Data Match 0"]
pub mod DMR0 {
    #[doc = "Match 0 Value"]
    pub mod MATCH0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Match 1"]
pub mod DMR1 {
    #[doc = "Match 1 Value"]
    pub mod MATCH1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Configuration"]
pub mod CCR {
    #[doc = "SCK Divider"]
    pub mod SCKDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay Between Transfers"]
    pub mod DBT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCS-to-SCK Delay"]
    pub mod PCSSCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SCK-to-PCS Delay"]
    pub mod SCKPCS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Configuration 1"]
pub mod CCR1 {
    #[doc = "SCK Setup"]
    pub mod SCKSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SCK Hold"]
    pub mod SCKHLD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCS to PCS delay"]
    pub mod PCSPCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SCK Inter-Frame Delay"]
    pub mod SCKSCK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO Control"]
pub mod FCR {
    #[doc = "Transmit FIFO Watermark"]
    pub mod TXWATER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Watermark"]
    pub mod RXWATER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO Status"]
pub mod FSR {
    #[doc = "Transmit FIFO Count"]
    pub mod TXCOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Count"]
    pub mod RXCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Command"]
pub mod TCR {
    #[doc = "Frame Size"]
    pub mod FRAMESZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transfer Width"]
    pub mod WIDTH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1-bit transfer"]
            pub const ONEBIT: u32 = 0;
            #[doc = "2-bit transfer"]
            pub const TWOBIT: u32 = 1;
            #[doc = "4-bit transfer"]
            pub const FOURBIT: u32 = 2;
        }
    }
    #[doc = "Transmit Data Mask"]
    pub mod TXMSK {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal transfer"]
            pub const NORMAL: u32 = 0;
            #[doc = "Mask transmit data"]
            pub const MASK: u32 = 1;
        }
    }
    #[doc = "Receive Data Mask"]
    pub mod RXMSK {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal transfer"]
            pub const NORMAL: u32 = 0;
            #[doc = "Receive data is masked"]
            pub const MASK: u32 = 1;
        }
    }
    #[doc = "Continuing Command"]
    pub mod CONTC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command word for start of new transfer"]
            pub const START: u32 = 0;
            #[doc = "Command word for continuing transfer"]
            pub const CONTINUE: u32 = 1;
        }
    }
    #[doc = "Continuous Transfer"]
    pub mod CONT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continuous transfer is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Continuous transfer is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Byte Swap"]
    pub mod BYSW {
        pub const offset: u32 = 22;
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
    #[doc = "LSB First"]
    pub mod LSBF {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data is transferred MSB first"]
            pub const MSB_FIRST: u32 = 0;
            #[doc = "Data is transferred LSB first"]
            pub const LSB_FIRST: u32 = 1;
        }
    }
    #[doc = "Peripheral Chip Select"]
    pub mod PCS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transfer using PCS[0]"]
            pub const TX_PCS0: u32 = 0;
            #[doc = "Transfer using PCS[1]"]
            pub const TX_PCS1: u32 = 1;
            #[doc = "Transfer using PCS[2]"]
            pub const TX_PCS2: u32 = 2;
            #[doc = "Transfer using PCS[3]"]
            pub const TX_PCS3: u32 = 3;
        }
    }
    #[doc = "Prescaler Value"]
    pub mod PRESCALE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDEBY1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDEBY2: u32 = 1;
            #[doc = "Divide by 4"]
            pub const DIVIDEBY4: u32 = 2;
            #[doc = "Divide by 8"]
            pub const DIVIDEBY8: u32 = 3;
            #[doc = "Divide by 16"]
            pub const DIVIDEBY16: u32 = 4;
            #[doc = "Divide by 32"]
            pub const DIVIDEBY32: u32 = 5;
            #[doc = "Divide by 64"]
            pub const DIVIDEBY64: u32 = 6;
            #[doc = "Divide by 128"]
            pub const DIVIDEBY128: u32 = 7;
        }
    }
    #[doc = "Clock Phase"]
    pub mod CPHA {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Captured"]
            pub const CAPTURED: u32 = 0;
            #[doc = "Changed"]
            pub const CHANGED: u32 = 1;
        }
    }
    #[doc = "Clock Polarity"]
    pub mod CPOL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inactive low"]
            pub const INACTIVE_LOW: u32 = 0;
            #[doc = "Inactive high"]
            pub const INACTIVE_HIGH: u32 = 1;
        }
    }
}
#[doc = "Transmit Data"]
pub mod TDR {
    #[doc = "Transmit Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Status"]
pub mod RSR {
    #[doc = "Start Of Frame"]
    pub mod SOF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Subsequent data word"]
            pub const NEXT_DATAWORD: u32 = 0;
            #[doc = "First data word"]
            pub const FIRST_DATAWORD: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX FIFO Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Data"]
pub mod RDR {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Data Read Only"]
pub mod RDROR {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Command Burst"]
pub mod TCBR {
    #[doc = "Command Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Data Burst"]
pub mod TDBR {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Data Burst"]
pub mod RDBR {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
