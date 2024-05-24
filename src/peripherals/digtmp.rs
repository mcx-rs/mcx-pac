#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "TDET"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "Control"]
    pub CR: crate::RWRegister<u32>,
    #[doc = "Status"]
    pub SR: crate::RWRegister<u32>,
    #[doc = "Lock"]
    pub LR: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub IER: crate::RWRegister<u32>,
    #[doc = "Tamper Seconds"]
    pub TSR: crate::RWRegister<u32>,
    #[doc = "Tamper Enable"]
    pub TER: crate::RWRegister<u32>,
    #[doc = "Pin Direction"]
    pub PDR: crate::RWRegister<u32>,
    #[doc = "Pin Polarity"]
    pub PPR: crate::RWRegister<u32>,
    #[doc = "Active Tamper"]
    pub ATR: [crate::RWRegister<u32>; 2usize],
    _reserved1: [u8; 0x8],
    #[doc = "Pin Glitch Filter"]
    pub PGFR: [crate::RWRegister<u32>; 8usize],
}
#[doc = "Control"]
pub mod CR {
    #[doc = "Software Reset"]
    pub mod SWR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Perform a software reset"]
            pub const SW_RESET: u32 = 1;
        }
    }
    #[doc = "Digital Tamper Enable"]
    pub mod DEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables TDET clock and prescaler"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables TDET clock and prescaler"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Tamper Force System Reset"]
    pub mod TFSR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not force chip reset"]
            pub const NO_RESET: u32 = 0;
            #[doc = "Force chip reset"]
            pub const RESET: u32 = 1;
        }
    }
    #[doc = "Update Mode"]
    pub mod UM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Allows the clearing of interrupts"]
            pub const CLEAR_INTS: u32 = 1;
        }
    }
    #[doc = "Active Tamper Clock Source"]
    pub mod ATCS0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 Hz prescaler clock"]
            pub const FREQ_1_HZ: u32 = 0;
            #[doc = "64 Hz prescaler clock"]
            pub const FREQ_64_HZ: u32 = 1;
        }
    }
    #[doc = "Active Tamper Clock Source"]
    pub mod ATCS1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 Hz prescaler clock"]
            pub const FREQ_1_HZ: u32 = 0;
            #[doc = "64 Hz prescaler clock"]
            pub const FREQ_64_HZ: u32 = 1;
        }
    }
    #[doc = "Disable Prescaler On Tamper"]
    pub mod DISTAM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Automatically disables the prescaler after tamper detection"]
            pub const AUTO_DIS: u32 = 1;
        }
    }
    #[doc = "Digital Tamper Prescaler"]
    pub mod DPR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status"]
pub mod SR {
    #[doc = "Digital Tamper Flag"]
    pub mod DTF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TDET tampering not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "TDET tampering detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Acknowledge Flag"]
    pub mod TAF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Digital Tamper Flag (SR[DTF]) is clear or chip reset has not occurred after Digital Tamper Flag (SR[DTF]) was set."]
            pub const NOT_OCCUR: u32 = 0;
            #[doc = "Chip reset has occurred after Digital Tamper Flag (SR[DTF]) was set."]
            pub const OCCUR: u32 = 1;
        }
    }
    #[doc = "Tamper Input n Flag"]
    pub mod TIF0 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On-chip tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "On-chip tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Input n Flag"]
    pub mod TIF1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On-chip tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "On-chip tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Input n Flag"]
    pub mod TIF2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On-chip tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "On-chip tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Input n Flag"]
    pub mod TIF3 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On-chip tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "On-chip tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Input n Flag"]
    pub mod TIF4 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On-chip tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "On-chip tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Input n Flag"]
    pub mod TIF5 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On-chip tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "On-chip tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Input n Flag"]
    pub mod TIF6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On-chip tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "On-chip tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Input n Flag"]
    pub mod TIF7 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On-chip tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "On-chip tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Input n Flag"]
    pub mod TIF8 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On-chip tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "On-chip tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Input n Flag"]
    pub mod TIF9 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On-chip tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "On-chip tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Flag"]
    pub mod TPF0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "Pin tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Flag"]
    pub mod TPF1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "Pin tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Flag"]
    pub mod TPF2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "Pin tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Flag"]
    pub mod TPF3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "Pin tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Flag"]
    pub mod TPF4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "Pin tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Flag"]
    pub mod TPF5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "Pin tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Flag"]
    pub mod TPF6 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "Pin tamper detected"]
            pub const DET: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Flag"]
    pub mod TPF7 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin tamper not detected"]
            pub const NO_DET: u32 = 0;
            #[doc = "Pin tamper detected"]
            pub const DET: u32 = 1;
        }
    }
}
#[doc = "Lock"]
pub mod LR {
    #[doc = "Control Register Lock"]
    pub mod CRL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Status Register Lock"]
    pub mod SRL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Lock Register Lock"]
    pub mod LRL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Interrupt Enable Lock"]
    pub mod IEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Tamper Seconds Lock"]
    pub mod TSL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Tamper Enable Lock"]
    pub mod TEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Pin Direction Lock"]
    pub mod PDL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Pin Polarity Lock"]
    pub mod PPL {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Active Tamper Lock"]
    pub mod ATL0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Active Tamper Lock"]
    pub mod ATL1 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Glitch Filter Lock"]
    pub mod GFL0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Glitch Filter Lock"]
    pub mod GFL1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Glitch Filter Lock"]
    pub mod GFL2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Glitch Filter Lock"]
    pub mod GFL3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Glitch Filter Lock"]
    pub mod GFL4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Glitch Filter Lock"]
    pub mod GFL5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Glitch Filter Lock"]
    pub mod GFL6 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
    #[doc = "Glitch Filter Lock"]
    pub mod GFL7 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked and writes are ignored"]
            pub const LOCK: u32 = 0;
            #[doc = "Not locked and writes complete as normal"]
            pub const NOT_LOCK: u32 = 1;
        }
    }
}
#[doc = "Interrupt Enable"]
pub mod IER {
    #[doc = "Digital Tamper Interrupt Enable"]
    pub mod DTIE {
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
    #[doc = "Tamper Input n Interrupt Enable"]
    pub mod TIIE0 {
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
    #[doc = "Tamper Input n Interrupt Enable"]
    pub mod TIIE1 {
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
    #[doc = "Tamper Input n Interrupt Enable"]
    pub mod TIIE2 {
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
    #[doc = "Tamper Input n Interrupt Enable"]
    pub mod TIIE3 {
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
    #[doc = "Tamper Input n Interrupt Enable"]
    pub mod TIIE4 {
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
    #[doc = "Tamper Input n Interrupt Enable"]
    pub mod TIIE5 {
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
    #[doc = "Tamper Input n Interrupt Enable"]
    pub mod TIIE6 {
        pub const offset: u32 = 8;
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
    #[doc = "Tamper Input n Interrupt Enable"]
    pub mod TIIE7 {
        pub const offset: u32 = 9;
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
    #[doc = "Tamper Input n Interrupt Enable"]
    pub mod TIIE8 {
        pub const offset: u32 = 10;
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
    #[doc = "Tamper Input n Interrupt Enable"]
    pub mod TIIE9 {
        pub const offset: u32 = 11;
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
    #[doc = "Tamper Pin n Interrupt Enable"]
    pub mod TPIE0 {
        pub const offset: u32 = 16;
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
    #[doc = "Tamper Pin n Interrupt Enable"]
    pub mod TPIE1 {
        pub const offset: u32 = 17;
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
    #[doc = "Tamper Pin n Interrupt Enable"]
    pub mod TPIE2 {
        pub const offset: u32 = 18;
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
    #[doc = "Tamper Pin n Interrupt Enable"]
    pub mod TPIE3 {
        pub const offset: u32 = 19;
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
    #[doc = "Tamper Pin n Interrupt Enable"]
    pub mod TPIE4 {
        pub const offset: u32 = 20;
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
    #[doc = "Tamper Pin n Interrupt Enable"]
    pub mod TPIE5 {
        pub const offset: u32 = 21;
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
    #[doc = "Tamper Pin n Interrupt Enable"]
    pub mod TPIE6 {
        pub const offset: u32 = 22;
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
    #[doc = "Tamper Pin n Interrupt Enable"]
    pub mod TPIE7 {
        pub const offset: u32 = 23;
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
#[doc = "Tamper Seconds"]
pub mod TSR {
    #[doc = "Tamper Time Seconds"]
    pub mod TTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper Enable"]
pub mod TER {
    #[doc = "Tamper Input Enable"]
    pub mod TIE0 {
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
    #[doc = "Tamper Input Enable"]
    pub mod TIE1 {
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
    #[doc = "Tamper Input Enable"]
    pub mod TIE2 {
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
    #[doc = "Tamper Input Enable"]
    pub mod TIE3 {
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
    #[doc = "Tamper Input Enable"]
    pub mod TIE4 {
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
    #[doc = "Tamper Input Enable"]
    pub mod TIE5 {
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
    #[doc = "Tamper Input Enable"]
    pub mod TIE6 {
        pub const offset: u32 = 8;
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
    #[doc = "Tamper Input Enable"]
    pub mod TIE7 {
        pub const offset: u32 = 9;
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
    #[doc = "Tamper Input Enable"]
    pub mod TIE8 {
        pub const offset: u32 = 10;
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
    #[doc = "Tamper Input Enable"]
    pub mod TIE9 {
        pub const offset: u32 = 11;
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
    #[doc = "Tamper Pin Enable"]
    pub mod TPE0 {
        pub const offset: u32 = 16;
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
    #[doc = "Tamper Pin Enable"]
    pub mod TPE1 {
        pub const offset: u32 = 17;
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
    #[doc = "Tamper Pin Enable"]
    pub mod TPE2 {
        pub const offset: u32 = 18;
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
    #[doc = "Tamper Pin Enable"]
    pub mod TPE3 {
        pub const offset: u32 = 19;
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
    #[doc = "Tamper Pin Enable"]
    pub mod TPE4 {
        pub const offset: u32 = 20;
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
    #[doc = "Tamper Pin Enable"]
    pub mod TPE5 {
        pub const offset: u32 = 21;
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
    #[doc = "Tamper Pin Enable"]
    pub mod TPE6 {
        pub const offset: u32 = 22;
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
    #[doc = "Tamper Pin Enable"]
    pub mod TPE7 {
        pub const offset: u32 = 23;
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
#[doc = "Pin Direction"]
pub mod PDR {
    #[doc = "Tamper Pin Direction"]
    pub mod TPD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input"]
            pub const INPUT: u32 = 0;
            #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
            pub const OUTPUT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin Direction"]
    pub mod TPD1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input"]
            pub const INPUT: u32 = 0;
            #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
            pub const OUTPUT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin Direction"]
    pub mod TPD2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input"]
            pub const INPUT: u32 = 0;
            #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
            pub const OUTPUT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin Direction"]
    pub mod TPD3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input"]
            pub const INPUT: u32 = 0;
            #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
            pub const OUTPUT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin Direction"]
    pub mod TPD4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input"]
            pub const INPUT: u32 = 0;
            #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
            pub const OUTPUT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin Direction"]
    pub mod TPD5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input"]
            pub const INPUT: u32 = 0;
            #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
            pub const OUTPUT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin Direction"]
    pub mod TPD6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input"]
            pub const INPUT: u32 = 0;
            #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
            pub const OUTPUT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin Direction"]
    pub mod TPD7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input"]
            pub const INPUT: u32 = 0;
            #[doc = "Output and drives the inverse of the expected value (tamper pin is asserted)"]
            pub const OUTPUT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin Output Data"]
    pub mod TPOD0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin Output Data"]
    pub mod TPOD1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin Output Data"]
    pub mod TPOD2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin Output Data"]
    pub mod TPOD3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin Output Data"]
    pub mod TPOD4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin Output Data"]
    pub mod TPOD5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin Output Data"]
    pub mod TPOD6 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin Output Data"]
    pub mod TPOD7 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Polarity"]
pub mod PPR {
    #[doc = "Tamper Pin n Polarity"]
    pub mod TPP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted"]
            pub const NO_INVERT: u32 = 0;
            #[doc = "Inverted"]
            pub const INVERT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Polarity"]
    pub mod TPP1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted"]
            pub const NO_INVERT: u32 = 0;
            #[doc = "Inverted"]
            pub const INVERT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Polarity"]
    pub mod TPP2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted"]
            pub const NO_INVERT: u32 = 0;
            #[doc = "Inverted"]
            pub const INVERT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Polarity"]
    pub mod TPP3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted"]
            pub const NO_INVERT: u32 = 0;
            #[doc = "Inverted"]
            pub const INVERT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Polarity"]
    pub mod TPP4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted"]
            pub const NO_INVERT: u32 = 0;
            #[doc = "Inverted"]
            pub const INVERT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Polarity"]
    pub mod TPP5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted"]
            pub const NO_INVERT: u32 = 0;
            #[doc = "Inverted"]
            pub const INVERT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Polarity"]
    pub mod TPP6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted"]
            pub const NO_INVERT: u32 = 0;
            #[doc = "Inverted"]
            pub const INVERT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Polarity"]
    pub mod TPP7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted"]
            pub const NO_INVERT: u32 = 0;
            #[doc = "Inverted"]
            pub const INVERT: u32 = 1;
        }
    }
    #[doc = "Tamper Pin n Input Data"]
    pub mod TPID0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin n Input Data"]
    pub mod TPID1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin n Input Data"]
    pub mod TPID2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin n Input Data"]
    pub mod TPID3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin n Input Data"]
    pub mod TPID4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin n Input Data"]
    pub mod TPID5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin n Input Data"]
    pub mod TPID6 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Pin n Input Data"]
    pub mod TPID7 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Zero"]
            pub const ZERO: u32 = 0;
            #[doc = "One"]
            pub const ONE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Active Tamper"]
pub mod ATR {
    #[doc = "Active Tamper Shift Register"]
    pub mod ATSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active Tamper Polynomial"]
    pub mod ATP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Glitch Filter"]
pub mod PGFR {
    #[doc = "Glitch Filter Width"]
    pub mod GFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch Filter Prescaler"]
    pub mod GFP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "512 Hz prescaler clock"]
            pub const FREQ_512_HZ: u32 = 0;
            #[doc = "32.768 kHz clock"]
            pub const FREQ_32_KHZ: u32 = 1;
        }
    }
    #[doc = "Glitch Filter Enable"]
    pub mod GFE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bypasses"]
            pub const BYPASS: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Tamper Pin Sample Width"]
    pub mod TPSW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continuous monitoring, pin sampling disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "2 cycles for pull enable and 1 cycle for input buffer enable"]
            pub const CYCLES_2: u32 = 1;
            #[doc = "4 cycles for pull enable and 2 cycles for input buffer enable"]
            pub const CYCLES_4: u32 = 2;
            #[doc = "8 cycles for pull enable and 4 cycles for input buffer enable"]
            pub const CYCLES_8: u32 = 3;
        }
    }
    #[doc = "Tamper Pin Sample Frequency"]
    pub mod TPSF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Every 8 cycles"]
            pub const CYCLES_8: u32 = 0;
            #[doc = "Every 32 cycles"]
            pub const CYCLES_32: u32 = 1;
            #[doc = "Every 128 cycles"]
            pub const CYCLES_128: u32 = 2;
            #[doc = "Every 512 cycles"]
            pub const CYCLES_512: u32 = 3;
        }
    }
    #[doc = "Tamper Pin Expected"]
    pub mod TPEX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Zero/passive tamper"]
            pub const ZERO: u32 = 0;
            #[doc = "Active Tamper 0 output"]
            pub const VAL_TAMP0: u32 = 1;
            #[doc = "Active Tamper 1 output"]
            pub const VAL_TAMP1: u32 = 2;
            #[doc = "Active Tamper 0 output XORed with Active Tamper 1 output"]
            pub const TAMP0_XOR_TAMP1: u32 = 3;
        }
    }
    #[doc = "Tamper Pull Enable"]
    pub mod TPE {
        pub const offset: u32 = 24;
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
    #[doc = "Tamper Pull Select"]
    pub mod TPS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asserts"]
            pub const ASSERT: u32 = 0;
            #[doc = "Negates"]
            pub const NEGATE: u32 = 1;
        }
    }
}
