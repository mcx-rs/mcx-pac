#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "LPTMR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Status"]
    pub CSR: crate::RWRegister<u32>,
    #[doc = "Prescale and Glitch Filter"]
    pub PSR: crate::RWRegister<u32>,
    #[doc = "Compare"]
    pub CMR: crate::RWRegister<u32>,
    #[doc = "Counter"]
    pub CNR: crate::RWRegister<u32>,
}
#[doc = "Control Status"]
pub mod CSR {
    #[doc = "Timer Enable"]
    pub mod TEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const TEN0: u32 = 0;
            #[doc = "Enable"]
            pub const TEN1: u32 = 1;
        }
    }
    #[doc = "Timer Mode Select"]
    pub mod TMS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Time Counter mode"]
            pub const TMS0: u32 = 0;
            #[doc = "Pulse Counter mode"]
            pub const TMS1: u32 = 1;
        }
    }
    #[doc = "Timer Free-Running Counter"]
    pub mod TFC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset if TCF set"]
            pub const TFC0: u32 = 0;
            #[doc = "Reset on overflow"]
            pub const TFC1: u32 = 1;
        }
    }
    #[doc = "Timer Pin Polarity"]
    pub mod TPP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active-high"]
            pub const TPP0: u32 = 0;
            #[doc = "Active-low"]
            pub const TPP1: u32 = 1;
        }
    }
    #[doc = "Timer Pin Select"]
    pub mod TPS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input 0"]
            pub const TPS00: u32 = 0;
            #[doc = "Input 1"]
            pub const TPS01: u32 = 1;
            #[doc = "Input 2"]
            pub const TPS10: u32 = 2;
            #[doc = "Input 3"]
            pub const TPS11: u32 = 3;
        }
    }
    #[doc = "Timer Interrupt Enable"]
    pub mod TIE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const TIE0: u32 = 0;
            #[doc = "Enable"]
            pub const TIE1: u32 = 1;
        }
    }
    #[doc = "Timer Compare Flag"]
    pub mod TCF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CNR != (CMR + 1)"]
            pub const TCF0: u32 = 0;
            #[doc = "CNR = (CMR + 1)"]
            pub const TCF1: u32 = 1;
        }
    }
    #[doc = "Timer DMA Request Enable"]
    pub mod TDRE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const TRDE0: u32 = 0;
            #[doc = "Enable"]
            pub const TRDE1: u32 = 1;
        }
    }
}
#[doc = "Prescale and Glitch Filter"]
pub mod PSR {
    #[doc = "Prescaler/Glitch Filter Clock Select"]
    pub mod PCS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock 0"]
            pub const PCS00: u32 = 0;
            #[doc = "Clock 1"]
            pub const PCS01: u32 = 1;
            #[doc = "Clock 2"]
            pub const PCS10: u32 = 2;
            #[doc = "Clock 3"]
            pub const PCS11: u32 = 3;
        }
    }
    #[doc = "Prescaler/Glitch Filter Bypass"]
    pub mod PBYP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Prescaler/glitch filter enable"]
            pub const PBYP0: u32 = 0;
            #[doc = "Prescaler/glitch filter bypass"]
            pub const PBYP1: u32 = 1;
        }
    }
    #[doc = "Prescale/Glitch Filter Value"]
    pub mod PRESCALE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration."]
            pub const PRESCALE0000: u32 = 0;
            #[doc = "Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after 2 rising clock edges."]
            pub const PRESCALE0001: u32 = 1;
            #[doc = "Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after 4 rising clock edges."]
            pub const PRESCALE0010: u32 = 2;
            #[doc = "Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after 8 rising clock edges."]
            pub const PRESCALE0011: u32 = 3;
            #[doc = "Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges."]
            pub const PRESCALE0100: u32 = 4;
            #[doc = "Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges."]
            pub const PRESCALE0101: u32 = 5;
            #[doc = "Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges."]
            pub const PRESCALE0110: u32 = 6;
            #[doc = "Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges."]
            pub const PRESCALE0111: u32 = 7;
            #[doc = "Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges."]
            pub const PRESCALE1000: u32 = 8;
            #[doc = "Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges."]
            pub const PRESCALE1001: u32 = 9;
            #[doc = "Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges."]
            pub const PRESCALE1010: u32 = 10;
            #[doc = "Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges."]
            pub const PRESCALE1011: u32 = 11;
            #[doc = "Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges."]
            pub const PRESCALE1100: u32 = 12;
            #[doc = "Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges."]
            pub const PRESCALE1101: u32 = 13;
            #[doc = "Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges."]
            pub const PRESCALE1110: u32 = 14;
            #[doc = "Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges."]
            pub const PRESCALE1111: u32 = 15;
        }
    }
}
#[doc = "Compare"]
pub mod CMR {
    #[doc = "Compare Value"]
    pub mod COMPARE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter"]
pub mod CNR {
    #[doc = "Counter Value"]
    pub mod COUNTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
