#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "OSTIMER"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "EVTIMER Low"]
    pub EVTIMERL: crate::RWRegister<u32>,
    #[doc = "EVTIMER High"]
    pub EVTIMERH: crate::RWRegister<u32>,
    #[doc = "Local Capture Low for CPU"]
    pub CAPTURE_L: crate::RWRegister<u32>,
    #[doc = "Local Capture High for CPU"]
    pub CAPTURE_H: crate::RWRegister<u32>,
    #[doc = "Local Match Low for CPU"]
    pub MATCH_L: crate::RWRegister<u32>,
    #[doc = "Local Match High for CPU"]
    pub MATCH_H: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "OSTIMER Control for CPU"]
    pub OSEVENT_CTRL: crate::RWRegister<u32>,
}
#[doc = "EVTIMER Low"]
pub mod EVTIMERL {
    #[doc = "EVTimer Count Value"]
    pub mod EVTIMER_COUNT_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EVTIMER High"]
pub mod EVTIMERH {
    #[doc = "EVTimer Count Value"]
    pub mod EVTIMER_COUNT_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Local Capture Low for CPU"]
pub mod CAPTURE_L {
    #[doc = "EVTimer Capture Value"]
    pub mod CAPTURE_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Local Capture High for CPU"]
pub mod CAPTURE_H {
    #[doc = "EVTimer Capture Value"]
    pub mod CAPTURE_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Local Match Low for CPU"]
pub mod MATCH_L {
    #[doc = "EVTimer Match Value"]
    pub mod MATCH_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Local Match High for CPU"]
pub mod MATCH_H {
    #[doc = "EVTimer Match Value"]
    pub mod MATCH_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OSTIMER Control for CPU"]
pub mod OSEVENT_CTRL {
    #[doc = "Interrupt Flag"]
    pub mod OSTIMER_INTRFLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt or Wake-Up Request"]
    pub mod OSTIMER_INTENA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupts blocked"]
            pub const INTERRUPTS_BLOCKED: u32 = 0;
            #[doc = "Interrupts enabled"]
            pub const INTERRUPTS_ENABLED: u32 = 1;
        }
    }
    #[doc = "EVTimer Match Write Ready"]
    pub mod MATCH_WR_RDY {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
