#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "TRNG0"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Miscellaneous Control Register"]
    pub MCTL: crate::RWRegister<u32>,
    #[doc = "Statistical Check Miscellaneous Register"]
    pub SCMISC: crate::RWRegister<u32>,
    _reserved0: [u8; 0x8],
    #[doc = "Seed Control Register"]
    pub SDCTL: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "Frequency Count Minimum Limit Register"]
    pub FRQMIN: crate::RWRegister<u32>,
    #[doc = "Oscillator-2 Frequency Count Register"]
    pub OSC2_FRQCNT: crate::RWRegister<u32>,
    #[doc = "Frequency Count Register"]
    pub FRQCNT: crate::RWRegister<u32>,
    #[doc = "Frequency Count Maximum Limit Register"]
    pub FRQMAX: crate::RWRegister<u32>,
    #[doc = "Statistical Check Monobit Count Register"]
    pub SCMC: crate::RWRegister<u32>,
    #[doc = "Statistical Check Monobit Limit Register"]
    pub SCML: crate::RWRegister<u32>,
    #[doc = "Statistical Check Run Length 1 Count Register"]
    pub SCR1C: crate::RWRegister<u32>,
    #[doc = "Statistical Check Run Length 1 Limit Register"]
    pub SCR1L: crate::RWRegister<u32>,
    #[doc = "Statistical Check Run Length 2 Count Register"]
    pub SCR2C: crate::RWRegister<u32>,
    #[doc = "Statistical Check Run Length 2 Limit Register"]
    pub SCR2L: crate::RWRegister<u32>,
    #[doc = "Statistical Check Run Length 3 Count Register"]
    pub SCR3C: crate::RWRegister<u32>,
    #[doc = "Statistical Check Run Length 3 Limit Register"]
    pub SCR3L: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub STATUS: crate::RWRegister<u32>,
    #[doc = "Entropy Read Register"]
    pub ENT: [crate::RWRegister<u32>; 8usize],
    _reserved2: [u8; 0x34],
    #[doc = "Security Configuration Register"]
    pub SEC_CFG: crate::RWRegister<u32>,
    #[doc = "Interrupt Control Register"]
    pub INT_CTRL: crate::RWRegister<u32>,
    #[doc = "Mask Register"]
    pub INT_MASK: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Register"]
    pub INT_STATUS: crate::RWRegister<u32>,
    #[doc = "Common Security Error Register"]
    pub CSER: crate::RWRegister<u32>,
    #[doc = "Common Security Clear Register"]
    pub CSCLR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x34],
    #[doc = "TRNG Oscillator 2 Control Register"]
    pub OSC2_CTL: crate::RWRegister<u32>,
    #[doc = "Version ID Register (MS)"]
    pub VID1: crate::RWRegister<u32>,
    #[doc = "Version ID Register (LS)"]
    pub VID2: crate::RWRegister<u32>,
}
#[doc = "Miscellaneous Control Register"]
pub mod MCTL {
    #[doc = "Oscillator1 Divide"]
    pub mod OSC_DIV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "use ring oscillator with no divide"]
            pub const OSC_DIV_DIV1: u32 = 0;
            #[doc = "use ring oscillator divided-by-2"]
            pub const OSC_DIV_DIV2: u32 = 1;
            #[doc = "use ring oscillator divided-by-4"]
            pub const OSC_DIV_DIV4: u32 = 2;
            #[doc = "use ring oscillator divided-by-8"]
            pub const OSC_DIV_DIV8: u32 = 3;
        }
    }
    #[doc = "Disable Self-Tests"]
    pub mod DIS_SLF_TST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset Defaults"]
    pub mod RST_DEF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No impact."]
            pub const DISABLE: u32 = 0;
            #[doc = "Writing a 1 to this bit clears various TRNG registers, and bits within registers, to their default state."]
            pub const ENABLE: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Frequency Count Fail"]
    pub mod FCT_FAIL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frequency Count Valid"]
    pub mod FCT_VAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Frequency Count is not valid"]
            pub const DISABLE: u32 = 0;
            #[doc = "Frequency Count is valid"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Entropy Valid"]
    pub mod ENT_VAL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Entropy is not valid"]
            pub const DISABLE: u32 = 0;
            #[doc = "Entropy is valid"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Status"]
    pub mod ERR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const DISABLE: u32 = 0;
            #[doc = "Error detected"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "TRNG is ok to stop"]
    pub mod TSTOP_OK {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "TRNG is generating entropy and is not ok to stop"]
            pub const DISABLE: u32 = 0;
            #[doc = "TRNG is not generating entropy and is ok to stop"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Long run count continues between entropy generations"]
    pub mod LRUN_CONT {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The internal test\'s long run count is restarted for each entropy re-generation."]
            pub const DISABLE: u32 = 0;
            #[doc = "The count value (of TRNG\'s internal long_run test), at the start of an entropy generation will continue where it left off at the end of the previous entropy generation."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Oscillator 2 Failure"]
    pub mod OSC2_FAIL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Oscillator 2 is running."]
            pub const DISABLE: u32 = 0;
            #[doc = "Oscillator 2 has failed (see OSC2_CTL[OSC_FAILSAFE_LMT])."]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Program Mode"]
    pub mod PRGM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRNG is in Run Mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "TRNG is in Program Mode"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Statistical Check Miscellaneous Register"]
pub mod SCMISC {
    #[doc = "Long run max limit"]
    pub mod LRUN_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Retry count"]
    pub mod RTY_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Seed Control Register"]
pub mod SDCTL {
    #[doc = "Sample Size"]
    pub mod SAMP_SIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Entropy Delay"]
    pub mod ENT_DLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frequency Count Minimum Limit Register"]
pub mod FRQMIN {
    #[doc = "Frequency Count Minimum Limit"]
    pub mod FRQ_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Oscillator-2 Frequency Count Register"]
pub mod OSC2_FRQCNT {
    #[doc = "Frequency Count"]
    pub mod OSC2_FRQ_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frequency Count Register"]
pub mod FRQCNT {
    #[doc = "Frequency Count"]
    pub mod FRQ_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frequency Count Maximum Limit Register"]
pub mod FRQMAX {
    #[doc = "Frequency Counter Maximum Limit"]
    pub mod FRQ_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Monobit Count Register"]
pub mod SCMC {
    #[doc = "Monobit Count"]
    pub mod MONO_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Monobit Limit Register"]
pub mod SCML {
    #[doc = "Monobit Maximum Limit"]
    pub mod MONO_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Monobit Range"]
    pub mod MONO_RNG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Run Length 1 Count Register"]
pub mod SCR1C {
    #[doc = "Runs of Zero, Length 1 Count"]
    pub mod R1_0_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Runs of One, Length 1 Count"]
    pub mod R1_1_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Run Length 1 Limit Register"]
pub mod SCR1L {
    #[doc = "Run Length 1 Maximum Limit"]
    pub mod RUN1_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Run Length 1 Range"]
    pub mod RUN1_RNG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Run Length 2 Count Register"]
pub mod SCR2C {
    #[doc = "Runs of Zero, Length 2 Count"]
    pub mod R2_0_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Runs of One, Length 2 Count"]
    pub mod R2_1_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Run Length 2 Limit Register"]
pub mod SCR2L {
    #[doc = "Run Length 2 Maximum Limit"]
    pub mod RUN2_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Run Length 2 Range"]
    pub mod RUN2_RNG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Run Length 3 Count Register"]
pub mod SCR3C {
    #[doc = "Runs of Zeroes, Length 3 Count"]
    pub mod R3_0_CT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Runs of Ones, Length 3 Count"]
    pub mod R3_1_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Statistical Check Run Length 3 Limit Register"]
pub mod SCR3L {
    #[doc = "Run Length 3 Maximum Limit"]
    pub mod RUN3_MAX {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Run Length 3 Range"]
    pub mod RUN3_RNG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod STATUS {
    #[doc = "Test Fail, 1-Bit Run, Sampling 0s."]
    pub mod TF1BR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The 1-Bit Run, Sampling 0s Test has passed"]
            pub const DISABLE: u32 = 0;
            #[doc = "The 1-Bit Run, Sampling 0s Test has failed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 1-Bit Run, Sampling 1s."]
    pub mod TF1BR1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The 1-Bit Run, Sampling 1s Test has passed"]
            pub const DISABLE: u32 = 0;
            #[doc = "The 1-Bit Run, Sampling 1s Test has failed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 0s."]
    pub mod TF2BR0 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The 2-Bit Run, Sampling 0s Test has passed"]
            pub const DISABLE: u32 = 0;
            #[doc = "The 2-Bit Run, Sampling 0s Test has failed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 1s."]
    pub mod TF2BR1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The 2-Bit Run, Sampling 1s Test has passed"]
            pub const DISABLE: u32 = 0;
            #[doc = "The 2-Bit Run, Sampling 1s Test has failed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, 3-Bit Run, Sampling 0s."]
    pub mod TF3BR0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The 3-Bit Run, Sampling 0s Test has passed"]
            pub const DISABLE: u32 = 0;
            #[doc = "The 3-Bit Run, Sampling 0s Test has failed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail"]
    pub mod TF3BR1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The 3-Bit Run, Sampling 1s Test has passed"]
            pub const DISABLE: u32 = 0;
            #[doc = "The 3-Bit Run, Sampling 1s Test has failed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, Long Run."]
    pub mod TFLR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The Long Run Test has passed"]
            pub const DISABLE: u32 = 0;
            #[doc = "The Long Run Test has failed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Test Fail, Mono Bit."]
    pub mod TFMB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The Mono Bit Test has passed"]
            pub const DISABLE: u32 = 0;
            #[doc = "The Mono Bit Test has failed"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RETRY COUNT"]
    pub mod RETRY_CT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Entropy Read Register"]
pub mod ENT {
    #[doc = "Entropy Value"]
    pub mod ENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Security Configuration Register"]
pub mod SEC_CFG {
    #[doc = "If set, below mentioned TRNG configuration registers cannot be programmed: Oscillator 2 Control Register (OSC2_CTL): TRNG Entropy Generation Control [1:0] Oscillator 2 Divider [3:2] Oscillator Fail Safe Limit [13:12] Oscillator Fail Safe Test [14] TRNG Seed Control Register (SDCTL) TRNG Frequency Count Minimum Limit Register (FRQMIN) TRNG Frequency Count Maximum Limit Register (FRQMAX) TRNG Statistical Check Monobit Limit Register (SCML) TRNG Statistical Check Run Length 1 Limit Register (SCR1L) TRNG Statistical Check Run Length 2 Limit Register (SCR2L) TRNG Statistical Check Run Length 3 Limit Register (SCR3L) TRNG Miscellaneous Control Register (MCTL): Sample Mode [1:0] Oscillator Divider [3:2] Reset Defaults [6] Force System Clock [7] Long Runs Continuation Mode [14] After this bit has been written to a 1, it cannot be changed"]
    pub mod NO_PRGM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRNG configuration registers can be modified."]
            pub const NO_PRGM_OFF: u32 = 0;
            #[doc = "TRNG configuration registers cannot be modified."]
            pub const NO_PRGM_ON: u32 = 1;
        }
    }
}
#[doc = "Interrupt Control Register"]
pub mod INT_CTRL {
    #[doc = "Clear the HW_ERR interrupt."]
    pub mod HW_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clears the INT_STATUS[HW_ERR] bit. Will automatically set after writing."]
            pub const HW_ERR_CLEAR: u32 = 0;
            #[doc = "Enables the INT_STATUS[HW_ERR] bit to be set, thereby enabling interrupt generation for the HW_ERR condition."]
            pub const HW_ERR_ON: u32 = 1;
        }
    }
    #[doc = "Clear the ENT_VAL interrupt."]
    pub mod ENT_VAL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clears the INT_STATUS[ENT_VAL] bit. Will automatically set after writing."]
            pub const ENT_VAL_CLEAR: u32 = 0;
            #[doc = "Enables the INT_STATUS[ENT_VAL] bit to be set, thereby enabling interrupt generation for the ENT_VAL condition."]
            pub const ENT_VAL_ON: u32 = 1;
        }
    }
    #[doc = "Clear the FRQ_CT_FAIL interrupt."]
    pub mod FRQ_CT_FAIL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clears the INT_STATUS[FRQ_CT_FAIL] bit. Will automatically set after writing."]
            pub const FRQ_CT_FAIL_CLEAR: u32 = 0;
            #[doc = "Enables the INT_STATUS[FRQ_CT_FAIL] bit to be set, thereby enabling interrupt generation for the FRQ_CT_FAIL condition."]
            pub const FRQ_CT_FAIL_ON: u32 = 1;
        }
    }
}
#[doc = "Mask Register"]
pub mod INT_MASK {
    #[doc = "Mask the HW_ERR interrupt."]
    pub mod HW_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HW_ERR interrupt is disabled."]
            pub const HW_ERR_MASKED: u32 = 0;
            #[doc = "HW_ERR interrupt is enabled."]
            pub const HW_ERR_ACTIVE: u32 = 1;
        }
    }
    #[doc = "Mask the ENT_VAL interrupt."]
    pub mod ENT_VAL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ENT_VAL interrupt is disabled."]
            pub const ENT_VAL_MASKED: u32 = 0;
            #[doc = "ENT_VAL interrupt is enabled."]
            pub const ENT_VAL_ACTIVE: u32 = 1;
        }
    }
    #[doc = "Mask the FRQ_CT_FAIL interrupt."]
    pub mod FRQ_CT_FAIL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FRQ_CT_FAIL interrupt is disabled."]
            pub const FRQ_CT_FAIL_MASKED: u32 = 0;
            #[doc = "FRQ_CT_FAIL interrupt is enabled."]
            pub const FRQ_CT_FAIL_ACTIVE: u32 = 1;
        }
    }
}
#[doc = "Interrupt Status Register"]
pub mod INT_STATUS {
    #[doc = "Read: TRNG Error. Any error in the TRNG will trigger this interrupt."]
    pub mod HW_ERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error."]
            pub const HW_ERR_NO: u32 = 0;
            #[doc = "Error detected."]
            pub const HW_ERR_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Entropy Valid"]
    pub mod ENT_VAL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Busy generating entropy. Any value read from the Entropy registers is invalid."]
            pub const ENT_VAL_INVALID: u32 = 0;
            #[doc = "Values read from the Entropy registers are valid."]
            pub const ENT_VAL_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frequency Count Fail"]
    pub mod FRQ_CT_FAIL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No hardware nor self test frequency errors."]
            pub const FRQ_CT_FAIL_NO_ERR: u32 = 0;
            #[doc = "The frequency counter has detected a failure."]
            pub const FRQ_CT_FAIL_ERR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Common Security Error Register"]
pub mod CSER {
    #[doc = "Redundant FSM error/fault detected"]
    pub mod RED_FSM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No redundant FSM error/fault"]
            pub const RED_FSM_NOERR: u32 = 0;
            #[doc = "Redundant FSM error/fault detected."]
            pub const RED_FSM_ERR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Common Security Clear Register"]
pub mod CSCLR {
    #[doc = "Read only: Redundant FSM error/fault detected"]
    pub mod RED_FSM_CLR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No effect, ignored"]
            pub const RED_FSM_NOEFFECT: u32 = 0;
            #[doc = "Clears the CSER[RED_FSM] bit."]
            pub const RED_FSM_CLEAR: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "TRNG Oscillator 2 Control Register"]
pub mod OSC2_CTL {
    #[doc = "TRNG entropy generation control."]
    pub mod TRNG_ENT_CTL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single oscillator mode, using OSC1 (default)"]
            pub const TRNG_ENT_CTL_SINGLE_OSC1: u32 = 0;
            #[doc = "Dual oscillator mode"]
            pub const TRNG_ENT_CTL_DUAL_OSCS: u32 = 1;
            #[doc = "Single oscillator mode, using OSC2"]
            pub const TRNG_ENT_CTL_SINGLE_OSC2: u32 = 2;
            #[doc = "Unused, (bit field cannot be written to this value)"]
            pub const OSC2_DIV_DIV8: u32 = 3;
        }
    }
    #[doc = "Oscillator 2 Divide."]
    pub mod OSC2_DIV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use ring oscillator 2 with no divide"]
            pub const OSC2_DIV_DIV1: u32 = 0;
            #[doc = "Use ring oscillator 2 divided-by-2"]
            pub const OSC2_DIV_DIV2: u32 = 1;
            #[doc = "Use ring oscillator 2 divided-by-4"]
            pub const OSC2_DIV_DIV4: u32 = 2;
            #[doc = "Use ring oscillator 2 divided-by-8"]
            pub const OSC2_DIV_DIV8: u32 = 3;
        }
    }
    #[doc = "TRNG Oscillator 2 Frequency Count Valid"]
    pub mod OSC2_FCT_VAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Frequency count is invalid."]
            pub const DISABLE: u32 = 0;
            #[doc = "If TRNG_ENT_CTL = 10b, valid frequency count may be read from OSC2_FRQCNT."]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Oscillator fail safe limit."]
    pub mod OSC_FAILSAFE_LMT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The limit N is 4096 (2^12) system clocks."]
            pub const OSC_FAILSAFE_LMT_4K: u32 = 0;
            #[doc = "The limit N is 65536 (2^16) system clocks. (default)"]
            pub const OSC_FAILSAFE_LMT_64K: u32 = 1;
            #[doc = "N is 2^20 system clocks."]
            pub const OSC_FAILSAFE_LMT_1M: u32 = 2;
            #[doc = "N is 2^22 system clocks (full range of the counter being used)."]
            pub const OSC_FAILSAFE_LMT_4M: u32 = 3;
        }
    }
    #[doc = "Oscillator fail safe test."]
    pub mod OSC_FAILSAFE_TEST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No impact."]
            pub const DISABLE: u32 = 0;
            #[doc = "Disables oscillator 2 while in dual-oscillator mode (TRNG_ENT_CTL = 01b)."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Version ID Register (MS)"]
pub mod VID1 {
    #[doc = "Shows the IP\'s Minor revision of the TRNG."]
    pub mod MIN_REV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "Minor revision number for TRNG."]
            pub const MIN_REV_VAL: u32 = 11;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shows the IP\'s Major revision of the TRNG"]
    pub mod MAJ_REV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "Major revision number for TRNG."]
            pub const MAJ_REV_VAL: u32 = 20;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shows the IP ID."]
    pub mod IP_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {
            #[doc = "ID for TRNG."]
            pub const IP_ID_VAL: u32 = 48;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Version ID Register (LS)"]
pub mod VID2 {
    #[doc = "Shows the IP\'s Configuaration options for the TRNG."]
    pub mod CONFIG_OPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "TRNG_CONFIG_OPT for TRNG."]
            pub const CONFIG_OPT_VAL: u32 = 0;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shows the IP\'s ECO revision of the TRNG."]
    pub mod ECO_REV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "TRNG_ECO_REV for TRNG."]
            pub const ECO_REV_VAL: u32 = 0;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shows the integration options for the TRNG."]
    pub mod INTG_OPT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "INTG_OPT for TRNG."]
            pub const INTG_OPT_VAL: u32 = 10;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shows the ERA of the TRNG."]
    pub mod ERA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "ERA of the TRNG."]
            pub const ERA_VAL: u32 = 11;
        }
        pub mod W {}
        pub mod RW {}
    }
}
