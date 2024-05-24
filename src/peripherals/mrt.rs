#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "Multi-Rate Timer (MRT)"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "no description available"]
    pub CHANNEL: [channel::RegisterBlock; 4usize],
    _reserved0: [u8; 0xb0],
    #[doc = "Module Configuration"]
    pub MODCFG: crate::RWRegister<u32>,
    #[doc = "Idle Channel"]
    pub IDLE_CH: crate::RWRegister<u32>,
    #[doc = "Global Interrupt Flag"]
    pub IRQ_FLAG: crate::RWRegister<u32>,
}
pub mod channel {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Time Interval Value"]
        pub INTVAL: crate::RWRegister<u32>,
        #[doc = "Timer"]
        pub TIMER: crate::RWRegister<u32>,
        #[doc = "Control"]
        pub CTRL: crate::RWRegister<u32>,
        #[doc = "Status"]
        pub STAT: crate::RWRegister<u32>,
    }
    #[doc = "Time Interval Value"]
    pub mod INTVAL {
        #[doc = "Time Interval Load Value."]
        pub mod IVALUE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Force Load Enable"]
        pub mod LOAD {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No force load"]
                pub const NO_FORCE_LOAD: u32 = 0;
                #[doc = "Force load"]
                pub const FORCE_LOAD: u32 = 1;
            }
        }
    }
    #[doc = "Timer"]
    pub mod TIMER {
        #[doc = "Current Timer Value"]
        pub mod VALUE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Control"]
    pub mod CTRL {
        #[doc = "Interrupt request"]
        pub mod INTEN {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled"]
                pub const DISABLED: u32 = 0;
                #[doc = "Enabled"]
                pub const ENABLED: u32 = 1;
            }
        }
        #[doc = "MRT Operating mode"]
        pub mod MODE {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Repeat Interrupt mode"]
                pub const REPEAT_INTERRUPT_MODE: u32 = 0;
                #[doc = "One-Shot Interrupt mode"]
                pub const ONE_SHOT_INTERRUPT_MODE: u32 = 1;
                #[doc = "One-Shot Stall mode"]
                pub const ONE_SHOT_STALL_MODE: u32 = 2;
            }
        }
    }
    #[doc = "Status"]
    pub mod STAT {
        #[doc = "Monitors the interrupt flag. Writing 0 indicates no pending interrupt or no operation. Writing 1 indicates pending interrupt, because TIMERn has reached the end of the time interval. If CTRL and the global interrupt are generated. Writing 1 to this field bit clears the interrupt request.n in the CONTROLn also gets 1, then the interrupt for timer channel n"]
        pub mod INTFLAG {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No pending interrupt."]
                pub const NO_PENDING_INTERRUPT: u32 = 0;
                #[doc = "Pending interrupt."]
                pub const PENDING_INTERRUPT: u32 = 1;
            }
        }
        #[doc = "Timer n State"]
        pub mod RUN {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Idle state."]
                pub const IDLE_STATE: u32 = 0;
                #[doc = "Running."]
                pub const RUNNING: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Channel-In-Use flag"]
        pub mod INUSE {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "This timer channel is not in use."]
                pub const NO: u32 = 0;
                #[doc = "This timer channel is in use."]
                pub const YES: u32 = 1;
            }
        }
    }
}
#[doc = "Module Configuration"]
pub mod MODCFG {
    #[doc = "Number of Channels"]
    pub mod NOC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Bits"]
    pub mod NOB {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MULTITASK"]
    pub mod MULTITASK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware status mode."]
            pub const HARDWARE_STATUS_MODE: u32 = 0;
            #[doc = "Multitask mode"]
            pub const MULTI_TASK_MODE: u32 = 1;
        }
    }
}
#[doc = "Idle Channel"]
pub mod IDLE_CH {
    #[doc = "Idle Channel"]
    pub mod CHAN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Global Interrupt Flag"]
pub mod IRQ_FLAG {
    #[doc = "Interrupt Flag"]
    pub mod GFLAG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No pending interrupt."]
            pub const NO_PENDING_INTERRUPT: u32 = 0;
            #[doc = "Pending interrupt"]
            pub const PENDING_INTERRUPT: u32 = 1;
        }
    }
    #[doc = "Interrupt Flag"]
    pub mod GFLAG1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Flag"]
    pub mod GFLAG2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Flag"]
    pub mod GFLAG3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
