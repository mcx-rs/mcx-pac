#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "SmartDMA"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    #[doc = "Boot Address"]
    pub BOOTADR: crate::RWRegister<u32>,
    #[doc = "Control"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Program Counter"]
    pub PC: crate::RWRegister<u32>,
    #[doc = "Stack Pointer"]
    pub SP: crate::RWRegister<u32>,
    #[doc = "Breakpoint Address"]
    pub BREAK_ADDR: crate::RWRegister<u32>,
    #[doc = "Breakpoint Vector"]
    pub BREAK_VECT: crate::RWRegister<u32>,
    #[doc = "Emergency Vector"]
    pub EMER_VECT: crate::RWRegister<u32>,
    #[doc = "Emergency Select"]
    pub EMER_SEL: crate::RWRegister<u32>,
    #[doc = "ARM to EZH Interrupt Control"]
    pub ARM2EZH: crate::RWRegister<u32>,
    #[doc = "EZH to ARM Trigger"]
    pub EZH2ARM: crate::RWRegister<u32>,
    #[doc = "Pending Trap Control"]
    pub PENDTRAP: crate::RWRegister<u32>,
}
#[doc = "Boot Address"]
pub mod BOOTADR {
    #[doc = "32-bit boot address, the boot address should be 4-byte aligned."]
    pub mod ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control"]
pub mod CTRL {
    #[doc = "Start Bit Ignition"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Flag"]
    pub mod EXF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Disable"]
    pub mod ERRDIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer Enable"]
    pub mod BUFEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sync Enable"]
    pub mod SYNCEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Key"]
    pub mod WKEY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Program Counter"]
pub mod PC {
    #[doc = "Program Counter"]
    pub mod PC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Stack Pointer"]
pub mod SP {
    #[doc = "Stack Pointer"]
    pub mod SP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Breakpoint Address"]
pub mod BREAK_ADDR {
    #[doc = "32-bit address to swap to EZHB_BREAK_VECT location"]
    pub mod ADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Breakpoint Vector"]
pub mod BREAK_VECT {
    #[doc = "Vector address of user debug routine."]
    pub mod VEC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Emergency Vector"]
pub mod EMER_VECT {
    #[doc = "Vector address of emergency code routine"]
    pub mod VEC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Emergency Select"]
pub mod EMER_SEL {
    #[doc = "Emergency code routine"]
    pub mod EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Software emergency request"]
    pub mod RQ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ARM to EZH Interrupt Control"]
pub mod ARM2EZH {
    #[doc = "Interrupt Enable"]
    pub mod IE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General purpose register bits"]
    pub mod GP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EZH to ARM Trigger"]
pub mod EZH2ARM {
    #[doc = "General purpose register bits Writing to EZH2ARM triggers the ARM interrupt when ARM2EZH [1:0] == 2h"]
    pub mod GP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pending Trap Control"]
pub mod PENDTRAP {
    #[doc = "Status Flag or Pending Trap Request"]
    pub mod STATUS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Polarity"]
    pub mod POL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable Pending Trap"]
    pub mod EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
