#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "12-bit DAC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version Identifier"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RWRegister<u32>,
    #[doc = "Data"]
    pub DATA: crate::RWRegister<u32>,
    #[doc = "Global Control"]
    pub GCR: crate::RWRegister<u32>,
    #[doc = "DAC FIFO Control"]
    pub FCR: crate::RWRegister<u32>,
    #[doc = "DAC FIFO Pointer"]
    pub FPR: crate::RWRegister<u32>,
    #[doc = "FIFO Status"]
    pub FSR: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub IER: crate::RWRegister<u32>,
    #[doc = "DMA Enable"]
    pub DER: crate::RWRegister<u32>,
    #[doc = "Reset Control"]
    pub RCR: crate::RWRegister<u32>,
    #[doc = "Trigger Control"]
    pub TCR: crate::RWRegister<u32>,
    #[doc = "Periodic Trigger Control"]
    pub PCR: crate::RWRegister<u32>,
}
#[doc = "Version Identifier"]
pub mod VERID {
    #[doc = "Feature Identification Number"]
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
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major Version Number"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parameter"]
pub mod PARAM {
    #[doc = "FIFO Size"]
    pub mod FIFOSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "FIFO depth is 4"]
            pub const VAL_1: u32 = 1;
            #[doc = "FIFO depth is 8"]
            pub const VAL_2: u32 = 2;
            #[doc = "FIFO depth is 16"]
            pub const VAL_3: u32 = 3;
            #[doc = "FIFO depth is 32"]
            pub const VAL_4: u32 = 4;
            #[doc = "FIFO depth is 64"]
            pub const VAL_5: u32 = 5;
            #[doc = "FIFO depth is 128"]
            pub const VAL_6: u32 = 6;
            #[doc = "FIFO depth is 256"]
            pub const VAL_7: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data"]
pub mod DATA {
    #[doc = "FIFO Entry or Buffer Entry"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Global Control"]
pub mod GCR {
    #[doc = "DAC Enable"]
    pub mod DACEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "DAC Reference Select"]
    pub mod DACRFS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selects VREFH0 as the reference voltage"]
            pub const VREFH0: u32 = 0;
            #[doc = "Selects VREFH1 as the reference voltage"]
            pub const VREFH1: u32 = 1;
            #[doc = "Selects VREFH2 as the reference voltage"]
            pub const VREFH2: u32 = 2;
        }
    }
    #[doc = "FIFO Enable"]
    pub mod FIFOEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables FIFO mode and disables Buffer mode. Any data written to goes to buffer then goes to conversion."]
            pub const BUFFER_MODE: u32 = 0;
            #[doc = "Enables FIFO mode. Data will be first read from FIFO to buffer and then goes to conversion."]
            pub const FIFO_MODE: u32 = 1;
        }
    }
    #[doc = "Swing Back Mode"]
    pub mod SWMD {
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
    #[doc = "DAC Trigger Select"]
    pub mod TRGSEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Hardware trigger"]
            pub const HARDWARE: u32 = 0;
            #[doc = "Software trigger"]
            pub const SOFTWARE: u32 = 1;
        }
    }
    #[doc = "DAC Periodic Trigger Mode Enable"]
    pub mod PTGEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "RCLK Cycles Before Data Latch"]
    pub mod LATCH_CYC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer Enable"]
    pub mod BUF_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not used"]
            pub const USE_BUF: u32 = 0;
            #[doc = "Used"]
            pub const NO_USE_BUF: u32 = 1;
        }
    }
    #[doc = "External On-Chip PTAT Current Reference Select"]
    pub mod IREF_PTAT_EXT_SEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not selected"]
            pub const NOT_SELECTED: u32 = 0;
            #[doc = "Selected"]
            pub const SELECTED: u32 = 1;
        }
    }
    #[doc = "External On-Chip ZTC Current Reference Select"]
    pub mod IREF_ZTC_EXT_SEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not selected"]
            pub const NOT_SELECTED: u32 = 0;
            #[doc = "Selected"]
            pub const SELECTED: u32 = 1;
        }
    }
    #[doc = "OPAMP as Buffer, Speed Control Signal"]
    pub mod BUF_SPD_CTRL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lower Low-Power mode"]
            pub const LLP_MODE: u32 = 0;
            #[doc = "Low-Power mode"]
            pub const LP_MODE: u32 = 1;
        }
    }
}
#[doc = "DAC FIFO Control"]
pub mod FCR {
    #[doc = "Watermark Level"]
    pub mod WML {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DAC FIFO Pointer"]
pub mod FPR {
    #[doc = "FIFO Read Pointer"]
    pub mod FIFO_RPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Write Pointer"]
    pub mod FIFO_WPT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIFO Status"]
pub mod FSR {
    #[doc = "FIFO Full Flag"]
    pub mod FULL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not full"]
            pub const NOT_FULL: u32 = 0;
            #[doc = "Full"]
            pub const FULL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Empty Flag"]
    pub mod EMPTY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Watermark Status Flag"]
    pub mod WM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Data in FIFO is more than watermark level"]
            pub const MORE_THAN_WLEVEL: u32 = 0;
            #[doc = "Data in FIFO is less than or equal to watermark level"]
            pub const LESS_THAN_WLEVEL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Swing Back One Cycle Complete Flag"]
    pub mod SWBK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No swing back cycle has completed since the last time the flag was cleared"]
            pub const NO_SWING: u32 = 0;
            #[doc = "At least one swing back cycle has occurred since the last time the flag was cleared"]
            pub const SWING_BACK: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Flag"]
    pub mod OF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overflow has occurred since the last time the flag was cleared"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "At least one FIFO overflow has occurred since the last time the flag was cleared"]
            pub const OVERFLOW: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Flag"]
    pub mod UF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No underflow has occurred since the last time the flag was cleared"]
            pub const NO_UNDERFLOW: u32 = 0;
            #[doc = "At least one trigger underflow has occurred since the last time the flag was cleared"]
            pub const UNDERFLOW: u32 = 1;
        }
    }
    #[doc = "Period Trigger Mode Conversion Complete Flag"]
    pub mod PTGCOCO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not completed or not started"]
            pub const NOT_START: u32 = 0;
            #[doc = "Completed"]
            pub const COMPLETED: u32 = 1;
        }
    }
}
#[doc = "Interrupt Enable"]
pub mod IER {
    #[doc = "FIFO Full Interrupt Enable"]
    pub mod FULL_IE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "FIFO Empty Interrupt Enable"]
    pub mod EMPTY_IE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "FIFO Watermark Interrupt Enable"]
    pub mod WM_IE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Swing Back One Cycle Complete Interrupt Enable"]
    pub mod SWBK_IE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    pub mod OF_IE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    pub mod UF_IE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "PTG Mode Conversion Complete Interrupt Enable"]
    pub mod PTGCOCO_IE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "DMA Enable"]
pub mod DER {
    #[doc = "FIFO Empty DMA Enable"]
    pub mod EMPTY_DMAEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "FIFO Watermark DMA Enable"]
    pub mod WM_DMAEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Reset Control"]
pub mod RCR {
    #[doc = "Software Reset"]
    pub mod SWRST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Software reset"]
            pub const SOFTWARE_RESET: u32 = 1;
        }
    }
    #[doc = "FIFO Reset"]
    pub mod FIFORST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "FIFO reset"]
            pub const FIFO_RESET: u32 = 1;
        }
    }
}
#[doc = "Trigger Control"]
pub mod TCR {
    #[doc = "Software Trigger"]
    pub mod SWTRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not valid"]
            pub const NOT_VALID: u32 = 0;
            #[doc = "Valid"]
            pub const VALID: u32 = 1;
        }
    }
}
#[doc = "Periodic Trigger Control"]
pub mod PCR {
    #[doc = "Periodic Trigger Number"]
    pub mod PTG_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Periodic Trigger Period Width"]
    pub mod PTG_PERIOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
