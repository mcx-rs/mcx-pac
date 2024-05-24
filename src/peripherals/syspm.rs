#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "CMX_PERFMON"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Performance Monitor Control"]
    pub PMCR0: crate::RWRegister<u32>,
    _reserved0: [u8; 0x14],
    #[doc = "Performance Monitor Event Counter"]
    pub PMECTR1_HI_0: crate::RWRegister<u8>,
    _reserved1: [u8; 0x3],
    #[doc = "Performance Monitor Event Counter"]
    pub PMECTR1_LO_0: crate::RWRegister<u32>,
    #[doc = "Performance Monitor Event Counter"]
    pub PMECTR2_HI_0: crate::RWRegister<u8>,
    _reserved2: [u8; 0x3],
    #[doc = "Performance Monitor Event Counter"]
    pub PMECTR2_LO_0: crate::RWRegister<u32>,
    #[doc = "Performance Monitor Event Counter"]
    pub PMECTR3_HI_0: crate::RWRegister<u8>,
    _reserved3: [u8; 0x3],
    #[doc = "Performance Monitor Event Counter"]
    pub PMECTR3_LO_0: crate::RWRegister<u32>,
}
#[doc = "Performance Monitor Control"]
pub mod PMCR0 {
    #[doc = "Module Is Enabled"]
    pub mod MENB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start and Stop Control"]
    pub mod SSC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle or no-op"]
            pub const IDLE: u32 = 0;
            #[doc = "Local stop"]
            pub const LSTOP: u32 = 1;
            #[doc = "Local start"]
            pub const LSTART_2: u32 = 2;
            #[doc = "Local start"]
            pub const LSTART_3: u32 = 3;
        }
    }
    #[doc = "Count Mode"]
    pub mod CMODE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counted in both User and Privileged modes"]
            pub const USER_AND_PRIV: u32 = 0;
            #[doc = "Counted only in User mode"]
            pub const USER_ONLY: u32 = 2;
            #[doc = "Counted only in Privileged mode"]
            pub const PRIV_ONLY: u32 = 3;
        }
    }
    #[doc = "Reset Event Counter 1"]
    pub mod RECTR1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Run normally"]
            pub const RUN: u32 = 0;
            #[doc = "Reset"]
            pub const RESET: u32 = 1;
        }
    }
    #[doc = "Reset Event Counter 2"]
    pub mod RECTR2 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Run normally"]
            pub const RUN: u32 = 0;
            #[doc = "Reset"]
            pub const RESET: u32 = 1;
        }
    }
    #[doc = "Reset Event Counter 3"]
    pub mod RECTR3 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Run normally"]
            pub const RUN: u32 = 0;
            #[doc = "Reset"]
            pub const RESET: u32 = 1;
        }
    }
    #[doc = "Select Event 1"]
    pub mod SELEVT1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select Event 2"]
    pub mod SELEVT2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Select Event 3"]
    pub mod SELEVT3 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Monitor Event Counter"]
pub mod PMECTR1_HI_0 {
    #[doc = "Event Counter"]
    pub mod ECTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Monitor Event Counter"]
pub mod PMECTR1_LO_0 {
    #[doc = "Event Counter"]
    pub mod ECTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Monitor Event Counter"]
pub mod PMECTR2_HI_0 {
    #[doc = "Event Counter"]
    pub mod ECTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Monitor Event Counter"]
pub mod PMECTR2_LO_0 {
    #[doc = "Event Counter"]
    pub mod ECTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Monitor Event Counter"]
pub mod PMECTR3_HI_0 {
    #[doc = "Event Counter"]
    pub mod ECTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Performance Monitor Event Counter"]
pub mod PMECTR3_LO_0 {
    #[doc = "Event Counter"]
    pub mod ECTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
