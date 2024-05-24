#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "FMC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "NPX Control Register"]
    pub NPXCR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "NPX Status Register"]
    pub NPXSR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "Flash Cache Obfuscation Mask"]
    pub CACMSK: crate::RWRegister<u32>,
    _reserved2: [u8; 0xc],
    #[doc = "Data Remap"]
    pub REMAP: crate::RWRegister<u32>,
    _reserved3: [u8; 0x1c],
    #[doc = "no description available"]
    pub CTX_VALID_IV_ARRAY: [ctx_valid_iv_array::RegisterBlock; 4usize],
}
#[doc = "NPX Control Register"]
pub mod NPXCR {
    #[doc = "Global Encryption Enable"]
    pub mod GEE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Global encryption disabled. NPX on-the-fly encryption is disabled. Subsequent reads return 0."]
            pub const ENCRYPTION_DISABLED: u32 = 0;
            #[doc = "Global encryption enabled. NPX on-the-fly encryption is enabled if the flash access hits in a valid memory context. Subsequent reads return 1."]
            pub const ENCRYPTION_ENABLED: u32 = 1;
        }
    }
    #[doc = "Global Decryption Enable"]
    pub mod GDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Global decryption disabled. NPX on-the-fly decryption is globally disabled. Subsequent reads return 0."]
            pub const DECRYPTION_DISABLED: u32 = 0;
            #[doc = "Global decryption enabled. NPX on-the-fly decryption is globally enabled. Subsequent reads return 1."]
            pub const DECRYPTION_ENABLED: u32 = 1;
        }
    }
    #[doc = "Global Lock Enable"]
    pub mod GLK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock disabled. Subsequent reads return 0."]
            pub const LOCK_DISABLED: u32 = 0;
            #[doc = "Lock enabled: cannot write to VMAPCTXn, NPXCR, or CACMSK. Subsequent reads return 1."]
            pub const LOCK_ENABLED: u32 = 1;
        }
    }
    #[doc = "Mask Lock Enable"]
    pub mod MLK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock disabled. Subsequent reads return 0."]
            pub const LOCK_DISABLED: u32 = 0;
            #[doc = "Lock enabled: cannot write to mask. Subsequent reads return 1."]
            pub const LOCK_ENABLED: u32 = 1;
        }
    }
    #[doc = "Lock Enable for Context 0"]
    pub mod CTX0LK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock disabled: VMAPCTX0 remains read-write"]
            pub const LOCK_DISABLED: u32 = 0;
            #[doc = "Lock enabled: cannot write to VMAPCTX0 (becomes read-only)"]
            pub const LOCK_ENABLED: u32 = 1;
        }
    }
    #[doc = "Lock Enable for Context 1"]
    pub mod CTX1LK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock disabled: VMAPCTX1 remains read-write"]
            pub const LOCK_DISABLED: u32 = 0;
            #[doc = "Lock enabled: cannot write to VMAPCTX1 (becomes read-only)"]
            pub const LOCK_ENABLED: u32 = 1;
        }
    }
    #[doc = "Lock Enable for Context 2"]
    pub mod CTX2LK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock disabled: VMAPCTX2 remains read-write"]
            pub const LOCK_DISABLED: u32 = 0;
            #[doc = "Lock enabled: cannot write to VMAPCTX2 (becomes read-only)"]
            pub const LOCK_ENABLED: u32 = 1;
        }
    }
    #[doc = "Lock Enable for Context 3"]
    pub mod CTX3LK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock disabled: VMAPCTX3 remains read-write"]
            pub const LOCK_DISABLED: u32 = 0;
            #[doc = "Lock enabled: cannot write to VMAPCTX3 (becomes read-only)"]
            pub const LOCK_ENABLED: u32 = 1;
        }
    }
}
#[doc = "NPX Status Register"]
pub mod NPXSR {
    #[doc = "Number of implemented memory contexts"]
    pub mod NUMCTX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "No (zero) implemented memory contexts"]
            pub const ZERO_CTX: u32 = 0;
            #[doc = "1 implemented memory contexts"]
            pub const ONE_CTX: u32 = 1;
            #[doc = "2 implemented memory contexts"]
            pub const TWO_CTX: u32 = 2;
            #[doc = "3 implemented memory contexts"]
            pub const THREE_CTX: u32 = 3;
            #[doc = "4 implemented memory contexts"]
            pub const FOUR_CTX: u32 = 4;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key n Valid"]
    pub mod V0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not valid"]
            pub const KEY_NOTVALID: u32 = 0;
            #[doc = "Valid"]
            pub const KEY_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key n Valid"]
    pub mod V1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not valid"]
            pub const KEY_NOTVALID: u32 = 0;
            #[doc = "Valid"]
            pub const KEY_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key n Valid"]
    pub mod V2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not valid"]
            pub const KEY_NOTVALID: u32 = 0;
            #[doc = "Valid"]
            pub const KEY_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key n Valid"]
    pub mod V3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not valid"]
            pub const KEY_NOTVALID: u32 = 0;
            #[doc = "Valid"]
            pub const KEY_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Cache Obfuscation Mask"]
pub mod CACMSK {
    #[doc = "Obfuscation Mask"]
    pub mod OBMASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Remap"]
pub mod REMAP {
    #[doc = "Remap Lock Enable"]
    pub mod REMAPLK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock disabled: can write to REMAP"]
            pub const LOCK_DISABLED: u32 = 0;
            #[doc = "Lock enabled: cannot write to REMAP"]
            pub const LOCK_ENABLED: u32 = 1;
        }
    }
    #[doc = "LIM data"]
    pub mod LIM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LIM_DP data"]
    pub mod LIMDP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod ctx_valid_iv_array {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Bitmap of Valid Control for Memory Context n"]
        pub VMAPCTX_WD: [crate::RWRegister<u32>; 2usize],
        #[doc = "Block Initial Vector for Memory Context n"]
        pub BIVCTX_WD: [crate::RWRegister<u32>; 2usize],
    }
    #[doc = "Bitmap of Valid Control for Memory Context n"]
    pub mod VMAPCTX_WD {
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL0 {
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL1 {
            pub const offset: u32 = 1;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL2 {
            pub const offset: u32 = 2;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL3 {
            pub const offset: u32 = 3;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL4 {
            pub const offset: u32 = 4;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL5 {
            pub const offset: u32 = 5;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL6 {
            pub const offset: u32 = 6;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL7 {
            pub const offset: u32 = 7;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL8 {
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL9 {
            pub const offset: u32 = 9;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL10 {
            pub const offset: u32 = 10;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL11 {
            pub const offset: u32 = 11;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL12 {
            pub const offset: u32 = 12;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL13 {
            pub const offset: u32 = 13;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL14 {
            pub const offset: u32 = 14;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL15 {
            pub const offset: u32 = 15;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL16 {
            pub const offset: u32 = 16;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL17 {
            pub const offset: u32 = 17;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL18 {
            pub const offset: u32 = 18;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL19 {
            pub const offset: u32 = 19;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL20 {
            pub const offset: u32 = 20;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL21 {
            pub const offset: u32 = 21;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL22 {
            pub const offset: u32 = 22;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL23 {
            pub const offset: u32 = 23;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL24 {
            pub const offset: u32 = 24;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL25 {
            pub const offset: u32 = 25;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL26 {
            pub const offset: u32 = 26;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL27 {
            pub const offset: u32 = 27;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL28 {
            pub const offset: u32 = 28;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL29 {
            pub const offset: u32 = 29;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL30 {
            pub const offset: u32 = 30;
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
        #[doc = "Block valid enable for encryption/decryption"]
        pub mod VAL31 {
            pub const offset: u32 = 31;
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
    }
    #[doc = "Block Initial Vector for Memory Context n"]
    pub mod BIVCTX_WD {
        #[doc = "Block Initial Vector Word0"]
        pub mod BIV_WD0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
