#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "GDET Configuration 0 Register"]
    pub GDET_CONF_0: crate::RWRegister<u32>,
    #[doc = "GDET Configuration 1 Register"]
    pub GDET_CONF_1: crate::RWRegister<u32>,
    #[doc = "GDET Enable Register"]
    pub GDET_ENABLE1: crate::RWRegister<u32>,
    #[doc = "GDET Configuration 2 Register"]
    pub GDET_CONF_2: crate::RWRegister<u32>,
    #[doc = "GDET Configuration 3 Register"]
    pub GDET_CONF_3: crate::RWRegister<u32>,
    #[doc = "GDET Configuration 4 Register"]
    pub GDET_CONF_4: crate::RWRegister<u32>,
    #[doc = "GDET Configuration 5 Register"]
    pub GDET_CONF_5: crate::RWRegister<u32>,
    _reserved0: [u8; 0xfa4],
    #[doc = "GDET Reset Register"]
    pub GDET_RESET: crate::RWRegister<u32>,
    #[doc = "GDET Test Register"]
    pub GDET_TEST: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "GDET Delay Control Register"]
    pub GDET_DLY_CTRL: crate::RWRegister<u32>,
}
#[doc = "GDET Configuration 0 Register"]
pub mod GDET_CONF_0 {
    #[doc = "GDET Configuration 0 Field 3_0"]
    pub mod FIELD_3_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Should Be Left to Zero"]
    pub mod SBZ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for Future Use"]
    pub mod RFU {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GDET Configuration 1 Register"]
pub mod GDET_CONF_1 {
    #[doc = "GDET Configuration 1 Field 1_0"]
    pub mod FIELD_1_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GDET Configuration 1 Field 3_2"]
    pub mod FIELD_3_2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Should Be Left to Zero"]
    pub mod SBZ1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Should Be Left to Zero"]
    pub mod SBZ2 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Should Be Left to Zero"]
    pub mod SBZ3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GDET Configuration 1 Field 7"]
    pub mod FIELD_7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GDET Configuration 1 Field 8"]
    pub mod FIELD_8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Should Be Left to Zero"]
    pub mod SBZ4 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Should Be Left to Zero"]
    pub mod SBZ5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for Future Use"]
    pub mod RFU {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GDET Enable Register"]
pub mod GDET_ENABLE1 {
    #[doc = "If set, the detector will be clock gated"]
    pub mod EN1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for Future Use"]
    pub mod RFU {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GDET Configuration 2 Register"]
pub mod GDET_CONF_2 {
    #[doc = "GDET Configuration 2 Field 6_0"]
    pub mod FIELD_6_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for Future Use"]
    pub mod RFU1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GDET Configuration 2 Field 21_16"]
    pub mod FIELD_21_16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for Future Use"]
    pub mod RFU2 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GDET Configuration 2 Field 29_24"]
    pub mod FIELD_29_24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for Future Use"]
    pub mod RFU3 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GDET Configuration 3 Register"]
pub mod GDET_CONF_3 {
    #[doc = "GDET Configuration 3 Field 6_0"]
    pub mod FIELD_6_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for Future Use"]
    pub mod RFU1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GDET Configuration 4 Register"]
pub mod GDET_CONF_4 {
    #[doc = "GDET Configuration 4 Field 6_0"]
    pub mod FIELD_6_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for Future Use"]
    pub mod RFU1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GDET Configuration 5 Register"]
pub mod GDET_CONF_5 {
    #[doc = "GDET Configuration 5 Field 5_0"]
    pub mod FIELD_5_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GDET Configuration 5 Field 11_6"]
    pub mod FIELD_11_6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for Future Use"]
    pub mod RFU1 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GDET Reset Register"]
pub mod GDET_RESET {
    #[doc = "Reserved for Future Use"]
    pub mod RFU1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Soft Reset for the Core Reset"]
    pub mod SFT_RST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for Future Use"]
    pub mod RFU2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GDET Test Register"]
pub mod GDET_TEST {
    #[doc = "Should Be Left to Zero"]
    pub mod SBZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for Future Use"]
    pub mod RFU {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "GDET Delay Control Register"]
pub mod GDET_DLY_CTRL {
    #[doc = "GDET Delay Control of the Voltage Mode"]
    pub mod VOL_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select the Control of the Trim Code to the Delay Line"]
    pub mod SW_VOL_CTRL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for Future Use"]
    pub mod RFU {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b11111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
