#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "EIM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Error Injection Module Configuration Register"]
    pub EIMCR: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Enable register"]
    pub EICHEN: crate::RWRegister<u32>,
    _reserved0: [u8; 0xf8],
    #[doc = "Error Injection Channel Descriptor 0, Word0"]
    pub EICHD0_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 0, Word1"]
    pub EICHD0_WORD1: crate::RWRegister<u32>,
    _reserved1: [u8; 0x38],
    #[doc = "Error Injection Channel Descriptor 1, Word0"]
    pub EICHD1_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 1, Word1"]
    pub EICHD1_WORD1: crate::RWRegister<u32>,
    _reserved2: [u8; 0x38],
    #[doc = "Error Injection Channel Descriptor 2, Word0"]
    pub EICHD2_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 2, Word1"]
    pub EICHD2_WORD1: crate::RWRegister<u32>,
    _reserved3: [u8; 0x38],
    #[doc = "Error Injection Channel Descriptor 3, Word0"]
    pub EICHD3_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 3, Word1"]
    pub EICHD3_WORD1: crate::RWRegister<u32>,
    _reserved4: [u8; 0x38],
    #[doc = "Error Injection Channel Descriptor 4, Word0"]
    pub EICHD4_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 4, Word1"]
    pub EICHD4_WORD1: crate::RWRegister<u32>,
    _reserved5: [u8; 0x38],
    #[doc = "Error Injection Channel Descriptor 5, Word0"]
    pub EICHD5_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 5, Word1"]
    pub EICHD5_WORD1: crate::RWRegister<u32>,
    _reserved6: [u8; 0x38],
    #[doc = "Error Injection Channel Descriptor 6, Word0"]
    pub EICHD6_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 6, Word1"]
    pub EICHD6_WORD1: crate::RWRegister<u32>,
    _reserved7: [u8; 0x38],
    #[doc = "Error Injection Channel Descriptor 7, Word0"]
    pub EICHD7_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 7, Word1"]
    pub EICHD7_WORD1: crate::RWRegister<u32>,
    _reserved8: [u8; 0x38],
    #[doc = "Error Injection Channel Descriptor 8, Word0"]
    pub EICHD8_WORD0: crate::RWRegister<u32>,
    #[doc = "Error Injection Channel Descriptor 8, Word1"]
    pub EICHD8_WORD1: crate::RWRegister<u32>,
}
#[doc = "Error Injection Module Configuration Register"]
pub mod EIMCR {
    #[doc = "Global Error Injection Enable"]
    pub mod GEIEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Error Injection Channel Enable register"]
pub mod EICHEN {
    #[doc = "Error Injection Channel 8 Enable"]
    pub mod EICH8EN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 8"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 8"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Error Injection Channel 7 Enable"]
    pub mod EICH7EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 7"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 7"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Error Injection Channel 6 Enable"]
    pub mod EICH6EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 6"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 6"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Error Injection Channel 5 Enable"]
    pub mod EICH5EN {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 5"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 5"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Error Injection Channel 4 Enable"]
    pub mod EICH4EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 4"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 4"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Error Injection Channel 3 Enable"]
    pub mod EICH3EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 3"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 3"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Error Injection Channel 2 Enable"]
    pub mod EICH2EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 2"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 2"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Error Injection Channel 1 Enable"]
    pub mod EICH1EN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 1"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 1"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Error Injection Channel 0 Enable"]
    pub mod EICH0EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error injection is disabled on Error Injection Channel 0"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error injection is enabled on Error Injection Channel 0"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Error Injection Channel Descriptor 0, Word0"]
pub mod EICHD0_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 0, Word1"]
pub mod EICHD0_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 1, Word0"]
pub mod EICHD1_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 1, Word1"]
pub mod EICHD1_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 2, Word0"]
pub mod EICHD2_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 2, Word1"]
pub mod EICHD2_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 3, Word0"]
pub mod EICHD3_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 3, Word1"]
pub mod EICHD3_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 4, Word0"]
pub mod EICHD4_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 4, Word1"]
pub mod EICHD4_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 5, Word0"]
pub mod EICHD5_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 5, Word1"]
pub mod EICHD5_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 6, Word0"]
pub mod EICHD6_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 6, Word1"]
pub mod EICHD6_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 7, Word0"]
pub mod EICHD7_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 7, Word1"]
pub mod EICHD7_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 8, Word0"]
pub mod EICHD8_WORD0 {
    #[doc = "Checkbit Mask"]
    pub mod CHKBIT_MASK {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Injection Channel Descriptor 8, Word1"]
pub mod EICHD8_WORD1 {
    #[doc = "Data Mask Bytes 0-3"]
    pub mod B0_3DATA_MASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
