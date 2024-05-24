#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "LP_FLEXCOMM"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xff4],
    #[doc = "Interrupt Status"]
    pub ISTAT: crate::RWRegister<u32>,
    #[doc = "Peripheral Select and ID"]
    pub PSELID: crate::RWRegister<u32>,
}
#[doc = "Interrupt Status"]
pub mod ISTAT {
    #[doc = "UART TX Interrupt"]
    pub mod UARTTX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Clear"]
            pub const CLR: u32 = 0;
            #[doc = "Set"]
            pub const SET: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UART RX Interrupt"]
    pub mod UARTRX {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Clear"]
            pub const CLR: u32 = 0;
            #[doc = "Set"]
            pub const SET: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPI Interrupt"]
    pub mod SPI {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Clear"]
            pub const CLR: u32 = 0;
            #[doc = "Set"]
            pub const SET: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2C Controller Interrupt"]
    pub mod I2CM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Clear"]
            pub const CLR: u32 = 0;
            #[doc = "Set"]
            pub const SET: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2C Subordinate Interrupt"]
    pub mod I2CS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Clear"]
            pub const CLR: u32 = 0;
            #[doc = "Set"]
            pub const SET: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Peripheral Select and ID"]
pub mod PSELID {
    #[doc = "Peripheral Select"]
    pub mod PERSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No peripheral selected"]
            pub const NONE: u32 = 0;
            #[doc = "UART"]
            pub const UART: u32 = 1;
            #[doc = "SPI"]
            pub const SPI: u32 = 2;
            #[doc = "I2C"]
            pub const I2C: u32 = 3;
            #[doc = "UART and I2C"]
            pub const UARTI2C: u32 = 7;
        }
    }
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PERSEL is writable"]
            pub const FALSE: u32 = 0;
            #[doc = "PERSEL is not writable"]
            pub const TRUE: u32 = 1;
        }
    }
    #[doc = "UART Present"]
    pub mod UARTPRESENT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not supported"]
            pub const FALSE: u32 = 0;
            #[doc = "Supported"]
            pub const TRUE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPI Present"]
    pub mod SPIPRESENT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not supported"]
            pub const FALSE: u32 = 0;
            #[doc = "Supported"]
            pub const TRUE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2C Present"]
    pub mod I2CPRESENT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not supported"]
            pub const FALSE: u32 = 0;
            #[doc = "Supported"]
            pub const TRUE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LP_FLEXCOMM interface ID"]
    pub mod ID {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
