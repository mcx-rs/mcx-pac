#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "VBAT"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    _reserved0: [u8; 0xc],
    #[doc = "Status A"]
    pub STATUSA: crate::RWRegister<u32>,
    #[doc = "Status B"]
    pub STATUSB: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable A"]
    pub IRQENA: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable B"]
    pub IRQENB: crate::RWRegister<u32>,
    #[doc = "Wake-up Enable A"]
    pub WAKENA: crate::RWRegister<u32>,
    #[doc = "Wake-up Enable B"]
    pub WAKENB: crate::RWRegister<u32>,
    #[doc = "Tamper Enable A"]
    pub TAMPERA: crate::RWRegister<u32>,
    #[doc = "Tamper Enable B"]
    pub TAMPERB: crate::RWRegister<u32>,
    #[doc = "Lock A"]
    pub LOCKA: crate::RWRegister<u32>,
    #[doc = "Lock B"]
    pub LOCKB: crate::RWRegister<u32>,
    #[doc = "Wake-up Configuration"]
    pub WAKECFG: crate::RWRegister<u32>,
    _reserved1: [u8; 0xc4],
    #[doc = "Oscillator Control A"]
    pub OSCCTLA: crate::RWRegister<u32>,
    #[doc = "Oscillator Control B"]
    pub OSCCTLB: crate::RWRegister<u32>,
    #[doc = "Oscillator Configuration A"]
    pub OSCCFGA: crate::RWRegister<u32>,
    #[doc = "Oscillator Configuration B"]
    pub OSCCFGB: crate::RWRegister<u32>,
    _reserved2: [u8; 0x8],
    #[doc = "Oscillator Lock A"]
    pub OSCLCKA: crate::RWRegister<u32>,
    #[doc = "Oscillator Lock B"]
    pub OSCLCKB: crate::RWRegister<u32>,
    #[doc = "Oscillator Clock Enable"]
    pub OSCCLKE: crate::RWRegister<u32>,
    _reserved3: [u8; 0xdc],
    #[doc = "FRO16K Control A"]
    pub FROCTLA: crate::RWRegister<u32>,
    #[doc = "FRO16K Control B"]
    pub FROCTLB: crate::RWRegister<u32>,
    _reserved4: [u8; 0x10],
    #[doc = "FRO16K Lock A"]
    pub FROLCKA: crate::RWRegister<u32>,
    #[doc = "FRO16K Lock B"]
    pub FROLCKB: crate::RWRegister<u32>,
    #[doc = "FRO16K Clock Enable"]
    pub FROCLKE: crate::RWRegister<u32>,
    _reserved5: [u8; 0xdc],
    #[doc = "LDO_RAM Control A"]
    pub LDOCTLA: crate::RWRegister<u32>,
    #[doc = "LDO_RAM Control B"]
    pub LDOCTLB: crate::RWRegister<u32>,
    _reserved6: [u8; 0x10],
    #[doc = "LDO_RAM Lock A"]
    pub LDOLCKA: crate::RWRegister<u32>,
    #[doc = "LDO_RAM Lock B"]
    pub LDOLCKB: crate::RWRegister<u32>,
    #[doc = "RAM Control"]
    pub LDORAMC: crate::RWRegister<u32>,
    _reserved7: [u8; 0xc],
    #[doc = "Bandgap Timer 0"]
    pub LDOTIMER0: crate::RWRegister<u32>,
    _reserved8: [u8; 0x4],
    #[doc = "Bandgap Timer 1"]
    pub LDOTIMER1: crate::RWRegister<u32>,
    _reserved9: [u8; 0xc4],
    #[doc = "CLKMON Control A"]
    pub MONCTLA: crate::RWRegister<u32>,
    #[doc = "CLKMON Control B"]
    pub MONCTLB: crate::RWRegister<u32>,
    #[doc = "CLKMON Configuration A"]
    pub MONCFGA: crate::RWRegister<u32>,
    #[doc = "CLKMON Configuration B"]
    pub MONCFGB: crate::RWRegister<u32>,
    _reserved10: [u8; 0x8],
    #[doc = "CLKMON Lock A"]
    pub MONLCKA: crate::RWRegister<u32>,
    #[doc = "CLKMON Lock B"]
    pub MONLCKB: crate::RWRegister<u32>,
    _reserved11: [u8; 0xe0],
    #[doc = "TAMPER Control A"]
    pub TAMCTLA: crate::RWRegister<u32>,
    #[doc = "TAMPER Control B"]
    pub TAMCTLB: crate::RWRegister<u32>,
    _reserved12: [u8; 0x10],
    #[doc = "TAMPER Lock A"]
    pub TAMLCKA: crate::RWRegister<u32>,
    #[doc = "TAMPER Lock B"]
    pub TAMLCKB: crate::RWRegister<u32>,
    _reserved13: [u8; 0xe0],
    #[doc = "Switch Control A"]
    pub SWICTLA: crate::RWRegister<u32>,
    #[doc = "Switch Control B"]
    pub SWICTLB: crate::RWRegister<u32>,
    _reserved14: [u8; 0x10],
    #[doc = "Switch Lock A"]
    pub SWILCKA: crate::RWRegister<u32>,
    #[doc = "Switch Lock B"]
    pub SWILCKB: crate::RWRegister<u32>,
    _reserved15: [u8; 0xe0],
    #[doc = "no description available"]
    pub WAKEUP: [wakeup::RegisterBlock; 2usize],
    _reserved16: [u8; 0xe8],
    #[doc = "Wakeup Lock A"]
    pub WAKLCKA: crate::RWRegister<u32>,
    #[doc = "Wakeup Lock B"]
    pub WAKLCKB: crate::RWRegister<u32>,
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
#[doc = "Status A"]
pub mod STATUSA {
    #[doc = "POR Detect Flag"]
    pub mod POR_DET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not reset"]
            pub const CLR: u32 = 0;
            #[doc = "Reset"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Wakeup Pin Flag"]
    pub mod WAKEUP_FLAG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not asserted"]
            pub const CLR: u32 = 0;
            #[doc = "Asserted"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Bandgap Timer 0 Flag"]
    pub mod TIMER0_FLAG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not reached"]
            pub const CLR: u32 = 0;
            #[doc = "Reached"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Bandgap Timer 1 Flag"]
    pub mod TIMER1_FLAG {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not reached"]
            pub const CLR: u32 = 0;
            #[doc = "Reached"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "LDO Ready"]
    pub mod LDO_RDY {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Disabled (not ready)"]
            pub const CLR: u32 = 0;
            #[doc = "Enabled (ready)"]
            pub const SET: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OSC32k Ready"]
    pub mod OSC_RDY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Disabled (clock not ready)"]
            pub const CLR: u32 = 0;
            #[doc = "Enabled (clock ready)"]
            pub const SET: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Detect"]
    pub mod CLOCK_DET {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock error not detected"]
            pub const CLR: u32 = 0;
            #[doc = "Clock error detected"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Configuration Detect Flag"]
    pub mod CONFIG_DET {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const CLR: u32 = 0;
            #[doc = "Detected"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Voltage Detect"]
    pub mod VOLT_DET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const CLR: u32 = 0;
            #[doc = "Detected"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Temperature Detect"]
    pub mod TEMP_DET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Temperature error not detected"]
            pub const CLR: u32 = 0;
            #[doc = "Temperature error detected"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Input 0 Detect"]
    pub mod SEC0_DET {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Security input 0 not detected"]
            pub const CLR: u32 = 0;
            #[doc = "Security input 0 detected"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Interrupt 0 Detect"]
    pub mod IRQ0_DET {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not asserted"]
            pub const CLR: u32 = 0;
            #[doc = "Asserted"]
            pub const SET: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt 1 Detect"]
    pub mod IRQ1_DET {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not asserted"]
            pub const CLR: u32 = 0;
            #[doc = "Asserted"]
            pub const SET: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt 2 Detect"]
    pub mod IRQ2_DET {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not asserted"]
            pub const CLR: u32 = 0;
            #[doc = "Asserted"]
            pub const SET: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt 3 Detect"]
    pub mod IRQ3_DET {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not asserted"]
            pub const CLR: u32 = 0;
            #[doc = "Asserted"]
            pub const SET: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status B"]
pub mod STATUSB {
    #[doc = "Inverse value"]
    pub mod INVERSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable A"]
pub mod IRQENA {
    #[doc = "POR Detect"]
    pub mod POR_DET {
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
    #[doc = "Wakeup Pin Flag"]
    pub mod WAKEUP_FLAG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CLR: u32 = 0;
            #[doc = "Enable"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Bandgap Timer 0"]
    pub mod TIMER0_FLAG {
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
    #[doc = "Bandgap Timer 2"]
    pub mod TIMER1_FLAG {
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
    #[doc = "LDO Ready"]
    pub mod LDO_RDY {
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
    #[doc = "OSC32k Ready"]
    pub mod OSC_RDY {
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
    #[doc = "Clock Detect"]
    pub mod CLOCK_DET {
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
    #[doc = "Configuration Detect"]
    pub mod CONFIG_DET {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CLR: u32 = 0;
            #[doc = "Enable"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Voltage Detect"]
    pub mod VOLT_DET {
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
    #[doc = "Temperature Detect"]
    pub mod TEMP_DET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Input 0 Detect"]
    pub mod SEC0_DET {
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
    #[doc = "Interrupt 0 Detect"]
    pub mod IRQ0_DET {
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
    #[doc = "Interrupt 1 Detect"]
    pub mod IRQ1_DET {
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
    #[doc = "Interrupt 2 Detect"]
    pub mod IRQ2_DET {
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
    #[doc = "Interrupt 3 Detect"]
    pub mod IRQ3_DET {
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
}
#[doc = "Interrupt Enable B"]
pub mod IRQENB {
    #[doc = "Inverse Value"]
    pub mod INVERSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Wake-up Enable A"]
pub mod WAKENA {
    #[doc = "POR Detect"]
    pub mod POR_DET {
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
    #[doc = "Wake-up Pin Flag"]
    pub mod WAKEUP_FLAG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CLR: u32 = 0;
            #[doc = "Enable"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Bandgap Timer 0"]
    pub mod TIMER0_FLAG {
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
    #[doc = "Bandgap Timer 2"]
    pub mod TIMER1_FLAG {
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
    #[doc = "LDO Ready"]
    pub mod LDO_RDY {
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
    #[doc = "OSC32K Ready"]
    pub mod OSC_RDY {
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
    #[doc = "Clock Detect"]
    pub mod CLOCK_DET {
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
    #[doc = "Configuration Detect"]
    pub mod CONFIG_DET {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CLR: u32 = 0;
            #[doc = "Enable"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Voltage Detect"]
    pub mod VOLT_DET {
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
    #[doc = "Temperature Detect"]
    pub mod TEMP_DET {
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
    #[doc = "Input 0 Detect"]
    pub mod SEC0_DET {
        pub const offset: u32 = 12;
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
    #[doc = "Interrupt 0 Detect"]
    pub mod IRQ0_DET {
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
    #[doc = "Interrupt 1 Detect"]
    pub mod IRQ1_DET {
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
    #[doc = "Interrupt 2 Detect"]
    pub mod IRQ2_DET {
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
    #[doc = "Interrupt 3 Detect"]
    pub mod IRQ3_DET {
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
}
#[doc = "Wake-up Enable B"]
pub mod WAKENB {
    #[doc = "Inverse Value"]
    pub mod INVERSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tamper Enable A"]
pub mod TAMPERA {
    #[doc = "POR Detect"]
    pub mod POR_DET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tamper disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Tamper enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Clock Detect"]
    pub mod CLOCK_DET {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tamper disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Tamper enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Configuration Detect"]
    pub mod CONFIG_DET {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tamper disabled"]
            pub const CLR: u32 = 0;
            #[doc = "Tamper enabled"]
            pub const SET: u32 = 1;
        }
    }
    #[doc = "Voltage Detect"]
    pub mod VOLT_DET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tamper disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Tamper enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Temperature Detect"]
    pub mod TEMP_DET {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tamper disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Tamper enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Input 0 Detect"]
    pub mod SEC0_DET {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tamper disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Tamper enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Tamper Enable B"]
pub mod TAMPERB {
    #[doc = "Inverse value"]
    pub mod INVERSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lock A"]
pub mod LOCKA {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables lock"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables lock. Cleared by VBAT POR."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Lock B"]
pub mod LOCKB {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables lock"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disables lock"]
            pub const DISABLE: u32 = 1;
        }
    }
}
#[doc = "Wake-up Configuration"]
pub mod WAKECFG {
    #[doc = "Output"]
    pub mod OUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic zero (asserted)"]
            pub const ON: u32 = 0;
            #[doc = "Logic one"]
            pub const OFF: u32 = 1;
        }
    }
}
#[doc = "Oscillator Control A"]
pub mod OSCCTLA {
    #[doc = "Crystal Oscillator Enable"]
    pub mod OSC_EN {
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
    #[doc = "Crystal Oscillator Bypass Enable"]
    pub mod OSC_BYP_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not bypass"]
            pub const DISABLE: u32 = 0;
            #[doc = "Bypass"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Amplifier Gain Coarse Adjustment"]
    pub mod COARSE_AMP_GAIN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0.5 uA/V"]
            pub const GAIN05: u32 = 0;
            #[doc = "1.0 uA/V"]
            pub const GAIN10: u32 = 1;
            #[doc = "1.8 uA/V"]
            pub const GAIN18: u32 = 2;
            #[doc = "3.3 uA/V"]
            pub const GAIN33: u32 = 3;
        }
    }
    #[doc = "Amplifier Gain Fine Adjustment"]
    pub mod FINE_AMP_GAIN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "200 mV"]
            pub const AGC200: u32 = 0;
        }
    }
    #[doc = "Crystal Load Capacitance Selection Enable"]
    pub mod CAP_SEL_EN {
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
    #[doc = "Crystal Load Capacitance Selection"]
    pub mod EXTAL_CAP_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0 pF"]
            pub const SEL0: u32 = 0;
            #[doc = "2 pF"]
            pub const SEL2: u32 = 1;
            #[doc = "4 pF"]
            pub const SEL4: u32 = 2;
            #[doc = "6 pF"]
            pub const SEL6: u32 = 3;
            #[doc = "8 pF"]
            pub const SEL8: u32 = 4;
            #[doc = "10 pF"]
            pub const SEL10: u32 = 5;
            #[doc = "12 pF"]
            pub const SEL12: u32 = 6;
            #[doc = "14 pF"]
            pub const SEL14: u32 = 7;
            #[doc = "16 pF"]
            pub const SEL16: u32 = 8;
            #[doc = "18 pF"]
            pub const SEL18: u32 = 9;
            #[doc = "20 pF"]
            pub const SEL20: u32 = 10;
            #[doc = "22 pF"]
            pub const SEL22: u32 = 11;
            #[doc = "24 pF"]
            pub const SEL24: u32 = 12;
            #[doc = "26 pF"]
            pub const SEL26: u32 = 13;
            #[doc = "28 pF"]
            pub const SEL28: u32 = 14;
            #[doc = "30 pF"]
            pub const SEL30: u32 = 15;
        }
    }
    #[doc = "Crystal Load Capacitance Selection"]
    pub mod XTAL_CAP_SEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0 pF"]
            pub const SEL0: u32 = 0;
            #[doc = "2 pF"]
            pub const SEL2: u32 = 1;
            #[doc = "4 pF"]
            pub const SEL4: u32 = 2;
            #[doc = "6 pF"]
            pub const SEL6: u32 = 3;
            #[doc = "8 pF"]
            pub const SEL8: u32 = 4;
            #[doc = "10 pF"]
            pub const SEL10: u32 = 5;
            #[doc = "12 pF"]
            pub const SEL12: u32 = 6;
            #[doc = "14 pF"]
            pub const SEL14: u32 = 7;
            #[doc = "16 pF"]
            pub const SEL16: u32 = 8;
            #[doc = "18 pF"]
            pub const SEL18: u32 = 9;
            #[doc = "20 pF"]
            pub const SEL20: u32 = 10;
            #[doc = "22 pF"]
            pub const SEL22: u32 = 11;
            #[doc = "24 pF"]
            pub const SEL24: u32 = 12;
            #[doc = "26 pF"]
            pub const SEL26: u32 = 13;
            #[doc = "28 pF"]
            pub const SEL28: u32 = 14;
            #[doc = "30 pF"]
            pub const SEL30: u32 = 15;
        }
    }
    #[doc = "Mode Enable"]
    pub mod MODE_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transconductance Oscillator mode"]
            pub const HP: u32 = 0;
            #[doc = "Low-Power Backup mode"]
            pub const LP: u32 = 1;
            #[doc = "Low-Power Switched mode"]
            pub const SW: u32 = 2;
        }
    }
    #[doc = "Supply Detector Trim"]
    pub mod SUPPLY_DET {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Oscillator Control B"]
pub mod OSCCTLB {
    #[doc = "Inverse Value"]
    pub mod INVERSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Oscillator Configuration A"]
pub mod OSCCFGA {
    #[doc = "Comparator Trim"]
    pub mod CMP_TRIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay Trim"]
    pub mod DLY_TRIM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Capacitor Trim"]
    pub mod CAP_TRIM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Initialization Trim"]
    pub mod INIT_TRIM {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8 s"]
            pub const SEL0: u32 = 0;
            #[doc = "4 s"]
            pub const SEL1: u32 = 1;
            #[doc = "2 s"]
            pub const SEL2: u32 = 2;
            #[doc = "1 s"]
            pub const SEL3: u32 = 3;
            #[doc = "0.5 s"]
            pub const SEL4: u32 = 4;
            #[doc = "0.25 s"]
            pub const SEL5: u32 = 5;
            #[doc = "0.125 s"]
            pub const SEL6: u32 = 6;
            #[doc = "0.5 ms"]
            pub const SEL7: u32 = 7;
        }
    }
}
#[doc = "Oscillator Configuration B"]
pub mod OSCCFGB {
    #[doc = "Inverse Value"]
    pub mod INVERSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Oscillator Lock A"]
pub mod OSCLCKA {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not block"]
            pub const DISABLE: u32 = 0;
            #[doc = "Block"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Oscillator Lock B"]
pub mod OSCLCKB {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block"]
            pub const ENABLE: u32 = 0;
            #[doc = "Do not block"]
            pub const DISABLE: u32 = 1;
        }
    }
}
#[doc = "Oscillator Clock Enable"]
pub mod OSCCLKE {
    #[doc = "Clock Enable"]
    pub mod CLKE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FRO16K Control A"]
pub mod FROCTLA {
    #[doc = "FRO16K Enable"]
    pub mod FRO_EN {
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
}
#[doc = "FRO16K Control B"]
pub mod FROCTLB {
    #[doc = "Inverse Value"]
    pub mod INVERSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FRO16K Lock A"]
pub mod FROLCKA {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not block"]
            pub const DISABLE: u32 = 0;
            #[doc = "Block"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "FRO16K Lock B"]
pub mod FROLCKB {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block"]
            pub const ENABLE: u32 = 0;
            #[doc = "Do not block"]
            pub const DISABLE: u32 = 1;
        }
    }
}
#[doc = "FRO16K Clock Enable"]
pub mod FROCLKE {
    #[doc = "Clock Enable"]
    pub mod CLKE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LDO_RAM Control A"]
pub mod LDOCTLA {
    #[doc = "Bandgap Enable"]
    pub mod BG_EN {
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
    #[doc = "LDO Enable"]
    pub mod LDO_EN {
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
    #[doc = "Refresh Enable"]
    pub mod REFRESH_EN {
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
}
#[doc = "LDO_RAM Control B"]
pub mod LDOCTLB {
    #[doc = "Inverse Value"]
    pub mod INVERSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LDO_RAM Lock A"]
pub mod LDOLCKA {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not block"]
            pub const DISABLE: u32 = 0;
            #[doc = "Block"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "LDO_RAM Lock B"]
pub mod LDOLCKB {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block"]
            pub const ENABLE: u32 = 0;
            #[doc = "Do not block"]
            pub const DISABLE: u32 = 1;
        }
    }
}
#[doc = "RAM Control"]
pub mod LDORAMC {
    #[doc = "Isolate SRAM"]
    pub mod ISO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "State follows the chip power modes"]
            pub const DISABLE: u32 = 0;
            #[doc = "Isolates SRAM and places it in Low-Power Retention mode"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Switch SRAM"]
    pub mod SWI {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Supply follows the chip power modes"]
            pub const DISABLE: u32 = 0;
            #[doc = "LDO_RAM powers the array"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Retention"]
    pub mod RET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Retained"]
            pub const DISABLE: u32 = 0;
            #[doc = "Not retained"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Bandgap Timer 0"]
pub mod LDOTIMER0 {
    #[doc = "Timeout Configuration"]
    pub mod TIMCFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 s"]
            pub const CFG1000: u32 = 0;
            #[doc = "500 ms"]
            pub const CFG500: u32 = 1;
            #[doc = "250 ms"]
            pub const CFG250: u32 = 2;
            #[doc = "125 ms"]
            pub const CFG125: u32 = 3;
            #[doc = "62.5 ms"]
            pub const CFG62: u32 = 4;
            #[doc = "31.25 ms"]
            pub const CFG31: u32 = 5;
            #[doc = "15.625 ms"]
            pub const CFG15: u32 = 6;
            #[doc = "7.8125 ms"]
            pub const CFG7: u32 = 7;
        }
    }
    #[doc = "Bandgap Timeout Period Enable"]
    pub mod TIMEN {
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
#[doc = "Bandgap Timer 1"]
pub mod LDOTIMER1 {
    #[doc = "Timeout Configuration"]
    pub mod TIMCFG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bandgap Timeout Period Enable"]
    pub mod TIMEN {
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
#[doc = "CLKMON Control A"]
pub mod MONCTLA {
    #[doc = "CLKMON Enable"]
    pub mod MON_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CLKMON is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "CLKMON is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "CLKMON Control B"]
pub mod MONCTLB {
    #[doc = "Inverse value"]
    pub mod INVERSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CLKMON Configuration A"]
pub mod MONCFGA {
    #[doc = "Frequency Trim"]
    pub mod FREQ_TRIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock monitor asserts 2 cycle after expected edge"]
            pub const CFG0: u32 = 0;
            #[doc = "Clock monitor asserts 4 cycles after expected edge"]
            pub const CFG1: u32 = 1;
            #[doc = "Clock monitor asserts 6 cycles after expected edge"]
            pub const CFG2: u32 = 2;
            #[doc = "Clock monitor asserts 8 cycles after expected edge"]
            pub const CFG3: u32 = 3;
        }
    }
    #[doc = "Divide Trim"]
    pub mod DIVIDE_TRIM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clock monitor operates at 1 kHz"]
            pub const CFG0: u32 = 0;
            #[doc = "Clock monitor operates at 64 Hz"]
            pub const CFG1: u32 = 1;
        }
    }
    #[doc = "Reserved Trim"]
    pub mod RSVD_TRIM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CLKMON Configuration B"]
pub mod MONCFGB {
    #[doc = "Inverse value"]
    pub mod INVERSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CLKMON Lock A"]
pub mod MONLCKA {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Lock is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "CLKMON Lock B"]
pub mod MONLCKB {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Lock is disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
}
#[doc = "TAMPER Control A"]
pub mod TAMCTLA {
    #[doc = "Voltage Detect Enable"]
    pub mod VOLT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Voltage detect is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Voltage detect is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Temperature Detect Enable"]
    pub mod TEMP_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Temperature detect is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Temperature detect is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "TAMPER Control B"]
pub mod TAMCTLB {
    #[doc = "Inverse value"]
    pub mod INVERSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TAMPER Lock A"]
pub mod TAMLCKA {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Lock is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "TAMPER Lock B"]
pub mod TAMLCKB {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Lock is disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
}
#[doc = "Switch Control A"]
pub mod SWICTLA {
    #[doc = "Switch Enable"]
    pub mod SWI_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VDD_BAT"]
            pub const DISABLE: u32 = 0;
            #[doc = "VDD_SYS"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Low Power Enable"]
    pub mod LP_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VDD_BAT always supplies VBAT modules in low-power modes"]
            pub const DISABLE: u32 = 0;
            #[doc = "VDD_SYS always supplies VBAT modules if SWI_EN is also 1"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Switch Control B"]
pub mod SWICTLB {
    #[doc = "Inverse Value"]
    pub mod INVERSE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Switch Lock A"]
pub mod SWILCKA {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not block"]
            pub const DISABLE: u32 = 0;
            #[doc = "Block"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Switch Lock B"]
pub mod SWILCKB {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Block"]
            pub const ENABLE: u32 = 0;
            #[doc = "Do not block"]
            pub const DISABLE: u32 = 1;
        }
    }
}
pub mod wakeup {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Wakeup 0 Register A"]
        pub WAKEUPA: crate::RWRegister<u32>,
        #[doc = "Wakeup 0 Register B"]
        pub WAKEUPB: crate::RWRegister<u32>,
    }
    #[doc = "Wakeup 0 Register A"]
    pub mod WAKEUPA {
        #[doc = "Register"]
        pub mod REG {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Wakeup 0 Register B"]
    pub mod WAKEUPB {
        #[doc = "Inverse value"]
        pub mod INVERSE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
#[doc = "Wakeup Lock A"]
pub mod WAKLCKA {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Lock is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Wakeup Lock B"]
pub mod WAKLCKB {
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Lock is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Lock is disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
}
