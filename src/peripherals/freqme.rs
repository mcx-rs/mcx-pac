#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "FREQME"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control (in Read mode)"]
    pub CTRL_R: crate::RWRegister<u32>,
    #[doc = "Control (in Write mode)"]
    pub CTRL_W: crate::RWRegister<u32>,
    #[doc = "Control Status"]
    pub CTRLSTAT: crate::RWRegister<u32>,
    #[doc = "Minimum"]
    pub MIN: crate::RWRegister<u32>,
    #[doc = "Maximum"]
    pub MAX: crate::RWRegister<u32>,
}
#[doc = "Control (in Read mode)"]
pub mod CTRL_R {
    #[doc = "Indicates the measurement result-either the target clock counter value (for Frequency Measurement mode) or pulse width measurement (for Pulse Width Measurement mode)"]
    pub mod RESULT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Measurement In Progress"]
    pub mod MEASURE_IN_PROGRESS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Complete"]
            pub const CYCLE_DONE: u32 = 0;
            #[doc = "In progress"]
            pub const IN_PROGRESS: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control (in Write mode)"]
pub mod CTRL_W {
    #[doc = "Reference Clock Scaling Factor"]
    pub mod REF_SCALE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pulse Width Measurement Mode Select"]
    pub mod PULSE_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Frequency Measurement mode"]
            pub const FREQ_ME_MODE: u32 = 0;
            #[doc = "Pulse Width Measurement mode"]
            pub const PULSE_ME_MODE: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Pulse Polarity"]
    pub mod PULSE_POL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "High period"]
            pub const HIGH_PERIOD: u32 = 0;
            #[doc = "Low period"]
            pub const LOW_PERIOD: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Less Than Minimum Interrupt Enable"]
    pub mod LT_MIN_INT_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Greater Than Maximum Interrupt Enable"]
    pub mod GT_MAX_INT_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Result Ready Interrupt Enable"]
    pub mod RESULT_READY_INT_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Continuous Mode Enable"]
    pub mod CONTINUOUS_MODE_EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Measurement In Progress"]
    pub mod MEASURE_IN_PROGRESS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Terminates measurement"]
            pub const FORCE_TERMINATE: u32 = 0;
            #[doc = "Initiates measurement"]
            pub const INITIATE_A_FREQME_CYCLE: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Control Status"]
pub mod CTRLSTAT {
    #[doc = "Reference Scale"]
    pub mod REF_SCALE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pulse Mode"]
    pub mod PULSE_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Frequency Measurement mode"]
            pub const FREQ: u32 = 0;
            #[doc = "Pulse Width Measurement mode"]
            pub const PULSE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pulse Polarity"]
    pub mod PULSE_POL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "High period"]
            pub const HIGH: u32 = 0;
            #[doc = "Low period"]
            pub const LOW: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Less Than Minimum Interrupt Enable"]
    pub mod LT_MIN_INT_EN {
        pub const offset: u32 = 12;
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
    #[doc = "Greater Than Maximum Interrupt Enable"]
    pub mod GT_MAX_INT_EN {
        pub const offset: u32 = 13;
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
    #[doc = "Result Ready Interrupt Enable"]
    pub mod RESULT_READY_INT_EN {
        pub const offset: u32 = 14;
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
    #[doc = "Less Than Minimum Results Status"]
    pub mod LT_MIN_STAT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Greater than MIN[MIN_VALUE]"]
            pub const IN_RANGE: u32 = 0;
            #[doc = "Less than MIN[MIN_VALUE]"]
            pub const LT_MIN: u32 = 1;
        }
    }
    #[doc = "Greater Than Maximum Result Status"]
    pub mod GT_MAX_STAT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Less than MAX[MAX_VALUE]"]
            pub const IN_RANGE: u32 = 0;
            #[doc = "Greater than MAX[MAX_VALUE]"]
            pub const GT_MAX: u32 = 1;
        }
    }
    #[doc = "Result Ready Status"]
    pub mod RESULT_READY_STAT {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not complete"]
            pub const NOT_COMPLETE: u32 = 0;
            #[doc = "Complete"]
            pub const COMPLETE: u32 = 1;
        }
    }
    #[doc = "Continuous Mode Enable Status"]
    pub mod CONTINUOUS_MODE_EN {
        pub const offset: u32 = 30;
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
    #[doc = "Measurement in Progress Status"]
    pub mod MEASURE_IN_PROGRESS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not in progress"]
            pub const IDLE: u32 = 0;
            #[doc = "In progress"]
            pub const ONGOING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Minimum"]
pub mod MIN {
    #[doc = "Minimum Value"]
    pub mod MIN_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Maximum"]
pub mod MAX {
    #[doc = "Maximum Value"]
    pub mod MAX_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
