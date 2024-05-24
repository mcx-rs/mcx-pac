#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "SCG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID Register"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameter Register"]
    pub PARAM: crate::RWRegister<u32>,
    #[doc = "Trim Lock register"]
    pub TRIM_LOCK: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "Clock Status Register"]
    pub CSR: crate::RWRegister<u32>,
    #[doc = "Run Clock Control Register"]
    pub RCCR: crate::RWRegister<u32>,
    _reserved1: [u8; 0xe8],
    #[doc = "SOSC Control Status Register"]
    pub SOSCCSR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x4],
    #[doc = "SOSC Configuration Register"]
    pub SOSCCFG: crate::RWRegister<u32>,
    _reserved3: [u8; 0xf4],
    #[doc = "SIRC Control Status Register"]
    pub SIRCCSR: crate::RWRegister<u32>,
    _reserved4: [u8; 0x8],
    #[doc = "SIRC Trim Configuration Register"]
    pub SIRCTCFG: crate::RWRegister<u32>,
    _reserved5: [u8; 0x8],
    #[doc = "SIRC Auto-trimming Status Register"]
    pub SIRCSTAT: crate::RWRegister<u32>,
    _reserved6: [u8; 0xe4],
    #[doc = "FIRC Control Status Register"]
    pub FIRCCSR: crate::RWRegister<u32>,
    _reserved7: [u8; 0x4],
    #[doc = "FIRC Configuration Register"]
    pub FIRCCFG: crate::RWRegister<u32>,
    #[doc = "FIRC Trim Configuration Register"]
    pub FIRCTCFG: crate::RWRegister<u32>,
    #[doc = "FIRC Trim Register"]
    pub FIRCTRIM: crate::RWRegister<u32>,
    _reserved8: [u8; 0x4],
    #[doc = "FIRC Auto-trimming Status Register"]
    pub FIRCSTAT: crate::RWRegister<u32>,
    _reserved9: [u8; 0xe4],
    #[doc = "ROSC Control Status Register"]
    pub ROSCCSR: crate::RWRegister<u32>,
    _reserved10: [u8; 0xfc],
    #[doc = "APLL Control Status Register"]
    pub APLLCSR: crate::RWRegister<u32>,
    #[doc = "APLL Control Register"]
    pub APLLCTRL: crate::RWRegister<u32>,
    #[doc = "APLL Status Register"]
    pub APLLSTAT: crate::RWRegister<u32>,
    #[doc = "APLL N Divider Register"]
    pub APLLNDIV: crate::RWRegister<u32>,
    #[doc = "APLL M Divider Register"]
    pub APLLMDIV: crate::RWRegister<u32>,
    #[doc = "APLL P Divider Register"]
    pub APLLPDIV: crate::RWRegister<u32>,
    #[doc = "APLL LOCK Configuration Register"]
    pub APLLLOCK_CNFG: crate::RWRegister<u32>,
    _reserved11: [u8; 0x4],
    #[doc = "APLL SSCG Status Register"]
    pub APLLSSCGSTAT: crate::RWRegister<u32>,
    #[doc = "APLL Spread Spectrum Control 0 Register"]
    pub APLLSSCG0: crate::RWRegister<u32>,
    #[doc = "APLL Spread Spectrum Control 1 Register"]
    pub APLLSSCG1: crate::RWRegister<u32>,
    _reserved12: [u8; 0xc8],
    #[doc = "APLL Override Register"]
    pub APLL_OVRD: crate::RWRegister<u32>,
    _reserved13: [u8; 0x8],
    #[doc = "SPLL Control Status Register"]
    pub SPLLCSR: crate::RWRegister<u32>,
    #[doc = "SPLL Control Register"]
    pub SPLLCTRL: crate::RWRegister<u32>,
    #[doc = "SPLL Status Register"]
    pub SPLLSTAT: crate::RWRegister<u32>,
    #[doc = "SPLL N Divider Register"]
    pub SPLLNDIV: crate::RWRegister<u32>,
    #[doc = "SPLL M Divider Register"]
    pub SPLLMDIV: crate::RWRegister<u32>,
    #[doc = "SPLL P Divider Register"]
    pub SPLLPDIV: crate::RWRegister<u32>,
    #[doc = "SPLL LOCK Configuration Register"]
    pub SPLLLOCK_CNFG: crate::RWRegister<u32>,
    _reserved14: [u8; 0x4],
    #[doc = "SPLL SSCG Status Register"]
    pub SPLLSSCGSTAT: crate::RWRegister<u32>,
    #[doc = "SPLL Spread Spectrum Control 0 Register"]
    pub SPLLSSCG0: crate::RWRegister<u32>,
    #[doc = "SPLL Spread Spectrum Control 1 Register"]
    pub SPLLSSCG1: crate::RWRegister<u32>,
    _reserved15: [u8; 0xc8],
    #[doc = "SPLL Override Register"]
    pub SPLL_OVRD: crate::RWRegister<u32>,
    _reserved16: [u8; 0x8],
    #[doc = "UPLL Control Status Register"]
    pub UPLLCSR: crate::RWRegister<u32>,
    _reserved17: [u8; 0xfc],
    #[doc = "LDO Control and Status Register"]
    pub LDOCSR: crate::RWRegister<u32>,
    _reserved18: [u8; 0xfc],
    #[doc = "TRO Control Status Register"]
    pub TROCSR: crate::RWRegister<u32>,
}
#[doc = "Version ID Register"]
pub mod VERID {
    #[doc = "SCG Version Number"]
    pub mod VERSION {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parameter Register"]
pub mod PARAM {
    #[doc = "SOSC Clock Present"]
    pub mod SOSCCLKPRES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "SOSC clock source is not present"]
            pub const NOPRES: u32 = 0;
            #[doc = "SOSC clock source is present"]
            pub const PRES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SIRC Clock Present"]
    pub mod SIRCCLKPRES {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "SIRC clock source is not present"]
            pub const NOPRES: u32 = 0;
            #[doc = "SIRC clock source is present"]
            pub const PRES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIRC Clock Present"]
    pub mod FIRCCLKPRES {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "FIRC clock source is not present"]
            pub const NOPRES: u32 = 0;
            #[doc = "FIRC clock source is present"]
            pub const PRES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ROSC Clock Present"]
    pub mod ROSCCLKPRES {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "ROSC clock source is not present"]
            pub const NOPRES: u32 = 0;
            #[doc = "ROSC clock source is present"]
            pub const PRES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APLL Clock Present"]
    pub mod APLLCLKPRES {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "APLL clock source is not present"]
            pub const NOPRES: u32 = 0;
            #[doc = "APLL clock source is present"]
            pub const PRES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPLL Clock Present"]
    pub mod SPLLCLKPRES {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "SPLL clock source is not present"]
            pub const NOPRES: u32 = 0;
            #[doc = "SPLL clock source is present"]
            pub const PRES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UPLL Clock Present"]
    pub mod UPLLCLKPRES {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "UPLL clock source is not present"]
            pub const NOPRES: u32 = 0;
            #[doc = "UPLL clock source is present"]
            pub const PRES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRO Clock Present"]
    pub mod TROCLKPRES {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "TRO clock source is not present"]
            pub const NOPRES: u32 = 0;
            #[doc = "TRO clock source is present"]
            pub const PRES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trim Lock register"]
pub mod TRIM_LOCK {
    #[doc = "TRIM_UNLOCK"]
    pub mod TRIM_UNLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SCG Trim registers are locked and not writable."]
            pub const LOCKED: u32 = 0;
            #[doc = "SCG Trim registers are unlocked and writable."]
            pub const NOT_LOCKED: u32 = 1;
        }
    }
    #[doc = "IFR_DISABLE"]
    pub mod IFR_DISABLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IFR write access to SCG trim registers not disabled. The SCG Trim registers are reprogrammed with the IFR values after any system reset."]
            pub const ENABLED: u32 = 0;
            #[doc = "IFR write access to SCG trim registers during system reset is blocked."]
            pub const DISABLED: u32 = 1;
        }
    }
    #[doc = "TRIM_LOCK_KEY"]
    pub mod TRIM_LOCK_KEY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Status Register"]
pub mod CSR {
    #[doc = "System Clock Source"]
    pub mod SCS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "SOSC"]
            pub const SOSC: u32 = 1;
            #[doc = "SIRC"]
            pub const SIRC: u32 = 2;
            #[doc = "FIRC"]
            pub const FIRC: u32 = 3;
            #[doc = "ROSC"]
            pub const ROSC: u32 = 4;
            #[doc = "APLL"]
            pub const APLL: u32 = 5;
            #[doc = "SPLL"]
            pub const SPLL: u32 = 6;
            #[doc = "UPLL"]
            pub const UPLL: u32 = 7;
            #[doc = "TRO"]
            pub const TRO: u32 = 8;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Run Clock Control Register"]
pub mod RCCR {
    #[doc = "System Clock Source"]
    pub mod SCS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOSC"]
            pub const SOSC: u32 = 1;
            #[doc = "SIRC"]
            pub const SIRC: u32 = 2;
            #[doc = "FIRC"]
            pub const FIRC: u32 = 3;
            #[doc = "ROSC"]
            pub const ROSC: u32 = 4;
            #[doc = "APLL"]
            pub const APLL: u32 = 5;
            #[doc = "SPLL"]
            pub const SPLL: u32 = 6;
            #[doc = "UPLL"]
            pub const UPLL: u32 = 7;
            #[doc = "TRO"]
            pub const TRO: u32 = 8;
        }
    }
}
#[doc = "SOSC Control Status Register"]
pub mod SOSCCSR {
    #[doc = "SOSC Enable"]
    pub mod SOSCEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOSC is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "SOSC is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SOSC Stop Enable"]
    pub mod SOSCSTEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOSC is disabled in Deep Sleep mode"]
            pub const DISABLED: u32 = 0;
            #[doc = "SOSC is enabled in Deep Sleep mode only if SOSCEN is set"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SOSC Clock Monitor Enable"]
    pub mod SOSCCM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOSC Clock Monitor is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "SOSC Clock Monitor is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SOSC Clock Monitor Reset Enable"]
    pub mod SOSCCMRE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock monitor generates an interrupt when an error is detected"]
            pub const GENERATE_INTERRUPT: u32 = 0;
            #[doc = "Clock monitor generates a reset when an error is detected"]
            pub const GENERATE_RESET: u32 = 1;
        }
    }
    #[doc = "Lock Register"]
    pub mod LK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "This Control Status Register can be written"]
            pub const WRITE_ENABLED: u32 = 0;
            #[doc = "This Control Status Register cannot be written"]
            pub const WRITE_DISABLED: u32 = 1;
        }
    }
    #[doc = "SOSC Valid"]
    pub mod SOSCVLD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "SOSC is not enabled or clock is not valid"]
            pub const DISABLED: u32 = 0;
            #[doc = "SOSC is enabled and output clock is valid"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SOSC Selected"]
    pub mod SOSCSEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "SOSC is not the system clock source"]
            pub const NOT_SOSC: u32 = 0;
            #[doc = "SOSC is the system clock source"]
            pub const SOSC: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SOSC Clock Error"]
    pub mod SOSCERR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOSC Clock Monitor is disabled or has not detected an error"]
            pub const DISABLED_OR_NO_ERROR: u32 = 0;
            #[doc = "SOSC Clock Monitor is enabled and detected an error"]
            pub const ENABLED_AND_ERROR: u32 = 1;
        }
    }
    #[doc = "SOSC Valid Interrupt Enable"]
    pub mod SOSCVLD_IE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOSCVLD interrupt is not enabled"]
            pub const NOT_SOSC: u32 = 0;
            #[doc = "SOSCVLD interrupt is enabled"]
            pub const SOSC: u32 = 1;
        }
    }
}
#[doc = "SOSC Configuration Register"]
pub mod SOSCCFG {
    #[doc = "External Reference Select"]
    pub mod EREFS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External reference clock selected. LDO can be disabled in this case."]
            pub const EXTERNAL: u32 = 0;
            #[doc = "Internal crystal oscillator of OSC selected."]
            pub const INTERNAL: u32 = 1;
        }
    }
    #[doc = "SOSC Range Select"]
    pub mod RANGE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frequency range select of 16-20 MHz."]
            pub const FREQ_16TO20MHZ: u32 = 0;
            #[doc = "Frequency range select of 20-30 MHz."]
            pub const LOW_FREQ: u32 = 1;
            #[doc = "Frequency range select of 30-50 MHz."]
            pub const MEDIUM_FREQ: u32 = 2;
            #[doc = "Frequency range select of 50-66 MHz."]
            pub const HIGH_FREQ: u32 = 3;
        }
    }
}
#[doc = "SIRC Control Status Register"]
pub mod SIRCCSR {
    #[doc = "SIRC Stop Enable"]
    pub mod SIRCSTEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SIRC is disabled in Deep Sleep mode"]
            pub const DISABLED: u32 = 0;
            #[doc = "SIRC is enabled in Deep Sleep mode"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SIRC Clock to Peripherals Enable"]
    pub mod SIRC_CLK_PERIPH_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SIRC clock to peripherals is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "SIRC clock to peripherals is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SIRC 12 MHz Trim Enable (SIRCCFG[RANGE]=1)"]
    pub mod SIRCTREN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables trimming SIRC to an external clock source"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables trimming SIRC to an external clock source"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SIRC Trim Update"]
    pub mod SIRCTRUP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables SIRC trimming updates"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables SIRC trimming updates"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SIRC TRIM LOCK"]
    pub mod TRIM_LOCK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "SIRC auto trim not locked to target frequency range"]
            pub const SIRC_NOT_LOCKED: u32 = 0;
            #[doc = "SIRC auto trim locked to target frequency range"]
            pub const SIRC_LOCKED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Coarse Auto Trim Bypass"]
    pub mod COARSE_TRIM_BYPASS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SIRC coarse auto-trim is not bypassed"]
            pub const NOT_BYPASSED: u32 = 0;
            #[doc = "SIRC coarse auto-trim is bypassed"]
            pub const BYPASSED: u32 = 1;
        }
    }
    #[doc = "Lock Register"]
    pub mod LK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Control Status Register can be written"]
            pub const WRITE_ENABLED: u32 = 0;
            #[doc = "Control Status Register cannot be written"]
            pub const WRITE_DISABLED: u32 = 1;
        }
    }
    #[doc = "SIRC Valid"]
    pub mod SIRCVLD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "SIRC is not enabled or clock is not valid"]
            pub const DISABLED_OR_NOT_VALID: u32 = 0;
            #[doc = "SIRC is enabled and output clock is valid"]
            pub const ENABLED_AND_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SIRC Selected"]
    pub mod SIRCSEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "SIRC is not the system clock source"]
            pub const NOT_SIRC: u32 = 0;
            #[doc = "SIRC is the system clock source"]
            pub const SIRC: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SIRC Clock Error"]
    pub mod SIRCERR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error not detected with the SIRC trimming"]
            pub const ERROR_NOT_DETECTED: u32 = 0;
            #[doc = "Error detected with the SIRC trimming"]
            pub const ERROR_DETECTED: u32 = 1;
        }
    }
    #[doc = "SIRC Clock Error Interrupt Enable"]
    pub mod SIRCERR_IE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SIRCERR interrupt is not enabled"]
            pub const ERROR_NOT_DETECTED: u32 = 0;
            #[doc = "SIRCERR interrupt is enabled"]
            pub const ERROR_DETECTED: u32 = 1;
        }
    }
}
#[doc = "SIRC Trim Configuration Register"]
pub mod SIRCTCFG {
    #[doc = "Trim Source"]
    pub mod TRIMSRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOSC"]
            pub const SOSC: u32 = 2;
            #[doc = "ROSC (32.768 kHz)"]
            pub const ROSC: u32 = 3;
        }
    }
    #[doc = "SIRC Trim Predivider"]
    pub mod TRIMDIV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SIRC Auto-trimming Status Register"]
pub mod SIRCSTAT {
    #[doc = "CCO Trim"]
    pub mod CCOTRIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CL Trim"]
    pub mod CLTRIM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIRC Control Status Register"]
pub mod FIRCCSR {
    #[doc = "FIRC Enable"]
    pub mod FIRCEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIRC is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "FIRC is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "FIRC Stop Enable"]
    pub mod FIRCSTEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIRC is disabled in Deep Sleep mode"]
            pub const DISABLED_IN_STOP_MODES: u32 = 0;
            #[doc = "FIRC is enabled in Deep Sleep mode"]
            pub const ENABLED_IN_STOP_MODES: u32 = 1;
        }
    }
    #[doc = "FIRC 48 MHz Clock to peripherals Enable"]
    pub mod FIRC_SCLK_PERIPH_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIRC 48 MHz to peripherals is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "FIRC 48 MHz to peripherals is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "FIRC 144 MHz Clock to peripherals Enable"]
    pub mod FIRC_FCLK_PERIPH_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIRC 144 MHz to peripherals is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "FIRC 144 MHz to peripherals is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "FIRC 144 MHz Trim Enable (FIRCCFG[RANGE]=1)"]
    pub mod FIRCTREN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables trimming FIRC to an external clock source"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables trimming FIRC to an external clock source"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "FIRC Trim Update"]
    pub mod FIRCTRUP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables FIRC trimming updates"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables FIRC trimming updates"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "FIRC TRIM LOCK"]
    pub mod TRIM_LOCK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "FIRC auto trim not locked to target frequency range"]
            pub const FIRC_NOT_LOCKED: u32 = 0;
            #[doc = "FIRC auto trim locked to target frequency range"]
            pub const FIRC_LOCKED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Coarse Auto Trim Bypass"]
    pub mod COARSE_TRIM_BYPASS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIRC coarse auto trim is not bypassed"]
            pub const NOT_BYPASSED: u32 = 0;
            #[doc = "FIRC coarse auto trim is bypassed"]
            pub const BYPASSED: u32 = 1;
        }
    }
    #[doc = "Lock Register"]
    pub mod LK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Control Status Register can be written"]
            pub const WRITE_ENABLED: u32 = 0;
            #[doc = "Control Status Register cannot be written"]
            pub const WRITE_DISABLED: u32 = 1;
        }
    }
    #[doc = "FIRC Valid status"]
    pub mod FIRCVLD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "FIRC is not enabled or clock is not valid."]
            pub const NOT_ENABLED_OR_NOT_VALID: u32 = 0;
            #[doc = "FIRC is enabled and output clock is valid. The clock is valid after there is an output clock from the FIRC analog."]
            pub const ENABLED_AND_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIRC Selected"]
    pub mod FIRCSEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "FIRC is not the system clock source"]
            pub const NOT_FIRC: u32 = 0;
            #[doc = "FIRC is the system clock source"]
            pub const FIRC: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIRC Clock Error"]
    pub mod FIRCERR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error not detected with the FIRC trimming"]
            pub const ERROR_NOT_DETECTED: u32 = 0;
            #[doc = "Error detected with the FIRC trimming"]
            pub const ERROR_DETECTED: u32 = 1;
        }
    }
    #[doc = "FIRC Clock Error Interrupt Enable"]
    pub mod FIRCERR_IE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIRCERR interrupt is not enabled"]
            pub const ERROR_NOT_DETECTED: u32 = 0;
            #[doc = "FIRCERR interrupt is enabled"]
            pub const ERROR_DETECTED: u32 = 1;
        }
    }
    #[doc = "FIRC Accurate Interrupt Enable"]
    pub mod FIRCACC_IE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIRCACC interrupt is not enabled"]
            pub const FIRCACCNOT: u32 = 0;
            #[doc = "FIRCACC interrupt is enabled"]
            pub const FIRCACCYES: u32 = 1;
        }
    }
    #[doc = "FIRC Frequency Accurate"]
    pub mod FIRCACC {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "FIRC is not enabled or clock is not accurate."]
            pub const NOT_ENABLED_OR_NOT_VALID: u32 = 0;
            #[doc = "FIRC is enabled and output clock is accurate. The clock is accurate after 4096 clock cycles of 144 MHz (RANGE=1) or 1365 clock cycles of 48 MHz(RANGE=0) from the FIRC analog."]
            pub const ENABLED_AND_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIRC Configuration Register"]
pub mod FIRCCFG {
    #[doc = "Frequency Range"]
    pub mod RANGE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "48 MHz FIRC clock selected"]
            pub const FIRC_48MHZ: u32 = 0;
            #[doc = "144 MHz FIRC clock selected"]
            pub const FIRC_144MHZ: u32 = 1;
        }
    }
}
#[doc = "FIRC Trim Configuration Register"]
pub mod FIRCTCFG {
    #[doc = "Trim Source"]
    pub mod TRIMSRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "USB0 Start of Frame (1 kHz). This option does not use TRIMDIV"]
            pub const USB0: u32 = 0;
            #[doc = "SOSC"]
            pub const SOSC: u32 = 2;
            #[doc = "ROSC"]
            pub const ROSC: u32 = 3;
        }
    }
    #[doc = "FIRC Trim Predivider"]
    pub mod TRIMDIV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIRC Trim Register"]
pub mod FIRCTRIM {
    #[doc = "Trim Fine"]
    pub mod TRIMFINE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trim Coarse"]
    pub mod TRIMCOAR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trim Temperature"]
    pub mod TRIMTEMP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trim Start"]
    pub mod TRIMSTART {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FIRC Auto-trimming Status Register"]
pub mod FIRCSTAT {
    #[doc = "Trim Fine"]
    pub mod TRIMFINE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trim Coarse"]
    pub mod TRIMCOAR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ROSC Control Status Register"]
pub mod ROSCCSR {
    #[doc = "ROSC Clock Monitor"]
    pub mod ROSCCM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROSC clock monitor is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "ROSC clock monitor is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "ROSC Clock Monitor Reset Enable"]
    pub mod ROSCCMRE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock monitor generates an interrupt when an error is detected"]
            pub const GENERATE_INTERRUPT: u32 = 0;
            #[doc = "Clock monitor generates a reset when an error is detected"]
            pub const GENERATE_RESET: u32 = 1;
        }
    }
    #[doc = "Lock Register"]
    pub mod LK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Control Status Register can be written"]
            pub const WRITE_ENABLED: u32 = 0;
            #[doc = "Control Status Register cannot be written"]
            pub const WRITE_DISABLED: u32 = 1;
        }
    }
    #[doc = "ROSC Valid"]
    pub mod ROSCVLD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "ROSC is not enabled or clock is not valid"]
            pub const DISABLED_OR_NOT_VALID: u32 = 0;
            #[doc = "ROSC is enabled and output clock is valid"]
            pub const ENABLED_AND_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ROSC Selected"]
    pub mod ROSCSEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "ROSC is not the system clock source"]
            pub const NOT_ROSC: u32 = 0;
            #[doc = "ROSC is the system clock source"]
            pub const ROSC: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ROSC Clock Error"]
    pub mod ROSCERR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ROSC Clock Monitor is disabled or has not detected an error"]
            pub const DISABLED_OR_NO_ERROR: u32 = 0;
            #[doc = "ROSC Clock Monitor is enabled and detected an RTC loss of clock error"]
            pub const ENABLED_AND_ERROR: u32 = 1;
        }
    }
}
#[doc = "APLL Control Status Register"]
pub mod APLLCSR {
    #[doc = "APLL Power Enable"]
    pub mod APLLPWREN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "APLL clock is powered off"]
            pub const DISABLED: u32 = 0;
            #[doc = "APLL clock is powered on"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "APLL Clock Enable"]
    pub mod APLLCLKEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "APLL clock is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "APLL clock is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "APLL Stop Enable"]
    pub mod APLLSTEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "APLL is disabled in Deep Sleep mode"]
            pub const DISABLED_IN_STOP: u32 = 0;
            #[doc = "APLL is enabled in Deep Sleep mode"]
            pub const ENABLED_IN_STOP: u32 = 1;
        }
    }
    #[doc = "Free running mode clock stable"]
    pub mod FRM_CLOCKSTABLE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free running mode clockstable is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Free running mode clockstable is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "APLL Clock Monitor"]
    pub mod APLLCM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "APLL Clock Monitor is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "APLL Clock Monitor is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "APLL Clock Monitor Reset Enable"]
    pub mod APLLCMRE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock monitor generates an interrupt when an error is detected"]
            pub const GENERATE_INTERRUPT: u32 = 0;
            #[doc = "Clock monitor generates a reset when an error is detected"]
            pub const GENERATE_RESET: u32 = 1;
        }
    }
    #[doc = "Lock Register"]
    pub mod LK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Control Status Register can be written"]
            pub const WRITE_ENABLED: u32 = 0;
            #[doc = "Control Status Register cannot be written"]
            pub const WRITE_DISABLED: u32 = 1;
        }
    }
    #[doc = "APLL LOCK"]
    pub mod APLL_LOCK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "APLL is not powered on or not locked"]
            pub const DISABLED_OR_NOT_VALID: u32 = 0;
            #[doc = "APLL is locked"]
            pub const ENABLED_AND_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APLL Selected"]
    pub mod APLLSEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "APLL is not the system clock source"]
            pub const NOT_APLL: u32 = 0;
            #[doc = "APLL is the system clock source"]
            pub const APLL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APLL Clock Error"]
    pub mod APLLERR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "APLL Clock Monitor is disabled or has not detected an error"]
            pub const DISABLED_OR_NO_ERROR: u32 = 0;
            #[doc = "APLL Clock Monitor is enabled and detected an error"]
            pub const ENABLED_AND_ERROR: u32 = 1;
        }
    }
    #[doc = "APLL LOCK Interrupt Enable"]
    pub mod APLL_LOCK_IE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "APLL_LOCK interrupt is not enabled"]
            pub const NOT_APLL: u32 = 0;
            #[doc = "APLL_LOCK interrupt is enabled"]
            pub const APLL: u32 = 1;
        }
    }
}
#[doc = "APLL Control Register"]
pub mod APLLCTRL {
    #[doc = "Bandwidth select R (resistor) value."]
    pub mod SELR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bandwidth select I (integration) value."]
    pub mod SELI {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bandwidth select P (proportional) value."]
    pub mod SELP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass of Divide-by-2 Divider"]
    pub mod BYPASSPOSTDIV2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the divide-by-2 divider in the postdivider"]
            pub const NOT_BYPASSED: u32 = 0;
            #[doc = "Bypass of the divide-by-2 divider in the postdivider"]
            pub const BYPASSED: u32 = 1;
        }
    }
    #[doc = "Up Limiter"]
    pub mod LIMUPOFF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Application set to non-Spectrum and Fractional applications."]
            pub const DISABLED: u32 = 0;
            #[doc = "Application set to Spectrum and Fractional applications."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Control of the bandwidth of the PLL."]
    pub mod BANDDIRECT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The bandwidth is changed synchronously with the feedback-divider"]
            pub const DISABLED: u32 = 0;
            #[doc = "Modifies the bandwidth of the PLL directly"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Bypass of the predivider"]
    pub mod BYPASSPREDIV {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the predivider."]
            pub const DISABLED: u32 = 0;
            #[doc = "Bypass of the predivider."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Bypass of the postdivider"]
    pub mod BYPASSPOSTDIV {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the postdivider."]
            pub const DISABLED: u32 = 0;
            #[doc = "Bypass of the postdivider"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Free Running Mode Enable"]
    pub mod FRM {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free running mode disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Free running mode enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Clock Source"]
    pub mod SOURCE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOSC"]
            pub const SOSC: u32 = 0;
            #[doc = "FIRC 48 MHz clock. FIRC_SCLK_PERIPH_EN must be set to use FIRC 48 MHz clock."]
            pub const FIRC: u32 = 1;
            #[doc = "ROSC"]
            pub const ROSC: u32 = 2;
            #[doc = "No clock"]
            pub const RSVD: u32 = 3;
        }
    }
}
#[doc = "APLL Status Register"]
pub mod APLLSTAT {
    #[doc = "Predivider(N) ratio change acknowledge."]
    pub mod NDIVACK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The predivider (N) ratio change is not accepted by the analog PLL"]
            pub const DISABLED: u32 = 0;
            #[doc = "The predivider (N) ratio change is accepted by the analog PLL"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Feedback(M) divider ratio change acknowledge."]
    pub mod MDIVACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The feedback (M) ratio change is not accepted by the analog PLL"]
            pub const DISABLED: u32 = 0;
            #[doc = "The feedback (M) ratio change is accepted by the analog PLL"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Postdivider(P) ratio change acknowledge."]
    pub mod PDIVACK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The postdivider (P) ratio change is not accepted by the analog PLL"]
            pub const DISABLED: u32 = 0;
            #[doc = "The postdivider (P) ratio change is accepted by the analog PLL"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Free running detector (active high)"]
    pub mod FRMDET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Free running is not detected"]
            pub const DISABLED: u32 = 0;
            #[doc = "Free running is detected"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APLL N Divider Register"]
pub mod APLLNDIV {
    #[doc = "Predivider divider ratio (N-divider)."]
    pub mod NDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Predivider ratio change request."]
    pub mod NREQ {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Predivider ratio change is not requested"]
            pub const DISABLED: u32 = 0;
            #[doc = "Predivider ratio change is requested"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "APLL M Divider Register"]
pub mod APLLMDIV {
    #[doc = "Feedback divider divider ratio (M-divider)."]
    pub mod MDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Feedback ratio change request."]
    pub mod MREQ {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Feedback ratio change is not requested"]
            pub const DISABLED: u32 = 0;
            #[doc = "Feedback ratio change is requested"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "APLL P Divider Register"]
pub mod APLLPDIV {
    #[doc = "Postdivider divider ratio (P-divider)"]
    pub mod PDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Postdivider ratio change request"]
    pub mod PREQ {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Postdivider ratio change is not requested"]
            pub const DISABLED: u32 = 0;
            #[doc = "Postdivider ratio change is requested"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "APLL LOCK Configuration Register"]
pub mod APLLLOCK_CNFG {
    #[doc = "Configures the number of reference clocks to count before APLL is considered locked."]
    pub mod LOCK_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APLL SSCG Status Register"]
pub mod APLLSSCGSTAT {
    #[doc = "SS_MDIV change acknowledge"]
    pub mod SS_MDIV_ACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The SS_MDIV, MF, MR, and MC ratio change is not accepted by the analog PLL"]
            pub const DISABLED: u32 = 0;
            #[doc = "The SS_MDIV, MF, MR, and MC ratio change is accepted by the analog PLL"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APLL Spread Spectrum Control 0 Register"]
pub mod APLLSSCG0 {
    #[doc = "SS_MDIV"]
    pub mod SS_MDIV_LSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "APLL Spread Spectrum Control 1 Register"]
pub mod APLLSSCG1 {
    #[doc = "SS_MDIV[32]"]
    pub mod SS_MDIV_MSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SS_MDIV[32:0] change request."]
    pub mod SS_MDIV_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SS_MDIV change is not requested"]
            pub const DISABLED: u32 = 0;
            #[doc = "SS_MDIV change is requested"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Modulation Frequency Control"]
    pub mod MF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Modulation Depth Control"]
    pub mod MR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Modulation Waveform Control"]
    pub mod MC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MC[1:0] no compensation"]
            pub const NO_COMP: u32 = 0;
            #[doc = "MC[1:0] maximum compensation"]
            pub const MAX_COMP: u32 = 3;
        }
    }
    #[doc = "Dither Enable"]
    pub mod DITHER {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dither is not enabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Dither is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SS_MDIV select."]
    pub mod SEL_SS_MDIV {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Feedback divider ratio is MDIV[15:0]"]
            pub const DISABLED: u32 = 0;
            #[doc = "Feedback divider ratio is SS_MDIV[32:0]"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SSCG Power Down"]
    pub mod SS_PD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SSCG is powered on"]
            pub const DISABLED: u32 = 0;
            #[doc = "SSCG is powered off"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "APLL Override Register"]
pub mod APLL_OVRD {
    #[doc = "APLL Power Enable Override if APLL_OVRD_EN=1"]
    pub mod APLLPWREN_OVRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "APLL clock is powered off"]
            pub const DISABLED: u32 = 0;
            #[doc = "APLL clock is powered on"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "APLL Clock Enable Override if APLL_OVRD_EN=1"]
    pub mod APLLCLKEN_OVRD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "APLL clock is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "APLL clock is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "APLL Override Enable"]
    pub mod APLL_OVRD_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "APLL override is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "APLL override is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "SPLL Control Status Register"]
pub mod SPLLCSR {
    #[doc = "SPLL Power Enable"]
    pub mod SPLLPWREN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPLL clock is powered off"]
            pub const DISABLED: u32 = 0;
            #[doc = "SPLL clock is powered on"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SPLL Clock Enable"]
    pub mod SPLLCLKEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPLL clock is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "SPLL clock is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SPLL Stop Enable"]
    pub mod SPLLSTEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPLL is disabled in Deep Sleep mode"]
            pub const DISABLED_IN_STOP: u32 = 0;
            #[doc = "SPLL is enabled in Deep Sleep mode"]
            pub const ENABLED_IN_STOP: u32 = 1;
        }
    }
    #[doc = "Free running mode clock stable"]
    pub mod FRM_CLOCKSTABLE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free running mode clockstable is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Free running mode clockstable is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SPLL Clock Monitor"]
    pub mod SPLLCM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPLL Clock Monitor is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "SPLL Clock Monitor is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SPLL Clock Monitor Reset Enable"]
    pub mod SPLLCMRE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock monitor generates an interrupt when an error is detected"]
            pub const GENERATE_INTERRUPT: u32 = 0;
            #[doc = "Clock monitor generates a reset when an error is detected"]
            pub const GENERATE_RESET: u32 = 1;
        }
    }
    #[doc = "Lock Register"]
    pub mod LK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Control Status Register can be written"]
            pub const WRITE_ENABLED: u32 = 0;
            #[doc = "Control Status Register cannot be written"]
            pub const WRITE_DISABLED: u32 = 1;
        }
    }
    #[doc = "SPLL LOCK"]
    pub mod SPLL_LOCK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "SPLL is not powered on or not locked"]
            pub const DISABLED_OR_NOT_VALID: u32 = 0;
            #[doc = "SPLL is locked"]
            pub const ENABLED_AND_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPLL Selected"]
    pub mod SPLLSEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "SPLL is not the system clock source"]
            pub const NOT_SPLL: u32 = 0;
            #[doc = "SPLL is the system clock source"]
            pub const SPLL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SPLL Clock Error"]
    pub mod SPLLERR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPLL Clock Monitor is disabled or has not detected an error"]
            pub const DISABLED_OR_NO_ERROR: u32 = 0;
            #[doc = "SPLL Clock Monitor is enabled and detected an error"]
            pub const ENABLED_AND_ERROR: u32 = 1;
        }
    }
    #[doc = "SPLL LOCK Interrupt Enable"]
    pub mod SPLL_LOCK_IE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPLL_LOCK interrupt is not enabled"]
            pub const NOT_SPLL: u32 = 0;
            #[doc = "SPLL_LOCK interrupt is enabled"]
            pub const SPLL: u32 = 1;
        }
    }
}
#[doc = "SPLL Control Register"]
pub mod SPLLCTRL {
    #[doc = "Bandwidth select R (resistor) value."]
    pub mod SELR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bandwidth select I (integration) value."]
    pub mod SELI {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bandwidth select P (proportional) value."]
    pub mod SELP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass of Divide-by-2 Divider"]
    pub mod BYPASSPOSTDIV2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the divide-by-2 divider in the postdivider."]
            pub const NOT_BYPASSED: u32 = 0;
            #[doc = "Bypass of the divide-by-2 divider in the postdivider"]
            pub const BYPASSED: u32 = 1;
        }
    }
    #[doc = "Up Limiter."]
    pub mod LIMUPOFF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Application set to non-Spectrum and Fractional applications."]
            pub const DISABLED: u32 = 0;
            #[doc = "Application set to Spectrum and Fractional applications."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Control of the bandwidth of the PLL."]
    pub mod BANDDIRECT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The bandwidth is changed synchronously with the feedback-divider"]
            pub const DISABLED: u32 = 0;
            #[doc = "Modifies the bandwidth of the PLL directly"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Bypass of the predivider."]
    pub mod BYPASSPREDIV {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the predivider"]
            pub const DISABLED: u32 = 0;
            #[doc = "Bypass of the predivider"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Bypass of the postdivider."]
    pub mod BYPASSPOSTDIV {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the postdivider"]
            pub const DISABLED: u32 = 0;
            #[doc = "Bypass of the postdivider"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Free Running Mode Enable"]
    pub mod FRM {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Free running mode disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Free running mode enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Clock Source"]
    pub mod SOURCE {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOSC"]
            pub const SOSC: u32 = 0;
            #[doc = "FIRC 48 MHz clock. FIRC_SCLK_PERIPH_EN must be set to use FIRC 48 MHz clock."]
            pub const FIRC: u32 = 1;
            #[doc = "ROSC"]
            pub const ROSC: u32 = 2;
            #[doc = "No clock"]
            pub const RSVD: u32 = 3;
        }
    }
}
#[doc = "SPLL Status Register"]
pub mod SPLLSTAT {
    #[doc = "Predivider (N) ratio change acknowledge"]
    pub mod NDIVACK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The predivider (N) ratio change is not accepted by the analog PLL."]
            pub const DISABLED: u32 = 0;
            #[doc = "The predivider (N) ratio change is accepted by the analog PLL."]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Feedback (M) divider ratio change acknowledge"]
    pub mod MDIVACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The feedback (M) ratio change is not accepted by the analog PLL."]
            pub const DISABLED: u32 = 0;
            #[doc = "The feedback (M) ratio change is accepted by the analog PLL."]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Postdivider (P) ratio change acknowledge"]
    pub mod PDIVACK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The postdivider (P) ratio change is not accepted by the analog PLL"]
            pub const DISABLED: u32 = 0;
            #[doc = "The postdivider (P) ratio change is accepted by the analog PLL"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Free running detector (active high)"]
    pub mod FRMDET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Free running is not detected"]
            pub const DISABLED: u32 = 0;
            #[doc = "Free running is detected"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPLL N Divider Register"]
pub mod SPLLNDIV {
    #[doc = "Predivider divider ratio (N-divider)."]
    pub mod NDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Predivider ratio change request."]
    pub mod NREQ {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Predivider ratio change is not requested"]
            pub const DISABLED: u32 = 0;
            #[doc = "Predivider ratio change is requested"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "SPLL M Divider Register"]
pub mod SPLLMDIV {
    #[doc = "Feedback divider divider ratio (M-divider)."]
    pub mod MDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Feedback ratio change request."]
    pub mod MREQ {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Feedback ratio change is not requested"]
            pub const DISABLED: u32 = 0;
            #[doc = "Feedback ratio change is requested"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "SPLL P Divider Register"]
pub mod SPLLPDIV {
    #[doc = "Postdivider divider ratio (P-divider)"]
    pub mod PDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Postdivider ratio change request"]
    pub mod PREQ {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Postdivider ratio change is not requested"]
            pub const DISABLED: u32 = 0;
            #[doc = "Postdivider ratio change is requested"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "SPLL LOCK Configuration Register"]
pub mod SPLLLOCK_CNFG {
    #[doc = "Configures the number of reference clocks to count before SPLL is considered locked."]
    pub mod LOCK_TIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPLL SSCG Status Register"]
pub mod SPLLSSCGSTAT {
    #[doc = "SS_MDIV change acknowledge"]
    pub mod SS_MDIV_ACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The SS_MDIV, MF, MR, and MC ratio change is not accepted by the analog PLL"]
            pub const DISABLED: u32 = 0;
            #[doc = "The SS_MDIV, MF, MR, and MC ratio change is accepted by the analog PLL"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPLL Spread Spectrum Control 0 Register"]
pub mod SPLLSSCG0 {
    #[doc = "SS_MDIV[31:0]"]
    pub mod SS_MDIV_LSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPLL Spread Spectrum Control 1 Register"]
pub mod SPLLSSCG1 {
    #[doc = "SS_MDIV[32]"]
    pub mod SS_MDIV_MSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SS_MDIV[32:0] change request."]
    pub mod SS_MDIV_REQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SS_MDIV change is not requested"]
            pub const DISABLED: u32 = 0;
            #[doc = "SS_MDIV change is requested"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Modulation Frequency Control"]
    pub mod MF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Modulation Depth Control"]
    pub mod MR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Modulation Waveform Control"]
    pub mod MC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MC[1:0] no compensation"]
            pub const NO_COMP: u32 = 0;
            #[doc = "MC[1:0] maximum compensation"]
            pub const MAX_COMP: u32 = 3;
        }
    }
    #[doc = "Dither Enable"]
    pub mod DITHER {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dither is not enabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Dither is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SS_MDIV select."]
    pub mod SEL_SS_MDIV {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Feedback divider ratio is MDIV[15:0]"]
            pub const DISABLED: u32 = 0;
            #[doc = "Feedback divider ratio is SS_MDIV[32:0]"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SSCG Power Down"]
    pub mod SS_PD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SSCG is powered on"]
            pub const DISABLED: u32 = 0;
            #[doc = "SSCG is powered off"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "SPLL Override Register"]
pub mod SPLL_OVRD {
    #[doc = "SPLL Power Enable Override if SPLL_OVRD_EN=1"]
    pub mod SPLLPWREN_OVRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPLL clock is powered off"]
            pub const DISABLED: u32 = 0;
            #[doc = "SPLL clock is powered on"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SPLL Clock Enable Override if SPLL_OVRD_EN=1"]
    pub mod SPLLCLKEN_OVRD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPLL clock is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "SPLL clock is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SPLL Override Enable"]
    pub mod SPLL_OVRD_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPLL override is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "SPLL override is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "UPLL Control Status Register"]
pub mod UPLLCSR {
    #[doc = "UPLL Clock Monitor"]
    pub mod UPLLCM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "UPLL Clock Monitor is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "UPLL Clock Monitor is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "UPLL Clock Monitor Reset Enable"]
    pub mod UPLLCMRE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock monitor generates an interrupt when an error is detected"]
            pub const GENERATE_INTERRUPT: u32 = 0;
            #[doc = "Clock monitor generates a reset when an error is detected"]
            pub const GENERATE_RESET: u32 = 1;
        }
    }
    #[doc = "Lock Register"]
    pub mod LK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Control Status Register can be written"]
            pub const WRITE_ENABLED: u32 = 0;
            #[doc = "Control Status Register cannot be written"]
            pub const WRITE_DISABLED: u32 = 1;
        }
    }
    #[doc = "UPLL Valid"]
    pub mod UPLLVLD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "UPLL is not enabled or clock is not valid"]
            pub const DISABLED_OR_NOT_VALID: u32 = 0;
            #[doc = "UPLL is enabled and output clock is valid"]
            pub const ENABLED_AND_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UPLL Selected"]
    pub mod UPLLSEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "UPLL is not the system clock source"]
            pub const NOT_USB_PLL: u32 = 0;
            #[doc = "UPLL is the system clock source"]
            pub const USB_PLL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UPLL Clock Error"]
    pub mod UPLLERR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "UPLL Clock Monitor is disabled or has not detected an error"]
            pub const DISABLED_OR_NO_ERROR: u32 = 0;
            #[doc = "UPLL Clock Monitor is enabled and detected an error"]
            pub const ENABLED_AND_ERROR: u32 = 1;
        }
    }
}
#[doc = "LDO Control and Status Register"]
pub mod LDOCSR {
    #[doc = "LDO Enable"]
    pub mod LDOEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LDO is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "LDO is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "LDO output voltage select"]
    pub mod VOUT_SEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VOUT = 1V"]
            pub const VOUT_1V_1: u32 = 0;
            #[doc = "VOUT = 1V"]
            pub const VOUT_1V_2: u32 = 1;
            #[doc = "VOUT = 1V"]
            pub const VOUT_1V_3: u32 = 2;
            #[doc = "VOUT = 1.05V"]
            pub const VOUT_105V: u32 = 3;
            #[doc = "VOUT = 1.1V"]
            pub const VOUT_11V: u32 = 4;
            #[doc = "VOUT = 1.15V"]
            pub const VOUT_115V: u32 = 5;
            #[doc = "VOUT = 1.2V"]
            pub const VOUT_12V: u32 = 6;
            #[doc = "VOUT = 1.25V"]
            pub const VOUT_125V: u32 = 7;
        }
    }
    #[doc = "LDO Bypass"]
    pub mod LDOBYPASS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LDO is not bypassed"]
            pub const DISABLED: u32 = 0;
            #[doc = "LDO is bypassed"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "LDO VOUT OK Inform."]
    pub mod VOUT_OK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "LDO output VOUT is not OK"]
            pub const DISABLED: u32 = 0;
            #[doc = "LDO output VOUT is OK"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TRO Control Status Register"]
pub mod TROCSR {
    #[doc = "TRO Clock Monitor"]
    pub mod TROCM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRO Clock Monitor is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "TRO Clock Monitor is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "TRO Clock Monitor Reset Enable"]
    pub mod TROCMRE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock monitor generates an interrupt when an error is detected"]
            pub const GENERATE_INTERRUPT: u32 = 0;
            #[doc = "Clock monitor generates a reset when an error is detected"]
            pub const GENERATE_RESET: u32 = 1;
        }
    }
    #[doc = "TRO reference clock selection"]
    pub mod TRO_REFCLK_SEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SOSC"]
            pub const V00: u32 = 0;
            #[doc = "SIRC"]
            pub const V01: u32 = 1;
            #[doc = "FIRC (144 MHz or 48 MHz, based on RANGE selection)"]
            pub const V10: u32 = 2;
        }
    }
    #[doc = "Lock Register"]
    pub mod LK {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Control Status Register can be written"]
            pub const WRITE_ENABLED: u32 = 0;
            #[doc = "Control Status Register cannot be written"]
            pub const WRITE_DISABLED: u32 = 1;
        }
    }
    #[doc = "TRO Valid"]
    pub mod TROVLD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "TRO is not enabled or clock is not valid"]
            pub const DISABLED_OR_NOT_VALID: u32 = 0;
            #[doc = "TRO is enabled and output clock is valid"]
            pub const ENABLED_AND_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRO Selected"]
    pub mod TROSEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "TRO is not the system clock source"]
            pub const NOT_USB_PLL: u32 = 0;
            #[doc = "TRO is the system clock source"]
            pub const USB_PLL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TRO Clock Error"]
    pub mod TROERR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TRO clock monitor is disabled or has not detected an error"]
            pub const DISABLED_OR_NO_ERROR: u32 = 0;
            #[doc = "TRO clock monitor is enabled and detected an error"]
            pub const ENABLED_AND_ERROR: u32 = 1;
        }
    }
}
