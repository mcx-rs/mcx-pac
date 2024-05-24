#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "DMA MP"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Management Page Control"]
    pub MP_CSR: crate::RWRegister<u32>,
    #[doc = "Management Page Error Status"]
    pub MP_ES: crate::RWRegister<u32>,
    #[doc = "Management Page Interrupt Request Status"]
    pub MP_INT: crate::RWRegister<u32>,
    #[doc = "Management Page Hardware Request Status"]
    pub MP_HRS: crate::RWRegister<u32>,
    _reserved0: [u8; 0xf0],
    #[doc = "Channel Arbitration Group"]
    pub CH_GRPRI: [crate::RWRegister<u32>; 16usize],
}
#[doc = "Management Page Control"]
pub mod MP_CSR {
    #[doc = "Enable Debug"]
    pub mod EDBG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Debug mode disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Debug mode is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Round Robin Channel Arbitration"]
    pub mod ERCA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Round-robin channel arbitration disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Round-robin channel arbitration enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Halt After Error"]
    pub mod HAE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPERATION: u32 = 0;
            #[doc = "Any error causes the HALT field to be set to 1"]
            pub const HALT: u32 = 1;
        }
    }
    #[doc = "Halt DMA Operations"]
    pub mod HALT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPERATION: u32 = 0;
            #[doc = "Stall the start of any new channels"]
            pub const STALL: u32 = 1;
        }
    }
    #[doc = "Global Channel Linking Control"]
    pub mod GCLC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel linking disabled for all channels"]
            pub const DISABLE: u32 = 0;
            #[doc = "Channel linking available and controlled by each channel\'s link settings"]
            pub const AVAILABLE: u32 = 1;
        }
    }
    #[doc = "Global Master ID Replication Control"]
    pub mod GMRC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master ID replication disabled for all channels"]
            pub const DISABLE: u32 = 0;
            #[doc = "Master ID replication available and controlled by each channel\'s CHn_SBR[EMI] setting"]
            pub const AVAILABLE: u32 = 1;
        }
    }
    #[doc = "Cancel Transfer With Error"]
    pub mod ECX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPERATION: u32 = 0;
            #[doc = "Cancel the remaining data transfer"]
            pub const CANCEL: u32 = 1;
        }
    }
    #[doc = "Cancel Transfer"]
    pub mod CX {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL_OPERATION: u32 = 0;
            #[doc = "Cancel the remaining data transfer"]
            pub const DATA_TRANSFER_CANCEL: u32 = 1;
        }
    }
    #[doc = "Active Channel ID"]
    pub mod ACTIVE_ID {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Active Status"]
    pub mod ACTIVE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "eDMA is idle"]
            pub const IDLE: u32 = 0;
            #[doc = "eDMA is executing a channel"]
            pub const EXECUTION: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Management Page Error Status"]
pub mod MP_ES {
    #[doc = "Destination Bus Error"]
    pub mod DBE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No destination bus error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Last recorded error was a bus error on a destination write"]
            pub const BUS_ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Bus Error"]
    pub mod SBE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No source bus error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Last recorded error was a bus error on a source read"]
            pub const BUS_ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Scatter/Gather Configuration Error"]
    pub mod SGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No scatter/gather configuration error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Last recorded error was a configuration error detected in the TCDn_DLAST_SGA field"]
            pub const CONFIGURATION_ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NBYTES/CITER Configuration Error"]
    pub mod NCE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No NBYTES/CITER configuration error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "The last recorded error was NBYTES equal to zero or a CITER not equal to BITER error"]
            pub const CONFIGURATION_ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Destination Offset Error"]
    pub mod DOE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No destination offset configuration error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Last recorded error was a configuration error detected in the TCDn_DOFF field"]
            pub const CONFIGURATION_ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Destination Address Error"]
    pub mod DAE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No destination address configuration error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Last recorded error was a configuration error detected in the TCDn_DADDR field"]
            pub const CONFIGURATION_ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Offset Error"]
    pub mod SOE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No source offset configuration error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Last recorded error was a configuration error detected in the TCDn_SOFF field"]
            pub const CONFIGURATION_ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address Error"]
    pub mod SAE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No source address configuration error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Last recorded error was a configuration error detected in the TCDn_SADDR field"]
            pub const CONFIGURATION_ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transfer Canceled"]
    pub mod ECX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No canceled transfers"]
            pub const NO_CANCELED_TRANSFERS: u32 = 0;
            #[doc = "Last recorded entry was a canceled transfer by the error cancel transfer input"]
            pub const CANCELED_TRANSFER: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    pub mod ERRCHN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No CHn_ES[ERR] fields are set to 1"]
            pub const NO_FIELD_SET_ONE: u32 = 0;
            #[doc = "At least one CHn_ES[ERR] field is set to 1, indicating a valid error exists that software has not cleared"]
            pub const ATLEAST_ONE_FIELD: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Management Page Interrupt Request Status"]
pub mod MP_INT {
    #[doc = "Interrupt Request Status"]
    pub mod INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Management Page Hardware Request Status"]
pub mod MP_HRS {
    #[doc = "Hardware Request Status"]
    pub mod HRS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Channel Arbitration Group"]
pub mod CH_GRPRI {
    #[doc = "Arbitration Group For Channel n"]
    pub mod GRPRI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
