#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "RTC_SUBSYSTEM"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x800],
    #[doc = "Subsecond Control"]
    pub SUBSECOND_CTRL: crate::RWRegister<u32>,
    #[doc = "Subsecond Counter"]
    pub SUBSECOND_CNT: crate::RWRegister<u32>,
    _reserved1: [u8; 0x3f8],
    #[doc = "Wake Timer Control"]
    pub WAKE_TIMER_CTRL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x8],
    #[doc = "Wake Timer Counter"]
    pub WAKE_TIMER_CNT: crate::RWRegister<u32>,
}
#[doc = "Subsecond Control"]
pub mod SUBSECOND_CTRL {
    #[doc = "Subsecond Counter Enable"]
    pub mod SUB_SECOND_CNT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Subsecond Counter"]
pub mod SUBSECOND_CNT {
    #[doc = "Current Subsecond Counter Value"]
    pub mod SUBSECOND_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Wake Timer Control"]
pub mod WAKE_TIMER_CTRL {
    #[doc = "Wake Timer Status Flag"]
    pub mod WAKE_FLAG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wake timer has not timed out."]
            pub const DISABLE: u32 = 0;
            #[doc = "Wake timer has timed out."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Clear Wake Timer"]
    pub mod CLR_WAKE_TIMER {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No effect."]
            pub const DISABLE: u32 = 0;
            #[doc = "Clears the wake timer counter and halts operation until a new count value is loaded."]
            pub const ENABLE: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "OSC Divide Enable"]
    pub mod OSC_DIV_ENA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Interrupt"]
    pub mod INTR_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Wake Timer Counter"]
pub mod WAKE_TIMER_CNT {
    #[doc = "Wake Counter"]
    pub mod WAKE_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
