#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "CMC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    _reserved0: [u8; 0xc],
    #[doc = "Clock Control"]
    pub CKCTRL: crate::RWRegister<u32>,
    #[doc = "Clock Status"]
    pub CKSTAT: crate::RWRegister<u32>,
    #[doc = "Power Mode Protection"]
    pub PMPROT: crate::RWRegister<u32>,
    #[doc = "Global Power Mode Control"]
    pub GPMCTRL: crate::RWRegister<u32>,
    #[doc = "Power Mode Control"]
    pub PMCTRLMAIN: crate::RWRegister<u32>,
    #[doc = "Power Mode Control"]
    pub PMCTRLWAKE: crate::RWRegister<u32>,
    _reserved1: [u8; 0x58],
    #[doc = "System Reset Status"]
    pub SRS: crate::RWRegister<u32>,
    #[doc = "Reset Pin Control"]
    pub RPC: crate::RWRegister<u32>,
    #[doc = "Sticky System Reset Status"]
    pub SSRS: crate::RWRegister<u32>,
    #[doc = "System Reset Interrupt Enable"]
    pub SRIE: crate::RWRegister<u32>,
    #[doc = "System Reset Interrupt Flag"]
    pub SRIF: crate::RWRegister<u32>,
    _reserved2: [u8; 0x8],
    #[doc = "Reset Count Register"]
    pub RSTCNT: crate::RWRegister<u32>,
    #[doc = "Mode"]
    pub MR0: crate::RWRegister<u32>,
    _reserved3: [u8; 0xc],
    #[doc = "Force Mode"]
    pub FM0: crate::RWRegister<u32>,
    _reserved4: [u8; 0xc],
    #[doc = "SRAM Disable"]
    pub SRAMDIS0: crate::RWRegister<u32>,
    _reserved5: [u8; 0xc],
    #[doc = "SRAM Retention"]
    pub SRAMRET0: crate::RWRegister<u32>,
    _reserved6: [u8; 0xc],
    #[doc = "Flash Control"]
    pub FLASHCR: crate::RWRegister<u32>,
    _reserved7: [u8; 0x1c],
    #[doc = "BootROM Status"]
    pub BSR: crate::RWRegister<u32>,
    _reserved8: [u8; 0x8],
    #[doc = "BootROM Lock Register"]
    pub BLR: crate::RWRegister<u32>,
    #[doc = "Core Control"]
    pub CORECTL: crate::RWRegister<u32>,
    _reserved9: [u8; 0xc],
    #[doc = "Debug Control"]
    pub DBGCTL: crate::RWRegister<u32>,
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minor Version Number"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major Version Number"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Control"]
pub mod CKCTRL {
    #[doc = "Clocking Mode"]
    pub mod CKMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock gating"]
            pub const CKMODE0000: u32 = 0;
            #[doc = "Core, platform, and peripheral clocks are gated, and core enters Low-Power mode."]
            pub const CKMODE1111: u32 = 15;
        }
    }
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allowed"]
            pub const DISABLED: u32 = 0;
            #[doc = "Blocked"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Clock Status"]
pub mod CKSTAT {
    #[doc = "Low Power Status"]
    pub mod CKMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "Core clock not gated"]
            pub const CKMODE0000: u32 = 0;
            #[doc = "Core, platform, and peripheral clocks were gated, and power domain entered Low-Power mode"]
            pub const CKMODE1111: u32 = 15;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wake-up Source"]
    pub mod WAKEUP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Status Valid"]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Core clock not gated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Core clock was gated due to Low-Power mode entry"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Power Mode Protection"]
pub mod PMPROT {
    #[doc = "Low-Power Mode"]
    pub mod LPMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not allowed"]
            pub const DISABLED: u32 = 0;
            #[doc = "Allowed"]
            pub const EN: u32 = 1;
            #[doc = "Allowed"]
            pub const EN1: u32 = 2;
            #[doc = "Allowed"]
            pub const EN2: u32 = 3;
            #[doc = "Allowed"]
            pub const EN3: u32 = 4;
            #[doc = "Allowed"]
            pub const EN4: u32 = 5;
            #[doc = "Allowed"]
            pub const EN5: u32 = 6;
            #[doc = "Allowed"]
            pub const EN6: u32 = 7;
            #[doc = "Allowed"]
            pub const EN7: u32 = 8;
            #[doc = "Allowed"]
            pub const EN8: u32 = 9;
            #[doc = "Allowed"]
            pub const EN9: u32 = 10;
            #[doc = "Allowed"]
            pub const EN10: u32 = 11;
            #[doc = "Allowed"]
            pub const EN11: u32 = 12;
            #[doc = "Allowed"]
            pub const EN12: u32 = 13;
            #[doc = "Allowed"]
            pub const EN13: u32 = 14;
            #[doc = "Allowed"]
            pub const EN14: u32 = 15;
        }
    }
    #[doc = "Lock Register"]
    pub mod LOCK {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Allowed"]
            pub const DISABLED: u32 = 0;
            #[doc = "Blocked"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Global Power Mode Control"]
pub mod GPMCTRL {
    #[doc = "Low-Power Mode"]
    pub mod LPMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Power Mode Control"]
pub mod PMCTRLMAIN {
    #[doc = "Low-Power Mode"]
    pub mod LPMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active/Sleep"]
            pub const LPMODE0000: u32 = 0;
            #[doc = "Deep Sleep"]
            pub const LPMODE0001: u32 = 1;
            #[doc = "Power Down"]
            pub const LPMODE0011: u32 = 3;
            #[doc = "Deep-Power Down"]
            pub const LPMODE1111: u32 = 15;
        }
    }
}
#[doc = "Power Mode Control"]
pub mod PMCTRLWAKE {
    #[doc = "Low-Power Mode"]
    pub mod LPMODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active/Sleep"]
            pub const LPMODE0000: u32 = 0;
            #[doc = "Deep Sleep"]
            pub const LPMODE0001: u32 = 1;
            #[doc = "Power Down"]
            pub const LPMODE0011: u32 = 3;
            #[doc = "Deep-Power Down"]
            pub const LPMODE1111: u32 = 15;
        }
    }
}
#[doc = "System Reset Status"]
pub mod SRS {
    #[doc = "Wake-up Reset"]
    pub mod WAKEUP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power-on Reset"]
    pub mod POR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Voltage Detect Reset"]
    pub mod VD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Warm Reset"]
    pub mod WARM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fatal Reset"]
    pub mod FATAL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset was not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset was generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pin Reset"]
    pub mod PIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset was not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset was generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Debug Access Port Reset"]
    pub mod DAP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset was not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset was generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset Timeout"]
    pub mod RSTACK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low Power Acknowledge Timeout Reset"]
    pub mod LPACK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System Clock Generation Reset"]
    pub mod SCG {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset is not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset is generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Windowed Watchdog 0 Reset"]
    pub mod WWDT0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset is not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset is generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software Reset"]
    pub mod SW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lockup Reset"]
    pub mod LOCKUP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CPU1 System Reset"]
    pub mod CPU1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VBAT System Reset"]
    pub mod VBAT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Windowed Watchdog 1 Reset"]
    pub mod WWDT1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset is not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset is generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code Watchdog 0 Reset"]
    pub mod CDOG0 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset is not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset is generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Code Watchdog 1 Reset"]
    pub mod CDOG1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset is not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset is generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "JTAG System Reset"]
    pub mod JTAG {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Security Violation Reset"]
    pub mod SECVIO {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tamper Reset"]
    pub mod TAMPER {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Pin Control"]
pub mod RPC {
    #[doc = "Reset Filter Configuration"]
    pub mod FILTCFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Filter Enable"]
    pub mod FILTEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Low-Power Filter Enable"]
    pub mod LPFEN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Sticky System Reset Status"]
pub mod SSRS {
    #[doc = "Wake-up Reset"]
    pub mod WAKEUP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Power-on Reset"]
    pub mod POR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Voltage Detect Reset"]
    pub mod VD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Warm Reset"]
    pub mod WARM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Fatal Reset"]
    pub mod FATAL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset was not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset was generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Pin Reset"]
    pub mod PIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "DAP Reset"]
    pub mod DAP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Reset Timeout"]
    pub mod RSTACK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Low Power Acknowledge Timeout Reset"]
    pub mod LPACK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "System Clock Generation Reset"]
    pub mod SCG {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset is generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Windowed Watchdog 0 Reset"]
    pub mod WWDT0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset is generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Software Reset"]
    pub mod SW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Lockup Reset"]
    pub mod LOCKUP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "CPU1 Reset"]
    pub mod CPU1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated from CPU1 reset source."]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated from CPU1 reset source."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "VBAT System Reset"]
    pub mod VBAT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Windowed Watchdog 1 Reset"]
    pub mod WWDT1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset is generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Code Watchdog 0 Reset"]
    pub mod CDOG0 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset is generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Code Watchdog 1 Reset"]
    pub mod CDOG1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset is not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset is generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "JTAG System Reset"]
    pub mod JTAG {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Security Violation Reset"]
    pub mod SECVIO {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Tamper Reset"]
    pub mod TAMPER {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset not generated"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset generated"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "System Reset Interrupt Enable"]
pub mod SRIE {
    #[doc = "Pin Reset"]
    pub mod PIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "DAP Reset"]
    pub mod DAP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Low Power Acknowledge Timeout Reset"]
    pub mod LPACK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "System Clock Generation Reset"]
    pub mod SCG {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Windowed Watchdog 0 Reset"]
    pub mod WWDT0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Software Reset"]
    pub mod SW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Lockup Reset"]
    pub mod LOCKUP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "CPU1 Reset"]
    pub mod CPU1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "VBAT System Reset"]
    pub mod VBAT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Windowed Watchdog 1 Reset"]
    pub mod WWDT1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Code Watchdog 0 Reset"]
    pub mod CDOG0 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Code Watchdog 1 Reset"]
    pub mod CDOG1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "System Reset Interrupt Flag"]
pub mod SRIF {
    #[doc = "Pin Reset"]
    pub mod PIN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset source not pending"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset source pending"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "DAP Reset"]
    pub mod DAP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset source not pending"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset source pending"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Low Power Acknowledge Timeout Reset"]
    pub mod LPACK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset source not pending"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset source pending"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Windowed Watchdog 0 Reset"]
    pub mod WWDT0 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset source not pending"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset source pending"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Software Reset"]
    pub mod SW {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset source not pending"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset source pending"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Lockup Reset"]
    pub mod LOCKUP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset source not pending"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset source pending"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "CPU1 Reset"]
    pub mod CPU1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset source not pending"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset source pending"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "VBAT System Reset"]
    pub mod VBAT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset source not pending"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset source pending"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Windowed Watchdog 1 Reset"]
    pub mod WWDT1 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset source not pending"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset source pending"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Code Watchdog 0 Reset"]
    pub mod CDOG0 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset source not pending"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset source pending"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Code Watchdog 1 Reset"]
    pub mod CDOG1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reset source not pending"]
            pub const DISABLED: u32 = 0;
            #[doc = "Reset source pending"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Reset Count Register"]
pub mod RSTCNT {
    #[doc = "Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Mode"]
pub mod MR0 {
    #[doc = "In System Programming Mode"]
    pub mod ISPMODE_n {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Force Mode"]
pub mod FM0 {
    #[doc = "Boot Configuration"]
    pub mod FORCECFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLED: u32 = 0;
            #[doc = "Asserts"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "SRAM Disable"]
pub mod SRAMDIS0 {
    #[doc = "SRAM Disable"]
    pub mod DIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRAM Retention"]
pub mod SRAMRET0 {
    #[doc = "SRAM Retention"]
    pub mod RET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control"]
pub mod FLASHCR {
    #[doc = "Flash Disable"]
    pub mod FLASHDIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLED: u32 = 0;
            #[doc = "Flash memory is disabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Flash Doze"]
    pub mod FLASHDOZE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLED: u32 = 0;
            #[doc = "Flash memory is disabled when core is sleeping (CKMODE > 0)"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Flash Wake"]
    pub mod FLASHWAKE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLED: u32 = 0;
            #[doc = "Flash memory is not disabled during flash memory accesses"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "BootROM Status"]
pub mod BSR {
    #[doc = "Provides status information written by the BootROM."]
    pub mod STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BootROM Lock Register"]
pub mod BLR {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "BootROM Status and Lock Registers can be written"]
            pub const LOCK010: u32 = 2;
            #[doc = "BootROM Status and Lock Registers cannot be written"]
            pub const LOCK101: u32 = 5;
        }
    }
}
#[doc = "Core Control"]
pub mod CORECTL {
    #[doc = "Non-maskable Pin Interrupt Enable"]
    pub mod NPIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Debug Control"]
pub mod DBGCTL {
    #[doc = "Sleep Or Debug"]
    pub mod SOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remains enabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Disabled"]
            pub const ENABLED: u32 = 1;
        }
    }
}
