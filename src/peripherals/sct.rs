#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "SCT"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SCT Configuration"]
    pub CONFIG: crate::RWRegister<u32>,
    #[doc = "SCT Control"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "SCT Limit Event Select"]
    pub LIMIT: crate::RWRegister<u32>,
    #[doc = "Halt Event Select"]
    pub HALT: crate::RWRegister<u32>,
    #[doc = "Stop Event Select"]
    pub STOP: crate::RWRegister<u32>,
    #[doc = "Start Event Select"]
    pub START: crate::RWRegister<u32>,
    #[doc = "Dither Condition"]
    pub DITHER: crate::RWRegister<u32>,
    _reserved0: [u8; 0x24],
    #[doc = "Counter Value"]
    pub COUNT: crate::RWRegister<u32>,
    #[doc = "State Variable"]
    pub STATE: crate::RWRegister<u32>,
    #[doc = "Input State"]
    pub INPUT: crate::RWRegister<u32>,
    #[doc = "Match and Capture Register Mode"]
    pub REGMODE: crate::RWRegister<u32>,
    #[doc = "Output State"]
    pub OUTPUT: crate::RWRegister<u32>,
    #[doc = "Output Counter Direction Control"]
    pub OUTPUTDIRCTRL: crate::RWRegister<u32>,
    #[doc = "Output Conflict Resolution"]
    pub RES: crate::RWRegister<u32>,
    #[doc = "DMA Request 0"]
    pub DMAREQ0: crate::RWRegister<u32>,
    #[doc = "DMA Request 1"]
    pub DMAREQ1: crate::RWRegister<u32>,
    _reserved1: [u8; 0x8c],
    #[doc = "Event Interrupt Enable"]
    pub EVEN: crate::RWRegister<u32>,
    #[doc = "Event Flag"]
    pub EVFLAG: crate::RWRegister<u32>,
    #[doc = "Conflict Interrupt Enable"]
    pub CONEN: crate::RWRegister<u32>,
    #[doc = "Conflict Flag"]
    pub CONFLAG: crate::RWRegister<u32>,
    #[doc = "Capture Value"]
    pub CAP: [crate::RWRegister<u32>; 16usize],
    #[doc = "Match Value"]
    pub MATCH: [crate::RWRegister<u32>; 16usize],
    #[doc = "Fractional Match"]
    pub FRACMAT: [crate::RWRegister<u32>; 6usize],
    _reserved2: [u8; 0x68],
    #[doc = "Capture Control"]
    pub CAPCTRL: [crate::RWRegister<u32>; 16usize],
    #[doc = "Match Reload Value"]
    pub MATCHREL: [crate::RWRegister<u32>; 16usize],
    #[doc = "Fractional Match Reload"]
    pub FRACMATREL: [crate::RWRegister<u32>; 6usize],
    _reserved3: [u8; 0x68],
    #[doc = "no description available"]
    pub EVENT: [event::RegisterBlock; 16usize],
    _reserved4: [u8; 0x180],
    #[doc = "no description available"]
    pub OUT: [out::RegisterBlock; 10usize],
}
#[doc = "SCT Configuration"]
pub mod CONFIG {
    #[doc = "SCT Operation"]
    pub mod UNIFY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dual counters, COUNTER_L and COUNTER_H"]
            pub const DUAL_COUNTER: u32 = 0;
            #[doc = "Unified counter"]
            pub const UNIFIED_COUNTER: u32 = 1;
        }
    }
    #[doc = "SCT Clock Mode"]
    pub mod CLKMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "System Clock mode"]
            pub const SYSTEM_CLOCK_MODE: u32 = 0;
            #[doc = "Sampled System Clock mode"]
            pub const SAMPLED_SYSTEM_CLOCK_MODE: u32 = 1;
            #[doc = "SCT Input Clock mode"]
            pub const SCT_INPUT_CLOCK_MODE: u32 = 2;
            #[doc = "Asynchronous mode"]
            pub const ASYNCHRONOUS_MODE: u32 = 3;
        }
    }
    #[doc = "SCT Clock Select"]
    pub mod CKSEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rising edges on input 0"]
            pub const INPUT_0_RISING_EDGES: u32 = 0;
            #[doc = "Falling edges on input 0"]
            pub const INPUT_0_FALLING_EDGES: u32 = 1;
            #[doc = "Rising edges on input 1"]
            pub const INPUT_1_RISING_EDGES: u32 = 2;
            #[doc = "Falling edges on input 1"]
            pub const INPUT_1_FALLING_EDGES: u32 = 3;
            #[doc = "Rising edges on input 2"]
            pub const INPUT_2_RISING_EDGES: u32 = 4;
            #[doc = "Falling edges on input 2"]
            pub const INPUT_2_FALLING_EDGES: u32 = 5;
            #[doc = "Rising edges on input 3"]
            pub const INPUT_3_RISING_EDGES: u32 = 6;
            #[doc = "Falling edges on input 3"]
            pub const INPUT_3_FALLING_EDGES: u32 = 7;
            #[doc = "Rising edges on input 4"]
            pub const INPUT_4_RISING_EDGES: u32 = 8;
            #[doc = "Falling edges on input 4"]
            pub const INPUT_4_FALLING_EDGES: u32 = 9;
            #[doc = "Rising edges on input 5"]
            pub const INPUT_5_RISING_EDGES: u32 = 10;
            #[doc = "Falling edges on input 5"]
            pub const INPUT_5_FALLING_EDGES: u32 = 11;
            #[doc = "Rising edges on input 6"]
            pub const INPUT_6_RISING_EDGES: u32 = 12;
            #[doc = "Falling edges on input 6"]
            pub const INPUT_6_FALLING_EDGES: u32 = 13;
            #[doc = "Rising edges on input 7"]
            pub const INPUT_7_RISING_EDGES: u32 = 14;
            #[doc = "Falling edges on input 7"]
            pub const INPUT_7_FALLING_EDGES: u32 = 15;
        }
    }
    #[doc = "No Reload Lower Match"]
    pub mod NORELOAD_L {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reloaded"]
            pub const RELOAD: u32 = 0;
            #[doc = "Not reloaded"]
            pub const NO_RELOAD: u32 = 1;
        }
    }
    #[doc = "No Reload Higher Match"]
    pub mod NORELOAD_H {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reloaded"]
            pub const RELOAD_H: u32 = 0;
            #[doc = "Not reloaded"]
            pub const NO_RELOAD_H: u32 = 1;
        }
    }
    #[doc = "Input Synchronization"]
    pub mod INSYNC {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Auto Limit Lower"]
    pub mod AUTOLIMIT_L {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Auto Limit Higher"]
    pub mod AUTOLIMIT_H {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "SCT Control"]
pub mod CTRL {
    #[doc = "Down Counter Low"]
    pub mod DOWN_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Up"]
            pub const UP: u32 = 0;
            #[doc = "Down"]
            pub const DOWN: u32 = 1;
        }
    }
    #[doc = "Stop Counter Low"]
    pub mod STOP_L {
        pub const offset: u32 = 1;
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
    #[doc = "Halt Counter Low"]
    pub mod HALT_L {
        pub const offset: u32 = 2;
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
    #[doc = "Clear Counter Low"]
    pub mod CLRCTR_L {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bidirectional Select Low"]
    pub mod BIDIR_L {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Up"]
            pub const UP: u32 = 0;
            #[doc = "Up-down"]
            pub const UP_DOWN: u32 = 1;
        }
    }
    #[doc = "Prescaler for Low Counter"]
    pub mod PRE_L {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Down Counter High"]
    pub mod DOWN_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Up"]
            pub const UP: u32 = 0;
            #[doc = "Down"]
            pub const DOWN: u32 = 1;
        }
    }
    #[doc = "Stop Counter High"]
    pub mod STOP_H {
        pub const offset: u32 = 17;
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
    #[doc = "Halt Counter High"]
    pub mod HALT_H {
        pub const offset: u32 = 18;
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
    #[doc = "Clear Counter High"]
    pub mod CLRCTR_H {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bidirectional Select High"]
    pub mod BIDIR_H {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Up"]
            pub const UP: u32 = 0;
            #[doc = "Up-down"]
            pub const UP_DOWN: u32 = 1;
        }
    }
    #[doc = "Prescaler for High Counter"]
    pub mod PRE_H {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SCT Limit Event Select"]
pub mod LIMIT {
    #[doc = "Limit Event Counter Low"]
    pub mod LIMMSK_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Limit Event Counter High"]
    pub mod LIMMSK_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Halt Event Select"]
pub mod HALT {
    #[doc = "Halt Event Low"]
    pub mod HALTMSK_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Halt Event High"]
    pub mod HALTMSK_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Stop Event Select"]
pub mod STOP {
    #[doc = "Stop Event Low"]
    pub mod STOPMSK_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stop Event High"]
    pub mod STOPMSK_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Start Event Select"]
pub mod START {
    #[doc = "Start Event Low"]
    pub mod STARTMSK_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start Event High"]
    pub mod STARTMSK_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Dither Condition"]
pub mod DITHER {
    #[doc = "Dither Low"]
    pub mod DITHER_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Dither High"]
    pub mod DITHER_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter Value"]
pub mod COUNT {
    #[doc = "Counter Low"]
    pub mod CTR_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Counter High"]
    pub mod CTR_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "State Variable"]
pub mod STATE {
    #[doc = "State Variable Low"]
    pub mod STATE_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "State Variable High"]
    pub mod STATE_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input State"]
pub mod INPUT {
    #[doc = "Input 0 state"]
    pub mod AIN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 1 state"]
    pub mod AIN1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 2 state"]
    pub mod AIN2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 3 state"]
    pub mod AIN3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 4 state"]
    pub mod AIN4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 5 state"]
    pub mod AIN5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 6 state"]
    pub mod AIN6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 7 state"]
    pub mod AIN7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 8 state"]
    pub mod AIN8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 9 state"]
    pub mod AIN9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 10 state"]
    pub mod AIN10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 11 state"]
    pub mod AIN11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 12 state"]
    pub mod AIN12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 13 state"]
    pub mod AIN13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 14 state"]
    pub mod AIN14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 15 state"]
    pub mod AIN15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 0 state"]
    pub mod SIN0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 1 state"]
    pub mod SIN1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 2 state"]
    pub mod SIN2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 3 state"]
    pub mod SIN3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 4 state"]
    pub mod SIN4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 5 state"]
    pub mod SIN5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 6 state"]
    pub mod SIN6 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 7 state"]
    pub mod SIN7 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 8 state"]
    pub mod SIN8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 9 state"]
    pub mod SIN9 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 10 state"]
    pub mod SIN10 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 11 state"]
    pub mod SIN11 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 12 state"]
    pub mod SIN12 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 13 state"]
    pub mod SIN13 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 14 state"]
    pub mod SIN14 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input 15 state"]
    pub mod SIN15 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Match and Capture Register Mode"]
pub mod REGMODE {
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode Low"]
    pub mod REGMOD_L15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H6 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H7 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H9 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H10 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H11 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H12 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H13 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H14 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
    #[doc = "Register Mode High"]
    pub mod REGMOD_H15 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match"]
            pub const MATCH: u32 = 0;
            #[doc = "Capture"]
            pub const CAPTURE: u32 = 1;
        }
    }
}
#[doc = "Output State"]
pub mod OUTPUT {
    #[doc = "Output Low and High"]
    pub mod OUT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forces the corresponding output low"]
            pub const LOW: u32 = 0;
            #[doc = "Forces the corresponding output high"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "Output Low and High"]
    pub mod OUT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forces the corresponding output low"]
            pub const LOW: u32 = 0;
            #[doc = "Forces the corresponding output high"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "Output Low and High"]
    pub mod OUT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forces the corresponding output low"]
            pub const LOW: u32 = 0;
            #[doc = "Forces the corresponding output high"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "Output Low and High"]
    pub mod OUT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forces the corresponding output low"]
            pub const LOW: u32 = 0;
            #[doc = "Forces the corresponding output high"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "Output Low and High"]
    pub mod OUT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forces the corresponding output low"]
            pub const LOW: u32 = 0;
            #[doc = "Forces the corresponding output high"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "Output Low and High"]
    pub mod OUT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forces the corresponding output low"]
            pub const LOW: u32 = 0;
            #[doc = "Forces the corresponding output high"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "Output Low and High"]
    pub mod OUT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forces the corresponding output low"]
            pub const LOW: u32 = 0;
            #[doc = "Forces the corresponding output high"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "Output Low and High"]
    pub mod OUT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forces the corresponding output low"]
            pub const LOW: u32 = 0;
            #[doc = "Forces the corresponding output high"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "Output Low and High"]
    pub mod OUT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forces the corresponding output low"]
            pub const LOW: u32 = 0;
            #[doc = "Forces the corresponding output high"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "Output Low and High"]
    pub mod OUT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Forces the corresponding output low"]
            pub const LOW: u32 = 0;
            #[doc = "Forces the corresponding output high"]
            pub const HIGH: u32 = 1;
        }
    }
}
#[doc = "Output Counter Direction Control"]
pub mod OUTPUTDIRCTRL {
    #[doc = "Set and Clear Operation on Output"]
    pub mod SETCLR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not dependent on the direction of any counter"]
            pub const INDEPENDENT: u32 = 0;
            #[doc = "Reversed when counter L or the unified counter is counting down"]
            pub const L_REVERSED: u32 = 1;
            #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG[UNIFY] = 1)"]
            pub const H_REVERSED: u32 = 2;
        }
    }
    #[doc = "Set and Clear Operation on Output"]
    pub mod SETCLR1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not dependent on the direction of any counter"]
            pub const INDEPENDENT: u32 = 0;
            #[doc = "Reversed when counter L or the unified counter is counting down"]
            pub const L_REVERSED: u32 = 1;
            #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG[UNIFY] = 1)"]
            pub const H_REVERSED: u32 = 2;
        }
    }
    #[doc = "Set and Clear Operation on Output"]
    pub mod SETCLR2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not dependent on the direction of any counter"]
            pub const INDEPENDENT: u32 = 0;
            #[doc = "Reversed when counter L or the unified counter is counting down"]
            pub const L_REVERSED: u32 = 1;
            #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG[UNIFY] = 1)"]
            pub const H_REVERSED: u32 = 2;
        }
    }
    #[doc = "Set and Clear Operation on Output"]
    pub mod SETCLR3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not dependent on the direction of any counter"]
            pub const INDEPENDENT: u32 = 0;
            #[doc = "Reversed when counter L or the unified counter is counting down"]
            pub const L_REVERSED: u32 = 1;
            #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG[UNIFY] = 1)"]
            pub const H_REVERSED: u32 = 2;
        }
    }
    #[doc = "Set and Clear Operation on Output"]
    pub mod SETCLR4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not dependent on the direction of any counter"]
            pub const INDEPENDENT: u32 = 0;
            #[doc = "Reversed when counter L or the unified counter is counting down"]
            pub const L_REVERSED: u32 = 1;
            #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG[UNIFY] = 1)"]
            pub const H_REVERSED: u32 = 2;
        }
    }
    #[doc = "Set and Clear Operation on Output"]
    pub mod SETCLR5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not dependent on the direction of any counter"]
            pub const INDEPENDENT: u32 = 0;
            #[doc = "Reversed when counter L or the unified counter is counting down"]
            pub const L_REVERSED: u32 = 1;
            #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG[UNIFY] = 1)"]
            pub const H_REVERSED: u32 = 2;
        }
    }
    #[doc = "Set and Clear Operation on Output"]
    pub mod SETCLR6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not dependent on the direction of any counter"]
            pub const INDEPENDENT: u32 = 0;
            #[doc = "Reversed when counter L or the unified counter is counting down"]
            pub const L_REVERSED: u32 = 1;
            #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG[UNIFY] = 1)"]
            pub const H_REVERSED: u32 = 2;
        }
    }
    #[doc = "Set and Clear Operation on Output"]
    pub mod SETCLR7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not dependent on the direction of any counter"]
            pub const INDEPENDENT: u32 = 0;
            #[doc = "Reversed when counter L or the unified counter is counting down"]
            pub const L_REVERSED: u32 = 1;
            #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG[UNIFY] = 1)"]
            pub const H_REVERSED: u32 = 2;
        }
    }
    #[doc = "Set and Clear Operation on Output"]
    pub mod SETCLR8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not dependent on the direction of any counter"]
            pub const INDEPENDENT: u32 = 0;
            #[doc = "Reversed when counter L or the unified counter is counting down"]
            pub const L_REVERSED: u32 = 1;
            #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG[UNIFY] = 1)"]
            pub const H_REVERSED: u32 = 2;
        }
    }
    #[doc = "Set and Clear Operation on Output"]
    pub mod SETCLR9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not dependent on the direction of any counter"]
            pub const INDEPENDENT: u32 = 0;
            #[doc = "Reversed when counter L or the unified counter is counting down"]
            pub const L_REVERSED: u32 = 1;
            #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG[UNIFY] = 1)"]
            pub const H_REVERSED: u32 = 2;
        }
    }
}
#[doc = "Output Conflict Resolution"]
pub mod RES {
    #[doc = "Output Resolution"]
    pub mod O0RES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Set output (or clear, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const SET: u32 = 1;
            #[doc = "Clear output (or set, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const CLEAR: u32 = 2;
            #[doc = "Toggle output"]
            pub const TOGGLE_OUTPUT: u32 = 3;
        }
    }
    #[doc = "Output Resolution"]
    pub mod O1RES {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Set output (or clear, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const SET: u32 = 1;
            #[doc = "Clear output (or set, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const CLEAR: u32 = 2;
            #[doc = "Toggle output"]
            pub const TOGGLE_OUTPUT: u32 = 3;
        }
    }
    #[doc = "Output Resolution"]
    pub mod O2RES {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Set output (or clear, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const SET: u32 = 1;
            #[doc = "Clear output (or set, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const CLEAR: u32 = 2;
            #[doc = "Toggle output"]
            pub const TOGGLE_OUTPUT: u32 = 3;
        }
    }
    #[doc = "Output Resolution"]
    pub mod O3RES {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Set output (or clear, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const SET: u32 = 1;
            #[doc = "Clear output (or set, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const CLEAR: u32 = 2;
            #[doc = "Toggle output"]
            pub const TOGGLE_OUTPUT: u32 = 3;
        }
    }
    #[doc = "Output Resolution"]
    pub mod O4RES {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Set output (or clear, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const SET: u32 = 1;
            #[doc = "Clear output (or set, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const CLEAR: u32 = 2;
            #[doc = "Toggle output"]
            pub const TOGGLE_OUTPUT: u32 = 3;
        }
    }
    #[doc = "Output Resolution"]
    pub mod O5RES {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Set output (or clear, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const SET: u32 = 1;
            #[doc = "Clear output (or set, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const CLEAR: u32 = 2;
            #[doc = "Toggle output"]
            pub const TOGGLE_OUTPUT: u32 = 3;
        }
    }
    #[doc = "Output Resolution"]
    pub mod O6RES {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Set output (or clear, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const SET: u32 = 1;
            #[doc = "Clear output (or set, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const CLEAR: u32 = 2;
            #[doc = "Toggle output"]
            pub const TOGGLE_OUTPUT: u32 = 3;
        }
    }
    #[doc = "Output Resolution"]
    pub mod O7RES {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Set output (or clear, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const SET: u32 = 1;
            #[doc = "Clear output (or set, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const CLEAR: u32 = 2;
            #[doc = "Toggle output"]
            pub const TOGGLE_OUTPUT: u32 = 3;
        }
    }
    #[doc = "Output Resolution"]
    pub mod O8RES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Set output (or clear, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const SET: u32 = 1;
            #[doc = "Clear output (or set, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const CLEAR: u32 = 2;
            #[doc = "Toggle output"]
            pub const TOGGLE_OUTPUT: u32 = 3;
        }
    }
    #[doc = "Output Resolution"]
    pub mod O9RES {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No change"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Set output (or clear, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const SET: u32 = 1;
            #[doc = "Clear output (or set, based on OUTPUTDIRCTRL[SETCLRn])"]
            pub const CLEAR: u32 = 2;
            #[doc = "Toggle output"]
            pub const TOGGLE_OUTPUT: u32 = 3;
        }
    }
}
#[doc = "DMA Request 0"]
pub mod DMAREQ0 {
    #[doc = "DMA Request Event"]
    pub mod DEV_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Low 0"]
    pub mod DRL0 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request 0 State"]
    pub mod DRQ0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Request 1"]
pub mod DMAREQ1 {
    #[doc = "DMA Request Event"]
    pub mod DEV_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Event"]
    pub mod DEV_15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request Low 1"]
    pub mod DRL1 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Request 1 State"]
    pub mod DRQ1 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Event Interrupt Enable"]
pub mod EVEN {
    #[doc = "Event Interrupt Enable"]
    pub mod IEN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod IEN15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Event Flag"]
pub mod EVFLAG {
    #[doc = "Event Flag"]
    pub mod FLAG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Event Flag"]
    pub mod FLAG15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No flag"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Event n flag"]
            pub const FLAG: u32 = 1;
        }
    }
}
#[doc = "Conflict Interrupt Enable"]
pub mod CONEN {
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    pub mod NCEN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    pub mod NCEN1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    pub mod NCEN2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    pub mod NCEN3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    pub mod NCEN4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    pub mod NCEN5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    pub mod NCEN6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    pub mod NCEN7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    pub mod NCEN8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event and Interrupt Enable"]
    pub mod NCEN9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 1;
        }
    }
}
#[doc = "Conflict Flag"]
pub mod CONFLAG {
    #[doc = "No Change Conflict Event Flag"]
    pub mod NCFLAG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const NO_NC_CONFLICT: u32 = 0;
            #[doc = "Occurred"]
            pub const NC_CONFLICT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event Flag"]
    pub mod NCFLAG1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const NO_NC_CONFLICT: u32 = 0;
            #[doc = "Occurred"]
            pub const NC_CONFLICT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event Flag"]
    pub mod NCFLAG2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const NO_NC_CONFLICT: u32 = 0;
            #[doc = "Occurred"]
            pub const NC_CONFLICT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event Flag"]
    pub mod NCFLAG3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const NO_NC_CONFLICT: u32 = 0;
            #[doc = "Occurred"]
            pub const NC_CONFLICT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event Flag"]
    pub mod NCFLAG4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const NO_NC_CONFLICT: u32 = 0;
            #[doc = "Occurred"]
            pub const NC_CONFLICT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event Flag"]
    pub mod NCFLAG5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const NO_NC_CONFLICT: u32 = 0;
            #[doc = "Occurred"]
            pub const NC_CONFLICT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event Flag"]
    pub mod NCFLAG6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const NO_NC_CONFLICT: u32 = 0;
            #[doc = "Occurred"]
            pub const NC_CONFLICT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event Flag"]
    pub mod NCFLAG7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const NO_NC_CONFLICT: u32 = 0;
            #[doc = "Occurred"]
            pub const NC_CONFLICT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event Flag"]
    pub mod NCFLAG8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const NO_NC_CONFLICT: u32 = 0;
            #[doc = "Occurred"]
            pub const NC_CONFLICT: u32 = 1;
        }
    }
    #[doc = "No Change Conflict Event Flag"]
    pub mod NCFLAG9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const NO_NC_CONFLICT: u32 = 0;
            #[doc = "Occurred"]
            pub const NC_CONFLICT: u32 = 1;
        }
    }
    #[doc = "Bus Error Low or Unified"]
    pub mod BUSERRL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus Error High"]
    pub mod BUSERRH {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Value"]
pub mod CAP {
    #[doc = "Capture Low"]
    pub mod CAPn_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Capture High"]
    pub mod CAPn_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Match Value"]
pub mod MATCH {
    #[doc = "Match Low"]
    pub mod MATCHn_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Match High"]
    pub mod MATCHn_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Match"]
pub mod FRACMAT {
    #[doc = "Fractional Match Low"]
    pub mod FRACMAT_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fractional Match High"]
    pub mod FRACMAT_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Control"]
pub mod CAPCTRL {
    #[doc = "Capture Control Low"]
    pub mod CAPCONn_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Capture Control High"]
    pub mod CAPCONn_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Match Reload Value"]
pub mod MATCHREL {
    #[doc = "Reload Low"]
    pub mod RELOADn_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reload High"]
    pub mod RELOADn_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Fractional Match Reload"]
pub mod FRACMATREL {
    #[doc = "Reload Fractional Match Low"]
    pub mod FRACMAT_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reload Fractional Match High"]
    pub mod RELFRAC_H {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod event {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Event n State"]
        pub EV_STATE: crate::RWRegister<u32>,
        #[doc = "Event n Control"]
        pub EV_CTRL: crate::RWRegister<u32>,
    }
    #[doc = "Event n State"]
    pub mod EV_STATE {
        #[doc = "Event State Mask"]
        pub mod STATEMSKn {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Event n Control"]
    pub mod EV_CTRL {
        #[doc = "Match Select"]
        pub mod MATCHSEL {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "High Event"]
        pub mod HEVENT {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Low counter (selects the L state and the L match register that the MATCHSEL field specifies)"]
                pub const L_COUNTER: u32 = 0;
                #[doc = "High counter (selects the H state and the H match register that the MATCHSEL field specifies)"]
                pub const H_COUNTER: u32 = 1;
            }
        }
        #[doc = "Input and Output Select"]
        pub mod OUTSEL {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Inputs"]
                pub const INPUT: u32 = 0;
                #[doc = "Outputs"]
                pub const OUTPUT: u32 = 1;
            }
        }
        #[doc = "Input or Output Signal Select"]
        pub mod IOSEL {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Input or Output Condition"]
        pub mod IOCOND {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Low"]
                pub const LOW: u32 = 0;
                #[doc = "Rise"]
                pub const RISE: u32 = 1;
                #[doc = "Fall"]
                pub const FALL: u32 = 2;
                #[doc = "High"]
                pub const HIGH: u32 = 3;
            }
        }
        #[doc = "Combination Mode"]
        pub mod COMBMODE {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "OR (the event occurs when either the specified match or I/O condition occurs)"]
                pub const OR: u32 = 0;
                #[doc = "MATCH (uses the specified match only)"]
                pub const MATCH: u32 = 1;
                #[doc = "IO (uses the specified I/O condition only)"]
                pub const IO: u32 = 2;
                #[doc = "AND (the event occurs when the specified match and I/O condition occur simultaneously)"]
                pub const AND: u32 = 3;
            }
        }
        #[doc = "State Load"]
        pub mod STATELD {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Value of STATEV added to that of STATE (the carry out is ignored)"]
                pub const ADD: u32 = 0;
                #[doc = "Value of STATEV loaded into that of STATE"]
                pub const LOAD: u32 = 1;
            }
        }
        #[doc = "State Value"]
        pub mod STATEV {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b11111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Match Mem"]
        pub mod MATCHMEM {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Direction"]
        pub mod DIRECTION {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Direction independent (event triggered regardless of the count direction)"]
                pub const DIRECTION_INDEPENDENT: u32 = 0;
                #[doc = "Counting up (event triggered only during up-counting when CTRL[BIDIR] = 1)"]
                pub const COUNTING_UP: u32 = 1;
                #[doc = "Counting down (event triggered only during down-counting when CTRL[BIDIR] = 1)"]
                pub const COUNTING_DOWN: u32 = 2;
            }
        }
    }
}
pub mod out {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Output n Set"]
        pub OUT_SET: crate::RWRegister<u32>,
        #[doc = "Output n Clear"]
        pub OUT_CLR: crate::RWRegister<u32>,
    }
    #[doc = "Output n Set"]
    pub mod OUT_SET {
        #[doc = "Set Output"]
        pub mod SET {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Output n Clear"]
    pub mod OUT_CLR {
        #[doc = "Clear Output"]
        pub mod CLR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
