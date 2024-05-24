#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "OPAMP"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RWRegister<u32>,
    #[doc = "OPAMP Control"]
    pub OPAMP_CTR: crate::RWRegister<u32>,
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
#[doc = "Parameter"]
pub mod PARAM {
    #[doc = "PGA Function Option"]
    pub mod PGA_FUNCTION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Core amplifier enabled"]
            pub const CORE_AMP: u32 = 0;
            #[doc = "PGA function enabled"]
            pub const PGA: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OPAMP Control"]
pub mod OPAMP_CTR {
    #[doc = "OPAMP Enable"]
    pub mod EN {
        pub const offset: u32 = 0;
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
    #[doc = "Mode Selection"]
    pub mod MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "High performance mode"]
            pub const LOW: u32 = 0;
            #[doc = "Low power mode"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "Bias Current Trim Selection"]
    pub mod BIASC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default"]
            pub const DEF: u32 = 0;
            #[doc = "Increase current"]
            pub const INC: u32 = 1;
            #[doc = "Decrease current"]
            pub const DEC: u32 = 2;
            #[doc = "Further decrease current"]
            pub const FUR_DEC: u32 = 3;
        }
    }
    #[doc = "Internal Reference Voltage Selection"]
    pub mod INTREF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select VDDA/2"]
            pub const VDDA2: u32 = 0;
            #[doc = "Select VDDA_3V"]
            pub const VDDA3V: u32 = 1;
            #[doc = "Select VSSA_3V"]
            pub const VSSA3V: u32 = 2;
            #[doc = "Not allowed"]
            pub const NOT: u32 = 3;
        }
    }
    #[doc = "Trigger Mode"]
    pub mod TRIGMD {
        pub const offset: u32 = 8;
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
    #[doc = "Positive Input Channel Selection"]
    pub mod INPSEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When OPAMP is not in trigger mode, select positive input 0 (INP0)"]
            pub const INP0: u32 = 0;
            #[doc = "When OPAMP is not in trigger mode, select positive input 1 (INP1)"]
            pub const INP1: u32 = 1;
        }
    }
    #[doc = "Positive Input Connection Status"]
    pub mod INPF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Positive input 0 (INP0)"]
            pub const INP0: u32 = 0;
            #[doc = "Positive input 1 (INP1)"]
            pub const INP1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reference Buffer"]
    pub mod BUFEN {
        pub const offset: u32 = 16;
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
    #[doc = "Positive Reference Voltage Selection"]
    pub mod PREF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DACx_OUT"]
            pub const VAL0: u32 = 0;
            #[doc = "VDDA/2"]
            pub const VAL1: u32 = 1;
            #[doc = "VREFO"]
            pub const VAL2: u32 = 2;
            #[doc = "520 mV"]
            pub const VAL3: u32 = 3;
        }
    }
    #[doc = "Measure Switch 1"]
    pub mod ADCSW1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Measure negative gain resistor ladder voltage switch off"]
            pub const OFF: u32 = 0;
            #[doc = "Measure negative gain resistor ladder voltage switch on"]
            pub const ON: u32 = 1;
        }
    }
    #[doc = "Measure Switch 2"]
    pub mod ADCSW2 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Measure positive gain resistor ladder reference voltage switch off"]
            pub const DISABLE: u32 = 0;
            #[doc = "Measure positive gain resistor ladder reference voltage switch on"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Output Switch"]
    pub mod OUTSW {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OPAMP out to negative gain resistor ladder switch off"]
            pub const OFF: u32 = 0;
            #[doc = "OPAMP out to negative gain resistor ladder switch on"]
            pub const ON: u32 = 1;
        }
    }
    #[doc = "Positive PGA Selection"]
    pub mod PGAIN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Positive input 1 (INP1)"]
            pub const INP1: u32 = 0;
            #[doc = "Gain application 1*"]
            pub const G2: u32 = 1;
            #[doc = "Gain application 2*"]
            pub const G3: u32 = 2;
            #[doc = "Gain application 4*"]
            pub const G5: u32 = 3;
            #[doc = "Gain application 8*"]
            pub const G9: u32 = 4;
            #[doc = "Gain application 16*"]
            pub const G17: u32 = 5;
            #[doc = "Gain application 33*"]
            pub const G34: u32 = 6;
            #[doc = "Gain application 64*"]
            pub const G65: u32 = 7;
        }
    }
    #[doc = "Negative PGA Selection"]
    pub mod NGAIN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Buffer"]
            pub const BUFFER: u32 = 0;
            #[doc = "Gain application 1*"]
            pub const G1: u32 = 1;
            #[doc = "Gain application 2*"]
            pub const G2: u32 = 2;
            #[doc = "Gain application 4*"]
            pub const G4: u32 = 3;
            #[doc = "Gain application 8*"]
            pub const G8: u32 = 4;
            #[doc = "Gain application 16*"]
            pub const G16: u32 = 5;
            #[doc = "Gain application 33*"]
            pub const G33: u32 = 6;
            #[doc = "Gain application 64*"]
            pub const G64: u32 = 7;
        }
    }
}
