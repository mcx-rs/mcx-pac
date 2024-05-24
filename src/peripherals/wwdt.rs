#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "WWDT"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Mode"]
    pub MOD: crate::RWRegister<u32>,
    #[doc = "Timer Constant"]
    pub TC: crate::RWRegister<u32>,
    #[doc = "Feed Sequence"]
    pub FEED: crate::RWRegister<u32>,
    #[doc = "Timer Value"]
    pub TV: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "Warning Interrupt Compare Value"]
    pub WARNINT: crate::RWRegister<u32>,
    #[doc = "Window Compare Value"]
    pub WINDOW: crate::RWRegister<u32>,
}
#[doc = "Mode"]
pub mod MOD {
    #[doc = "Watchdog Enable"]
    pub mod WDEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer stopped"]
            pub const STOP: u32 = 0;
            #[doc = "Timer running"]
            pub const RUN: u32 = 1;
        }
    }
    #[doc = "Watchdog Reset Enable"]
    pub mod WDRESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "Reset"]
            pub const RESET: u32 = 1;
        }
    }
    #[doc = "Watchdog Timeout Flag"]
    pub mod WDTOF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog event has not occurred."]
            pub const CLEAR: u32 = 0;
            #[doc = "Watchdog event has occurred (causes a chip reset if WDRESET = 1)."]
            pub const RESET: u32 = 1;
        }
    }
    #[doc = "Warning Interrupt Flag"]
    pub mod WDINT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Watchdog Update Mode"]
    pub mod WDPROTECT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flexible"]
            pub const FLEXIBLE: u32 = 0;
            #[doc = "Threshold"]
            pub const THRESHOLD: u32 = 1;
        }
    }
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Lock"]
            pub const NO_LOCK: u32 = 0;
            #[doc = "Lock"]
            pub const LOCK: u32 = 1;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DEBUG_EN {
        pub const offset: u32 = 6;
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
#[doc = "Timer Constant"]
pub mod TC {
    #[doc = "Watchdog Timeout Value"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Feed Sequence"]
pub mod FEED {
    #[doc = "Feed Value"]
    pub mod FEED {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Value"]
pub mod TV {
    #[doc = "Counter Timer Value"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Warning Interrupt Compare Value"]
pub mod WARNINT {
    #[doc = "Watchdog Warning Interrupt Compare Value"]
    pub mod WARNINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Window Compare Value"]
pub mod WINDOW {
    #[doc = "Watchdog Window Value"]
    pub mod WINDOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
