#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "OTPC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameters"]
    pub PARAM: crate::RWRegister<u32>,
    #[doc = "Status"]
    pub SR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "Read and Write Control"]
    pub RWC: crate::RWRegister<u32>,
    #[doc = "Reload Control"]
    pub RLC: crate::RWRegister<u32>,
    #[doc = "Power Control"]
    pub PCR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "Write Data"]
    pub WDATA: crate::RWRegister<u32>,
    #[doc = "Read Data"]
    pub RDATA: crate::RWRegister<u32>,
    _reserved2: [u8; 0x8],
    #[doc = "Timing1"]
    pub TIMING1: crate::RWRegister<u32>,
    #[doc = "Timing2"]
    pub TIMING2: crate::RWRegister<u32>,
    _reserved3: [u8; 0x1c8],
    #[doc = "Lock"]
    pub LOCK: crate::RWRegister<u32>,
    #[doc = "Secure"]
    pub SECURE: crate::RWRegister<u32>,
    #[doc = "Inverted Secure"]
    pub SECURE_INV: crate::RWRegister<u32>,
    #[doc = "Debug and Key"]
    pub DBG_KEY: crate::RWRegister<u32>,
    #[doc = "MISC Config"]
    pub MISC_CFG: crate::RWRegister<u32>,
    #[doc = "PHANTOM Config"]
    pub PHANTOM_CFG: crate::RWRegister<u32>,
    #[doc = "Flexible Config 0"]
    pub FLEX_CFG0: crate::RWRegister<u32>,
    #[doc = "Flexible Config 1"]
    pub FLEX_CFG1: crate::RWRegister<u32>,
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {
            #[doc = "Standard feature set"]
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
#[doc = "Parameters"]
pub mod PARAM {
    #[doc = "Number of fuse bytes"]
    pub mod NUM_FUSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status"]
pub mod SR {
    #[doc = "Busy status"]
    pub mod BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not busy (transaction complete)"]
            pub const NOT_BUSY: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error flag"]
    pub mod ERROR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "ECC single fault"]
    pub mod ECC_SF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No fault"]
            pub const NO_FAULT: u32 = 0;
            #[doc = "Fault"]
            pub const FAULT: u32 = 1;
        }
    }
    #[doc = "ECC double fault"]
    pub mod ECC_DF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No fault"]
            pub const NO_FAULT: u32 = 0;
            #[doc = "Fault"]
            pub const FAULT: u32 = 1;
        }
    }
    #[doc = "Triple voting fault"]
    pub mod TRI_F {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No fault"]
            pub const NO_FAULT: u32 = 0;
            #[doc = "Fault"]
            pub const FAULT: u32 = 1;
        }
    }
    #[doc = "Read fuse lock error"]
    pub mod RD_FUSE_LOCK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write fuse lock error"]
    pub mod WR_FUSE_LOCK {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read register lock error"]
    pub mod RD_REG_LOCK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write register lock error"]
    pub mod WR_REG_LOCK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write register when busy error"]
    pub mod WR_REG_BUSY {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write when power off error"]
    pub mod WR_POWER_OFF {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Finite-state machine error"]
    pub mod FSM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Fuse load counter error"]
    pub mod FLC {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Address and data compare error"]
    pub mod ADC {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Inverted register compare error"]
    pub mod IRC {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Fuse and shadow register compare error"]
    pub mod FSC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
}
#[doc = "Read and Write Control"]
pub mod RWC {
    #[doc = "EFUSE address"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write all 1s"]
    pub mod WR_ALL1S {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Uses the WDATA value"]
            pub const USE_WDATA: u32 = 0;
            #[doc = "Writes all 1s"]
            pub const USE_ALL1S: u32 = 1;
        }
    }
    #[doc = "Read EFUSE"]
    pub mod READ_EFUSE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Starts program operation when the WR_UNLOCK value is 0x9527; otherwise, takes no action."]
            pub const PROGRAM: u32 = 0;
            #[doc = "Starts read operation"]
            pub const READ: u32 = 1;
        }
    }
    #[doc = "Read update"]
    pub mod READ_UPDATE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Shadow register does not update"]
            pub const NO_UPDATE: u32 = 0;
            #[doc = "Shadow register updates"]
            pub const UPDATE: u32 = 1;
        }
    }
    #[doc = "Write Unlock"]
    pub mod WR_UNLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reload Control"]
pub mod RLC {
    #[doc = "Reload shadow registers"]
    pub mod RELOAD_SHADOWS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action (when writing) or reload complete (when reading)"]
            pub const NO_ACTION: u32 = 0;
            #[doc = "Reload"]
            pub const RELOAD: u32 = 1;
        }
    }
}
#[doc = "Power Control"]
pub mod PCR {
    #[doc = "Strong switch request"]
    pub mod HVREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Turn off"]
            pub const TURN_OFF: u32 = 0;
            #[doc = "Turn on"]
            pub const TURN_ON: u32 = 1;
        }
    }
    #[doc = "Weak switch request"]
    pub mod LVREQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Turn off"]
            pub const TURN_OFF: u32 = 0;
            #[doc = "Turn on"]
            pub const TURN_ON: u32 = 1;
        }
    }
    #[doc = "Power down request"]
    pub mod PDREQ {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PD pin is set to low when OTPC is in idle state. It means EFUSE hardmacro is in standby mode. Idle state means OTPC is not in read and program modes."]
            pub const NO_ACTION: u32 = 0;
            #[doc = "PD pin is set to high when OTPC is in idle state. It means EFUSE hardmacro is in power down mode."]
            pub const POWER_DN: u32 = 1;
        }
    }
}
#[doc = "Write Data"]
pub mod WDATA {
    #[doc = "Write data"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Read Data"]
pub mod RDATA {
    #[doc = "Read data"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timing1"]
pub mod TIMING1 {
    #[doc = "Address to STROBE setup and hold time"]
    pub mod TADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSB, PGENB and LOAD to STROBE setup and hold time"]
    pub mod TRELAX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read strobe pulse width time"]
    pub mod TRD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PS to CSB setup and hold time between power switch and chip select assertion"]
    pub mod TPS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PD to CSB setup time between power down signal deassertion and chip select signal assertion"]
    pub mod TPD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timing2"]
pub mod TIMING2 {
    #[doc = "Typical program strobe pulse width time"]
    pub mod TPGM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lock"]
pub mod LOCK {
    #[doc = "NXP Part Config Lock"]
    pub mod NXP_PART_CFG_LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NXP EXT Lock"]
    pub mod NXP_EXT_LOCK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Boot config Lock"]
    pub mod BOOT_CFG_LOCK {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Prince Config Lock"]
    pub mod PRINCE_CFG_LOCK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OSCAA Key Lock"]
    pub mod OSCAA_KEY_LOCK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CUST Lock 0"]
    pub mod CUST_LOCK0 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CUST Lock 1"]
    pub mod CUST_LOCK1 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CUST Lock 2"]
    pub mod CUST_LOCK2 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CUST Lock 3"]
    pub mod CUST_LOCK3 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Secure"]
pub mod SECURE {
    #[doc = "Data"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Inverted Secure"]
pub mod SECURE_INV {
    #[doc = "Data"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Debug and Key"]
pub mod DBG_KEY {
    #[doc = "Data"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MISC Config"]
pub mod MISC_CFG {
    #[doc = "Data"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PHANTOM Config"]
pub mod PHANTOM_CFG {
    #[doc = "Data"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flexible Config 0"]
pub mod FLEX_CFG0 {
    #[doc = "Data"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flexible Config 1"]
pub mod FLEX_CFG1 {
    #[doc = "Data"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
