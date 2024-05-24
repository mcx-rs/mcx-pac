#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "CACHE64_POLSEL"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    #[doc = "Region 0 Top Boundary"]
    pub REG0_TOP: crate::RWRegister<u32>,
    #[doc = "Region 1 Top Boundary"]
    pub REG1_TOP: crate::RWRegister<u32>,
    #[doc = "Policy Select"]
    pub POLSEL: crate::RWRegister<u32>,
}
#[doc = "Region 0 Top Boundary"]
pub mod REG0_TOP {
    #[doc = "Upper Limit Of Region 0"]
    pub mod REG0_TOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Region 1 Top Boundary"]
pub mod REG1_TOP {
    #[doc = "Upper Limit Of Region 1"]
    pub mod REG1_TOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Policy Select"]
pub mod POLSEL {
    #[doc = "Policy Select For Region 0"]
    pub mod REG0_POLICY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Noncacheable"]
            pub const REG0_00: u32 = 0;
            #[doc = "Write-through"]
            pub const REG0_01: u32 = 1;
            #[doc = "Write-back"]
            pub const REG0_10: u32 = 2;
            #[doc = "Invalid"]
            pub const REG0_11: u32 = 3;
        }
    }
    #[doc = "Policy Select For Region 1"]
    pub mod REG1_POLICY {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Noncacheable"]
            pub const REG1_00: u32 = 0;
            #[doc = "Write-through"]
            pub const REG1_01: u32 = 1;
            #[doc = "Write-back"]
            pub const REG1_10: u32 = 2;
            #[doc = "Invalid"]
            pub const REG1_11: u32 = 3;
        }
    }
    #[doc = "Policy Select For Region 2"]
    pub mod REG2_POLICY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Noncacheable"]
            pub const REG2_00: u32 = 0;
            #[doc = "Write-through"]
            pub const REG2_01: u32 = 1;
            #[doc = "Write-back"]
            pub const REG2_10: u32 = 2;
            #[doc = "Invalid"]
            pub const REG2_11: u32 = 3;
        }
    }
}
