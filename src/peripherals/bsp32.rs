#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "CoolFlux BSP32"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Offset address register for program memory"]
    pub offset_pmem: crate::RWRegister<u32>,
    #[doc = "Offset address register for X-data memory"]
    pub offset_xmem: crate::RWRegister<u32>,
    #[doc = "Offset address register for Y-data memory"]
    pub offset_ymem: crate::RWRegister<u32>,
    #[doc = "Offset address register for mailbox peripheral"]
    pub offset_mailbox: crate::RWRegister<u32>,
    #[doc = "External interrupt register"]
    pub interrupts_external: crate::RWRegister<u32>,
    #[doc = "Interrupt status register"]
    pub interrupts_status: crate::RWRegister<u32>,
    #[doc = "CoolFlux BSP32 gating override"]
    pub cf_gating_override: crate::RWRegister<u32>,
    #[doc = "CoolFlux BSP32 IVT offset register"]
    pub ivt_offset: crate::RWRegister<u32>,
    #[doc = "CoolFlux BSP32 sleep mode register"]
    pub sleep_mode: crate::RWRegister<u32>,
    #[doc = "CoolFlux BSP32 IVT register 0 content"]
    pub ivt0: crate::RWRegister<u32>,
    #[doc = "CoolFlux BSP32 IVT register 1 content"]
    pub ivt1: crate::RWRegister<u32>,
    #[doc = "CoolFlux BSP32 IVT register 2 content"]
    pub ivt2: crate::RWRegister<u32>,
    #[doc = "CoolFlux BSP32 IVT register 3 content"]
    pub ivt3: crate::RWRegister<u32>,
    #[doc = "CoolFlux BSP32 IVT disable register"]
    pub ivt_disable: crate::RWRegister<u32>,
}
#[doc = "Offset address register for program memory"]
pub mod offset_pmem {
    #[doc = "Offset address register for program memory"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Offset address register for X-data memory"]
pub mod offset_xmem {
    #[doc = "Offset address register for X-data memory"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Offset address register for Y-data memory"]
pub mod offset_ymem {
    #[doc = "Offset address register for Y-data memory"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Offset address register for mailbox peripheral"]
pub mod offset_mailbox {
    #[doc = "Offset address register for mailbox peripheral"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External interrupt register"]
pub mod interrupts_external {
    #[doc = "External interrupt register"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt status register"]
pub mod interrupts_status {
    #[doc = "Interrupt status register"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CoolFlux BSP32 gating override"]
pub mod cf_gating_override {
    #[doc = "CoolFlux BSP32 gating override"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CoolFlux BSP32 IVT offset register"]
pub mod ivt_offset {
    #[doc = "CoolFlux BSP32 IVT offset register"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CoolFlux BSP32 sleep mode register"]
pub mod sleep_mode {
    #[doc = "CoolFlux BSP32 sleep mode register"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CoolFlux BSP32 IVT register 0 content"]
pub mod ivt0 {
    #[doc = "CoolFlux BSP32 IVT register 0 content"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CoolFlux BSP32 IVT register 1 content"]
pub mod ivt1 {
    #[doc = "CoolFlux BSP32 IVT register 1 content"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CoolFlux BSP32 IVT register 2 content"]
pub mod ivt2 {
    #[doc = "CoolFlux BSP32 IVT register 2 content"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CoolFlux BSP32 IVT register 3 content"]
pub mod ivt3 {
    #[doc = "CoolFlux BSP32 IVT register 3 content"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CoolFlux BSP32 IVT disable register"]
pub mod ivt_disable {
    #[doc = "CoolFlux BSP32 IVT disable register"]
    pub mod val {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
