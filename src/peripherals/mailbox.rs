#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "MAILBOX"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "no description available"]
    pub IRQ: [irq::RegisterBlock; 2usize],
    _reserved0: [u8; 0xd8],
    #[doc = "Mutual Exclusion"]
    pub MUTEX: crate::RWRegister<u32>,
}
pub mod irq {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "name (CPUn) Interrupt"]
        pub IRQ: crate::RWRegister<u32>,
        #[doc = "name (CPUn) Interrupt Set"]
        pub IRQSET: crate::RWRegister<u32>,
        #[doc = "name (CPUn) Interrupt Clear"]
        pub IRQCLR: crate::RWRegister<u32>,
    }
    #[doc = "name (CPUn) Interrupt"]
    pub mod IRQ {
        #[doc = "Interrupt Request"]
        pub mod INTREQ {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "name (CPUn) Interrupt Set"]
    pub mod IRQSET {
        #[doc = "Interrupt Request Set 0"]
        pub mod INTREQSET {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "name (CPUn) Interrupt Clear"]
    pub mod IRQCLR {
        #[doc = "Interrupt Request Clear 0"]
        pub mod INTREQCLR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
#[doc = "Mutual Exclusion"]
pub mod MUTEX {
    #[doc = "Mutual Exclusion Request"]
    pub mod EX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Resource unavailable"]
            pub const UNAVAILABLE: u32 = 0;
            #[doc = "Resource available"]
            pub const AVAILABLE: u32 = 1;
        }
    }
}
