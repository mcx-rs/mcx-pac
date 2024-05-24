#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Status Register"]
    pub STATUS: crate::RWRegister<u32>,
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Command Configuration Register"]
    pub CMDCFG0: crate::RWRegister<u32>,
    #[doc = "Configuration Register"]
    pub CFG: crate::RWRegister<u32>,
    #[doc = "Keystore Index 0 Register"]
    pub KIDX0: crate::RWRegister<u32>,
    #[doc = "Keystore Index 1 Register"]
    pub KIDX1: crate::RWRegister<u32>,
    #[doc = "Key Properties Request Register"]
    pub KPROPIN: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "DMA Source 0 Register"]
    pub DMA_SRC0: crate::RWRegister<u32>,
    #[doc = "DMA Source 0 Length Register"]
    pub DMA_SRC0_LEN: crate::RWRegister<u32>,
    #[doc = "DMA Source 1 Register"]
    pub DMA_SRC1: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "DMA Source 2 Register"]
    pub DMA_SRC2: crate::RWRegister<u32>,
    #[doc = "DMA Source 2 Length Register"]
    pub DMA_SRC2_LEN: crate::RWRegister<u32>,
    #[doc = "DMA Result 0 Register"]
    pub DMA_RES0: crate::RWRegister<u32>,
    #[doc = "DMA Result 0 Size Register"]
    pub DMA_RES0_LEN: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register"]
    pub INT_ENABLE: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Clear Register"]
    pub INT_STATUS_CLR: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Set Register"]
    pub INT_STATUS_SET: crate::RWRegister<u32>,
    #[doc = "Error Status Register"]
    pub ERR_STATUS: crate::RWRegister<u32>,
    #[doc = "Error Status Clear Register"]
    pub ERR_STATUS_CLR: crate::RWRegister<u32>,
    #[doc = "Version Register"]
    pub VERSION: crate::RWRegister<u32>,
    _reserved2: [u8; 0x4],
    #[doc = "PRNG SW Read Out Register"]
    pub PRNG_DATOUT: crate::RWRegister<u32>,
    #[doc = "CRC Configuration Register"]
    pub CMDCRC_CTRL: crate::RWRegister<u32>,
    #[doc = "Command CRC Value Register"]
    pub CMDCRC: crate::RWRegister<u32>,
    #[doc = "Session ID Register"]
    pub SESSION_ID: crate::RWRegister<u32>,
    _reserved3: [u8; 0x4],
    #[doc = "Final DMA Address Register"]
    pub DMA_FIN_ADDR: crate::RWRegister<u32>,
    #[doc = "Master ID Register"]
    pub MASTER_ID: crate::RWRegister<u32>,
    #[doc = "Keystore Index 2 Register"]
    pub KIDX2: crate::RWRegister<u32>,
    _reserved4: [u8; 0xd4],
    #[doc = "Status Register"]
    pub ELS_KS0: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS1: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS2: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS3: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS4: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS5: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS6: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS7: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS8: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS9: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS10: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS11: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS12: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS13: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS14: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS15: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS16: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS17: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS18: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub ELS_KS19: crate::RWRegister<u32>,
}
#[doc = "Status Register"]
pub mod STATUS {
    #[doc = "When set, indicates the ELS is executing a crypto sequence"]
    pub mod ELS_BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Crypto sequence not executing"]
            pub const NTCRY: u32 = 0;
            #[doc = "Crypto sequence executing"]
            pub const CRYP: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, indicates the ELS has an active interrupt"]
    pub mod ELS_IRQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No active interrupt"]
            pub const NTINT: u32 = 0;
            #[doc = "Active interrupt"]
            pub const INT: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, indicates the ELS has detected an internal error"]
    pub mod ELS_ERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Internal error not detected"]
            pub const NTERR: u32 = 0;
            #[doc = "Internal error detected"]
            pub const ERR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, indicates the internal PRNG is ready"]
    pub mod PRNG_RDY {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Internal PRNG not ready"]
            pub const NTREADY: u32 = 0;
            #[doc = "Internal PRNG ready"]
            pub const READY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Signature verify result status"]
    pub mod ECDSA_VFY_STATUS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "No verify run"]
            pub const NO_V_RUN: u32 = 0;
            #[doc = "Signature verify failed"]
            pub const SIG_FAIL: u32 = 1;
            #[doc = "Signature verify passed"]
            pub const SIG_PASS: u32 = 2;
            #[doc = "Invalid, Error"]
            pub const ERR: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current command privilege level"]
    pub mod PPROT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Secure, non-privileged"]
            pub const SECNP: u32 = 0;
            #[doc = "Secure, privileged"]
            pub const SECP: u32 = 1;
            #[doc = "Non-secure, non-privileged"]
            pub const NSECNP: u32 = 2;
            #[doc = "Non-secure, privileged"]
            pub const NSECP: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Entropy quality of the current DRBG instance"]
    pub mod DRBG_ENT_LVL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "NONE"]
            pub const NONE: u32 = 0;
            #[doc = "LOW, DRBG generates random numbers of low quality entropy"]
            pub const LOW: u32 = 1;
            #[doc = "HIGH, DRBG generates random numbers of high quality entropy"]
            pub const HIGH: u32 = 2;
            #[doc = "RFU, Reserved for Future Use"]
            pub const RFU: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, it indicates TRNG is gathering entropy"]
    pub mod DTRNG_BUSY {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not gathering entropy"]
            pub const NOTENT: u32 = 0;
            #[doc = "Gathering entropy"]
            pub const ENT: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, indicates that ELS is locked by a master"]
    pub mod ELS_LOCKED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not locked by master"]
            pub const NOTL: u32 = 0;
            #[doc = "Locked by master"]
            pub const LOCK: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "ELS enable"]
    pub mod ELS_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DIS: u32 = 0;
            #[doc = "Enabled"]
            pub const EN: u32 = 1;
        }
    }
    #[doc = "Write to 1 to start an ELS operation. Writing 0 has no effect."]
    pub mod ELS_START {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write to 1 to perform a ELS synchronous reset. Writing 0 has no effect."]
    pub mod ELS_RESET {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ELS Command ID"]
    pub mod ELS_CMD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Defines endianness"]
    pub mod BYTE_ORDER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Little endian"]
            pub const LIT: u32 = 0;
            #[doc = "Big endian"]
            pub const BIG: u32 = 1;
        }
    }
}
#[doc = "Command Configuration Register"]
pub mod CMDCFG0 {
    #[doc = "See"]
    pub mod CMDCFG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configuration Register"]
pub mod CFG {
    #[doc = "Maximum aes start delay"]
    pub mod ADCTRL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Keystore Index 0 Register"]
pub mod KIDX0 {
    #[doc = "Keystore is indexed as an array of 128 bit key slots"]
    pub mod KIDX0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Keystore Index 1 Register"]
pub mod KIDX1 {
    #[doc = "Keystore is indexed as an array of 128 bit key slots"]
    pub mod KIDX1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Key Properties Request Register"]
pub mod KPROPIN {
    #[doc = "For commands that create a key - requested properties of the key that is being created"]
    pub mod KPROPIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Source 0 Register"]
pub mod DMA_SRC0 {
    #[doc = "Defines the system address of the start of the data to be transferred to the ELS via DMA"]
    pub mod ADDR_SRC0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Source 0 Length Register"]
pub mod DMA_SRC0_LEN {
    #[doc = "Size in bytes of the data to be transferred from the target defined in SFR DMA_SRC0"]
    pub mod SIZE_SRC0_LEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Source 1 Register"]
pub mod DMA_SRC1 {
    #[doc = "Defines the system address of the start of the data to be transferred to the ELS via DMA"]
    pub mod ADDR_SRC1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Source 2 Register"]
pub mod DMA_SRC2 {
    #[doc = "Defines the system address of the start of the data to be transferred to the ELS via DMA"]
    pub mod ADDR_SRC2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Source 2 Length Register"]
pub mod DMA_SRC2_LEN {
    #[doc = "Size in bytes of the data to be transferred from the target defined in SFR DMA_SRC2"]
    pub mod SIZE_SRC2_LEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Result 0 Register"]
pub mod DMA_RES0 {
    #[doc = "Defines the system start address where the result of the ELS operation is transferred via DMA"]
    pub mod ADDR_RES0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Result 0 Size Register"]
pub mod DMA_RES0_LEN {
    #[doc = "Size in bytes of the data to be transferred"]
    pub mod SIZE_RES0_LEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable Register"]
pub mod INT_ENABLE {
    #[doc = "Interrupt enable bit"]
    pub mod INT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Clear Register"]
pub mod INT_STATUS_CLR {
    #[doc = "Interrupt status clear"]
    pub mod INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Set Register"]
pub mod INT_STATUS_SET {
    #[doc = "Sets interrupt by software"]
    pub mod INT_SET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Status Register"]
pub mod ERR_STATUS {
    #[doc = "Indicates bus access error: public or private bus"]
    pub mod BUS_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates operational error: ELS has been incorrectly operated"]
    pub mod OPN_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates algorithm error: an internal algorithm has produced an unexpected result"]
    pub mod ALG_ERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates data integrity error: internal data integrity check failed"]
    pub mod ITG_ERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates hardware fault error: attempt to change the value of an internal register"]
    pub mod FLT_ERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User read of PRNG_DATOUT when STATUS[PRNG_RDY] is 0"]
    pub mod PRNG_ERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the triggered error level: 0, 1 ,2"]
    pub mod ERR_LVL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRNG unable to gather entropy with the current configuration"]
    pub mod DTRNG_ERR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Status Clear Register"]
pub mod ERR_STATUS_CLR {
    #[doc = "no description available"]
    pub mod ERR_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Exits ELS error state"]
            pub const EXIT: u32 = 0;
            #[doc = "Clears ELS error status bits"]
            pub const CLR: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Version Register"]
pub mod VERSION {
    #[doc = "Extended release version digit1: possible values are from 0-9"]
    pub mod Z {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minor release version digit0: possible values are from 0-9"]
    pub mod Y2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minor release version digit1: possible values are from 0-9"]
    pub mod Y1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major release version: possible values are from 1-9"]
    pub mod X {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software extended revision version: possible values are from 0-9"]
    pub mod SW_Z {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software minor release version digit0: possible values are from 0-9"]
    pub mod SW_Y2 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software minor release version digit1: possible values are from 0-9"]
    pub mod SW_Y1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software major release version: possible values are from 1-9"]
    pub mod SW_X {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PRNG SW Read Out Register"]
pub mod PRNG_DATOUT {
    #[doc = "32-bit wide pseudo-random number"]
    pub mod PRNG_DATOUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CRC Configuration Register"]
pub mod CMDCRC_CTRL {
    #[doc = "CRC reset to initial value"]
    pub mod CMDCRC_RST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC enable bit"]
    pub mod CMDCRC_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Command CRC Value Register"]
pub mod CMDCRC {
    #[doc = "Current CRC value"]
    pub mod CMDCRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Session ID Register"]
pub mod SESSION_ID {
    #[doc = "Session ID"]
    pub mod SESSION_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Final DMA Address Register"]
pub mod DMA_FIN_ADDR {
    #[doc = "Final AHB address presented by ELS DMA to the system"]
    pub mod DMA_FIN_ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Master ID Register"]
pub mod MASTER_ID {
    #[doc = "High Priviledge Master ID"]
    pub mod MASTER_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Keystore Index 2 Register"]
pub mod KIDX2 {
    #[doc = "Keystore is indexed as an array of 128 bit key slots"]
    pub mod KIDX2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS0 {
    #[doc = "Key size"]
    pub mod KS0_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS0_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS0_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS0_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS0_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS0_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS0_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS0_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS0_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS0_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS0_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS0_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS0_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS0_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS0_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS0_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS0_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS0_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS0_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS0_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS0_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS0_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS0_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wrap key"]
    pub mod KS0_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS0_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS0_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS1 {
    #[doc = "Key size"]
    pub mod KS1_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS1_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS1_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS1_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS1_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS1_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS1_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS1_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS1_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS1_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS1_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS1_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS1_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS1_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS1_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS1_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS1_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS1_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS1_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS1_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS1_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS1_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS1_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS1_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS1_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS1_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS2 {
    #[doc = "Key size"]
    pub mod KS2_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS2_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS2_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS2_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS2_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS2_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS2_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS2_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS2_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS2_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS2_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS2_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS2_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS2_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS2_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS2_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS2_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS2_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS2_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS2_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS2_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS2_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS2_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS2_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS2_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS2_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS3 {
    #[doc = "Key size"]
    pub mod KS3_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS3_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS3_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS3_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS3_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS3_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS3_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS3_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS3_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS3_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS3_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS3_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS3_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS3_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS3_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS3_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS3_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS3_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS3_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS3_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS3_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS3_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS3_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS3_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS3_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS3_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS4 {
    #[doc = "Key size"]
    pub mod KS4_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS4_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS4_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS4_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS4_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS4_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS4_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS4_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS4_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS4_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS4_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS4_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS4_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS4_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS4_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS4_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS4_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS4_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS4_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS4_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS4_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS4_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS4_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS4_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS4_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS4_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS5 {
    #[doc = "Key size"]
    pub mod KS5_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS5_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS5_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS5_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS5_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS5_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS5_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS5_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS5_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS5_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS5_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS5_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS5_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS5_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS5_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS5_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS5_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS5_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS5_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS5_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS5_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS5_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS5_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS5_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS5_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS5_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS6 {
    #[doc = "Key size"]
    pub mod KS6_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS6_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS6_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS6_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS6_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS6_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS6_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS6_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS6_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS6_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS6_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS6_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS6_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS6_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS6_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS6_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS6_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS6_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS6_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS6_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS6_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS6_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS6_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS6_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS6_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS6_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS7 {
    #[doc = "Key size"]
    pub mod KS7_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS7_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS7_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS7_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS7_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS7_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS7_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS7_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS7_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS7_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS7_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS7_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS7_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS7_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS7_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS7_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS7_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS7_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS7_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS7_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS7_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS7_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS7_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS7_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS7_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS7_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS8 {
    #[doc = "Key size"]
    pub mod KS8_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS8_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS8_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS8_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS8_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS8_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS8_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS8_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS8_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS8_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS8_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS8_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS8_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS8_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS8_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS8_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS8_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS8_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS8_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS8_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS8_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS8_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS8_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS8_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS8_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS8_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS9 {
    #[doc = "Key size"]
    pub mod KS9_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS9_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS9_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS9_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS9_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS9_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS9_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS9_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS9_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS9_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS9_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS9_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS9_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS9_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS9_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS9_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS9_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS9_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS9_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS9_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS9_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS9_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS9_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS9_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS9_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS9_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS10 {
    #[doc = "Key size"]
    pub mod KS10_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS10_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS10_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS10_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS10_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS10_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS10_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS10_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS10_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS10_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS10_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS10_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS10_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS10_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS10_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS10_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS10_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS10_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS10_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS10_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS10_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS10_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS10_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS10_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS10_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS10_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS11 {
    #[doc = "Key size"]
    pub mod KS11_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS11_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS11_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS11_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS11_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS11_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS11_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS11_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS11_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS11_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS11_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS11_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS11_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS11_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS11_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS11_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS11_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS11_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS11_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS11_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS11_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS11_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS11_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS11_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS11_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS11_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS12 {
    #[doc = "Key size"]
    pub mod KS12_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS12_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS12_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS12_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS12_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS12_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS12_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS12_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS12_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS12_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS12_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS12_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS12_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS12_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS12_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS12_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS12_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS12_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS12_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS12_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS12_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS12_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS12_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS12_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS12_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS12_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS13 {
    #[doc = "Key size"]
    pub mod KS13_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS13_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS13_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS13_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS13_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS13_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS13_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS13_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS13_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS13_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS13_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS13_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS13_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS13_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS13_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS13_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS13_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS13_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS13_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS13_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS13_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS13_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS13_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS13_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS13_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS13_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS14 {
    #[doc = "Key size"]
    pub mod KS14_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS14_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS14_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS14_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS14_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS14_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS14_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS14_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS14_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS14_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS14_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS14_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS14_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS14_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS14_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS14_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS14_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS14_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS14_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS14_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS14_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS14_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS14_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS14_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS14_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS14_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS15 {
    #[doc = "Key size"]
    pub mod KS15_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS15_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS15_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS15_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS15_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS15_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS15_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS15_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS15_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS15_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS15_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS15_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS15_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS15_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS15_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS15_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS15_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS15_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS15_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS15_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS15_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS15_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS15_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS15_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS15_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS15_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS16 {
    #[doc = "Key size"]
    pub mod KS16_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS16_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS16_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS16_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS16_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS16_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS16_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS16_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS16_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS16_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS16_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS16_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS16_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS16_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS16_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS16_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS16_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS16_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS16_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS16_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS16_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS16_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS16_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS16_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS16_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS16_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS17 {
    #[doc = "Key size"]
    pub mod KS17_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS17_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS17_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS17_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS17_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS17_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS17_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS17_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS17_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS17_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS17_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS17_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS17_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS17_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS17_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS17_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS17_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS17_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS17_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS17_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS17_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS17_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS17_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS17_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS17_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS17_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS18 {
    #[doc = "Key size"]
    pub mod KS18_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS18_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS18_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS18_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS18_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS18_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS18_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS18_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS18_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS18_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS18_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS18_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS18_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS18_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS18_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS18_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS18_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS18_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS18_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS18_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS18_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS18_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS18_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS18_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS18_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS18_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod ELS_KS19 {
    #[doc = "Key size"]
    pub mod KS19_KSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "128"]
            pub const SIZE128: u32 = 0;
            #[doc = "256"]
            pub const SIZE256: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key is active"]
    pub mod KS19_KACT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "First slot in a multislot key"]
    pub mod KS19_KBASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature General Purpose"]
    pub mod KS19_FGP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Retention"]
    pub mod KS19_FRTN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware Feature Output"]
    pub mod KS19_FHWO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS19_UKPUK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS19_UTECDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CMAC key"]
    pub mod KS19_UCMAC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "KSK key"]
    pub mod KS19_UKSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Real Time Fingerprint key"]
    pub mod KS19_URTF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for CKDF command"]
    pub mod KS19_UCKDF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Derivation key for HKDF command"]
    pub mod KS19_UHKDF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc signing key"]
    pub mod KS19_UECSG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ecc diffie hellman key"]
    pub mod KS19_UECDH {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Aes key"]
    pub mod KS19_UAES {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hmac key"]
    pub mod KS19_UHMAC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key wrapping key"]
    pub mod KS19_UKWK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key unwrapping key"]
    pub mod KS19_UKUOK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Pre Master Secret"]
    pub mod KS19_UTLSPMS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TLS Master Secret"]
    pub mod KS19_UTLSMS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supply KEYGEN source"]
    pub mod KS19_UKGSRC {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hardware out key"]
    pub mod KS19_UHWO {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok to wrap key"]
    pub mod KS19_UWRPOK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Device Unique Key"]
    pub mod KS19_UDUK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priviledge level"]
    pub mod KS19_UPPROT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
