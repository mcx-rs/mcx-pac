#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "PUF Key Context Management"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x100],
    #[doc = "PUF command blocking configuration"]
    pub CONFIG: crate::RWRegister<u32>,
    #[doc = "Security level lock"]
    pub SEC_LOCK: crate::RWRegister<u32>,
    #[doc = "Application defined context mask"]
    pub APP_CTX_MASK: crate::RWRegister<u32>,
}
#[doc = "PUF command blocking configuration"]
pub mod CONFIG {
    #[doc = "Disable PUF enroll command"]
    pub mod DIS_PUF_ENROLL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Command disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Disable PUF start command"]
    pub mod DIS_PUF_START {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Command disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Disable PUF stop command"]
    pub mod DIS_PUF_STOP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Command disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Disable PUF get key command"]
    pub mod DIS_PUF_GET_KEY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Command disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Disable PUF unwrap key command"]
    pub mod DIS_PUF_UNWRAP_KEY {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Command disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Disable PUF generate and wrap key command"]
    pub mod DIS_PUF_GEN_WRAP_KEY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Command disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Disable PUF wrap key command"]
    pub mod DIS_PUF_WRAP_KEY {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Command disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Disable PUF generate and wrap key command"]
    pub mod DIS_PUF_GEN_RANDOM_NUMBER {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Command disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Disable PUF test command"]
    pub mod DIS_PUF_TEST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Command disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
}
#[doc = "Security level lock"]
pub mod SEC_LOCK {
    #[doc = "Security Level"]
    pub mod SEC_LEVEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Non-secure and non-privileged Master"]
            pub const NONSECURE_NONPRIV_MASTER: u32 = 0;
            #[doc = "Non-secure and privileged Master"]
            pub const NONSECURE_PRIV_MASTER: u32 = 1;
            #[doc = "Secure and non-privileged Master"]
            pub const SECURE_NONPRIV_MASTER: u32 = 2;
            #[doc = "Secure and privileged Master"]
            pub const SECURE_PRIV_MASTER: u32 = 3;
        }
    }
    #[doc = "Anti-pole of security level"]
    pub mod ANTI_POLE_SEC_LEVEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure and privileged Master"]
            pub const NONSECURE_NONPRIV_MASTER: u32 = 0;
            #[doc = "Secure and non-privileged Master"]
            pub const NONSECURE_PRIV_MASTER: u32 = 1;
            #[doc = "Non-secure and privileged Master"]
            pub const SECURE_NONPRIV_MASTER: u32 = 2;
            #[doc = "Non-secure and non-privileged Master"]
            pub const SECURE_PRIV_MASTER: u32 = 3;
        }
    }
    #[doc = "Pattern"]
    pub mod PATTERN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Application defined context mask"]
pub mod APP_CTX_MASK {
    #[doc = "Application defined context"]
    pub mod APP_CTX_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
