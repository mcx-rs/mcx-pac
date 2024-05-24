#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "EWM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control"]
    pub CTRL: crate::RWRegister<u8>,
    #[doc = "Service"]
    pub SERV: crate::RWRegister<u8>,
    #[doc = "Compare Low"]
    pub CMPL: crate::RWRegister<u8>,
    #[doc = "Compare High"]
    pub CMPH: crate::RWRegister<u8>,
    #[doc = "Clock Control"]
    pub CLKCTRL: crate::RWRegister<u8>,
    #[doc = "Clock Prescaler"]
    pub CLKPRESCALER: crate::RWRegister<u8>,
}
#[doc = "Control"]
pub mod CTRL {
    #[doc = "EWM Enable"]
    pub mod EWMEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
    }
    #[doc = "Assertion State Select"]
    pub mod ASSIN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
    }
    #[doc = "Input Enable"]
    pub mod INEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
    }
    #[doc = "Interrupt Enable"]
    pub mod INTEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deasserts interrupt requests"]
            pub const ZERO: u32 = 0;
            #[doc = "Generates interrupt requests"]
            pub const INT_REQ: u32 = 1;
        }
    }
}
#[doc = "Service"]
pub mod SERV {
    #[doc = "Service"]
    pub mod SERVICE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Compare Low"]
pub mod CMPL {
    #[doc = "Compare Low"]
    pub mod COMPAREL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Compare High"]
pub mod CMPH {
    #[doc = "Compare High"]
    pub mod COMPAREH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Control"]
pub mod CLKCTRL {
    #[doc = "Clock Select"]
    pub mod CLKSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Prescaler"]
pub mod CLKPRESCALER {
    #[doc = "Clock Divider"]
    pub mod CLK_DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
