#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "CDOG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub CONTROL: crate::RWRegister<u32>,
    #[doc = "Instruction Timer Reload Register"]
    pub RELOAD: crate::RWRegister<u32>,
    #[doc = "Instruction Timer Register"]
    pub INSTRUCTION_TIMER: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "Status 1 Register"]
    pub STATUS: crate::RWRegister<u32>,
    #[doc = "Status 2 Register"]
    pub STATUS2: crate::RWRegister<u32>,
    #[doc = "Flags Register"]
    pub FLAGS: crate::RWRegister<u32>,
    #[doc = "Persistent Data Storage Register"]
    pub PERSISTENT: crate::RWRegister<u32>,
    #[doc = "START Command Register"]
    pub START: crate::RWRegister<u32>,
    #[doc = "STOP Command Register"]
    pub STOP: crate::RWRegister<u32>,
    #[doc = "RESTART Command Register"]
    pub RESTART: crate::RWRegister<u32>,
    #[doc = "ADD Command Register"]
    pub ADD: crate::RWRegister<u32>,
    #[doc = "ADD1 Command Register"]
    pub ADD1: crate::RWRegister<u32>,
    #[doc = "ADD16 Command Register"]
    pub ADD16: crate::RWRegister<u32>,
    #[doc = "ADD256 Command Register"]
    pub ADD256: crate::RWRegister<u32>,
    #[doc = "SUB Command Register"]
    pub SUB: crate::RWRegister<u32>,
    #[doc = "SUB1 Command Register"]
    pub SUB1: crate::RWRegister<u32>,
    #[doc = "SUB16 Command Register"]
    pub SUB16: crate::RWRegister<u32>,
    #[doc = "SUB256 Command Register"]
    pub SUB256: crate::RWRegister<u32>,
    #[doc = "ASSERT16 Command Register"]
    pub ASSERT16: crate::RWRegister<u32>,
}
#[doc = "Control Register"]
pub mod CONTROL {
    #[doc = "Lock control"]
    pub mod LOCK_CTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked"]
            pub const LOCKED: u32 = 1;
            #[doc = "Unlocked"]
            pub const UNLOCKED: u32 = 2;
        }
    }
    #[doc = "TIMEOUT fault control"]
    pub mod TIMEOUT_CTRL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 1;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 2;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 4;
        }
    }
    #[doc = "MISCOMPARE fault control"]
    pub mod MISCOMPARE_CTRL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 1;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 2;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 4;
        }
    }
    #[doc = "SEQUENCE fault control"]
    pub mod SEQUENCE_CTRL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 1;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 2;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 4;
        }
    }
    #[doc = "STATE fault control"]
    pub mod STATE_CTRL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 1;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 2;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 4;
        }
    }
    #[doc = "ADDRESS fault control"]
    pub mod ADDRESS_CTRL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable reset"]
            pub const ENABLE_RESET: u32 = 1;
            #[doc = "Enable interrupt"]
            pub const ENABLE_INTERRUPT: u32 = 2;
            #[doc = "Disable both reset and interrupt"]
            pub const DISABLE_BOTH: u32 = 4;
        }
    }
    #[doc = "IRQ pause control"]
    pub mod IRQ_PAUSE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep the timer running"]
            pub const RUN_TIMER: u32 = 1;
            #[doc = "Stop the timer"]
            pub const PAUSE_TIMER: u32 = 2;
        }
    }
    #[doc = "DEBUG_HALT control"]
    pub mod DEBUG_HALT_CTRL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep the timer running"]
            pub const RUN_TIMER: u32 = 1;
            #[doc = "Stop the timer"]
            pub const PAUSE_TIMER: u32 = 2;
        }
    }
}
#[doc = "Instruction Timer Reload Register"]
pub mod RELOAD {
    #[doc = "Instruction Timer reload value"]
    pub mod RLOAD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Instruction Timer Register"]
pub mod INSTRUCTION_TIMER {
    #[doc = "Current value of the Instruction Timer"]
    pub mod INSTIM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status 1 Register"]
pub mod STATUS {
    #[doc = "Number of TIMEOUT faults since the last POR"]
    pub mod NUMTOF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of MISCOMPARE faults since the last POR"]
    pub mod NUMMISCOMPF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of SEQUENCE faults since the last POR"]
    pub mod NUMILSEQF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current State"]
    pub mod CURST {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status 2 Register"]
pub mod STATUS2 {
    #[doc = "Number of CONTROL faults since the last POR"]
    pub mod NUMCNTF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of STATE faults since the last POR"]
    pub mod NUMILLSTF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of ADDRESS faults since the last POR"]
    pub mod NUMILLA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flags Register"]
pub mod FLAGS {
    #[doc = "TIMEOUT fault flag"]
    pub mod TO_FLAG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A TIMEOUT fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A TIMEOUT fault has occurred"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "MISCOMPARE fault flag"]
    pub mod MISCOM_FLAG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A MISCOMPARE fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A MISCOMPARE fault has occurred"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "SEQUENCE fault flag"]
    pub mod SEQ_FLAG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A SEQUENCE fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A SEQUENCE fault has occurred"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "CONTROL fault flag"]
    pub mod CNT_FLAG {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A CONTROL fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A CONTROL fault has occurred"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "STATE fault flag"]
    pub mod STATE_FLAG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A STATE fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A STATE fault has occurred"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "ADDRESS fault flag"]
    pub mod ADDR_FLAG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "An ADDRESS fault has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "An ADDRESS fault has occurred"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Power-on reset flag"]
    pub mod POR_FLAG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "A Power-on reset event has not occurred"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "A Power-on reset event has occurred"]
            pub const FLAG: u32 = 1;
        }
    }
}
#[doc = "Persistent Data Storage Register"]
pub mod PERSISTENT {
    #[doc = "Persistent Storage"]
    pub mod PERSIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "START Command Register"]
pub mod START {
    #[doc = "Start command"]
    pub mod STRT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "STOP Command Register"]
pub mod STOP {
    #[doc = "Stop command"]
    pub mod STP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RESTART Command Register"]
pub mod RESTART {
    #[doc = "Restart command"]
    pub mod RSTRT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ADD Command Register"]
pub mod ADD {
    #[doc = "ADD Write Value"]
    pub mod AD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ADD1 Command Register"]
pub mod ADD1 {
    #[doc = "ADD 1"]
    pub mod AD1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ADD16 Command Register"]
pub mod ADD16 {
    #[doc = "ADD 16"]
    pub mod AD16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ADD256 Command Register"]
pub mod ADD256 {
    #[doc = "ADD 256"]
    pub mod AD256 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SUB Command Register"]
pub mod SUB {
    #[doc = "Subtract Write Value"]
    pub mod SB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SUB1 Command Register"]
pub mod SUB1 {
    #[doc = "Subtract 1"]
    pub mod SB1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SUB16 Command Register"]
pub mod SUB16 {
    #[doc = "Subtract 16"]
    pub mod SB16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SUB256 Command Register"]
pub mod SUB256 {
    #[doc = "Subtract 256"]
    pub mod SB256 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASSERT16 Command Register"]
pub mod ASSERT16 {
    #[doc = "ASSERT16 Command"]
    pub mod AST16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
