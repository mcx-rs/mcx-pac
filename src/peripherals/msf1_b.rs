#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "Flash"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Flash Status Register"]
    pub FSTAT: crate::RWRegister<u32>,
    #[doc = "Flash Configuration Register"]
    pub FCNFG: crate::RWRegister<u32>,
    #[doc = "Flash Control Register"]
    pub FCTRL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "Flash Common Command Object Registers"]
    pub FCCOB: [crate::RWRegister<u32>; 8usize],
}
#[doc = "Flash Status Register"]
pub mod FSTAT {
    #[doc = "Command Fail Flag"]
    pub mod FAIL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Error not detected"]
            pub const FAIL0: u32 = 0;
            #[doc = "Error detected"]
            pub const FAIL1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Abort Flag"]
    pub mod CMDABT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No command abort detected"]
            pub const CMDABT0: u32 = 0;
            #[doc = "Command abort detected"]
            pub const CMDABT1: u32 = 1;
        }
    }
    #[doc = "Command Protection Violation Flag"]
    pub mod PVIOL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No protection violation detected"]
            pub const PVIOL0: u32 = 0;
            #[doc = "Protection violation detected"]
            pub const PVIOL1: u32 = 1;
        }
    }
    #[doc = "Command Access Error Flag"]
    pub mod ACCERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No access error detected"]
            pub const ACCERR0: u32 = 0;
            #[doc = "Access error detected"]
            pub const ACCERR1: u32 = 1;
        }
    }
    #[doc = "Command Write Sequence Abort Flag"]
    pub mod CWSABT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command write sequence not aborted"]
            pub const CWSABT0: u32 = 0;
            #[doc = "Command write sequence aborted"]
            pub const CWSABT1: u32 = 1;
        }
    }
    #[doc = "Command Complete Interrupt Flag"]
    pub mod CCIF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flash command, initialization, or power mode recovery in progress"]
            pub const CCIF0: u32 = 0;
            #[doc = "Flash command, initialization, or power mode recovery has completed"]
            pub const CCIF1: u32 = 1;
        }
    }
    #[doc = "Command protection level"]
    pub mod CMDPRT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Secure, normal access"]
            pub const CMDPRT00: u32 = 0;
            #[doc = "Secure, privileged access"]
            pub const CMDPRT01: u32 = 1;
            #[doc = "Nonsecure, normal access"]
            pub const CMDPRT10: u32 = 2;
            #[doc = "Nonsecure, privileged access"]
            pub const CMDPRT11: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command protection status flag"]
    pub mod CMDP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Command protection level and domain ID are stale"]
            pub const CMDP0: u32 = 0;
            #[doc = "Command protection level (CMDPRT) and domain ID (CMDDID) are set"]
            pub const CMDP1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command domain ID"]
    pub mod CMDDID {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double Bit Fault Detect Interrupt Flag"]
    pub mod DFDIF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Double bit fault not detected during a valid flash read access"]
            pub const DFDIF0: u32 = 0;
            #[doc = "Double bit fault detected (or FCTRL[FDFD] is set) during a valid flash read access"]
            pub const DFDIF1: u32 = 1;
        }
    }
    #[doc = "Salvage Used for Erase operation"]
    pub mod SALV_USED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Salvage not used during last operation"]
            pub const SALV_USED0: u32 = 0;
            #[doc = "Salvage used during the last erase operation"]
            pub const SALV_USED1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Program-Erase Write Enable Control"]
    pub mod PEWEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Writes are not enabled"]
            pub const PEWEN00: u32 = 0;
            #[doc = "Writes are enabled for one flash or IFR phrase (phrase programming, sector erase)"]
            pub const PEWEN01: u32 = 1;
            #[doc = "Writes are enabled for one flash or IFR page (page programming)"]
            pub const PEWEN10: u32 = 2;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Program-Erase Ready Control/Status Flag"]
    pub mod PERDY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Program or sector erase command operation not stalled"]
            pub const PERDY0: u32 = 0;
            #[doc = "Program or sector erase command operation ready to execute"]
            pub const PERDY1: u32 = 1;
        }
    }
}
#[doc = "Flash Configuration Register"]
pub mod FCNFG {
    #[doc = "Command Complete Interrupt Enable"]
    pub mod CCIE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command complete interrupt disabled"]
            pub const CCIE0: u32 = 0;
            #[doc = "Command complete interrupt enabled"]
            pub const CCIE1: u32 = 1;
        }
    }
    #[doc = "Mass Erase Request"]
    pub mod ERSREQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No request or request complete"]
            pub const ERSREQ0: u32 = 0;
            #[doc = "Request to run the Mass Erase operation"]
            pub const ERSREQ1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double Bit Fault Detect Interrupt Enable"]
    pub mod DFDIE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Double bit fault detect interrupt disabled"]
            pub const DFDIE0: u32 = 0;
            #[doc = "Double bit fault detect interrupt enabled"]
            pub const DFDIE1: u32 = 1;
        }
    }
    #[doc = "Erase IFR Sector Enable - Block 0"]
    pub mod ERSIEN0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "Block 0 IFR Sector X is protected from erase by ERSSCR command"]
            pub const ERSIEN00: u32 = 0;
            #[doc = "Block 0 IFR Sector X is not protected from erase by ERSSCR command"]
            pub const ERSIEN01: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Erase IFR Sector Enable - Block 1 (for dual block configs)"]
    pub mod ERSIEN1 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "Block 1 IFR Sector X is protected from erase by ERSSCR command"]
            pub const ERSIEN10: u32 = 0;
            #[doc = "Block 1 IFR Sector X is not protected from erase by ERSSCR command"]
            pub const ERSIEN11: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control Register"]
pub mod FCTRL {
    #[doc = "Read Wait-State Control"]
    pub mod RWSC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Double Bit Fault Detect"]
    pub mod FDFD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FSTAT[DFDIF] sets only if a double bit fault is detected during a valid flash read access from the platform flash controller"]
            pub const FDFD0: u32 = 0;
            #[doc = "FSTAT[DFDIF] sets during any valid flash read access from the platform flash controller. An interrupt request is generated if the DFDIE bit is set."]
            pub const FDFD1: u32 = 1;
        }
    }
    #[doc = "Abort Request"]
    pub mod ABTREQ {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No request to abort a command write sequence"]
            pub const ABTREQ0: u32 = 0;
            #[doc = "Request to abort a command write sequence"]
            pub const ABTREQ1: u32 = 1;
        }
    }
}
#[doc = "Flash Common Command Object Registers"]
pub mod FCCOB {
    #[doc = "CCOBn"]
    pub mod CCOBn {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
