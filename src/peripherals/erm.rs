#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "ERM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "ERM Configuration Register 0"]
    pub CR0: crate::RWRegister<u32>,
    #[doc = "ERM Configuration Register 1"]
    pub CR1: crate::RWRegister<u32>,
    _reserved0: [u8; 0x8],
    #[doc = "ERM Status Register 0"]
    pub SR0: crate::RWRegister<u32>,
    #[doc = "ERM Status Register 1"]
    pub SR1: crate::RWRegister<u32>,
    _reserved1: [u8; 0xe8],
    #[doc = "ERM Memory 0 Error Address Register"]
    pub EAR0: crate::RWRegister<u32>,
    #[doc = "ERM Memory 0 Syndrome Register"]
    pub SYN0: crate::RWRegister<u32>,
    #[doc = "ERM Memory 0 Correctable Error Count Register"]
    pub CORR_ERR_CNT0: crate::RWRegister<u32>,
    _reserved2: [u8; 0x4],
    #[doc = "ERM Memory 1 Error Address Register"]
    pub EAR1: crate::RWRegister<u32>,
    #[doc = "ERM Memory 1 Syndrome Register"]
    pub SYN1: crate::RWRegister<u32>,
    #[doc = "ERM Memory 1 Correctable Error Count Register"]
    pub CORR_ERR_CNT1: crate::RWRegister<u32>,
    _reserved3: [u8; 0x4],
    #[doc = "ERM Memory 2 Error Address Register"]
    pub EAR2: crate::RWRegister<u32>,
    #[doc = "ERM Memory 2 Syndrome Register"]
    pub SYN2: crate::RWRegister<u32>,
    #[doc = "ERM Memory 2 Correctable Error Count Register"]
    pub CORR_ERR_CNT2: crate::RWRegister<u32>,
    _reserved4: [u8; 0x4],
    #[doc = "ERM Memory 3 Error Address Register"]
    pub EAR3: crate::RWRegister<u32>,
    #[doc = "ERM Memory 3 Syndrome Register"]
    pub SYN3: crate::RWRegister<u32>,
    #[doc = "ERM Memory 3 Correctable Error Count Register"]
    pub CORR_ERR_CNT3: crate::RWRegister<u32>,
    _reserved5: [u8; 0x4],
    #[doc = "ERM Memory 4 Error Address Register"]
    pub EAR4: crate::RWRegister<u32>,
    #[doc = "ERM Memory 4 Syndrome Register"]
    pub SYN4: crate::RWRegister<u32>,
    #[doc = "ERM Memory 4 Correctable Error Count Register"]
    pub CORR_ERR_CNT4: crate::RWRegister<u32>,
    _reserved6: [u8; 0x4],
    #[doc = "ERM Memory 5 Error Address Register"]
    pub EAR5: crate::RWRegister<u32>,
    #[doc = "ERM Memory 5 Syndrome Register"]
    pub SYN5: crate::RWRegister<u32>,
    #[doc = "ERM Memory 5 Correctable Error Count Register"]
    pub CORR_ERR_CNT5: crate::RWRegister<u32>,
    _reserved7: [u8; 0x4],
    #[doc = "ERM Memory 6 Error Address Register"]
    pub EAR6: crate::RWRegister<u32>,
    #[doc = "ERM Memory 6 Syndrome Register"]
    pub SYN6: crate::RWRegister<u32>,
    #[doc = "ERM Memory 6 Correctable Error Count Register"]
    pub CORR_ERR_CNT6: crate::RWRegister<u32>,
    _reserved8: [u8; 0xc],
    #[doc = "ERM Memory 7 Correctable Error Count Register"]
    pub CORR_ERR_CNT7: crate::RWRegister<u32>,
    _reserved9: [u8; 0x8],
    #[doc = "ERM Memory 8 Syndrome Register"]
    pub SYN8: crate::RWRegister<u32>,
    #[doc = "ERM Memory 8 Correctable Error Count Register"]
    pub CORR_ERR_CNT8: crate::RWRegister<u32>,
    _reserved10: [u8; 0xc],
    #[doc = "ERM Memory 9 Correctable Error Count Register"]
    pub CORR_ERR_CNT9: crate::RWRegister<u32>,
}
#[doc = "ERM Configuration Register 0"]
pub mod CR0 {
    #[doc = "ENCIE7"]
    pub mod ENCIE7 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 7 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 7 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ESCIE7"]
    pub mod ESCIE7 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 7 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 7 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ENCIE6"]
    pub mod ENCIE6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 6 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 6 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ESCIE6"]
    pub mod ESCIE6 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 6 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 6 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ENCIE5"]
    pub mod ENCIE5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 5 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 5 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ESCIE5"]
    pub mod ESCIE5 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 5 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 5 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ENCIE4"]
    pub mod ENCIE4 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 4 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 4 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ESCIE4"]
    pub mod ESCIE4 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 4 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 4 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ENCIE3"]
    pub mod ENCIE3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 3 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 3 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ESCIE3"]
    pub mod ESCIE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 3 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 3 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ENCIE2"]
    pub mod ENCIE2 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 2 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 2 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ESCIE2"]
    pub mod ESCIE2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 2 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 2 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ENCIE1"]
    pub mod ENCIE1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 1 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 1 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ESCIE1"]
    pub mod ESCIE1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 1 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 1 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ENCIE0"]
    pub mod ENCIE0 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 0 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 0 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ESCIE0"]
    pub mod ESCIE0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 0 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 0 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "ERM Configuration Register 1"]
pub mod CR1 {
    #[doc = "ENCIE9"]
    pub mod ENCIE9 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 9 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 9 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ESCIE9"]
    pub mod ESCIE9 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 9 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 9 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ENCIE8"]
    pub mod ENCIE8 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 8 non-correctable error events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 8 non-correctable error events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ESCIE8"]
    pub mod ESCIE8 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt notification of Memory 8 single-bit correction events is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt notification of Memory 8 single-bit correction events is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "ERM Status Register 0"]
pub mod SR0 {
    #[doc = "NCE7"]
    pub mod NCE7 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No non-correctable error event on Memory 7 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 7 detected."]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "SBC7"]
    pub mod SBC7 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No single-bit correction event on Memory 7 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 7 detected."]
            pub const EVENT: u32 = 1;
        }
    }
    #[doc = "NCE6"]
    pub mod NCE6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No non-correctable error event on Memory 6 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 6 detected."]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "SBC6"]
    pub mod SBC6 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No single-bit correction event on Memory 6 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 6 detected."]
            pub const EVENT: u32 = 1;
        }
    }
    #[doc = "NCE5"]
    pub mod NCE5 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No non-correctable error event on Memory 5 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 5 detected."]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "SBC5"]
    pub mod SBC5 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No single-bit correction event on Memory 5 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 5 detected."]
            pub const CORR_EVENT: u32 = 1;
        }
    }
    #[doc = "NCE4"]
    pub mod NCE4 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No non-correctable error event on Memory 4 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 4 detected."]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "SBC4"]
    pub mod SBC4 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No single-bit correction event on Memory 4 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 4 detected."]
            pub const EVENT: u32 = 1;
        }
    }
    #[doc = "NCE3"]
    pub mod NCE3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No non-correctable error event on Memory 3 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 3 detected."]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "SBC3"]
    pub mod SBC3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No single-bit correction event on Memory 3 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 3 detected."]
            pub const EVENT: u32 = 1;
        }
    }
    #[doc = "NCE2"]
    pub mod NCE2 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No non-correctable error event on Memory 2 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 2 detected."]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "SBC2"]
    pub mod SBC2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No single-bit correction event on Memory 2 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 2 detected."]
            pub const EVENT: u32 = 1;
        }
    }
    #[doc = "NCE1"]
    pub mod NCE1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No non-correctable error event on Memory 1 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 1 detected."]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "SBC1"]
    pub mod SBC1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No single-bit correction event on Memory 1 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 1 detected."]
            pub const EVENT: u32 = 1;
        }
    }
    #[doc = "NCE0"]
    pub mod NCE0 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No non-correctable error event on Memory 0 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 0 detected."]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "SBC0"]
    pub mod SBC0 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No single-bit correction event on Memory 0 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 0 detected."]
            pub const EVENT: u32 = 1;
        }
    }
}
#[doc = "ERM Status Register 1"]
pub mod SR1 {
    #[doc = "NCE9"]
    pub mod NCE9 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No non-correctable error event on Memory 9 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 9 detected."]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "SBC9"]
    pub mod SBC9 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No single-bit correction event on Memory 9 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 9 detected."]
            pub const EVENT: u32 = 1;
        }
    }
    #[doc = "NCE8"]
    pub mod NCE8 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No non-correctable error event on Memory 8 detected."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Non-correctable error event on Memory 8 detected."]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "SBC8"]
    pub mod SBC8 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No single-bit correction event on Memory 8 detected."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "Single-bit correction event on Memory 8 detected."]
            pub const EVENT: u32 = 1;
        }
    }
}
#[doc = "ERM Memory 0 Error Address Register"]
pub mod EAR0 {
    #[doc = "EAR"]
    pub mod EAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 0 Syndrome Register"]
pub mod SYN0 {
    #[doc = "SYNDROME"]
    pub mod SYNDROME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 0 Correctable Error Count Register"]
pub mod CORR_ERR_CNT0 {
    #[doc = "Memory n Correctable Error Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 1 Error Address Register"]
pub mod EAR1 {
    #[doc = "EAR"]
    pub mod EAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 1 Syndrome Register"]
pub mod SYN1 {
    #[doc = "SYNDROME"]
    pub mod SYNDROME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 1 Correctable Error Count Register"]
pub mod CORR_ERR_CNT1 {
    #[doc = "Memory n Correctable Error Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 2 Error Address Register"]
pub mod EAR2 {
    #[doc = "EAR"]
    pub mod EAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 2 Syndrome Register"]
pub mod SYN2 {
    #[doc = "SYNDROME"]
    pub mod SYNDROME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 2 Correctable Error Count Register"]
pub mod CORR_ERR_CNT2 {
    #[doc = "Memory n Correctable Error Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 3 Error Address Register"]
pub mod EAR3 {
    #[doc = "EAR"]
    pub mod EAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 3 Syndrome Register"]
pub mod SYN3 {
    #[doc = "SYNDROME"]
    pub mod SYNDROME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 3 Correctable Error Count Register"]
pub mod CORR_ERR_CNT3 {
    #[doc = "Memory n Correctable Error Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 4 Error Address Register"]
pub mod EAR4 {
    #[doc = "EAR"]
    pub mod EAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 4 Syndrome Register"]
pub mod SYN4 {
    #[doc = "SYNDROME"]
    pub mod SYNDROME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 4 Correctable Error Count Register"]
pub mod CORR_ERR_CNT4 {
    #[doc = "Memory n Correctable Error Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 5 Error Address Register"]
pub mod EAR5 {
    #[doc = "EAR"]
    pub mod EAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 5 Syndrome Register"]
pub mod SYN5 {
    #[doc = "SYNDROME"]
    pub mod SYNDROME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 5 Correctable Error Count Register"]
pub mod CORR_ERR_CNT5 {
    #[doc = "Memory n Correctable Error Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 6 Error Address Register"]
pub mod EAR6 {
    #[doc = "EAR"]
    pub mod EAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 6 Syndrome Register"]
pub mod SYN6 {
    #[doc = "SYNDROME"]
    pub mod SYNDROME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 6 Correctable Error Count Register"]
pub mod CORR_ERR_CNT6 {
    #[doc = "Memory n Correctable Error Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 7 Correctable Error Count Register"]
pub mod CORR_ERR_CNT7 {
    #[doc = "Memory n Correctable Error Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 8 Syndrome Register"]
pub mod SYN8 {
    #[doc = "SYNDROME"]
    pub mod SYNDROME {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 8 Correctable Error Count Register"]
pub mod CORR_ERR_CNT8 {
    #[doc = "Memory n Correctable Error Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ERM Memory 9 Correctable Error Count Register"]
pub mod CORR_ERR_CNT9 {
    #[doc = "Memory n Correctable Error Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
