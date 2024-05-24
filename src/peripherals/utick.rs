#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "UTICK"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Status"]
    pub STAT: crate::RWRegister<u32>,
    #[doc = "Capture Configuration"]
    pub CFG: crate::RWRegister<u32>,
    #[doc = "Capture Clear"]
    pub CAPCLR: crate::RWRegister<u32>,
    #[doc = "Capture"]
    pub CAP: [crate::RWRegister<u32>; 4usize],
}
#[doc = "Control"]
pub mod CTRL {
    #[doc = "Tick Interval"]
    pub mod DELAYVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Repeat Delay"]
    pub mod REPEAT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "One-time delay"]
            pub const DELAYONCE: u32 = 0;
            #[doc = "Delay repeats continuously"]
            pub const DELAYREPEATS: u32 = 1;
        }
    }
}
#[doc = "Status"]
pub mod STAT {
    #[doc = "Interrupt Flag"]
    pub mod INTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not pending"]
            pub const NOPENDINGINTERRUPT: u32 = 0;
            #[doc = "Pending"]
            pub const PENDINGINTERRUPT: u32 = 1;
        }
    }
    #[doc = "Timer Active Flag"]
    pub mod ACTIVE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Inactive (stopped)"]
            pub const TIMERISNOTACTIVE: u32 = 0;
            #[doc = "Active"]
            pub const TIMERISACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Configuration"]
pub mod CFG {
    #[doc = "Enable Capture 0"]
    pub mod CAPEN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CAPEN0ISDISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const CAPEN0ISENABLED: u32 = 1;
        }
    }
    #[doc = "Enable Capture 1"]
    pub mod CAPEN1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CAPEN1ISDISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const CAPEN1ISENABLED: u32 = 1;
        }
    }
    #[doc = "Enable Capture 2"]
    pub mod CAPEN2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CAPEN2ISDISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const CAPEN2ISENABLED: u32 = 1;
        }
    }
    #[doc = "Enable Capture 3"]
    pub mod CAPEN3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CAPEN3ISDISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const CAPEN3ISENABLED: u32 = 1;
        }
    }
    #[doc = "Capture Polarity 0"]
    pub mod CAPPOL0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Positive"]
            pub const CAPPOL0POSEDGECAPTURE: u32 = 0;
            #[doc = "Negative"]
            pub const CAPPOL0NEGEDGECAPTURE: u32 = 1;
        }
    }
    #[doc = "Capture-Polarity 1"]
    pub mod CAPPOL1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Positive"]
            pub const CAPPOL1POSEDGECAPTURE: u32 = 0;
            #[doc = "Negative"]
            pub const CAPPOL1NEGEDGECAPTURE: u32 = 1;
        }
    }
    #[doc = "Capture Polarity 2"]
    pub mod CAPPOL2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Positive"]
            pub const CAPPOL2POSEDGECAPTURE: u32 = 0;
            #[doc = "Negative"]
            pub const CAPPOL2NEGEDGECAPTURE: u32 = 1;
        }
    }
    #[doc = "Capture Polarity 3"]
    pub mod CAPPOL3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Positive"]
            pub const CAPPOL3POSEDGECAPTURE: u32 = 0;
            #[doc = "Negative"]
            pub const CAPPOL3NEGEDGECAPTURE: u32 = 1;
        }
    }
}
#[doc = "Capture Clear"]
pub mod CAPCLR {
    #[doc = "Clear Capture 0"]
    pub mod CAPCLR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Does nothing"]
            pub const CAPCLR0NOTHING: u32 = 0;
            #[doc = "Clears the CAP0 register value"]
            pub const CAPCLR0CLEARED: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Clear Capture 1"]
    pub mod CAPCLR1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Does nothing"]
            pub const CAPCLR1NOTHING: u32 = 0;
            #[doc = "Clears the CAP1 register value"]
            pub const CAPCLR1CLEARED: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Clear Capture 2"]
    pub mod CAPCLR2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Does nothing"]
            pub const CAPCLR2NOTHING: u32 = 0;
            #[doc = "Clears the CAP2 register value"]
            pub const CAPCLR2CLEARED: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Clear Capture 3"]
    pub mod CAPCLR3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Does nothing"]
            pub const CAPCLR3NOTHING: u32 = 0;
            #[doc = "Clears the CAP3 register value"]
            pub const CAPCLR3CLEARED: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Capture"]
pub mod CAP {
    #[doc = "Captured Value for the Related Capture Event"]
    pub mod CAP_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Captured Value Valid Flag"]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Valid value not captured"]
            pub const NOTVALID: u32 = 0;
            #[doc = "Valid value captured"]
            pub const VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
