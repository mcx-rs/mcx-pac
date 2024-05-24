#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "DBGMB"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Command and Status Word"]
    pub CSW: crate::RWRegister<u32>,
    #[doc = "Request Value"]
    pub REQUEST: crate::RWRegister<u32>,
    #[doc = "Return Value"]
    pub RETURN: crate::RWRegister<u32>,
    _reserved0: [u8; 0xf0],
    #[doc = "Identification"]
    pub ID: crate::RWRegister<u32>,
}
#[doc = "Command and Status Word"]
pub mod CSW {
    #[doc = "Resynchronization Request"]
    pub mod RESYNCH_REQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No request"]
            pub const NO_REQUEST: u32 = 0;
            #[doc = "Request for resynchronization"]
            pub const REQUEST: u32 = 1;
        }
    }
    #[doc = "Request Pending"]
    pub mod REQ_PENDING {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No request pending"]
            pub const NO_REQUEST_PENDING: u32 = 0;
            #[doc = "Request for resynchronization pending"]
            pub const REQUEST_PENDING: u32 = 1;
        }
    }
    #[doc = "DBGMB Overrun Error"]
    pub mod DBG_OR_ERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No DBGMB Overrun error"]
            pub const NO_OVERRUN_ERR: u32 = 0;
            #[doc = "DBGMB overrun error. A DBGMB overrun occurred."]
            pub const OVERRUN_ERR: u32 = 1;
        }
    }
    #[doc = "AHB Overrun Error"]
    pub mod AHB_OR_ERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No AHB Overrun Error"]
            pub const NO_AHB_OVERRUN_ERR: u32 = 0;
            #[doc = "AHB Overrun Error. An AHB overrun occurred."]
            pub const AHB_OVERRUN_ERR: u32 = 1;
        }
    }
    #[doc = "Soft Reset"]
    pub mod SOFT_RESET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Chip Reset Request"]
    pub mod CHIP_RESET_REQ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Request Value"]
pub mod REQUEST {
    #[doc = "Request Value"]
    pub mod REQUEST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Return Value"]
pub mod RETURN {
    #[doc = "Return Value"]
    pub mod RET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Identification"]
pub mod ID {
    #[doc = "Identification Value"]
    pub mod ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
