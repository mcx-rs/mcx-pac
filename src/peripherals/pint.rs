#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "Pin Interrupts and Pattern Match"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Pin Interrupt Mode"]
    pub ISEL: crate::RWRegister<u32>,
    #[doc = "Pin Interrupt Level or Rising-Edge Interrupt Enable"]
    pub IENR: crate::RWRegister<u32>,
    #[doc = "Pin Interrupt Level or Rising-Edge Interrupt Set"]
    pub SIENR: crate::RWRegister<u32>,
    #[doc = "Pin Interrupt Level (Rising-Edge Interrupt) Clear"]
    pub CIENR: crate::RWRegister<u32>,
    #[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Enable"]
    pub IENF: crate::RWRegister<u32>,
    #[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Set"]
    pub SIENF: crate::RWRegister<u32>,
    #[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Clear"]
    pub CIENF: crate::RWRegister<u32>,
    #[doc = "Pin Interrupt Rising Edge"]
    pub RISE: crate::RWRegister<u32>,
    #[doc = "Pin Interrupt Falling Edge"]
    pub FALL: crate::RWRegister<u32>,
    #[doc = "Pin Interrupt Status"]
    pub IST: crate::RWRegister<u32>,
    #[doc = "Pattern-Match Interrupt Control"]
    pub PMCTRL: crate::RWRegister<u32>,
    #[doc = "Pattern-Match Interrupt Bit-Slice Source"]
    pub PMSRC: crate::RWRegister<u32>,
    #[doc = "Pattern-Match Interrupt Bit Slice Configuration"]
    pub PMCFG: crate::RWRegister<u32>,
}
#[doc = "Pin Interrupt Mode"]
pub mod ISEL {
    #[doc = "Interrupt mode"]
    pub mod PMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "In bit n configures the interrupt to be edge-sensitive"]
            pub const ISEL_0: u32 = 0;
            #[doc = "In bit n configures the interrupt to be level-sensitive"]
            pub const ISEL_1: u32 = 1;
        }
    }
}
#[doc = "Pin Interrupt Level or Rising-Edge Interrupt Enable"]
pub mod IENR {
    #[doc = "Enables Interrupt"]
    pub mod ENRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "In bit n disables the corresponding interrupt"]
            pub const ENRL_0: u32 = 0;
            #[doc = "In bit n enables the corresponding interrupt"]
            pub const ENRL_1: u32 = 1;
        }
    }
}
#[doc = "Pin Interrupt Level or Rising-Edge Interrupt Set"]
pub mod SIENR {
    #[doc = "Configures IENR"]
    pub mod SETENRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No operation for interrupt n"]
            pub const SETENRL_0: u32 = 0;
            #[doc = "Enable rising edge or level interrupt for interrupt n"]
            pub const SETENRL_1: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Pin Interrupt Level (Rising-Edge Interrupt) Clear"]
pub mod CIENR {
    #[doc = "Clear bits in IENR"]
    pub mod CENRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No operation"]
            pub const CENRL_0: u32 = 0;
            #[doc = "Disable rising edge or level interrupt"]
            pub const CENRL_1: u32 = 1;
        }
    }
}
#[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Enable"]
pub mod IENF {
    #[doc = "Enables Interrupt"]
    pub mod ENAF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable (set active interrupt level LOW)"]
            pub const ENAF_0: u32 = 0;
            #[doc = "Enable (set active interrupt level HIGH)"]
            pub const ENAF_1: u32 = 1;
        }
    }
}
#[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Set"]
pub mod SIENF {
    #[doc = "Write 1 to this address to clear to disable interrupts. Bit a sets bit n in IENF."]
    pub mod SETENAF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Writes 0 to IENF."]
            pub const SETENAF_0: u32 = 0;
            #[doc = "Select HIGH-active interrupt or enable falling-edge interrupt"]
            pub const SETENAF_1: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Clear"]
pub mod CIENF {
    #[doc = "Writes 0 to IENF"]
    pub mod CENAF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No operation"]
            pub const CENAF_0: u32 = 0;
            #[doc = "LOW-active interrupt selected or falling-edge interrupt disabled"]
            pub const CENAF_1: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Pin Interrupt Rising Edge"]
pub mod RISE {
    #[doc = "Rising-Edge Detect"]
    pub mod RDET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read 0- No rising edge (since Reset or you wrote a 1 to this field last time), Write 0- No operation"]
            pub const RDET_0: u32 = 0;
            #[doc = "Read 1- Rising edge (since Reset or you wrote a 1 to this field last time), Write 1- Clear rising-edge detection for this pin"]
            pub const RDET_1: u32 = 1;
        }
    }
}
#[doc = "Pin Interrupt Falling Edge"]
pub mod FALL {
    #[doc = "Falling-Edge Detect"]
    pub mod FDET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read 0- No falling edge (since Reset or you wrote a 1 to this field last time), Write 0- No operation"]
            pub const FDET_0: u32 = 0;
            #[doc = "Read 1- Falling edge (since Reset or you wrote a 1 to this field last time), Write 1- Clear falling-edge detection for this bit"]
            pub const FDET_1: u32 = 1;
        }
    }
}
#[doc = "Pin Interrupt Status"]
pub mod IST {
    #[doc = "Pin Interrupt Status"]
    pub mod PSTAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read 0- Interrupt is not requested, Write 0- No operation"]
            pub const PSTAT_0: u32 = 0;
            #[doc = "Read 1- Interrupt is requested, Write 1 (edge-sensitive)- clear rising- and falling-edge detection for this pin, Write 1 (level-sensitive)- switch the active level for this pin in"]
            pub const PSTAT_1: u32 = 1;
        }
    }
}
#[doc = "Pattern-Match Interrupt Control"]
pub mod PMCTRL {
    #[doc = "Specifies whether the pin interrupts are controlled by the pin interrupt function or by the pattern-match function. If this value is 0b, interrupts are driven in response to the standard pin interrupt function. If this value is 1b, interrupts are driven in response to pattern matches."]
    pub mod SEL_PMATCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin interrupt"]
            pub const PIN_INTERRUPT: u32 = 0;
            #[doc = "Pattern match"]
            pub const PATTERN_MATCH: u32 = 1;
        }
    }
    #[doc = "Enables the RXEV output to the CPU and/or to a GPIO output, when the specified Boolean expression evaluates to true. If this value is 0b, RXEV output to the CPU is disabled. If this value is 1b, RXEV output to the CPU is enabled."]
    pub mod ENA_RXEV {
        pub const offset: u32 = 1;
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
    #[doc = "Pattern Matches"]
    pub mod PMAT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "The corresponding product term is matched by the current state of the appropriate inputs"]
            pub const PMAT_1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pattern-Match Interrupt Bit-Slice Source"]
pub mod PMSRC {
    #[doc = "Selects the input source for bit slice 0"]
    pub mod SRC0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input 0 (selects the pin identified in PINT_SEL0)"]
            pub const INPUT0: u32 = 0;
            #[doc = "Input 1 (selects the pin identified in PINT_SEL1)"]
            pub const INPUT1: u32 = 1;
            #[doc = "Input 2 (selects the pin identified in PINT_SEL2)"]
            pub const INPUT2: u32 = 2;
            #[doc = "Input 3 (selects the pin identified in PINT_SEL3)"]
            pub const INPUT3: u32 = 3;
            #[doc = "Input 4 (selects the pin identified in PINT_SEL4)"]
            pub const INPUT4: u32 = 4;
            #[doc = "Input 5 (selects the pin identified in PINT_SEL5)"]
            pub const INPUT5: u32 = 5;
            #[doc = "Input 6 (selects the pin identified in PINT_SEL6)"]
            pub const INPUT6: u32 = 6;
            #[doc = "Input 7 (selects the pin identified in PINT_SEL7)"]
            pub const INPUT7: u32 = 7;
        }
    }
    #[doc = "Selects the input source for bit slice 1"]
    pub mod SRC1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input 0 (selects the pin identified in PINT_SEL0)"]
            pub const INPUT0: u32 = 0;
            #[doc = "Input 1 (selects the pin identified in PINT_SEL1)"]
            pub const INPUT1: u32 = 1;
            #[doc = "Input 2 (selects the pin identified in PINT_SEL2)"]
            pub const INPUT2: u32 = 2;
            #[doc = "Input 3 (selects the pin identified in PINT_SEL3)"]
            pub const INPUT3: u32 = 3;
            #[doc = "Input 4 (selects the pin identified in PINT_SEL4)"]
            pub const INPUT4: u32 = 4;
            #[doc = "Input 5 (selects the pin identified in PINT_SEL5)"]
            pub const INPUT5: u32 = 5;
            #[doc = "Input 6 (selects the pin identified in PINT_SEL6)"]
            pub const INPUT6: u32 = 6;
            #[doc = "Input 7 (selects the pin identified in PINT_SEL7)"]
            pub const INPUT7: u32 = 7;
        }
    }
    #[doc = "Selects the input source for bit slice 2"]
    pub mod SRC2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input 0 (selects the pin identified in PINT_SEL0)"]
            pub const INPUT0: u32 = 0;
            #[doc = "Input 1 (selects the pin identified in PINT_SEL1)"]
            pub const INPUT1: u32 = 1;
            #[doc = "Input 2 (selects the pin identified in PINT_SEL2)"]
            pub const INPUT2: u32 = 2;
            #[doc = "Input 3 (selects the pin identified in PINT_SEL3)"]
            pub const INPUT3: u32 = 3;
            #[doc = "Input 4 (selects the pin identified in PINT_SEL4)"]
            pub const INPUT4: u32 = 4;
            #[doc = "Input 5 (selects the pin identified in PINT_SEL5)"]
            pub const INPUT5: u32 = 5;
            #[doc = "Input 6 (selects the pin identified in PINT_SEL6)"]
            pub const INPUT6: u32 = 6;
            #[doc = "Input 7 (selects the pin identified in PINT_SEL7)"]
            pub const INPUT7: u32 = 7;
        }
    }
    #[doc = "Selects the input source for bit slice 3"]
    pub mod SRC3 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input 0 (selects the pin identified in PINT_SEL0)"]
            pub const INPUT0: u32 = 0;
            #[doc = "Input 1 (selects the pin identified in PINT_SEL1)"]
            pub const INPUT1: u32 = 1;
            #[doc = "Input 2 (selects the pin identified in PINT_SEL2)"]
            pub const INPUT2: u32 = 2;
            #[doc = "Input 3 (selects the pin identified in PINT_SEL3)"]
            pub const INPUT3: u32 = 3;
            #[doc = "Input 4 (selects the pin identified in PINT_SEL4)"]
            pub const INPUT4: u32 = 4;
            #[doc = "Input 5 (selects the pin identified in PINT_SEL5)"]
            pub const INPUT5: u32 = 5;
            #[doc = "Input 6 (selects the pin identified in PINT_SEL6)"]
            pub const INPUT6: u32 = 6;
            #[doc = "Input 7 (selects the pin identified in PINT_SEL7)"]
            pub const INPUT7: u32 = 7;
        }
    }
    #[doc = "Selects the input source for bit slice 4"]
    pub mod SRC4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input 0 (selects the pin identified in PINT_SEL0)"]
            pub const INPUT0: u32 = 0;
            #[doc = "Input 1 (selects the pin identified in PINT_SEL1)"]
            pub const INPUT1: u32 = 1;
            #[doc = "Input 2 (selects the pin identified in PINT_SEL2)"]
            pub const INPUT2: u32 = 2;
            #[doc = "Input 3 (selects the pin identified in PINT_SEL3)"]
            pub const INPUT3: u32 = 3;
            #[doc = "Input 4 (selects the pin identified in PINT_SEL4)"]
            pub const INPUT4: u32 = 4;
            #[doc = "Input 5 (selects the pin identified in PINT_SEL5)"]
            pub const INPUT5: u32 = 5;
            #[doc = "Input 6 (selects the pin identified in PINT_SEL6)"]
            pub const INPUT6: u32 = 6;
            #[doc = "Input 7 (selects the pin identified in PINT_SEL7)"]
            pub const INPUT7: u32 = 7;
        }
    }
    #[doc = "Selects the input source for bit slice 5"]
    pub mod SRC5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input 0 (selects the pin identified in PINT_SEL0)"]
            pub const INPUT0: u32 = 0;
            #[doc = "Input 1 (selects the pin identified in PINT_SEL1)"]
            pub const INPUT1: u32 = 1;
            #[doc = "Input 2 (selects the pin identified in PINT_SEL2)"]
            pub const INPUT2: u32 = 2;
            #[doc = "Input 3 (selects the pin identified in PINT_SEL3)"]
            pub const INPUT3: u32 = 3;
            #[doc = "Input 4 (selects the pin identified in PINT_SEL4)"]
            pub const INPUT4: u32 = 4;
            #[doc = "Input 5 (selects the pin identified in PINT_SEL5)"]
            pub const INPUT5: u32 = 5;
            #[doc = "Input 6 (selects the pin identified in PINT_SEL6)"]
            pub const INPUT6: u32 = 6;
            #[doc = "Input 7 (selects the pin identified in PINT_SEL7)"]
            pub const INPUT7: u32 = 7;
        }
    }
    #[doc = "Selects the input source for bit slice 6"]
    pub mod SRC6 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input 0 (selects the pin identified in PINT_SEL0)"]
            pub const INPUT0: u32 = 0;
            #[doc = "Input 1 (selects the pin identified in PINT_SEL1)"]
            pub const INPUT1: u32 = 1;
            #[doc = "Input 2 (selects the pin identified in PINT_SEL2)"]
            pub const INPUT2: u32 = 2;
            #[doc = "Input 3 (selects the pin identified in PINT_SEL3)"]
            pub const INPUT3: u32 = 3;
            #[doc = "Input 4 (selects the pin identified in PINT_SEL4)"]
            pub const INPUT4: u32 = 4;
            #[doc = "Input 5 (selects the pin identified in PINT_SEL5)"]
            pub const INPUT5: u32 = 5;
            #[doc = "Input 6 (selects the pin identified in PINT_SEL6)"]
            pub const INPUT6: u32 = 6;
            #[doc = "Input 7 (selects the pin identified in PINT_SEL7)"]
            pub const INPUT7: u32 = 7;
        }
    }
    #[doc = "Selects the input source for bit slice 7"]
    pub mod SRC7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input 0 (selects the pin identified in PINT_SEL0)"]
            pub const INPUT0: u32 = 0;
            #[doc = "Input 1 (selects the pin identified in PINT_SEL1)"]
            pub const INPUT1: u32 = 1;
            #[doc = "Input 2 (selects the pin identified in PINT_SEL2)"]
            pub const INPUT2: u32 = 2;
            #[doc = "Input 3 (selects the pin identified in PINT_SEL3)"]
            pub const INPUT3: u32 = 3;
            #[doc = "Input 4 (selects the pin identified in PINT_SEL4)"]
            pub const INPUT4: u32 = 4;
            #[doc = "Input 5 (selects the pin identified in PINT_SEL5)"]
            pub const INPUT5: u32 = 5;
            #[doc = "Input 6 (selects the pin identified in PINT_SEL6)"]
            pub const INPUT6: u32 = 6;
            #[doc = "Input 7 (selects the pin identified in PINT_SEL7)"]
            pub const INPUT7: u32 = 7;
        }
    }
}
#[doc = "Pattern-Match Interrupt Bit Slice Configuration"]
pub mod PMCFG {
    #[doc = "Determines whether slice 0 is an endpoint. Slice 0 is not an endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    pub mod PROD_ENDPTS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Endpoint"]
            pub const ENDPOINT: u32 = 1;
        }
    }
    #[doc = "Determines whether slice 1 is an endpoint. Slice 1 is not an endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    pub mod PROD_ENDPTS1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Endpoint"]
            pub const ENDPOINT: u32 = 1;
        }
    }
    #[doc = "Determines whether slice 2 is an endpoint. Slice 2 is not an endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    pub mod PROD_ENDPTS2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Endpoint"]
            pub const ENDPOINT: u32 = 1;
        }
    }
    #[doc = "Determines whether slice 3 is an endpoint. Slice 3 is not an endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    pub mod PROD_ENDPTS3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Endpoint"]
            pub const ENDPOINT: u32 = 1;
        }
    }
    #[doc = "Determines whether slice 4 is an endpoint. Slice 4 is not an endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    pub mod PROD_ENDPTS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Endpoint"]
            pub const ENDPOINT: u32 = 1;
        }
    }
    #[doc = "Determines whether slice 5 is an endpoint. Slice 5 is not an endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    pub mod PROD_ENDPTS5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Endpoint"]
            pub const ENDPOINT: u32 = 1;
        }
    }
    #[doc = "Determines whether slice 6 is an endpoint. Slice 6 is not an endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    pub mod PROD_ENDPTS6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Endpoint"]
            pub const ENDPOINT: u32 = 1;
        }
    }
    #[doc = "Match Configuration"]
    pub mod CFG0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Constant HIGH"]
            pub const CONSTANT_HIGH: u32 = 0;
            #[doc = "Sticky rising edge"]
            pub const STICKY_RISING_EDGE: u32 = 1;
            #[doc = "Sticky falling edge"]
            pub const STICKY_FALLING_EDGE: u32 = 2;
            #[doc = "Sticky rising or falling edge"]
            pub const STICKY_RISING_FALLING_EDGE: u32 = 3;
            #[doc = "High level"]
            pub const HIGH_LEVEL: u32 = 4;
            #[doc = "Low level"]
            pub const LOW_LEVEL: u32 = 5;
            #[doc = "Constant 0"]
            pub const CONSTANT_ZERO: u32 = 6;
            #[doc = "Event (Nonsticky rising or falling edge)"]
            pub const EVENT: u32 = 7;
        }
    }
    #[doc = "Match Configuration"]
    pub mod CFG1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Constant HIGH"]
            pub const CONSTANT_HIGH: u32 = 0;
            #[doc = "Sticky rising edge"]
            pub const STICKY_RISING_EDGE: u32 = 1;
            #[doc = "Sticky falling edge"]
            pub const STICKY_FALLING_EDGE: u32 = 2;
            #[doc = "Sticky rising or falling edge"]
            pub const STICKY_RISING_FALLING_EDGE: u32 = 3;
            #[doc = "High level"]
            pub const HIGH_LEVEL: u32 = 4;
            #[doc = "Low level"]
            pub const LOW_LEVEL: u32 = 5;
            #[doc = "Constant 0"]
            pub const CONSTANT_ZERO: u32 = 6;
            #[doc = "Event (Nonsticky rising or falling edge)"]
            pub const EVENT: u32 = 7;
        }
    }
    #[doc = "Match Configuration"]
    pub mod CFG2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Constant HIGH"]
            pub const CONSTANT_HIGH: u32 = 0;
            #[doc = "Sticky rising edge"]
            pub const STICKY_RISING_EDGE: u32 = 1;
            #[doc = "Sticky falling edge"]
            pub const STICKY_FALLING_EDGE: u32 = 2;
            #[doc = "Sticky rising or falling edge"]
            pub const STICKY_RISING_FALLING_EDGE: u32 = 3;
            #[doc = "High level"]
            pub const HIGH_LEVEL: u32 = 4;
            #[doc = "Low level"]
            pub const LOW_LEVEL: u32 = 5;
            #[doc = "Constant 0"]
            pub const CONSTANT_ZERO: u32 = 6;
            #[doc = "Event (Nonsticky rising or falling edge)"]
            pub const EVENT: u32 = 7;
        }
    }
    #[doc = "Match Configuration"]
    pub mod CFG3 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Constant HIGH"]
            pub const CONSTANT_HIGH: u32 = 0;
            #[doc = "Sticky rising edge"]
            pub const STICKY_RISING_EDGE: u32 = 1;
            #[doc = "Sticky falling edge"]
            pub const STICKY_FALLING_EDGE: u32 = 2;
            #[doc = "Sticky rising or falling edge"]
            pub const STICKY_RISING_FALLING_EDGE: u32 = 3;
            #[doc = "High level"]
            pub const HIGH_LEVEL: u32 = 4;
            #[doc = "Low level"]
            pub const LOW_LEVEL: u32 = 5;
            #[doc = "Constant 0"]
            pub const CONSTANT_ZERO: u32 = 6;
            #[doc = "Event (Nonsticky rising or falling edge)"]
            pub const EVENT: u32 = 7;
        }
    }
    #[doc = "Match Configuration"]
    pub mod CFG4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Constant HIGH"]
            pub const CONSTANT_HIGH: u32 = 0;
            #[doc = "Sticky rising edge"]
            pub const STICKY_RISING_EDGE: u32 = 1;
            #[doc = "Sticky falling edge"]
            pub const STICKY_FALLING_EDGE: u32 = 2;
            #[doc = "Sticky rising or falling edge"]
            pub const STICKY_RISING_FALLING_EDGE: u32 = 3;
            #[doc = "High level"]
            pub const HIGH_LEVEL: u32 = 4;
            #[doc = "Low level"]
            pub const LOW_LEVEL: u32 = 5;
            #[doc = "Constant 0"]
            pub const CONSTANT_ZERO: u32 = 6;
            #[doc = "Event (Nonsticky rising or falling edge)"]
            pub const EVENT: u32 = 7;
        }
    }
    #[doc = "Match Configuration"]
    pub mod CFG5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Constant HIGH"]
            pub const CONSTANT_HIGH: u32 = 0;
            #[doc = "Sticky rising edge"]
            pub const STICKY_RISING_EDGE: u32 = 1;
            #[doc = "Sticky falling edge"]
            pub const STICKY_FALLING_EDGE: u32 = 2;
            #[doc = "Sticky rising or falling edge"]
            pub const STICKY_RISING_FALLING_EDGE: u32 = 3;
            #[doc = "High level"]
            pub const HIGH_LEVEL: u32 = 4;
            #[doc = "Low level"]
            pub const LOW_LEVEL: u32 = 5;
            #[doc = "Constant 0"]
            pub const CONSTANT_ZERO: u32 = 6;
            #[doc = "Event (Nonsticky rising or falling edge)"]
            pub const EVENT: u32 = 7;
        }
    }
    #[doc = "Match Configuration"]
    pub mod CFG6 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Constant HIGH"]
            pub const CONSTANT_HIGH: u32 = 0;
            #[doc = "Sticky rising edge"]
            pub const STICKY_RISING_EDGE: u32 = 1;
            #[doc = "Sticky falling edge"]
            pub const STICKY_FALLING_EDGE: u32 = 2;
            #[doc = "Sticky rising or falling edge"]
            pub const STICKY_RISING_FALLING_EDGE: u32 = 3;
            #[doc = "High level"]
            pub const HIGH_LEVEL: u32 = 4;
            #[doc = "Low level"]
            pub const LOW_LEVEL: u32 = 5;
            #[doc = "Constant 0"]
            pub const CONSTANT_ZERO: u32 = 6;
            #[doc = "Event (Nonsticky rising or falling edge)"]
            pub const EVENT: u32 = 7;
        }
    }
    #[doc = "Match Configuration"]
    pub mod CFG7 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Constant HIGH"]
            pub const CONSTANT_HIGH: u32 = 0;
            #[doc = "Sticky rising edge"]
            pub const STICKY_RISING_EDGE: u32 = 1;
            #[doc = "Sticky falling edge"]
            pub const STICKY_FALLING_EDGE: u32 = 2;
            #[doc = "Sticky rising or falling edge"]
            pub const STICKY_RISING_FALLING_EDGE: u32 = 3;
            #[doc = "High level"]
            pub const HIGH_LEVEL: u32 = 4;
            #[doc = "Low level"]
            pub const LOW_LEVEL: u32 = 5;
            #[doc = "Constant 0"]
            pub const CONSTANT_ZERO: u32 = 6;
            #[doc = "Event (Nonsticky rising or falling edge)"]
            pub const EVENT: u32 = 7;
        }
    }
}
