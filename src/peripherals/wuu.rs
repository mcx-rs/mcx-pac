#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "WUU"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RWRegister<u32>,
    #[doc = "Pin Enable 1"]
    pub PE1: crate::RWRegister<u32>,
    #[doc = "Pin Enable 2"]
    pub PE2: crate::RWRegister<u32>,
    _reserved0: [u8; 0x8],
    #[doc = "Module Interrupt Enable"]
    pub ME: crate::RWRegister<u32>,
    #[doc = "Module DMA/Trigger Enable"]
    pub DE: crate::RWRegister<u32>,
    #[doc = "Pin Flag"]
    pub PF: crate::RWRegister<u32>,
    _reserved1: [u8; 0xc],
    #[doc = "Pin Filter"]
    pub FILT: crate::RWRegister<u32>,
    _reserved2: [u8; 0x4],
    #[doc = "Pin DMA/Trigger Configuration 1"]
    pub PDC1: crate::RWRegister<u32>,
    #[doc = "Pin DMA/Trigger Configuration 2"]
    pub PDC2: crate::RWRegister<u32>,
    _reserved3: [u8; 0x8],
    #[doc = "Pin Filter DMA/Trigger Configuration"]
    pub FDC: crate::RWRegister<u32>,
    _reserved4: [u8; 0x4],
    #[doc = "Pin Mode Configuration"]
    pub PMC: crate::RWRegister<u32>,
    _reserved5: [u8; 0x4],
    #[doc = "Pin Filter Mode Configuration"]
    pub FMC: crate::RWRegister<u32>,
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {
            #[doc = "Standard features implemented"]
            pub const STANDARD: u32 = 0;
            #[doc = "Support for DMA/Trigger generation from wake-up pins and filters enabled. Support for external pin/filter detection during all power modes enabled."]
            pub const FILT_ALL_PWR: u32 = 1;
        }
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
    #[doc = "Filter Number"]
    pub mod FILTERS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Number"]
    pub mod DMAS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Module Number"]
    pub mod MODULES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pin Number"]
    pub mod PINS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Enable 1"]
pub mod PE1 {
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
}
#[doc = "Pin Enable 2"]
pub mod PE2 {
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE17 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE18 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE19 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE20 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE21 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE22 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE23 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE24 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE25 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE26 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE27 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn"]
    pub mod WUPE29 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Reserved"]
    pub mod WUPE30 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Not supported"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Not supported"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Not supported"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Reserved"]
    pub mod WUPE31 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const DISABLE: u32 = 0;
            #[doc = "Not supported"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Not supported"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Not supported"]
            pub const EN_ANY: u32 = 3;
        }
    }
}
#[doc = "Module Interrupt Enable"]
pub mod ME {
    #[doc = "Module Interrupt Wake-up Enable for Module n"]
    pub mod WUME0 {
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
    #[doc = "Module Interrupt Wake-up Enable for Module n"]
    pub mod WUME1 {
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
    #[doc = "Module Interrupt Wake-up Enable for Module n"]
    pub mod WUME2 {
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
    #[doc = "Module Interrupt Wake-up Enable for Module n"]
    pub mod WUME3 {
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
    #[doc = "Module Interrupt Wake-up Enable for Module n"]
    pub mod WUME4 {
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
    #[doc = "Module Interrupt Wake-up Enable for Module n"]
    pub mod WUME5 {
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
    #[doc = "Module Interrupt Wake-up Enable for Module n"]
    pub mod WUME6 {
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
    #[doc = "Module Interrupt Wake-up Enable for Module n"]
    pub mod WUME7 {
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
    #[doc = "Module Interrupt Wake-up Enable for Module n"]
    pub mod WUME8 {
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
    #[doc = "Module Interrupt Wake-up Enable for Module n"]
    pub mod WUME9 {
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
}
#[doc = "Module DMA/Trigger Enable"]
pub mod DE {
    #[doc = "DMA/Trigger Wake-up Enable for Module n"]
    pub mod WUDE0 {
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
    #[doc = "DMA/Trigger Wake-up Enable for Module n"]
    pub mod WUDE1 {
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
    #[doc = "DMA/Trigger Wake-up Enable for Module n"]
    pub mod WUDE2 {
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
    #[doc = "DMA/Trigger Wake-up Enable for Module n"]
    pub mod WUDE3 {
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
    #[doc = "DMA/Trigger Wake-up Enable for Module n"]
    pub mod WUDE4 {
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
    #[doc = "DMA/Trigger Wake-up Enable for Module n"]
    pub mod WUDE5 {
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
    #[doc = "DMA/Trigger Wake-up Enable for Module n"]
    pub mod WUDE6 {
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
    #[doc = "DMA/Trigger Wake-up Enable for Module n"]
    pub mod WUDE7 {
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
    #[doc = "DMA/Trigger Wake-up Enable for Module n"]
    pub mod WUDE8 {
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
    #[doc = "DMA/Trigger Wake-up Enable for Module n"]
    pub mod WUDE9 {
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
}
#[doc = "Pin Flag"]
pub mod PF {
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Wake-up Flag for WUU_Pn"]
    pub mod WUF29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Reserved"]
    pub mod WUF30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Not supported"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Reserved"]
    pub mod WUF31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Not supported"]
            pub const FLAG: u32 = 1;
        }
    }
}
#[doc = "Pin Filter"]
pub mod FILT {
    #[doc = "Filter 1 Pin Select"]
    pub mod FILTSEL1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Filter 1 Enable"]
    pub mod FILTE1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (Detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (Detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (Detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Filter 1 Flag"]
    pub mod FILTF1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Filter 2 Pin Select"]
    pub mod FILTSEL2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Filter 2 Enable"]
    pub mod FILTE2 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable (Detect on rising edge or high level)"]
            pub const EN_RISE_HI: u32 = 1;
            #[doc = "Enable (Detect on falling edge or low level)"]
            pub const EN_FALL_LO: u32 = 2;
            #[doc = "Enable (Detect on any edge)"]
            pub const EN_ANY: u32 = 3;
        }
    }
    #[doc = "Filter 2 Flag"]
    pub mod FILTF2 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Yes"]
            pub const FLAG: u32 = 1;
        }
    }
}
#[doc = "Pin DMA/Trigger Configuration 1"]
pub mod PDC1 {
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC4 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC6 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC7 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC8 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC9 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC10 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC11 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC12 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC14 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC15 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
}
#[doc = "Pin DMA/Trigger Configuration 2"]
pub mod PDC2 {
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC17 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC18 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC19 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC20 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC21 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC22 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC23 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC24 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC25 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC26 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC27 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC28 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn"]
    pub mod WUPDC29 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Reserved"]
    pub mod WUPDC30 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "Not supported"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Not supported"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
    #[doc = "Reserved"]
    pub mod WUPDC31 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "Not supported"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Not supported"]
            pub const TRIGGER: u32 = 2;
            #[doc = "Reserved"]
            pub const RES: u32 = 3;
        }
    }
}
#[doc = "Pin Filter DMA/Trigger Configuration"]
pub mod FDC {
    #[doc = "Filter Configuration for FILTn"]
    pub mod FILTC1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
        }
    }
    #[doc = "Filter Configuration for FILTn"]
    pub mod FILTC2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt"]
            pub const INTERRUPT: u32 = 0;
            #[doc = "DMA request"]
            pub const DMA_REQ: u32 = 1;
            #[doc = "Trigger event"]
            pub const TRIGGER: u32 = 2;
        }
    }
}
#[doc = "Pin Mode Configuration"]
pub mod PMC {
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn"]
    pub mod WUPMC29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Reserved"]
    pub mod WUPMC30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Not supported"]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Reserved"]
    pub mod WUPMC31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not supported"]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Not supported"]
            pub const ANY_PWR: u32 = 1;
        }
    }
}
#[doc = "Pin Filter Mode Configuration"]
pub mod FMC {
    #[doc = "Filter Mode for FILTn"]
    pub mod FILTM1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during Power Down/Deep Power Down mode"]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes"]
            pub const ANY_PWR: u32 = 1;
        }
    }
    #[doc = "Filter Mode for FILTn"]
    pub mod FILTM2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active only during Power Down/Deep Power Down mode"]
            pub const LOW_PWR_ONLY: u32 = 0;
            #[doc = "Active during all power modes"]
            pub const ANY_PWR: u32 = 1;
        }
    }
}
