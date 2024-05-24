#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "TRDC"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    #[doc = "MBC Global Configuration Register"]
    pub MBC0_MEM0_GLBCFG: crate::RWRegister<u32>,
    #[doc = "MBC Global Configuration Register"]
    pub MBC0_MEM1_GLBCFG: crate::RWRegister<u32>,
    #[doc = "MBC Global Configuration Register"]
    pub MBC0_MEM2_GLBCFG: crate::RWRegister<u32>,
    #[doc = "MBC Global Configuration Register"]
    pub MBC0_MEM3_GLBCFG: crate::RWRegister<u32>,
    #[doc = "MBC NonSecure Enable Block Index"]
    pub MBC0_NSE_BLK_INDEX: crate::RWRegister<u32>,
    #[doc = "MBC NonSecure Enable Block Set"]
    pub MBC0_NSE_BLK_SET: crate::RWRegister<u32>,
    #[doc = "MBC NonSecure Enable Block Clear"]
    pub MBC0_NSE_BLK_CLR: crate::RWRegister<u32>,
    #[doc = "MBC NonSecure Enable Block Clear All"]
    pub MBC0_NSE_BLK_CLR_ALL: crate::RWRegister<u32>,
    #[doc = "MBC Global Access Control"]
    pub MBC0_MEMN_GLBAC0: crate::RWRegister<u32>,
    #[doc = "MBC Global Access Control"]
    pub MBC0_MEMN_GLBAC1: crate::RWRegister<u32>,
    #[doc = "MBC Global Access Control"]
    pub MBC0_MEMN_GLBAC2: crate::RWRegister<u32>,
    #[doc = "MBC Global Access Control"]
    pub MBC0_MEMN_GLBAC3: crate::RWRegister<u32>,
    #[doc = "MBC Global Access Control"]
    pub MBC0_MEMN_GLBAC4: crate::RWRegister<u32>,
    #[doc = "MBC Global Access Control"]
    pub MBC0_MEMN_GLBAC5: crate::RWRegister<u32>,
    #[doc = "MBC Global Access Control"]
    pub MBC0_MEMN_GLBAC6: crate::RWRegister<u32>,
    #[doc = "MBC Global Access Control"]
    pub MBC0_MEMN_GLBAC7: crate::RWRegister<u32>,
    #[doc = "MBC Memory Block Configuration Word"]
    pub MBC0_DOM0_MEM0_BLK_CFG_W0: crate::RWRegister<u32>,
    #[doc = "MBC Memory Block Configuration Word"]
    pub MBC0_DOM0_MEM0_BLK_CFG_W1: crate::RWRegister<u32>,
    #[doc = "MBC Memory Block Configuration Word"]
    pub MBC0_DOM0_MEM0_BLK_CFG_W2: crate::RWRegister<u32>,
    #[doc = "MBC Memory Block Configuration Word"]
    pub MBC0_DOM0_MEM0_BLK_CFG_W3: crate::RWRegister<u32>,
    #[doc = "MBC Memory Block Configuration Word"]
    pub MBC0_DOM0_MEM0_BLK_CFG_W4: crate::RWRegister<u32>,
    #[doc = "MBC Memory Block Configuration Word"]
    pub MBC0_DOM0_MEM0_BLK_CFG_W5: crate::RWRegister<u32>,
    #[doc = "MBC Memory Block Configuration Word"]
    pub MBC0_DOM0_MEM0_BLK_CFG_W6: crate::RWRegister<u32>,
    #[doc = "MBC Memory Block Configuration Word"]
    pub MBC0_DOM0_MEM0_BLK_CFG_W7: crate::RWRegister<u32>,
    _reserved1: [u8; 0xe0],
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    pub MBC0_DOM0_MEM0_BLK_NSE_W0: crate::RWRegister<u32>,
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    pub MBC0_DOM0_MEM0_BLK_NSE_W1: crate::RWRegister<u32>,
    _reserved2: [u8; 0x38],
    #[doc = "MBC Memory Block Configuration Word"]
    pub MBC0_DOM0_MEM1_BLK_CFG_W0: crate::RWRegister<u32>,
    _reserved3: [u8; 0x1c],
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    pub MBC0_DOM0_MEM1_BLK_NSE_W0: crate::RWRegister<u32>,
    _reserved4: [u8; 0x4],
    #[doc = "MBC Memory Block Configuration Word"]
    pub MBC0_DOM0_MEM2_BLK_CFG_W0: crate::RWRegister<u32>,
    _reserved5: [u8; 0x1c],
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    pub MBC0_DOM0_MEM2_BLK_NSE_W0: crate::RWRegister<u32>,
}
#[doc = "MBC Global Configuration Register"]
pub mod MBC0_MEM0_GLBCFG {
    #[doc = "Number of blocks in this memory"]
    pub mod NBLKS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Log2 size per block"]
    pub mod SIZE_LOG2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MBC Global Configuration Register"]
pub mod MBC0_MEM1_GLBCFG {
    #[doc = "Number of blocks in this memory"]
    pub mod NBLKS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Log2 size per block"]
    pub mod SIZE_LOG2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MBC Global Configuration Register"]
pub mod MBC0_MEM2_GLBCFG {
    #[doc = "Number of blocks in this memory"]
    pub mod NBLKS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Log2 size per block"]
    pub mod SIZE_LOG2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MBC Global Configuration Register"]
pub mod MBC0_MEM3_GLBCFG {
    #[doc = "Number of blocks in this memory"]
    pub mod NBLKS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Log2 size per block"]
    pub mod SIZE_LOG2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear Error"]
    pub mod CLRE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MBC NonSecure Enable Block Index"]
pub mod MBC0_NSE_BLK_INDEX {
    #[doc = "Word index into the block NSE bitmap. It selects the BLK_NSE_Wn register, where WNDX determines the value of n."]
    pub mod WNDX {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory Select"]
    pub mod MEM_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DID Select"]
    pub mod DID_SEL0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const LOGIC_0: u32 = 0;
            #[doc = "Selects NSE bits for this domain."]
            pub const LOGIC_1: u32 = 1;
        }
    }
    #[doc = "Auto Increment"]
    pub mod AI {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const LOGIC_0: u32 = 0;
            #[doc = "Add 1 to the WNDX field after the register write."]
            pub const LOGIC_1: u32 = 1;
        }
    }
}
#[doc = "MBC NonSecure Enable Block Set"]
pub mod MBC0_NSE_BLK_SET {
    #[doc = "Write-1 Set"]
    pub mod W1SET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MBC NonSecure Enable Block Clear"]
pub mod MBC0_NSE_BLK_CLR {
    #[doc = "Write-1 Clear"]
    pub mod W1CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MBC NonSecure Enable Block Clear All"]
pub mod MBC0_NSE_BLK_CLR_ALL {
    #[doc = "Memory Select"]
    pub mod MEMSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DID Select"]
    pub mod DID_SEL0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const LOGIC_0: u32 = 0;
            #[doc = "Clear all NSE bits for this domain."]
            pub const LOGIC_1: u32 = 1;
        }
    }
}
#[doc = "MBC Global Access Control"]
pub mod MBC0_MEMN_GLBAC0 {
    #[doc = "NonsecureUser Execute"]
    pub mod NUX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Write"]
    pub mod NUW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Read"]
    pub mod NUR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Execute"]
    pub mod NPX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Write"]
    pub mod NPW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Read"]
    pub mod NPR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Execute"]
    pub mod SUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Write"]
    pub mod SUW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Read"]
    pub mod SUR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Execute"]
    pub mod SPX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Write"]
    pub mod SPW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Read"]
    pub mod SPR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Global Access Control"]
pub mod MBC0_MEMN_GLBAC1 {
    #[doc = "NonsecureUser Execute"]
    pub mod NUX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Write"]
    pub mod NUW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Read"]
    pub mod NUR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Execute"]
    pub mod NPX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Write"]
    pub mod NPW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Read"]
    pub mod NPR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Execute"]
    pub mod SUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Write"]
    pub mod SUW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Read"]
    pub mod SUR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Execute"]
    pub mod SPX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Write"]
    pub mod SPW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Read"]
    pub mod SPR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "LOCK"]
    pub mod LK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This register is not locked and can be altered."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "This register is locked and cannot be altered."]
            pub const LOCKED: u32 = 1;
        }
    }
}
#[doc = "MBC Global Access Control"]
pub mod MBC0_MEMN_GLBAC2 {
    #[doc = "NonsecureUser Execute"]
    pub mod NUX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Write"]
    pub mod NUW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Read"]
    pub mod NUR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Execute"]
    pub mod NPX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Write"]
    pub mod NPW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Read"]
    pub mod NPR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Execute"]
    pub mod SUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Write"]
    pub mod SUW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Read"]
    pub mod SUR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Execute"]
    pub mod SPX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Write"]
    pub mod SPW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Read"]
    pub mod SPR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "LOCK"]
    pub mod LK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This register is not locked and can be altered."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "This register is locked and cannot be altered."]
            pub const LOCKED: u32 = 1;
        }
    }
}
#[doc = "MBC Global Access Control"]
pub mod MBC0_MEMN_GLBAC3 {
    #[doc = "NonsecureUser Execute"]
    pub mod NUX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Write"]
    pub mod NUW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Read"]
    pub mod NUR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Execute"]
    pub mod NPX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Write"]
    pub mod NPW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Read"]
    pub mod NPR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Execute"]
    pub mod SUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Write"]
    pub mod SUW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Read"]
    pub mod SUR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Execute"]
    pub mod SPX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Write"]
    pub mod SPW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Read"]
    pub mod SPR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "LOCK"]
    pub mod LK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This register is not locked and can be altered."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "This register is locked and cannot be altered."]
            pub const LOCKED: u32 = 1;
        }
    }
}
#[doc = "MBC Global Access Control"]
pub mod MBC0_MEMN_GLBAC4 {
    #[doc = "NonsecureUser Execute"]
    pub mod NUX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Write"]
    pub mod NUW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Read"]
    pub mod NUR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Execute"]
    pub mod NPX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Write"]
    pub mod NPW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Read"]
    pub mod NPR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Execute"]
    pub mod SUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Write"]
    pub mod SUW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Read"]
    pub mod SUR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Execute"]
    pub mod SPX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Write"]
    pub mod SPW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Read"]
    pub mod SPR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "LOCK"]
    pub mod LK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This register is not locked and can be altered."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "This register is locked and cannot be altered."]
            pub const LOCKED: u32 = 1;
        }
    }
}
#[doc = "MBC Global Access Control"]
pub mod MBC0_MEMN_GLBAC5 {
    #[doc = "NonsecureUser Execute"]
    pub mod NUX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Write"]
    pub mod NUW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Read"]
    pub mod NUR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Execute"]
    pub mod NPX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Write"]
    pub mod NPW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Read"]
    pub mod NPR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Execute"]
    pub mod SUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Write"]
    pub mod SUW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Read"]
    pub mod SUR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Execute"]
    pub mod SPX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Write"]
    pub mod SPW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Read"]
    pub mod SPR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "LOCK"]
    pub mod LK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This register is not locked and can be altered."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "This register is locked and cannot be altered."]
            pub const LOCKED: u32 = 1;
        }
    }
}
#[doc = "MBC Global Access Control"]
pub mod MBC0_MEMN_GLBAC6 {
    #[doc = "NonsecureUser Execute"]
    pub mod NUX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Write"]
    pub mod NUW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Read"]
    pub mod NUR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Execute"]
    pub mod NPX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Write"]
    pub mod NPW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Read"]
    pub mod NPR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Execute"]
    pub mod SUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Write"]
    pub mod SUW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Read"]
    pub mod SUR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Execute"]
    pub mod SPX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Write"]
    pub mod SPW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Read"]
    pub mod SPR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "LOCK"]
    pub mod LK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This register is not locked and can be altered."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "This register is locked and cannot be altered."]
            pub const LOCKED: u32 = 1;
        }
    }
}
#[doc = "MBC Global Access Control"]
pub mod MBC0_MEMN_GLBAC7 {
    #[doc = "NonsecureUser Execute"]
    pub mod NUX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Write"]
    pub mod NUW {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecureUser Read"]
    pub mod NUR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Execute"]
    pub mod NPX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Write"]
    pub mod NPW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "NonsecurePriv Read"]
    pub mod NPR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Nonsecure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Nonsecure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Execute"]
    pub mod SUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Write"]
    pub mod SUW {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecureUser Read"]
    pub mod SUR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure User mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure User mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Execute"]
    pub mod SPX {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Execute access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Execute access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Write"]
    pub mod SPW {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Write access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "SecurePriv Read"]
    pub mod SPR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read access is not allowed in Secure Privilege mode."]
            pub const NOTALLOWED: u32 = 0;
            #[doc = "Read access is allowed in Secure Privilege mode."]
            pub const ALLOWED: u32 = 1;
        }
    }
    #[doc = "LOCK"]
    pub mod LK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This register is not locked and can be altered."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "This register is locked and cannot be altered."]
            pub const LOCKED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block Configuration Word"]
pub mod MBC0_DOM0_MEM0_BLK_CFG_W0 {
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE2 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE6 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE7 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block Configuration Word"]
pub mod MBC0_DOM0_MEM0_BLK_CFG_W1 {
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE2 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE6 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE7 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block Configuration Word"]
pub mod MBC0_DOM0_MEM0_BLK_CFG_W2 {
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE2 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE6 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE7 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block Configuration Word"]
pub mod MBC0_DOM0_MEM0_BLK_CFG_W3 {
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE2 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE6 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE7 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block Configuration Word"]
pub mod MBC0_DOM0_MEM0_BLK_CFG_W4 {
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE2 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE6 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE7 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block Configuration Word"]
pub mod MBC0_DOM0_MEM0_BLK_CFG_W5 {
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE2 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE6 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE7 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block Configuration Word"]
pub mod MBC0_DOM0_MEM0_BLK_CFG_W6 {
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE2 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE6 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE7 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block Configuration Word"]
pub mod MBC0_DOM0_MEM0_BLK_CFG_W7 {
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE2 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE6 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE7 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word"]
pub mod MBC0_DOM0_MEM0_BLK_NSE_W0 {
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word"]
pub mod MBC0_DOM0_MEM0_BLK_NSE_W1 {
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block Configuration Word"]
pub mod MBC0_DOM0_MEM1_BLK_CFG_W0 {
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE2 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE6 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE7 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word"]
pub mod MBC0_DOM0_MEM1_BLK_NSE_W0 {
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block Configuration Word"]
pub mod MBC0_DOM0_MEM2_BLK_CFG_W0 {
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE2 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE3 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL4 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE4 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL5 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE5 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL6 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE6 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Memory Block Access Control Select for block B"]
    pub mod MBACSEL7 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "select MBC_MEMN_GLBAC0 access control policy for block B"]
            pub const GLBAC0: u32 = 0;
            #[doc = "select MBC_MEMN_GLBAC1 access control policy for block B"]
            pub const GLBAC1: u32 = 1;
            #[doc = "select MBC_MEMN_GLBAC2 access control policy for block B"]
            pub const GLBAC2: u32 = 2;
            #[doc = "select MBC_MEMN_GLBAC3 access control policy for block B"]
            pub const GLBAC3: u32 = 3;
            #[doc = "select MBC_MEMN_GLBAC4 access control policy for block B"]
            pub const GLBAC4: u32 = 4;
            #[doc = "select MBC_MEMN_GLBAC5 access control policy for block B"]
            pub const GLBAC5: u32 = 5;
            #[doc = "select MBC_MEMN_GLBAC6 access control policy for block B"]
            pub const GLBAC6: u32 = 6;
            #[doc = "select MBC_MEMN_GLBAC7 access control policy for block B"]
            pub const GLBAC7: u32 = 7;
        }
    }
    #[doc = "NonSecure Enable for block B"]
    pub mod NSE7 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in this register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
#[doc = "MBC Memory Block NonSecure Enable Word"]
pub mod MBC0_DOM0_MEM2_BLK_NSE_W0 {
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
    #[doc = "Bit b NonSecure Enable [b = 0 - 31]"]
    pub mod BIT31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Secure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL]), nonsecure accesses to block B are not allowed."]
            pub const ALLOWED: u32 = 0;
            #[doc = "Secure accesses to block B are not allowed, nonsecure accesses to block B are based on corresponding MBACSEL field in register (MBCm_DOMd_MEMs_BLK_CFG_Ww[MBACSEL])."]
            pub const NOTALLOWED: u32 = 1;
        }
    }
}
