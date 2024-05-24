#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "Programmable Logic Unit (PLU)"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "no description available"]
    pub LUT: [lut::RegisterBlock; 26usize],
    _reserved0: [u8; 0x4c0],
    #[doc = "PLU LUT truth table"]
    pub LUT_TRUTH: [crate::RWRegister<u32>; 26usize],
    _reserved1: [u8; 0x98],
    #[doc = "PLU outputs"]
    pub OUTPUTS: crate::RWRegister<u32>,
    #[doc = "Wakeup interrupt control"]
    pub WAKEINT_CTRL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x2f8],
    #[doc = "PLU output multiplexer"]
    pub OUTPUT_MUX: [crate::RWRegister<u32>; 8usize],
}
pub mod lut {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Input select register for LUTn (0 to 25), Inputx (5 inputs)"]
        pub LUT_INP_MUX: [crate::RWRegister<u32>; 5usize],
    }
    #[doc = "Input select register for LUTn (0 to 25), Inputx (5 inputs)"]
    pub mod LUT_INP_MUX {
        #[doc = "Selects the input source to be connected to LUTn_INPx"]
        pub mod LUTn_INPx {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "PLU primary inputs 0"]
                pub const PLU_INPUTS0: u32 = 0;
                #[doc = "PLU primary inputs 1"]
                pub const PLU_INPUTS1: u32 = 1;
                #[doc = "PLU primary inputs 2"]
                pub const PLU_INPUTS2: u32 = 2;
                #[doc = "PLU primary inputs 3"]
                pub const PLU_INPUTS3: u32 = 3;
                #[doc = "PLU primary inputs 4"]
                pub const PLU_INPUTS4: u32 = 4;
                #[doc = "PLU primary inputs 5"]
                pub const PLU_INPUTS5: u32 = 5;
                #[doc = "Output of LUT0"]
                pub const LUT_OUTPUTS0: u32 = 6;
                #[doc = "Output of LUT1"]
                pub const LUT_OUTPUTS1: u32 = 7;
                #[doc = "Output of LUT2"]
                pub const LUT_OUTPUTS2: u32 = 8;
                #[doc = "Output of LUT3"]
                pub const LUT_OUTPUTS3: u32 = 9;
                #[doc = "Output of LUT4"]
                pub const LUT_OUTPUTS4: u32 = 10;
                #[doc = "Output of LUT5"]
                pub const LUT_OUTPUTS5: u32 = 11;
                #[doc = "Output of LUT6"]
                pub const LUT_OUTPUTS6: u32 = 12;
                #[doc = "Output of LUT7"]
                pub const LUT_OUTPUTS7: u32 = 13;
                #[doc = "Output of LUT8"]
                pub const LUT_OUTPUTS8: u32 = 14;
                #[doc = "Output of LUT9"]
                pub const LUT_OUTPUTS9: u32 = 15;
                #[doc = "Output of LUT10"]
                pub const LUT_OUTPUTS10: u32 = 16;
                #[doc = "Output of LUT11"]
                pub const LUT_OUTPUTS11: u32 = 17;
                #[doc = "Output of LUT12"]
                pub const LUT_OUTPUTS12: u32 = 18;
                #[doc = "Output of LUT13"]
                pub const LUT_OUTPUTS13: u32 = 19;
                #[doc = "Output of LUT14"]
                pub const LUT_OUTPUTS14: u32 = 20;
                #[doc = "Output of LUT15"]
                pub const LUT_OUTPUTS15: u32 = 21;
                #[doc = "Output of LUT16"]
                pub const LUT_OUTPUTS16: u32 = 22;
                #[doc = "Output of LUT17"]
                pub const LUT_OUTPUTS17: u32 = 23;
                #[doc = "Output of LUT18"]
                pub const LUT_OUTPUTS18: u32 = 24;
                #[doc = "Output of LUT19"]
                pub const LUT_OUTPUTS19: u32 = 25;
                #[doc = "Output of LUT20"]
                pub const LUT_OUTPUTS20: u32 = 26;
                #[doc = "Output of LUT21"]
                pub const LUT_OUTPUTS21: u32 = 27;
                #[doc = "Output of LUT22"]
                pub const LUT_OUTPUTS22: u32 = 28;
                #[doc = "Output of LUT23"]
                pub const LUT_OUTPUTS23: u32 = 29;
                #[doc = "Output of LUT24"]
                pub const LUT_OUTPUTS24: u32 = 30;
                #[doc = "Output of LUT25"]
                pub const LUT_OUTPUTS25: u32 = 31;
                #[doc = "State[0]"]
                pub const STATE0: u32 = 32;
                #[doc = "State[1]"]
                pub const STATE1: u32 = 33;
                #[doc = "State[2]"]
                pub const STATE2: u32 = 34;
                #[doc = "State[3]"]
                pub const STATE3: u32 = 35;
            }
        }
    }
}
#[doc = "PLU LUT truth table"]
pub mod LUT_TRUTH {
    #[doc = "LUT truth table"]
    pub mod LUT_TRUTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLU outputs"]
pub mod OUTPUTS {
    #[doc = "Output state"]
    pub mod OUTPUT_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Wakeup interrupt control"]
pub mod WAKEINT_CTRL {
    #[doc = "Interrupt mask"]
    pub mod MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Filter Mode"]
    pub mod FILTER_MODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bypass mode"]
            pub const BYPASS: u32 = 0;
            #[doc = "Filter 1 clock period"]
            pub const FILTER1CLK: u32 = 1;
            #[doc = "Filter 2 clock period"]
            pub const FILTER2CLK: u32 = 2;
            #[doc = "Filter 3 clock period"]
            pub const FILTER3CLK: u32 = 3;
        }
    }
    #[doc = "Filter clock select"]
    pub mod FILTER_CLKSEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selects the 1 MHz low-power oscillator as the filter clock."]
            pub const FRO1MHZ: u32 = 0;
            #[doc = "Selects the 12 MHz FRO as the filter clock."]
            pub const FRO12MHZ: u32 = 1;
        }
    }
    #[doc = "Latch the interrupt"]
    pub mod LATCH_ENABLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write to clear wakeint_latched"]
    pub mod INTR_CLEAR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PLU output multiplexer"]
pub mod OUTPUT_MUX {
    #[doc = "Selects the source to be connected to PLU output n."]
    pub mod OUTPUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LUT output 0"]
            pub const PLU_OUTPUT0: u32 = 0;
            #[doc = "LUT output 1"]
            pub const PLU_OUTPUT1: u32 = 1;
            #[doc = "LUT output 2"]
            pub const PLU_OUTPUT2: u32 = 2;
            #[doc = "LUT output 3"]
            pub const PLU_OUTPUT3: u32 = 3;
            #[doc = "LUT output 4"]
            pub const PLU_OUTPUT4: u32 = 4;
            #[doc = "LUT output 5"]
            pub const PLU_OUTPUT5: u32 = 5;
            #[doc = "LUT output 6"]
            pub const PLU_OUTPUT6: u32 = 6;
            #[doc = "LUT output 7"]
            pub const PLU_OUTPUT7: u32 = 7;
            #[doc = "LUT output 8"]
            pub const PLU_OUTPUT8: u32 = 8;
            #[doc = "LUT output 9"]
            pub const PLU_OUTPUT9: u32 = 9;
            #[doc = "LUT output 10"]
            pub const PLU_OUTPUT10: u32 = 10;
            #[doc = "LUT output 11"]
            pub const PLU_OUTPUT11: u32 = 11;
            #[doc = "LUT output 12"]
            pub const PLU_OUTPUT12: u32 = 12;
            #[doc = "LUT output 13"]
            pub const PLU_OUTPUT13: u32 = 13;
            #[doc = "LUT output 14"]
            pub const PLU_OUTPUT14: u32 = 14;
            #[doc = "LUT output 15"]
            pub const PLU_OUTPUT15: u32 = 15;
            #[doc = "LUT output 16"]
            pub const PLU_OUTPUT16: u32 = 16;
            #[doc = "LUT output 17"]
            pub const PLU_OUTPUT17: u32 = 17;
            #[doc = "LUT output 18"]
            pub const PLU_OUTPUT18: u32 = 18;
            #[doc = "LUT output 19"]
            pub const PLU_OUTPUT19: u32 = 19;
            #[doc = "LUT output 20"]
            pub const PLU_OUTPUT20: u32 = 20;
            #[doc = "LUT output 21"]
            pub const PLU_OUTPUT21: u32 = 21;
            #[doc = "LUT output 22"]
            pub const PLU_OUTPUT22: u32 = 22;
            #[doc = "LUT output 23"]
            pub const PLU_OUTPUT23: u32 = 23;
            #[doc = "LUT output 24"]
            pub const PLU_OUTPUT24: u32 = 24;
            #[doc = "LUT output 25"]
            pub const PLU_OUTPUT25: u32 = 25;
            #[doc = "State[0]"]
            pub const STATE0: u32 = 26;
            #[doc = "State[1]"]
            pub const STATE1: u32 = 27;
            #[doc = "State[2]"]
            pub const STATE2: u32 = 28;
            #[doc = "State[3]"]
            pub const STATE3: u32 = 29;
        }
    }
}
