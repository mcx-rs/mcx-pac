#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "CRC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "CRC Data"]
    pub DATA: crate::RWRegister<u32>,
    #[doc = "CRC Polynomial"]
    pub GPOLY: crate::RWRegister<u32>,
    #[doc = "CRC Control"]
    pub CTRL: crate::RWRegister<u32>,
}
#[doc = "CRC Data"]
pub mod DATA {
    #[doc = "CRC Low Lower Byte"]
    pub mod LL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC Low Upper Byte"]
    pub mod LU {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC High Lower Byte"]
    pub mod HL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC High Upper Byte"]
    pub mod HU {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CRC Polynomial"]
pub mod GPOLY {
    #[doc = "Low Polynomial Half-Word"]
    pub mod LOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Polynomial Half-Word"]
    pub mod HIGH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CRC Control"]
pub mod CTRL {
    #[doc = "TCRC"]
    pub mod TCRC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16-bit"]
            pub const TCRC_0: u32 = 0;
            #[doc = "32-bit"]
            pub const TCRC_1: u32 = 1;
        }
    }
    #[doc = "Write as Seed"]
    pub mod WAS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data values"]
            pub const WAS_0: u32 = 0;
            #[doc = "Seed values"]
            pub const WAS_1: u32 = 1;
        }
    }
    #[doc = "Complement Read of CRC Data Register"]
    pub mod FXOR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No XOR on reading"]
            pub const FXOR_0: u32 = 0;
            #[doc = "Inverts or complements the read value of the CRC Data"]
            pub const FXOR_1: u32 = 1;
        }
    }
    #[doc = "Transpose Type for Read"]
    pub mod TOTR {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No transposition"]
            pub const TOTR_0: u32 = 0;
            #[doc = "Bits in bytes are transposed; bytes are not transposed"]
            pub const TOTR_1: u32 = 1;
            #[doc = "Both bits in bytes and bytes are transposed"]
            pub const TOTR_2: u32 = 2;
            #[doc = "Only bytes are transposed; no bits in a byte are transposed"]
            pub const TOTR_3: u32 = 3;
        }
    }
    #[doc = "Transpose Type for Writes"]
    pub mod TOT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No transposition"]
            pub const TOT_0: u32 = 0;
            #[doc = "Bits in bytes are transposed; bytes are not transposed"]
            pub const TOT_1: u32 = 1;
            #[doc = "Both bits in bytes and bytes are transposed"]
            pub const TOT_2: u32 = 2;
            #[doc = "Only bytes are transposed; no bits in a byte are transposed"]
            pub const TOT_3: u32 = 3;
        }
    }
}
