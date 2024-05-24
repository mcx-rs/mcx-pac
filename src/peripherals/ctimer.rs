#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "CTIMER"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Interrupt"]
    pub IR: crate::RWRegister<u32>,
    #[doc = "Timer Control"]
    pub TCR: crate::RWRegister<u32>,
    #[doc = "Timer Counter"]
    pub TC: crate::RWRegister<u32>,
    #[doc = "Prescale"]
    pub PR: crate::RWRegister<u32>,
    #[doc = "Prescale Counter"]
    pub PC: crate::RWRegister<u32>,
    #[doc = "Match Control"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "Match"]
    pub MR: [crate::RWRegister<u32>; 4usize],
    #[doc = "Capture Control"]
    pub CCR: crate::RWRegister<u32>,
    #[doc = "Capture"]
    pub CR: [crate::RWRegister<u32>; 4usize],
    #[doc = "External Match"]
    pub EMR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x30],
    #[doc = "Count Control"]
    pub CTCR: crate::RWRegister<u32>,
    #[doc = "PWM Control"]
    pub PWMC: crate::RWRegister<u32>,
    #[doc = "Match Shadow"]
    pub MSR: [crate::RWRegister<u32>; 4usize],
}
#[doc = "Interrupt"]
pub mod IR {
    #[doc = "Interrupt Flag for Match Channel 0 Event"]
    pub mod MR0INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Flag for Match Channel 1 Event"]
    pub mod MR1INT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Flag for Match Channel 2 Event"]
    pub mod MR2INT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Flag for Match Channel 3 Event"]
    pub mod MR3INT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Flag for Capture Channel 0 Event"]
    pub mod CR0INT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Flag for Capture Channel 1 Event"]
    pub mod CR1INT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Flag for Capture Channel 2 Event"]
    pub mod CR2INT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Flag for Capture Channel 3 Event"]
    pub mod CR3INT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Control"]
pub mod TCR {
    #[doc = "Counter Enable"]
    pub mod CEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Counter Reset Enable"]
    pub mod CRST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Allow Global Count Enable"]
    pub mod AGCEN {
        pub const offset: u32 = 4;
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
    #[doc = "Allow Trigger Count Enable"]
    pub mod ATCEN {
        pub const offset: u32 = 5;
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
#[doc = "Timer Counter"]
pub mod TC {
    #[doc = "Timer Counter Value"]
    pub mod TCVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Prescale"]
pub mod PR {
    #[doc = "Prescale Reload Value"]
    pub mod PRVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Prescale Counter"]
pub mod PC {
    #[doc = "Prescale Counter Value"]
    pub mod PCVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Match Control"]
pub mod MCR {
    #[doc = "Interrupt on MR0"]
    pub mod MR0I {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not generate"]
            pub const MR0I_0: u32 = 0;
            #[doc = "Generates"]
            pub const MR0I_1: u32 = 1;
        }
    }
    #[doc = "Reset on MR0"]
    pub mod MR0R {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not reset"]
            pub const MR0R_0: u32 = 0;
            #[doc = "Resets"]
            pub const MR0R_1: u32 = 1;
        }
    }
    #[doc = "Stop on MR0"]
    pub mod MR0S {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not stop"]
            pub const MR0S_0: u32 = 0;
            #[doc = "Stops"]
            pub const MR0S_1: u32 = 1;
        }
    }
    #[doc = "Interrupt on MR1"]
    pub mod MR1I {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not generate"]
            pub const MR1I_0: u32 = 0;
            #[doc = "Generates"]
            pub const MR1I_1: u32 = 1;
        }
    }
    #[doc = "Reset on MR1"]
    pub mod MR1R {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not reset"]
            pub const MR1R_0: u32 = 0;
            #[doc = "Resets"]
            pub const MR1R_1: u32 = 1;
        }
    }
    #[doc = "Stop on MR1"]
    pub mod MR1S {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not stop"]
            pub const MRIS_0: u32 = 0;
            #[doc = "Stops"]
            pub const MRIS_1: u32 = 1;
        }
    }
    #[doc = "Interrupt on MR2"]
    pub mod MR2I {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not generate"]
            pub const MR2I_0: u32 = 0;
            #[doc = "Generates"]
            pub const MR2I_1: u32 = 1;
        }
    }
    #[doc = "Reset on MR2"]
    pub mod MR2R {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not reset"]
            pub const MR2R_0: u32 = 0;
            #[doc = "Resets"]
            pub const MR2R_1: u32 = 1;
        }
    }
    #[doc = "Stop on MR2"]
    pub mod MR2S {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not stop"]
            pub const MR2S_0: u32 = 0;
            #[doc = "Stops"]
            pub const MR2S_1: u32 = 1;
        }
    }
    #[doc = "Interrupt on MR3"]
    pub mod MR3I {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not generate"]
            pub const MR3I_0: u32 = 0;
            #[doc = "Generates"]
            pub const MR3I_1: u32 = 1;
        }
    }
    #[doc = "Reset on MR3"]
    pub mod MR3R {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not reset"]
            pub const MR3R_0: u32 = 0;
            #[doc = "Resets"]
            pub const MR3R_1: u32 = 1;
        }
    }
    #[doc = "Stop on MR3"]
    pub mod MR3S {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not stop"]
            pub const MR3S_0: u32 = 0;
            #[doc = "Stops"]
            pub const MR3S_1: u32 = 1;
        }
    }
    #[doc = "Reload MR"]
    pub mod MR0RL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not reload"]
            pub const MR0RL_0: u32 = 0;
            #[doc = "Reloads"]
            pub const MR0RL_1: u32 = 1;
        }
    }
    #[doc = "Reload MR"]
    pub mod MR1RL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not reload"]
            pub const MR1RL_0: u32 = 0;
            #[doc = "Reloads"]
            pub const MR1RL_1: u32 = 1;
        }
    }
    #[doc = "Reload MR"]
    pub mod MR2RL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not reload"]
            pub const MR2RL_0: u32 = 0;
            #[doc = "Reloads"]
            pub const MR2RL_1: u32 = 1;
        }
    }
    #[doc = "Reload MR"]
    pub mod MR3RL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not reload"]
            pub const MR3RL_0: u32 = 0;
            #[doc = "Reloads"]
            pub const MR3RL_1: u32 = 1;
        }
    }
}
#[doc = "Match"]
pub mod MR {
    #[doc = "Timer Counter Match Value"]
    pub mod MATCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Capture Control"]
pub mod CCR {
    #[doc = "Rising Edge of Capture Channel 0"]
    pub mod CAP0RE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not load"]
            pub const CAP0RE_0: u32 = 0;
            #[doc = "Loads"]
            pub const CAPORE_1: u32 = 1;
        }
    }
    #[doc = "Falling Edge of Capture Channel 0"]
    pub mod CAP0FE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not load"]
            pub const CAP0FE_0: u32 = 0;
            #[doc = "Loads"]
            pub const CAPOFE_1: u32 = 1;
        }
    }
    #[doc = "Generate Interrupt on Channel 0 Capture Event"]
    pub mod CAP0I {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not generate"]
            pub const CAP0I_0: u32 = 0;
            #[doc = "Generates"]
            pub const CAPOI_1: u32 = 1;
        }
    }
    #[doc = "Rising Edge of Capture Channel 1"]
    pub mod CAP1RE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not load"]
            pub const CAP1RE_0: u32 = 0;
            #[doc = "Loads"]
            pub const CAP1RE_1: u32 = 1;
        }
    }
    #[doc = "Falling Edge of Capture Channel 1"]
    pub mod CAP1FE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not load"]
            pub const CAP1FE_0: u32 = 0;
            #[doc = "Loads"]
            pub const CAP1FE_1: u32 = 1;
        }
    }
    #[doc = "Generate Interrupt on Channel 1 Capture Event"]
    pub mod CAP1I {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not generates"]
            pub const CAP1I_0: u32 = 0;
            #[doc = "Generates"]
            pub const CAP1I_1: u32 = 1;
        }
    }
    #[doc = "Rising Edge of Capture Channel 2"]
    pub mod CAP2RE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not load"]
            pub const CAP2RE_0: u32 = 0;
            #[doc = "Loads"]
            pub const CAP2RE_1: u32 = 1;
        }
    }
    #[doc = "Falling Edge of Capture Channel 2"]
    pub mod CAP2FE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not load"]
            pub const CAP2FE_0: u32 = 0;
            #[doc = "Loads"]
            pub const CAP2FE_1: u32 = 1;
        }
    }
    #[doc = "Generate Interrupt on Channel 2 Capture Event"]
    pub mod CAP2I {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not generate"]
            pub const CAP2I_0: u32 = 0;
            #[doc = "Generates"]
            pub const CAP2I_1: u32 = 1;
        }
    }
    #[doc = "Rising Edge of Capture Channel 3"]
    pub mod CAP3RE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not load"]
            pub const CAP3RE_0: u32 = 0;
            #[doc = "Loads"]
            pub const CAP3RE_1: u32 = 1;
        }
    }
    #[doc = "Falling Edge of Capture Channel 3"]
    pub mod CAP3FE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not load"]
            pub const CAP3FE_0: u32 = 0;
            #[doc = "Loads"]
            pub const CAP3FE_1: u32 = 1;
        }
    }
    #[doc = "Generate Interrupt on Channel 3 Capture Event"]
    pub mod CAP3I {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not generate"]
            pub const CAP3I_0: u32 = 0;
            #[doc = "Generates"]
            pub const CAP3I_1: u32 = 1;
        }
    }
}
#[doc = "Capture"]
pub mod CR {
    #[doc = "Timer Counter Capture Value"]
    pub mod CAP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External Match"]
pub mod EMR {
    #[doc = "External Match 0"]
    pub mod EM0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low"]
            pub const CLEAR: u32 = 0;
            #[doc = "High"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "External Match 1"]
    pub mod EM1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low"]
            pub const CLEAR: u32 = 0;
            #[doc = "High"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "External Match 2"]
    pub mod EM2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low"]
            pub const CLEAR: u32 = 0;
            #[doc = "High"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "External Match 3"]
    pub mod EM3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low"]
            pub const CLEAR: u32 = 0;
            #[doc = "High"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "External Match Control 0"]
    pub mod EMC0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does nothing"]
            pub const DO_NOTHING: u32 = 0;
            #[doc = "Goes low"]
            pub const CLEAR: u32 = 1;
            #[doc = "Goes high"]
            pub const SET: u32 = 2;
            #[doc = "Toggles"]
            pub const TOGGLE: u32 = 3;
        }
    }
    #[doc = "External Match Control 1"]
    pub mod EMC1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does nothing"]
            pub const DO_NOTHING: u32 = 0;
            #[doc = "Goes low"]
            pub const CLEAR: u32 = 1;
            #[doc = "Goes high"]
            pub const SET: u32 = 2;
            #[doc = "Toggles"]
            pub const TOGGLE: u32 = 3;
        }
    }
    #[doc = "External Match Control 2"]
    pub mod EMC2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does nothing"]
            pub const DO_NOTHING: u32 = 0;
            #[doc = "Goes low"]
            pub const CLEAR: u32 = 1;
            #[doc = "Goes high"]
            pub const SET: u32 = 2;
            #[doc = "Toggles"]
            pub const TOGGLE: u32 = 3;
        }
    }
    #[doc = "External Match Control 3"]
    pub mod EMC3 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does nothing"]
            pub const DO_NOTHING: u32 = 0;
            #[doc = "Goes low"]
            pub const CLEAR: u32 = 1;
            #[doc = "Goes high"]
            pub const SET: u32 = 2;
            #[doc = "Toggles"]
            pub const TOGGLE: u32 = 3;
        }
    }
}
#[doc = "Count Control"]
pub mod CTCR {
    #[doc = "Counter Timer Mode"]
    pub mod CTMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer mode"]
            pub const TIMER: u32 = 0;
            #[doc = "Counter mode rising edge"]
            pub const COUNTER_RISING_EDGE: u32 = 1;
            #[doc = "Counter mode falling edge"]
            pub const COUNTER_FALLING_EDGE: u32 = 2;
            #[doc = "Counter mode dual edge"]
            pub const COUNTER_DUAL_EDGE: u32 = 3;
        }
    }
    #[doc = "Count Input Select"]
    pub mod CINSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel 0, CAPn[0] for CTIMERn"]
            pub const CHANNEL_0: u32 = 0;
            #[doc = "Channel 1, CAPn[1] for CTIMERn"]
            pub const CHANNEL_1: u32 = 1;
            #[doc = "Channel 2, CAPn[2] for CTIMERn"]
            pub const CHANNEL_2: u32 = 2;
            #[doc = "Channel 3, CAPn[3] for CTIMERn"]
            pub const CHANNEL_3: u32 = 3;
        }
    }
    #[doc = "Capture Channel Enable"]
    pub mod ENCC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Edge Select"]
    pub mod SELCC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Capture channel 0 rising edge"]
            pub const CHANNEL_0_RISING: u32 = 0;
            #[doc = "Capture channel 0 falling edge"]
            pub const CHANNEL_0_FALLING: u32 = 1;
            #[doc = "Capture channel 1 rising edge"]
            pub const CHANNEL_1_RISING: u32 = 2;
            #[doc = "Capture channel 1 falling edge"]
            pub const CHANNEL_1_FALLING: u32 = 3;
            #[doc = "Capture channel 2 rising edge"]
            pub const CHANNEL_2_RISING: u32 = 4;
            #[doc = "Capture channel 2 falling edge"]
            pub const CHANNEL_2_FALLING: u32 = 5;
        }
    }
}
#[doc = "PWM Control"]
pub mod PWMC {
    #[doc = "PWM Mode Enable for Channel 0"]
    pub mod PWMEN0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const MATCH: u32 = 0;
            #[doc = "Enable"]
            pub const PWM: u32 = 1;
        }
    }
    #[doc = "PWM Mode Enable for Channel 1"]
    pub mod PWMEN1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const MATCH: u32 = 0;
            #[doc = "Enable"]
            pub const PWM: u32 = 1;
        }
    }
    #[doc = "PWM Mode Enable for Channel 2"]
    pub mod PWMEN2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const MATCH: u32 = 0;
            #[doc = "Enable"]
            pub const PWM: u32 = 1;
        }
    }
    #[doc = "PWM Mode Enable for Channel 3"]
    pub mod PWMEN3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const MATCH: u32 = 0;
            #[doc = "Enable"]
            pub const PWM: u32 = 1;
        }
    }
}
#[doc = "Match Shadow"]
pub mod MSR {
    #[doc = "Timer Counter Match Shadow Value"]
    pub mod MATCH_SHADOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
