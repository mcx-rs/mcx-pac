#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "INTM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Monitor Mode"]
    pub INTM_MM: crate::RWRegister<u32>,
    #[doc = "Interrupt Acknowledge"]
    pub INTM_IACK: crate::RWRegister<u32>,
    #[doc = "Monitoring"]
    pub mon: [mon::RegisterBlock; 4usize],
}
#[doc = "Monitor Mode"]
pub mod INTM_MM {
    #[doc = "Monitor Mode"]
    pub mod MM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Interrupt Acknowledge"]
pub mod INTM_IACK {
    #[doc = "Interrupt Request"]
    pub mod IRQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod mon {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "Monitoring"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Interrupt Request Select for Monitor mon_index"]
        pub INTM_IRQSEL: crate::RWRegister<u32>,
        #[doc = "Interrupt Latency for Monitor mon_index"]
        pub INTM_LATENCY: crate::RWRegister<u32>,
        #[doc = "Timer for Monitor mon_index"]
        pub INTM_TIMER: crate::RWRegister<u32>,
        #[doc = "Status for Monitor mon_index"]
        pub INTM_STATUS: crate::RWRegister<u32>,
    }
    #[doc = "Interrupt Request Select for Monitor mon_index"]
    pub mod INTM_IRQSEL {
        #[doc = "Interrupt Request"]
        pub mod IRQ {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Interrupt Latency for Monitor mon_index"]
    pub mod INTM_LATENCY {
        #[doc = "Latency"]
        pub mod LAT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Timer for Monitor mon_index"]
    pub mod INTM_TIMER {
        #[doc = "Timer"]
        pub mod TIMER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Status for Monitor mon_index"]
    pub mod INTM_STATUS {
        #[doc = "Monitor status"]
        pub mod STATUS {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Did not exceed"]
                pub const ST02: u32 = 0;
                #[doc = "Exceeded"]
                pub const ST01: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
    }
}
