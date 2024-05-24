#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "SINC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameters"]
    pub PARAMETER: crate::RWRegister<u32>,
    #[doc = "Main Control"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "Normal Interrupt Enable"]
    pub NIE: crate::RWRegister<u32>,
    #[doc = "Error Interrupt Enable"]
    pub EIE: crate::RWRegister<u32>,
    #[doc = "FIFO And CAD Error Interrupt Enable"]
    pub FIFOIE: crate::RWRegister<u32>,
    #[doc = "Normal Interrupt Status"]
    pub NIS: crate::RWRegister<u32>,
    #[doc = "Error Interrupt Status"]
    pub EIS: crate::RWRegister<u32>,
    #[doc = "FIFO And CAD Error Interrupt Status"]
    pub FIFOIS: crate::RWRegister<u32>,
    #[doc = "Status"]
    pub SR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x10],
    #[doc = "Prefetch configuration array"]
    pub CHANNEL: [channel::RegisterBlock; 5usize],
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Code"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minor Version Number"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "x.0"]
            pub const VER_2_0: u32 = 0;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major Version Number"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "1.x"]
            pub const VER_1: u32 = 1;
            #[doc = "2.x"]
            pub const VER_2: u32 = 2;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parameters"]
pub mod PARAMETER {
    #[doc = "FIFO Depth"]
    pub mod FIFO_DEPTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Filter Channel Number"]
    pub mod FLT_NUM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PF Order Select"]
    pub mod PF_ORD_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "3"]
            pub const ORDER_3: u32 = 2;
            #[doc = "2"]
            pub const ORDER_2: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Main Control"]
pub mod MCR {
    #[doc = "Software Trigger For Channel 0"]
    pub mod STRIG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Trigger"]
            pub const TRIGGER: u32 = 1;
        }
    }
    #[doc = "Software Trigger For Channel 1"]
    pub mod STRIG1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Trigger"]
            pub const TRIGGER: u32 = 1;
        }
    }
    #[doc = "Software Trigger For Channel 2"]
    pub mod STRIG2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Trigger"]
            pub const TRIGGER: u32 = 1;
        }
    }
    #[doc = "Software Trigger For Channel 3"]
    pub mod STRIG3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Trigger"]
            pub const TRIGGER: u32 = 1;
        }
    }
    #[doc = "Software Trigger For Channel 4"]
    pub mod STRIG4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Trigger"]
            pub const TRIGGER: u32 = 1;
        }
    }
    #[doc = "Doze Or Stop Enable"]
    pub mod DOZEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables"]
            pub const ENABLED: u32 = 0;
            #[doc = "Disables"]
            pub const DISABLED: u32 = 1;
        }
    }
    #[doc = "Software Reset"]
    pub mod RST {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not reset"]
            pub const RESET_NO: u32 = 0;
            #[doc = "Reset"]
            pub const RESET_YES: u32 = 1;
        }
    }
    #[doc = "Master Enable"]
    pub mod MEN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Modulator Clock Divider"]
    pub mod MCLKDIV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Prohibited"]
            pub const PROHIBITED: u32 = 0;
        }
    }
    #[doc = "Prescale Before Clock Divider"]
    pub mod PRESCALE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No prescale"]
            pub const PRESCALE_NO: u32 = 0;
            #[doc = "2"]
            pub const PRESCALE_2: u32 = 1;
            #[doc = "4"]
            pub const PRESCALE_4: u32 = 2;
            #[doc = "8"]
            pub const PRESCALE_8: u32 = 3;
        }
    }
    #[doc = "Disable Modulator Clock 0 Output"]
    pub mod MCLK0DIS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled when MEN = 1"]
            pub const ENABLED: u32 = 0;
            #[doc = "Disabled regardless of MEN value"]
            pub const DISABLED: u32 = 1;
        }
    }
    #[doc = "Disable Modulator Clock 1 Output"]
    pub mod MCLK1DIS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled when MEN = 1"]
            pub const ENABLED: u32 = 0;
            #[doc = "Disabled regardless of MEN value"]
            pub const DISABLED: u32 = 1;
        }
    }
    #[doc = "Disable Modulator Clock 2 Output"]
    pub mod MCLK2DIS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled when MEN = 1"]
            pub const ENABLED: u32 = 0;
            #[doc = "Disabled regardless of MEN value"]
            pub const DISABLED: u32 = 1;
        }
    }
}
#[doc = "Normal Interrupt Enable"]
pub mod NIE {
    #[doc = "Conversion Complete Interrupt Enable"]
    pub mod COCIE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Conversion Complete Interrupt Enable"]
    pub mod COCIE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Conversion Complete Interrupt Enable"]
    pub mod COCIE2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Conversion Complete Interrupt Enable"]
    pub mod COCIE3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Conversion Complete Interrupt Enable"]
    pub mod COCIE4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    pub mod CHFIE0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    pub mod CHFIE1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    pub mod CHFIE2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    pub mod CHFIE3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    pub mod CHFIE4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    pub mod ZCDIE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    pub mod ZCDIE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    pub mod ZCDIE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    pub mod ZCDIE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    pub mod ZCDIE4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
}
#[doc = "Error Interrupt Enable"]
pub mod EIE {
    #[doc = "Short Circuit Detected Interrupt Enable"]
    pub mod SCDIE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Short Circuit Detected Interrupt Enable"]
    pub mod SCDIE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Short Circuit Detected Interrupt Enable"]
    pub mod SCDIE2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Short Circuit Detected Interrupt Enable"]
    pub mod SCDIE3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Short Circuit Detected Interrupt Enable"]
    pub mod SCDIE4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Window Limit Interrupt Enable"]
    pub mod WLMTIE0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Window Limit Interrupt Enable"]
    pub mod WLMTIE1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Window Limit Interrupt Enable"]
    pub mod WLMTIE2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Window Limit Interrupt Enable"]
    pub mod WLMTIE3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Window Limit Interrupt Enable"]
    pub mod WLMTIE4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Low Limit Interrupt Enable"]
    pub mod LLMTIE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Low Limit Interrupt Enable"]
    pub mod LLMTIE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Low Limit Interrupt Enable"]
    pub mod LLMTIE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Low Limit Interrupt Enable"]
    pub mod LLMTIE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Low Limit Interrupt Enable"]
    pub mod LLMTIE4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "High Limit Interrupt Enable"]
    pub mod HLMTIE0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "High Limit Interrupt Enable"]
    pub mod HLMTIE1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "High Limit Interrupt Enable"]
    pub mod HLMTIE2 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "High Limit Interrupt Enable"]
    pub mod HLMTIE3 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "High Limit Interrupt Enable"]
    pub mod HLMTIE4 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
}
#[doc = "FIFO And CAD Error Interrupt Enable"]
pub mod FIFOIE {
    #[doc = "FIFO Underflow Interrupt Enable"]
    pub mod FUNFIE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    pub mod FUNFIE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    pub mod FUNFIE2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    pub mod FUNFIE3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    pub mod FUNFIE4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    pub mod FOVFIE0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    pub mod FOVFIE1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    pub mod FOVFIE2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    pub mod FOVFIE3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    pub mod FOVFIE4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Clock Absence Interrupt Enable"]
    pub mod CADIE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Clock Absence Interrupt Enable"]
    pub mod CADIE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Clock Absence Interrupt Enable"]
    pub mod CADIE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Clock Absence Interrupt Enable"]
    pub mod CADIE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Clock Absence Interrupt Enable"]
    pub mod CADIE4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Saturation Interrupt Enable"]
    pub mod SATIE0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Saturation Interrupt Enable"]
    pub mod SATIE1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Saturation Interrupt Enable"]
    pub mod SATIE2 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Saturation Interrupt Enable"]
    pub mod SATIE3 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
    #[doc = "Saturation Interrupt Enable"]
    pub mod SATIE4 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLES: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLES: u32 = 1;
        }
    }
}
#[doc = "Normal Interrupt Status"]
pub mod NIS {
    #[doc = "Conversion Complete Flag"]
    pub mod COC0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not finished; data not available"]
            pub const COC_NO: u32 = 0;
            #[doc = "Finished; data available"]
            pub const COC_YES: u32 = 1;
        }
    }
    #[doc = "Conversion Complete Flag"]
    pub mod COC1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not finished; data not available"]
            pub const COC_NO: u32 = 0;
            #[doc = "Finished; data available"]
            pub const COC_YES: u32 = 1;
        }
    }
    #[doc = "Conversion Complete Flag"]
    pub mod COC2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not finished; data not available"]
            pub const COC_NO: u32 = 0;
            #[doc = "Finished; data available"]
            pub const COC_YES: u32 = 1;
        }
    }
    #[doc = "Conversion Complete Flag"]
    pub mod COC3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not finished; data not available"]
            pub const COC_NO: u32 = 0;
            #[doc = "Finished; data available"]
            pub const COC_YES: u32 = 1;
        }
    }
    #[doc = "Conversion Complete Flag"]
    pub mod COC4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not finished; data not available"]
            pub const COC_NO: u32 = 0;
            #[doc = "Finished; data available"]
            pub const COC_YES: u32 = 1;
        }
    }
    #[doc = "Data Output Ready Flag"]
    pub mod CHF0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overflow; data not available"]
            pub const OVFLW_NO: u32 = 0;
            #[doc = "Overflow; data available"]
            pub const OVFLW_YES: u32 = 1;
        }
    }
    #[doc = "Data Output Ready Flag"]
    pub mod CHF1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overflow; data not available"]
            pub const OVFLW_NO: u32 = 0;
            #[doc = "Overflow; data available"]
            pub const OVFLW_YES: u32 = 1;
        }
    }
    #[doc = "Data Output Ready Flag"]
    pub mod CHF2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overflow; data not available"]
            pub const OVFLW_NO: u32 = 0;
            #[doc = "Overflow; data available"]
            pub const OVFLW_YES: u32 = 1;
        }
    }
    #[doc = "Data Output Ready Flag"]
    pub mod CHF3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overflow; data not available"]
            pub const OVFLW_NO: u32 = 0;
            #[doc = "Overflow; data available"]
            pub const OVFLW_YES: u32 = 1;
        }
    }
    #[doc = "Data Output Ready Flag"]
    pub mod CHF4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overflow; data not available"]
            pub const OVFLW_NO: u32 = 0;
            #[doc = "Overflow; data available"]
            pub const OVFLW_YES: u32 = 1;
        }
    }
    #[doc = "Zero Cross Detected Flag"]
    pub mod ZCD0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const ZC_NO: u32 = 0;
            #[doc = "Detected"]
            pub const ZC_YES: u32 = 1;
        }
    }
    #[doc = "Zero Cross Detected Flag"]
    pub mod ZCD1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const ZC_NO: u32 = 0;
            #[doc = "Detected"]
            pub const ZC_YES: u32 = 1;
        }
    }
    #[doc = "Zero Cross Detected Flag"]
    pub mod ZCD2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const ZC_NO: u32 = 0;
            #[doc = "Detected"]
            pub const ZC_YES: u32 = 1;
        }
    }
    #[doc = "Zero Cross Detected Flag"]
    pub mod ZCD3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const ZC_NO: u32 = 0;
            #[doc = "Detected"]
            pub const ZC_YES: u32 = 1;
        }
    }
    #[doc = "Zero Cross Detected Flag"]
    pub mod ZCD4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const ZC_NO: u32 = 0;
            #[doc = "Detected"]
            pub const ZC_YES: u32 = 1;
        }
    }
}
#[doc = "Error Interrupt Status"]
pub mod EIS {
    #[doc = "Short Circuit Detected Flag"]
    pub mod SCD0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const SC_NO: u32 = 0;
            #[doc = "Detected"]
            pub const SC_YES: u32 = 1;
        }
    }
    #[doc = "Short Circuit Detected Flag"]
    pub mod SCD1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const SC_NO: u32 = 0;
            #[doc = "Detected"]
            pub const SC_YES: u32 = 1;
        }
    }
    #[doc = "Short Circuit Detected Flag"]
    pub mod SCD2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const SC_NO: u32 = 0;
            #[doc = "Detected"]
            pub const SC_YES: u32 = 1;
        }
    }
    #[doc = "Short Circuit Detected Flag"]
    pub mod SCD3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const SC_NO: u32 = 0;
            #[doc = "Detected"]
            pub const SC_YES: u32 = 1;
        }
    }
    #[doc = "Short Circuit Detected Flag"]
    pub mod SCD4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const SC_NO: u32 = 0;
            #[doc = "Detected"]
            pub const SC_YES: u32 = 1;
        }
    }
    #[doc = "Window Limit Flag"]
    pub mod WLMT0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const WLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const WLMT_YES: u32 = 1;
        }
    }
    #[doc = "Window Limit Flag"]
    pub mod WLMT1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const WLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const WLMT_YES: u32 = 1;
        }
    }
    #[doc = "Window Limit Flag"]
    pub mod WLMT2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const WLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const WLMT_YES: u32 = 1;
        }
    }
    #[doc = "Window Limit Flag"]
    pub mod WLMT3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const WLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const WLMT_YES: u32 = 1;
        }
    }
    #[doc = "Window Limit Flag"]
    pub mod WLMT4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const WLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const WLMT_YES: u32 = 1;
        }
    }
    #[doc = "Low Limit Flag"]
    pub mod LLMT0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const LLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const LLMT_YES: u32 = 1;
        }
    }
    #[doc = "Low Limit Flag"]
    pub mod LLMT1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const LLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const LLMT_YES: u32 = 1;
        }
    }
    #[doc = "Low Limit Flag"]
    pub mod LLMT2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const LLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const LLMT_YES: u32 = 1;
        }
    }
    #[doc = "Low Limit Flag"]
    pub mod LLMT3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const LLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const LLMT_YES: u32 = 1;
        }
    }
    #[doc = "Low Limit Flag"]
    pub mod LLMT4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const LLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const LLMT_YES: u32 = 1;
        }
    }
    #[doc = "High Limit Flag"]
    pub mod HLMT0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const HLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const HLMT_YES: u32 = 1;
        }
    }
    #[doc = "High Limit Flag"]
    pub mod HLMT1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const HLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const HLMT_YES: u32 = 1;
        }
    }
    #[doc = "High Limit Flag"]
    pub mod HLMT2 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const HLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const HLMT_YES: u32 = 1;
        }
    }
    #[doc = "High Limit Flag"]
    pub mod HLMT3 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const HLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const HLMT_YES: u32 = 1;
        }
    }
    #[doc = "High Limit Flag"]
    pub mod HLMT4 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const HLMT_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const HLMT_YES: u32 = 1;
        }
    }
}
#[doc = "FIFO And CAD Error Interrupt Status"]
pub mod FIFOIS {
    #[doc = "FIFO Underflow Flag"]
    pub mod FUNF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const FUNF_NO: u32 = 0;
            #[doc = "Occurred"]
            pub const FUNF_YES: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Flag"]
    pub mod FUNF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const FUNF_NO: u32 = 0;
            #[doc = "Occurred"]
            pub const FUNF_YES: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Flag"]
    pub mod FUNF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const FUNF_NO: u32 = 0;
            #[doc = "Occurred"]
            pub const FUNF_YES: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Flag"]
    pub mod FUNF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const FUNF_NO: u32 = 0;
            #[doc = "Occurred"]
            pub const FUNF_YES: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Flag"]
    pub mod FUNF4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const FUNF_NO: u32 = 0;
            #[doc = "Occurred"]
            pub const FUNF_YES: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Flag"]
    pub mod FOVF0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const FOVF_NO: u32 = 0;
            #[doc = "Occurred"]
            pub const FOVF_YES: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Flag"]
    pub mod FOVF1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const FOVF_NO: u32 = 0;
            #[doc = "Occurred"]
            pub const FOVF_YES: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Flag"]
    pub mod FOVF2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const FOVF_NO: u32 = 0;
            #[doc = "Occurred"]
            pub const FOVF_YES: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Flag"]
    pub mod FOVF3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const FOVF_NO: u32 = 0;
            #[doc = "Occurred"]
            pub const FOVF_YES: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Flag"]
    pub mod FOVF4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const FOVF_NO: u32 = 0;
            #[doc = "Occurred"]
            pub const FOVF_YES: u32 = 1;
        }
    }
    #[doc = "Clock Absence Flag"]
    pub mod CAD0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock present"]
            pub const CAD_NO: u32 = 0;
            #[doc = "Clock absent"]
            pub const CAD_YES: u32 = 1;
        }
    }
    #[doc = "Clock Absence Flag"]
    pub mod CAD1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock present"]
            pub const CAD_NO: u32 = 0;
            #[doc = "Clock absent"]
            pub const CAD_YES: u32 = 1;
        }
    }
    #[doc = "Clock Absence Flag"]
    pub mod CAD2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock present"]
            pub const CAD_NO: u32 = 0;
            #[doc = "Clock absent"]
            pub const CAD_YES: u32 = 1;
        }
    }
    #[doc = "Clock Absence Flag"]
    pub mod CAD3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock present"]
            pub const CAD_NO: u32 = 0;
            #[doc = "Clock absent"]
            pub const CAD_YES: u32 = 1;
        }
    }
    #[doc = "Clock Absence Flag"]
    pub mod CAD4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock present"]
            pub const CAD_NO: u32 = 0;
            #[doc = "Clock absent"]
            pub const CAD_YES: u32 = 1;
        }
    }
    #[doc = "Saturation Flag"]
    pub mod SAT0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not saturated"]
            pub const SAT_NO: u32 = 0;
            #[doc = "Saturated"]
            pub const SAT_YES: u32 = 1;
        }
    }
    #[doc = "Saturation Flag"]
    pub mod SAT1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not saturated"]
            pub const SAT_NO: u32 = 0;
            #[doc = "Saturated"]
            pub const SAT_YES: u32 = 1;
        }
    }
    #[doc = "Saturation Flag"]
    pub mod SAT2 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not saturated"]
            pub const SAT_NO: u32 = 0;
            #[doc = "Saturated"]
            pub const SAT_YES: u32 = 1;
        }
    }
    #[doc = "Saturation Flag"]
    pub mod SAT3 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not saturated"]
            pub const SAT_NO: u32 = 0;
            #[doc = "Saturated"]
            pub const SAT_YES: u32 = 1;
        }
    }
    #[doc = "Saturation Flag"]
    pub mod SAT4 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not saturated"]
            pub const SAT_NO: u32 = 0;
            #[doc = "Saturated"]
            pub const SAT_YES: u32 = 1;
        }
    }
}
#[doc = "Status"]
pub mod SR {
    #[doc = "Conversion In Progress"]
    pub mod CIP0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not in progress"]
            pub const CONV_NO: u32 = 0;
            #[doc = "In progress"]
            pub const CONV_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Conversion In Progress"]
    pub mod CIP1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not in progress"]
            pub const CONV_NO: u32 = 0;
            #[doc = "In progress"]
            pub const CONV_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Conversion In Progress"]
    pub mod CIP2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not in progress"]
            pub const CONV_NO: u32 = 0;
            #[doc = "In progress"]
            pub const CONV_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Conversion In Progress"]
    pub mod CIP3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not in progress"]
            pub const CONV_NO: u32 = 0;
            #[doc = "In progress"]
            pub const CONV_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Conversion In Progress"]
    pub mod CIP4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not in progress"]
            pub const CONV_NO: u32 = 0;
            #[doc = "In progress"]
            pub const CONV_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Ready For Conversion"]
    pub mod CHRDY0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not ready"]
            pub const READY_NO: u32 = 0;
            #[doc = "Ready"]
            pub const READY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Ready For Conversion"]
    pub mod CHRDY1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not ready"]
            pub const READY_NO: u32 = 0;
            #[doc = "Ready"]
            pub const READY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Ready For Conversion"]
    pub mod CHRDY2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not ready"]
            pub const READY_NO: u32 = 0;
            #[doc = "Ready"]
            pub const READY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Ready For Conversion"]
    pub mod CHRDY3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not ready"]
            pub const READY_NO: u32 = 0;
            #[doc = "Ready"]
            pub const READY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Ready For Conversion"]
    pub mod CHRDY4 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not ready"]
            pub const READY_NO: u32 = 0;
            #[doc = "Ready"]
            pub const READY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Empty"]
    pub mod FIFOEMPTY0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not empty"]
            pub const EMPTY_NO: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Empty"]
    pub mod FIFOEMPTY1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not empty"]
            pub const EMPTY_NO: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Empty"]
    pub mod FIFOEMPTY2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not empty"]
            pub const EMPTY_NO: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Empty"]
    pub mod FIFOEMPTY3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not empty"]
            pub const EMPTY_NO: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Empty"]
    pub mod FIFOEMPTY4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not empty"]
            pub const EMPTY_NO: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Modulator Clock 0 Ready"]
    pub mod MCLKRDY0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not ready"]
            pub const READY_NO: u32 = 0;
            #[doc = "Ready"]
            pub const READY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Modulator Clock 1 Ready"]
    pub mod MCLKRDY1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not ready"]
            pub const READY_NO: u32 = 0;
            #[doc = "Ready"]
            pub const READY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Modulator Clock 2 Ready"]
    pub mod MCLKRDY2 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not ready"]
            pub const READY_NO: u32 = 0;
            #[doc = "Ready"]
            pub const READY_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
pub mod channel {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "Prefetch configuration array"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Channel n Control"]
        pub CCR: crate::RWRegister<u32>,
        #[doc = "Channel n Data Rate"]
        pub CDR: crate::RWRegister<u32>,
        #[doc = "Channel n Configuration"]
        pub CCFR: crate::RWRegister<u32>,
        #[doc = "Channel n Protection"]
        pub CPROT: crate::RWRegister<u32>,
        #[doc = "Channel n Bias"]
        pub CBIAS: crate::RWRegister<u32>,
        #[doc = "Channel n Low Limit"]
        pub CLOLMT: crate::RWRegister<u32>,
        #[doc = "Channel n High Limit"]
        pub CHILMT: crate::RWRegister<u32>,
        #[doc = "Channel n Result Data"]
        pub CRDATA: crate::RWRegister<u32>,
        #[doc = "Channel n Multipurpose Data"]
        pub CMPDATA: crate::RWRegister<u32>,
        #[doc = "Channel n Advanced Configuration"]
        pub CACFR: crate::RWRegister<u32>,
        #[doc = "Channel n Status"]
        pub CSR: crate::RWRegister<u32>,
        #[doc = "Channel n Debug"]
        pub CDBGR: crate::RWRegister<u32>,
    }
    #[doc = "Channel n Control"]
    pub mod CCR {
        #[doc = "Channel Enable"]
        pub mod CHEN {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
        #[doc = "PF Enable"]
        pub mod PFEN {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
        #[doc = "DMA Enable"]
        pub mod DMAEN {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
        #[doc = "Short Circuit Detect Enable"]
        pub mod SCDEN {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
        #[doc = "Clock Absence Detect Enable"]
        pub mod CADEN {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
        #[doc = "Zero Cross Detect Enable"]
        pub mod ZCDEN {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
        #[doc = "Limit Enable"]
        pub mod LMTEN {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
        #[doc = "FIFO Enable"]
        pub mod FIFOEN {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
        #[doc = "Debug Output Selection"]
        pub mod DBGSEL {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Final data from the PF (24 bits)"]
                pub const RSLT: u32 = 0;
                #[doc = "Offset data (24 bits)"]
                pub const PFBIS: u32 = 1;
                #[doc = "Shifted data from the PF (24 bits)"]
                pub const PFSFT: u32 = 2;
                #[doc = "DC remover (HPF) data (32 bits)"]
                pub const HPF: u32 = 3;
                #[doc = "Raw data from the PF\'s CIC filter"]
                pub const PFCIC: u32 = 4;
                #[doc = "Historical data from SCD"]
                pub const SCD: u32 = 6;
                #[doc = "Data from the Manchester decoder"]
                pub const MM: u32 = 7;
                #[doc = "Data from CAD"]
                pub const CAD: u32 = 8;
                #[doc = "Number of available entries in the FIFO"]
                pub const FIFO: u32 = 9;
                #[doc = "Status of the parallel or serial data converter"]
                pub const PS: u32 = 10;
            }
        }
    }
    #[doc = "Channel n Data Rate"]
    pub mod CDR {
        #[doc = "PF OSR"]
        pub mod PFOSR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "PF Order"]
        pub mod PFORD {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "FastSinc"]
                pub const FASTSINC: u32 = 0;
                #[doc = "First order"]
                pub const ORDER_1: u32 = 1;
                #[doc = "Second order"]
                pub const ORDER_2: u32 = 2;
                #[doc = "Third order"]
                pub const ORDER_3: u32 = 3;
            }
        }
        #[doc = "PF Conversion Mode"]
        pub mod PFCM {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Single"]
                pub const SINGLE: u32 = 0;
                #[doc = "Continuous"]
                pub const CONTINUOUS: u32 = 1;
                #[doc = "Always"]
                pub const ALWAYS: u32 = 2;
                #[doc = "Fixed number"]
                pub const FIX: u32 = 3;
            }
        }
    }
    #[doc = "Channel n Configuration"]
    pub mod CCFR {
        #[doc = "PF Shift"]
        pub mod PFSFT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Result Data Format"]
        pub mod RDFMT {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Left justified, signed"]
                pub const SIGNED: u32 = 0;
                #[doc = "Left justified, unsigned"]
                pub const UNSIGNED: u32 = 1;
            }
        }
        #[doc = "FIFO Watermark"]
        pub mod FIFOWMK {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Input Bit Format"]
        pub mod IBFMT {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "External bitstream from the MBIT[n] signal"]
                pub const E1B: u32 = 0;
                #[doc = "External Manchester code; ICESEL selects the rise or fall decoder"]
                pub const EMB: u32 = 1;
                #[doc = "Internal 16-bit parallel data from MPDATA"]
                pub const IPB: u32 = 2;
                #[doc = "Internal 32-bit serial data from MPDATA"]
                pub const ISB: u32 = 3;
            }
        }
        #[doc = "Input Clock Select"]
        pub mod ICSEL {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "MCLK_OUT0 with internal routeback"]
                pub const MCLK_OUT0: u32 = 0;
                #[doc = "MCLK_OUT1 with internal routeback"]
                pub const MCLK_OUT1: u32 = 1;
                #[doc = "MCLK_OUT2 with internal routeback"]
                pub const MCLK_OUT2: u32 = 2;
                #[doc = "External modulator clock dedicated to this channel"]
                pub const EXT: u32 = 3;
                #[doc = "Grouped clock shared with an adjacent channel; the adjacent channel\'s ICSEL field determines the input clock"]
                pub const GRP: u32 = 7;
            }
        }
        #[doc = "Input Clock Edge Select"]
        pub mod ICESEL {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Positive edge"]
                pub const POS: u32 = 1;
                #[doc = "Negative edge"]
                pub const NEG: u32 = 2;
                #[doc = "Both edges"]
                pub const BOTH: u32 = 3;
                #[doc = "Every other odd positive edge"]
                pub const OPOS: u32 = 4;
                #[doc = "Every other even positive edge"]
                pub const EPOS: u32 = 5;
                #[doc = "Every other odd negative edge"]
                pub const ONEG: u32 = 6;
                #[doc = "Every other even negative edge"]
                pub const ENEG: u32 = 7;
            }
        }
        #[doc = "Input Trigger Select"]
        pub mod ITSEL {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Software"]
                pub const SW: u32 = 0;
                #[doc = "Hardware trigger dedicated to the channel"]
                pub const HW: u32 = 1;
                #[doc = "Grouped trigger shared with an adjacent channel; the adjacent channel\'s ITSEL field determines the trigger"]
                pub const GP: u32 = 3;
            }
        }
        #[doc = "Input Bit Select"]
        pub mod IBSEL {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "External bitstream from the MBIT[n] signal"]
                pub const EPB: u32 = 0;
                #[doc = "Alternate internal bitstream from the INP[n] signal"]
                pub const ESB: u32 = 1;
                #[doc = "Grouped bitstream shared with an adjacent channel; the adjacent channel\'s IBSEL field determines the input"]
                pub const GRP: u32 = 3;
            }
        }
        #[doc = "Input Trigger Level Type"]
        pub mod ITLVL {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Edge"]
                pub const EDGE: u32 = 0;
                #[doc = "Level"]
                pub const LEVEL: u32 = 1;
            }
        }
        #[doc = "Zero Cross Option"]
        pub mod ZCOP {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Both rise and fall"]
                pub const BOTH: u32 = 0;
                #[doc = "Fall"]
                pub const FALL: u32 = 1;
                #[doc = "Rise"]
                pub const RISE: u32 = 2;
            }
        }
    }
    #[doc = "Channel n Protection"]
    pub mod CPROT {
        #[doc = "SCD Limit Threshold"]
        pub mod SCDLMT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables SCD"]
                pub const DISABLES_0: u32 = 0;
                #[doc = "Disables SCD"]
                pub const DISABLES_1: u32 = 1;
            }
        }
        #[doc = "SCD Conversion Mode"]
        pub mod SCDCM {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Constantly when CnCR[CHEN] = MCR[MEN] = 1"]
                pub const ALWAYS: u32 = 0;
                #[doc = "Only when the PF is performing a conversion"]
                pub const DURING_CONV: u32 = 1;
            }
        }
        #[doc = "SCD Option"]
        pub mod SCDOP {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Both 0 and 1"]
                pub const BOTH: u32 = 0;
                #[doc = "Only 1"]
                pub const ONE: u32 = 1;
                #[doc = "Only 0"]
                pub const ZERO: u32 = 2;
            }
        }
        #[doc = "Limit Detection Option"]
        pub mod LMTOP {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Both high and low limits"]
                pub const BOTH: u32 = 0;
                #[doc = "High limit"]
                pub const HIGH: u32 = 1;
                #[doc = "Low limit"]
                pub const LOW: u32 = 2;
                #[doc = "Windowed value"]
                pub const WINDOW: u32 = 3;
            }
        }
        #[doc = "CAD Limit Threshold"]
        pub mod CADLMT {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables CAD"]
                pub const DISABLES: u32 = 0;
            }
        }
        #[doc = "CAD Break Signal"]
        pub mod CADBK {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
        #[doc = "SCD Break Signal"]
        pub mod SCDBK {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
        #[doc = "Low Limit Break Signal"]
        pub mod LLMTBK {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
        #[doc = "Window Limit Break Signal"]
        pub mod WLMTBK {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
        #[doc = "High Limit Break Signal"]
        pub mod HLMTBK {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disables"]
                pub const DISABLES: u32 = 0;
                #[doc = "Enables"]
                pub const ENABLES: u32 = 1;
            }
        }
    }
    #[doc = "Channel n Bias"]
    pub mod CBIAS {
        #[doc = "Bias Value"]
        pub mod BIAS {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel n Low Limit"]
    pub mod CLOLMT {
        #[doc = "Low Limit Threshold"]
        pub mod LOLMT {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel n High Limit"]
    pub mod CHILMT {
        #[doc = "High Limit Threshold"]
        pub mod HILMT {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel n Result Data"]
    pub mod CRDATA {
        #[doc = "Result Data"]
        pub mod RDATA {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel n Multipurpose Data"]
    pub mod CMPDATA {
        #[doc = "Multipurpose Data"]
        pub mod MPDATA {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel n Advanced Configuration"]
    pub mod CACFR {
        #[doc = "Alternate DMA Source Selection"]
        pub mod ADMASEL {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Alternate DMA disabled"]
                pub const DISABLED: u32 = 0;
                #[doc = "PF conversion complete"]
                pub const PF_CONV_COMPLETE: u32 = 1;
                #[doc = "PF data output ready"]
                pub const PF_DATA_READY: u32 = 2;
                #[doc = "Zero crossing detected"]
                pub const ZCD: u32 = 3;
                #[doc = "Short circuit detected"]
                pub const SCD: u32 = 4;
                #[doc = "Window limit detected"]
                pub const WINDOW_LMT: u32 = 5;
                #[doc = "Low limit detected"]
                pub const LOW_LMT: u32 = 6;
                #[doc = "High limit"]
                pub const HIGH_LMT: u32 = 7;
                #[doc = "FIFO underflow"]
                pub const FIFO_UF: u32 = 8;
                #[doc = "FIFO overflow"]
                pub const FIFO_OF: u32 = 9;
                #[doc = "Clock absence"]
                pub const CLK_ABS: u32 = 10;
                #[doc = "Saturation"]
                pub const SAT: u32 = 11;
            }
        }
        #[doc = "HPF DC Remover Alpha Coefficient"]
        pub mod HPFA {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Input Modulator Bitstream Delay"]
        pub mod IBDLY {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled"]
                pub const DISABLED: u32 = 0;
            }
        }
    }
    #[doc = "Channel n Status"]
    pub mod CSR {
        #[doc = "FIFO Available Data"]
        pub mod FIFOAVIL {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Parallel or Serial Data Ready"]
        pub mod PSRDY {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Not ready"]
                pub const READY_NO: u32 = 0;
                #[doc = "Ready"]
                pub const READY_YES: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Primary CIC Saturation Flag"]
        pub mod PFSAT {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Did not occur"]
                pub const SAT_NO: u32 = 0;
                #[doc = "Occurred"]
                pub const SAT_YES: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "HPF Saturation Flag"]
        pub mod HPFSAT {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Did not occur"]
                pub const SAT_NO: u32 = 0;
                #[doc = "Occurred"]
                pub const SAT_YES: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Shift Saturation Flag"]
        pub mod SFTSAT {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Did not occur"]
                pub const SAT_NO: u32 = 0;
                #[doc = "Occurred"]
                pub const SAT_YES: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Bias Saturation Flag"]
        pub mod BIASSAT {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Did not occur"]
                pub const SAT_NO: u32 = 0;
                #[doc = "Occurred"]
                pub const SAT_YES: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Result Data Direct Read Status"]
        pub mod RDRS {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Valid"]
                pub const VALID: u32 = 0;
                #[doc = "Invalid"]
                pub const INVALID: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Start Read Debug Data Sync"]
        pub mod SRDS {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Data valid"]
                pub const DATA_VALID: u32 = 0;
                #[doc = "Procedure in progress"]
                pub const IN_PROGRESS: u32 = 1;
            }
        }
        #[doc = "Debug Data Read Status"]
        pub mod DBGRS {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {
                #[doc = "Valid"]
                pub const VALID: u32 = 0;
                #[doc = "Invalid"]
                pub const INVALID_1: u32 = 1;
                #[doc = "Invalid"]
                pub const INVALID_2: u32 = 2;
                #[doc = "Invalid"]
                pub const INVALID_3: u32 = 3;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Number Of Conversions"]
        pub mod CNUM {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Overflow In Number Of Conversions"]
        pub mod CNUM_OV {
            pub const offset: u32 = 23;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "No overflow"]
                pub const OFLW_NO: u32 = 0;
                #[doc = "Overflow"]
                pub const OFLW_YES: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel n Debug"]
    pub mod CDBGR {
        #[doc = "Debug Data"]
        pub mod DBGDATA {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
