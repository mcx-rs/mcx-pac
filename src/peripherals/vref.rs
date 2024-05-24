#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "VREF"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameters"]
    pub PARAM: crate::RWRegister<u32>,
    #[doc = "Control and Status"]
    pub CSR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "User Trim"]
    pub UTRIM: crate::RWRegister<u32>,
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
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
#[doc = "Parameters"]
pub mod PARAM {}
#[doc = "Control and Status"]
pub mod CSR {
    #[doc = "HC Bandgap Enabled"]
    pub mod HCBGEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DIS: u32 = 0;
            #[doc = "Enables"]
            pub const ENA: u32 = 1;
        }
    }
    #[doc = "Low-Power Bandgap Enable"]
    pub mod LPBGEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DIS: u32 = 0;
            #[doc = "Enables"]
            pub const ENA: u32 = 1;
        }
    }
    #[doc = "Low-Power Bandgap Buffer Enable"]
    pub mod LPBG_BUF_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DIS: u32 = 0;
            #[doc = "Enables"]
            pub const ENA: u32 = 1;
        }
    }
    #[doc = "Chop Oscillator Enable"]
    pub mod CHOPEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DIS: u32 = 0;
            #[doc = "Enables"]
            pub const ENA: u32 = 1;
        }
    }
    #[doc = "Current Compensation Enable"]
    pub mod ICOMPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DIS: u32 = 0;
            #[doc = "Enables"]
            pub const ENA: u32 = 1;
        }
    }
    #[doc = "Regulator Enable"]
    pub mod REGEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DIS: u32 = 0;
            #[doc = "Enables"]
            pub const ENA: u32 = 1;
        }
    }
    #[doc = "Reference Channel Select Negative Enable"]
    pub mod REFCHSELN_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DIS: u32 = 0;
            #[doc = "Enables"]
            pub const ENA: u32 = 1;
        }
    }
    #[doc = "Reference Channel Select Positive Enable"]
    pub mod REFCHSELP_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DIS: u32 = 0;
            #[doc = "Enables"]
            pub const ENA: u32 = 1;
        }
    }
    #[doc = "Voltage Reference Selection"]
    pub mod VRSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal bandgap"]
            pub const BANDGAP: u32 = 0;
            #[doc = "Low-power buffered 1v"]
            pub const ONE_V: u32 = 1;
            #[doc = "Buffer 2.1v output"]
            pub const TWO_PT_ONE_V: u32 = 2;
        }
    }
    #[doc = "Ground Select"]
    pub mod REFL_GRD_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "vrefl_3v"]
            pub const VREFL_3V: u32 = 0;
            #[doc = "VSSA"]
            pub const VSSA: u32 = 1;
        }
    }
    #[doc = "High-Power Level"]
    pub mod HI_PWR_LV {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low-power"]
            pub const LOW: u32 = 0;
            #[doc = "High-power"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "Internal Buffer21 Enable"]
    pub mod BUF21EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DIS: u32 = 0;
            #[doc = "Enables"]
            pub const ENA: u32 = 1;
        }
    }
    #[doc = "Internal HC Voltage Reference Stable"]
    pub mod VREFST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Disabled and unstable"]
            pub const DIS_NOTSTABLE: u32 = 0;
            #[doc = "Stable"]
            pub const STABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "User Trim"]
pub mod UTRIM {
    #[doc = "VREF 2.1V Trim"]
    pub mod TRIM2V1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VREF Trim"]
    pub mod VREFTRIM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
