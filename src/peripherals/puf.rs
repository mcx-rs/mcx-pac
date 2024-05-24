#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "PUF"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control"]
    pub CR: crate::RWRegister<u32>,
    #[doc = "Operation Result"]
    pub ORR: crate::RWRegister<u32>,
    #[doc = "Status"]
    pub SR: crate::RWRegister<u32>,
    #[doc = "Allow"]
    pub AR: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub IER: crate::RWRegister<u32>,
    #[doc = "Interrupt Mask"]
    pub IMR: crate::RWRegister<u32>,
    #[doc = "Interrupt Status"]
    pub ISR: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "Data Destination"]
    pub DATA_DEST: crate::RWRegister<u32>,
    #[doc = "Data Source"]
    pub DATA_SRC: crate::RWRegister<u32>,
    _reserved1: [u8; 0x78],
    #[doc = "Data Input"]
    pub DIR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x4],
    #[doc = "Data Output"]
    pub DOR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x14],
    #[doc = "Miscellaneous"]
    pub MISC: crate::RWRegister<u32>,
    _reserved4: [u8; 0xc],
    #[doc = "Interface Status"]
    pub IF_SR: crate::RWRegister<u32>,
    _reserved5: [u8; 0x8],
    #[doc = "PUF Score"]
    pub PSR: crate::RWRegister<u32>,
    #[doc = "Hardware Restrict User Context 0"]
    pub HW_RUC0: crate::RWRegister<u32>,
    #[doc = "Hardware Restrict User Context 1"]
    pub HW_RUC1: crate::RWRegister<u32>,
    _reserved6: [u8; 0xc],
    #[doc = "Hardware Information"]
    pub HW_INFO: crate::RWRegister<u32>,
    #[doc = "Hardware Identifier"]
    pub HW_ID: crate::RWRegister<u32>,
    #[doc = "Hardware Version"]
    pub HW_VER: crate::RWRegister<u32>,
    _reserved7: [u8; 0x200],
    #[doc = "SRAM Configuration"]
    pub SRAM_CFG: crate::RWRegister<u32>,
    #[doc = "Status"]
    pub SRAM_STATUS: crate::RWRegister<u32>,
    _reserved8: [u8; 0xd0],
    #[doc = "Interrupt Enable Clear"]
    pub SRAM_INT_CLR_ENABLE: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Set"]
    pub SRAM_INT_SET_ENABLE: crate::RWRegister<u32>,
    #[doc = "Interrupt Status"]
    pub SRAM_INT_STATUS: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub SRAM_INT_ENABLE: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Clear"]
    pub SRAM_INT_CLR_STATUS: crate::RWRegister<u32>,
    #[doc = "Interrupt Status set"]
    pub SRAM_INT_SET_STATUS: crate::RWRegister<u32>,
}
#[doc = "Control"]
pub mod CR {
    #[doc = "Zeroize operation"]
    pub mod ZEROIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enroll operation"]
    pub mod ENROLL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start operation"]
    pub mod START {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reconstruct operation"]
    pub mod RECONSTRUCT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stop operation"]
    pub mod STOP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Get Key operation"]
    pub mod GET_KEY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unwrap operation"]
    pub mod UNWRAP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wrap Generated Random operation"]
    pub mod WRAP_GENERATED_RANDOM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wrap operation"]
    pub mod WRAP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Generate Random operation"]
    pub mod GENERATE_RANDOM {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test memory operation"]
    pub mod TEST_MEMORY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test PUF operation"]
    pub mod TEST_PUF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Operation Result"]
pub mod ORR {
    #[doc = "Result code of last operation"]
    pub mod RESULT_CODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "Indicates that the last operation was successful or operation is in progress."]
            pub const OK: u32 = 0;
            #[doc = "Indicates that the AC is not for the current product/version."]
            pub const ERR_PRODUCT: u32 = 240;
            #[doc = "Indicates that the AC in the second phase is not for the current product/version."]
            pub const ERR_PRODUCT_PH2: u32 = 241;
            #[doc = "Indicates that the AC is corrupted."]
            pub const ERR_TRANSFER: u32 = 242;
            #[doc = "Indicates that the AC in the second phase is corrupted."]
            pub const ERR_TRANSFER_PH2: u32 = 243;
            #[doc = "Indicates that the authentication of the provided AC failed."]
            pub const ERR_AUTH: u32 = 244;
            #[doc = "Indicates that the authentication of the provided AC failed in the second phase."]
            pub const ERR_AUTH_PH2: u32 = 245;
            #[doc = "Indicates that the SRAM PUF quality verification fails."]
            pub const ERR_PUF_QUALITY: u32 = 246;
            #[doc = "Indicates that the incorrect or unsupported context is provided."]
            pub const ERR_CONTEXT: u32 = 247;
            #[doc = "Indicates that a data destination was set that is not allowed according to other settings and the current PUF state."]
            pub const ERR_DESTINATION: u32 = 248;
            #[doc = "Indicates that the PUF SRAM access has failed."]
            pub const FAILURE: u32 = 255;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Last operation type"]
    pub mod LAST_OPERATION {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "Indicates that the operation is in progress."]
            pub const LO_PROGRESS: u32 = 0;
            #[doc = "Indicates that the last operation was Enroll."]
            pub const LO_ENROLL: u32 = 1;
            #[doc = "Indicates that the last operation was Start."]
            pub const LO_START: u32 = 2;
            #[doc = "Indicates that the last operation was Reconstruct"]
            pub const LO_RECONSTRUCT: u32 = 3;
            #[doc = "Indicates that the last operation was Stop."]
            pub const LO_STOP: u32 = 5;
            #[doc = "Indicates that the last operation was Get Key."]
            pub const LO_GET_KEY: u32 = 6;
            #[doc = "Indicates that the last operation was Unwrap."]
            pub const LO_UNWRAP: u32 = 7;
            #[doc = "Indicates that the last operation was Wrap Generated Random."]
            pub const LO_WRAP_GEN_RND: u32 = 8;
            #[doc = "Indicates that the last operation was Wrap."]
            pub const LO_WRAP: u32 = 9;
            #[doc = "Indicates that the last operation was Generate Random."]
            pub const LO_GEN_RND: u32 = 15;
            #[doc = "Indicates that the last operation was Test Memory."]
            pub const LO_TEST_MEMORY: u32 = 30;
            #[doc = "Indicates that the last operation was Test PUF."]
            pub const LO_TEST_PUF: u32 = 31;
            #[doc = "Indicates that the last operation was Initialization."]
            pub const LO_INITIALIZATION: u32 = 32;
            #[doc = "Indicates that the last operation was Zeroize."]
            pub const LO_ZEROIZE: u32 = 47;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status"]
pub mod SR {
    #[doc = "Operation in progress"]
    pub mod BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Last operation successful"]
    pub mod OK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Last operation failed"]
    pub mod ERROR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Zeroized or Locked state"]
    pub mod ZEROIZED {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Operation rejected"]
    pub mod REJECTED {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the request for data in transfer via the DIR register"]
    pub mod DI_REQUEST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the request for data out transfer via the DOR register"]
    pub mod DO_REQUEST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Allow"]
pub mod AR {
    #[doc = "Enroll operation"]
    pub mod ALLOW_ENROLL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Indicates that the Enroll operation is not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Indicates that the Enroll operation is allowed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start operation"]
    pub mod ALLOW_START {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Indicates that the Start operation is not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Indicates that the Start operation is allowed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reconstruct operation"]
    pub mod ALLOW_RECONSTRUCT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Indicates that the Reconstruct operation is not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Indicates that the Reconstruct operation is allowed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stop operation"]
    pub mod ALLOW_STOP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Indicates that the Stop operation is not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Indicates that the Stop operation is allowed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Get Key operation"]
    pub mod ALLOW_GET_KEY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Indicates that the Get Key operation is not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Indicates that the Get Key operation is allowed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unwrap operation"]
    pub mod ALLOW_UNWRAP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Indicates that the Unwrap operation is not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Indicates that the Unwrap operation is allowed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wrap Generated Random operation"]
    pub mod ALLOW_WRAP_GENERATED_RANDOM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Indicates that the Wrap Generated Random operation is not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Indicates that the Wrap Generated Random operation is allowed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wrap operation"]
    pub mod ALLOW_WRAP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Indicates that the Wrap operation is not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Indicates that the Wrap operation is allowed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Generate Random operation"]
    pub mod ALLOW_GENERATE_RANDOM {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Indicates that the Generate Random operation is not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Indicates that the Generate Random operation is allowed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "no description available"]
    pub mod ALLOW_TEST_MEMORY {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Indicates that the Test Memory operation is not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Indicates that the Test Memory operation is allowed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test PUF operation"]
    pub mod ALLOW_TEST_PUF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Test PUF operation is not allowed"]
            pub const DISABLE: u32 = 0;
            #[doc = "Test PUF operation is allowed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable"]
pub mod IER {
    #[doc = "Interrupt enable"]
    pub mod INT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables all PUF interrupts"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables all PUF interrupts that are enabled in the Interrupt Mask register"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Interrupt Mask"]
pub mod IMR {
    #[doc = "Busy interrupt"]
    pub mod INT_EN_BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ok interrupt"]
    pub mod INT_EN_OK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error interrupt"]
    pub mod INT_EN_ERROR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Zeroized interrupt"]
    pub mod INT_EN_ZEROIZED {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rejected interrupt"]
    pub mod INT_EN_REJECTED {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data in request interrupt"]
    pub mod INT_EN_DI_REQUEST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data out request interrupt"]
    pub mod INT_EN_DO_REQUEST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status"]
pub mod ISR {
    #[doc = "Negative edge occurred on Busy"]
    pub mod INT_BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive edge occurred on Ok"]
    pub mod INT_OK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive edge occurred on Error"]
    pub mod INT_ERROR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive edge occurred on Zeroized"]
    pub mod INT_ZEROIZED {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive edge occurred on Rejected"]
    pub mod INT_REJECTED {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive edge occurred on di_request"]
    pub mod INT_DI_REQUEST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Positive edge occurred on do_request"]
    pub mod INT_DO_REQUEST {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Destination"]
pub mod DATA_DEST {
    #[doc = "Key available via the DOR register"]
    pub mod DEST_DOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key available to ELS"]
    pub mod DEST_SO {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Source"]
pub mod DATA_SRC {
    #[doc = "Data provided via the DIR register"]
    pub mod SRC_DIR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data provided via the SI interface"]
    pub mod SRC_SI {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Input"]
pub mod DIR {
    #[doc = "Input data"]
    pub mod DI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Output"]
pub mod DOR {
    #[doc = "Output data"]
    pub mod DO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Miscellaneous"]
pub mod MISC {
    #[doc = "Defines the endianness of data in DIR and DOR:"]
    pub mod DATA_ENDIANNESS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Little endian"]
            pub const DISABLE: u32 = 0;
            #[doc = "Big endian (default)"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Interface Status"]
pub mod IF_SR {
    #[doc = "APB error"]
    pub mod APB_ERROR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PUF Score"]
pub mod PSR {
    #[doc = "Provides the PUF score obtained during the last Test PUF, Enroll or Start operation."]
    pub mod PUF_SCORE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hardware Restrict User Context 0"]
pub mod HW_RUC0 {
    #[doc = "Life cycle state based restrictions"]
    pub mod LC_STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "OEM Develop"]
            pub const OEM_OPEN: u32 = 3;
            #[doc = "OEM Develop 2"]
            pub const OEM_SECURE_WORLD: u32 = 7;
            #[doc = "OEM In-field"]
            pub const OEM_CLOSED: u32 = 15;
            #[doc = "OEM Field return"]
            pub const OEM_FIELD_RETURN: u32 = 31;
            #[doc = "NXP Field Return/Failure Analysis"]
            pub const FIELD_RETURN_NXP: u32 = 63;
            #[doc = "In-field Locked"]
            pub const OEM_LOCKED: u32 = 207;
            #[doc = "Bricked"]
            pub const OEM_SHREDDED: u32 = 255;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Temporal boot state"]
    pub mod BOOT_STATE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable key access when debugger is attached to CPU0 after power-up"]
    pub mod CPU0_DEBUG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable key access when debugger is attached to COOLFLUX after power-up"]
    pub mod COOLFLUX_DEBUG {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DSP debug status."]
    pub mod dsp_debug {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Restrict the key access based on TrustZone security level"]
    pub mod ACCESS_LEVEL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hardware Restrict User Context 1"]
pub mod HW_RUC1 {
    #[doc = "Application customizable context"]
    pub mod APP_CTX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hardware Information"]
pub mod HW_INFO {
    #[doc = "Wrap configuration"]
    pub mod CONFIG_WRAP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Indicates that Wrap is not included"]
            pub const ENABLE: u32 = 0;
            #[doc = "Indicates that Wrap is included"]
            pub const DISABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PUF configuration"]
    pub mod CONFIG_TYPE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "Indicates that PUF configuration is Safe."]
            pub const SAFE: u32 = 1;
            #[doc = "Indicates that PUF configuration is Plus."]
            pub const PLUS: u32 = 2;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hardware Identifier"]
pub mod HW_ID {
    #[doc = "Provides the hardware identifier"]
    pub mod HW_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hardware Version"]
pub mod HW_VER {
    #[doc = "Provides the hardware version, patch part"]
    pub mod HW_REV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Provides the hardware version, minor part"]
    pub mod HW_VERSION_MINOR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Provides the hardware version, major part"]
    pub mod HW_VERSION_MAJOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRAM Configuration"]
pub mod SRAM_CFG {
    #[doc = "PUF SRAM Controller activation"]
    pub mod ENABLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "PUF SRAM Clock Gating control"]
    pub mod CKGATING {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Status"]
pub mod SRAM_STATUS {
    #[doc = "PUF SRAM Controller State"]
    pub mod READY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable Clear"]
pub mod SRAM_INT_CLR_ENABLE {
    #[doc = "READY Interrupt Enable clear"]
    pub mod READY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APB_ERR Interrupt Enable clear"]
    pub mod APB_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable Set"]
pub mod SRAM_INT_SET_ENABLE {
    #[doc = "READY Interrupt Enable set"]
    pub mod READY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APB_ERR Interrupt Enable set"]
    pub mod APB_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status"]
pub mod SRAM_INT_STATUS {
    #[doc = "READY Interrupt Status"]
    pub mod READY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APB_ERR Interrupt Status"]
    pub mod APB_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable"]
pub mod SRAM_INT_ENABLE {
    #[doc = "READY Interrupt Enable"]
    pub mod READY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APB_ERR Interrupt Enable"]
    pub mod SRAM_APB_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Clear"]
pub mod SRAM_INT_CLR_STATUS {
    #[doc = "READY Interrupt Status clear"]
    pub mod READY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APB_ERR Interrupt Status Clear"]
    pub mod APB_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Clears the APB_ERR bit field in register INT_STATUS. Automatically reset by the Hardware"]
            pub const ENABLE: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Interrupt Status set"]
pub mod SRAM_INT_SET_STATUS {
    #[doc = "READY Interrupt Status set"]
    pub mod READY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APB_ERR Interrupt Status Set"]
    pub mod APB_ERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Clears the APB_ERR bit field in register INT_STATUS. Automatically reset by the Hardware"]
            pub const ENABLE: u32 = 1;
        }
        pub mod RW {}
    }
}
