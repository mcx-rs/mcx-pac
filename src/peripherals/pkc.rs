#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "no description available"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Status register"]
    pub pkc_status: crate::RWRegister<u32>,
    #[doc = "Control register"]
    pub pkc_ctrl: crate::RWRegister<u32>,
    #[doc = "Configuration register"]
    pub pkc_cfg: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "Mode register, parameter set 1"]
    pub pkc_mode1: crate::RWRegister<u32>,
    #[doc = "X+Y pointer register, parameter set 1"]
    pub pkc_xyptr1: crate::RWRegister<u32>,
    #[doc = "Z+R pointer register, parameter set 1"]
    pub pkc_zrptr1: crate::RWRegister<u32>,
    #[doc = "Length register, parameter set 1"]
    pub pkc_len1: crate::RWRegister<u32>,
    #[doc = "Mode register, parameter set 2"]
    pub pkc_mode2: crate::RWRegister<u32>,
    #[doc = "X+Y pointer register, parameter set 2"]
    pub pkc_xyptr2: crate::RWRegister<u32>,
    #[doc = "Z+R pointer register, parameter set 2"]
    pub pkc_zrptr2: crate::RWRegister<u32>,
    #[doc = "Length register, parameter set 2"]
    pub pkc_len2: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "Universal pointer FUP program"]
    pub pkc_uptr: crate::RWRegister<u32>,
    #[doc = "Universal pointer FUP table"]
    pub pkc_uptrt: crate::RWRegister<u32>,
    #[doc = "Universal pointer length"]
    pub pkc_ulen: crate::RWRegister<u32>,
    _reserved2: [u8; 0x4],
    #[doc = "MC pattern data interface"]
    pub pkc_mcdata: crate::RWRegister<u32>,
    _reserved3: [u8; 0xc],
    #[doc = "PKC version register"]
    pub pkc_version: crate::RWRegister<u32>,
    _reserved4: [u8; 0xf4c],
    #[doc = "Software reset"]
    pub pkc_soft_rst: crate::RWRegister<u32>,
    _reserved5: [u8; 0xc],
    #[doc = "Access Error"]
    pub pkc_access_err: crate::RWRegister<u32>,
    #[doc = "Clear Access Error"]
    pub pkc_access_err_clr: crate::RWRegister<u32>,
    _reserved6: [u8; 0x10],
    #[doc = "Interrupt enable clear"]
    pub pkc_int_clr_enable: crate::RWRegister<u32>,
    #[doc = "Interrupt enable set"]
    pub pkc_int_set_enable: crate::RWRegister<u32>,
    #[doc = "Interrupt status"]
    pub pkc_int_status: crate::RWRegister<u32>,
    #[doc = "Interrupt enable"]
    pub pkc_int_enable: crate::RWRegister<u32>,
    #[doc = "Interrupt status clear"]
    pub pkc_int_clr_status: crate::RWRegister<u32>,
    #[doc = "Interrupt status set"]
    pub pkc_int_set_status: crate::RWRegister<u32>,
    _reserved7: [u8; 0xc],
    #[doc = "Module ID"]
    pub pkc_module_id: crate::RWRegister<u32>,
}
#[doc = "Status register"]
pub mod pkc_status {
    #[doc = "PKC active: ACTIV=1 signals that a calculation is in progress or about to start. At the end of a calculation ACTIV is automatically reset to logic 0 in case no further GO bit is set. If the next PKC operation has been started by setting a GO bit during a calculation, ACTIV remains high. ACTIV is always \'1\' in case PKC_STATUS.GOANY is set. ACTIV is always \'0\' in case PKC_CTRL_RESET is set."]
    pub mod activ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Carry overflow flag: CARRY is set by the PKC at the end of a calculation in case"]
    pub mod carry {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Zero result flag: ZERO is set by the PKC at the end of a calculation in case the result of the calculation is equal zero. ZERO is updated for each PKC calculation mode, except for MUL1 (opcode 0x20) and MUL1_GF2 (opcode 0x24)."]
    pub mod zero {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Combined GO status flag: GOANY is set in case either PKC_CTRL.GOD1, GOD2, GOM1, GOM2 or GOU is set. The 1-to-0 transition of GOANY indicates that a calculation has been started and that a new GO bit can be set. If GOANY is cleared also all PKC_STATUS.LOCKED bits are cleared to indicate that the parameter set can be updated. GOANY is always \'0\' in case PKC_CTRL.RESET is set."]
    pub mod goany {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parameter set locked: Indicates if parameter set is locked due to a pending calculation start or can be overwritten."]
    pub mod locked {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control register"]
pub mod pkc_ctrl {
    #[doc = "PKC reset control bit: RESET=1 enforces the PKC\'s reset state during which a calculation cannot be started and by which any ongoing calculation process is stopped. RESET can be set/cleared by the CPU in order to switch between PKC reset and calculation enable. RESET=1 is the default state after a chip reset."]
    pub mod reset {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Freeze PKC calculation: STOP=1 freezes all PKC activity incl. RAM accesses and reduces the PKC power consumption to its minimum. The difference compared to the reset of the PKC is that a stopped calculation can be continued when STOP is released (reset to \'0\') again. The status flags are not affected by the STOP control bit."]
    pub mod stop {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to start direct operation using parameter set 1: If GOD1 is set PKC will start a direct / L0 operation using parameter set 1 (PKC_MODE1, PKC_XYPTR1, PKC_ZRPTR1, PKC_LEN1)."]
    pub mod god1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to start direct operation using parameter set 2: If GOD2 is set PKC will start a direct / L0 operation using parameter set 2 (PKC_MODE2, PKC_XYPTR2, PKC_ZRPTR2, PKC_LEN2)."]
    pub mod god2 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to start MC pattern using parameter set 1: If GOM1 is set PKC will start a MC pattern / L1 operation using parameter set 1 (PKC_MODE1, PKC_XYPTR1, PKC_ZRPTR1, PKC_LEN1)."]
    pub mod gom1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to start MC pattern using parameter set 2: If GOM2 is set PKC will start a MC pattern / L1 operation using parameter set 2 (PKC_MODE2, PKC_XYPTR2, PKC_ZRPTR2, PKC_LEN2)."]
    pub mod gom2 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Control bit to start pipe operation: If GOU is set PKC will start the pipe / L2 operation (parameter fetch & calculation) described in section \'PKC Universal Pointer Fetch Operation\'."]
    pub mod gou {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Convert to GF2 calculation modes: If GF2CONV is set operations are mapped to their GF(2) equivalent operation modes."]
    pub mod gf2conv {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear universal pointer cache: Invalidates the cache such that all previously fetched parameters are withdrawn and have to be fetched again via DMA accesses."]
    pub mod clrcache {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable universal pointer cache: If CACHE_EN=1 the cache for the universal pointer parameters is enabled. In case a parameter value is found in the cache (from a previous fetch) no DMA access is triggered. As such the amount of DMA accesses for the parameter fetch vary between 0 and 4. To further optimize the cache utilization not used parameters, e.g. XPTR for a plain addition (opcode 0x0A), could be defined equal to a used one (e.g. equal YPTR or RPTR) or a previously fetched parameter."]
    pub mod cache_en {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reduced multiplier mode: REDMUL defines the operand width processed by the PKC coprocessor."]
    pub mod redmul {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "full size mode, 3 least significant bits of pointer and length are ignored, minimum supported length 0x0008"]
            pub const FULLSZ: u32 = 0;
            #[doc = "RFU Error Generated if selected"]
            pub const VALUE_32BIT: u32 = 1;
            #[doc = "64-bit mode, 3 least significant bits of pointer and length are ignored, minimum supported length 0x0008"]
            pub const VALUE_64BIT: u32 = 2;
            #[doc = "RFU Error Generated if selected"]
            pub const VALUE_128BIT: u32 = 3;
        }
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Configuration register"]
pub mod pkc_cfg {
    #[doc = "Idle operation feature not available in this version (flag is don\'t care)."]
    pub mod idleop {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RFU"]
    pub mod rfu1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RFU"]
    pub mod rfu2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock randomization feature not available in this version (flag is don\'t care)."]
    pub mod clkrnd {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Noise in reduced multiplier mode feature not available in this version (flag is don\'t care)."]
    pub mod redmulnoise {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Random delay feature not available in this version (flag is don\'t care)."]
    pub mod rnddly {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Noise feature not available in this version (flag is don\'t care)."]
    pub mod sbxnoise {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Noise feature not available in this version (flag is don\'t care)."]
    pub mod alpnoise {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Noise feature not available in this version (flag is don\'t care)."]
    pub mod fmulnoise {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Mode register, parameter set 1"]
pub mod pkc_mode1 {
    #[doc = "Calculation Mode / MC Start address:"]
    pub mod mode {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "X+Y pointer register, parameter set 1"]
pub mod pkc_xyptr1 {
    #[doc = "Start address of X operand in PKCRAM with byte granularity: Least significant bits are ignored depending on PKC_CTRL.REDMUL setting. Most significant bits are ignored depending on available PKCRAM size."]
    pub mod xptr {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start address of Y operand in PKCRAM with byte granularity: Least significant bits are ignored depending on PKC_CTRL.REDMUL setting. Most significant bits are ignored depending on available PKCRAM size."]
    pub mod yptr {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Z+R pointer register, parameter set 1"]
pub mod pkc_zrptr1 {
    #[doc = "Start address of Z operand in PKCRAM with byte granularity or constant for calculation modes using CONST:"]
    pub mod zptr {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start address of R result in PKCRAM with byte granularity: Least significant bits are ignored depending on PKC_CTRL.REDMUL setting. Most significant bits are ignored depending on available PKCRAM size."]
    pub mod rptr {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Length register, parameter set 1"]
pub mod pkc_len1 {
    #[doc = "Operand length: LEN defines the length of the operands and the result in bytes. The length of Y, Z and R depend furthermore on the selected calculation mode."]
    pub mod len {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loop counter for microcode pattern: MCLEN defines the length of the loop counter that can be used in L1 calculation mode, e.g. in MC opcode DecrTBNZ. For the hardcoded MC patterns Modular Multiplication (MC start address 0x00), Plain Multiplication (0x13), Plain Multiplication with Addition (0x1D) and Modular Reduction (0x33) MCLEN defines the length of the X operand in bytes."]
    pub mod mclen {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Mode register, parameter set 2"]
pub mod pkc_mode2 {
    #[doc = "Calculation Mode / MC Start address:"]
    pub mod mode {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "X+Y pointer register, parameter set 2"]
pub mod pkc_xyptr2 {
    #[doc = "Start address of X operand in PKCRAM with byte granularity: Least significant bits are ignored depending on PKC_CTRL.REDMUL setting. Most significant bits are ignored depending on available PKCRAM size."]
    pub mod xptr {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start address of Y operand in PKCRAM with byte granularity: Least significant bits are ignored depending on PKC_CTRL.REDMUL setting. Most significant bits are ignored depending on available PKCRAM size."]
    pub mod yptr {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Z+R pointer register, parameter set 2"]
pub mod pkc_zrptr2 {
    #[doc = "Start address of Z operand in PKCRAM with byte granularity or constant for calculation modes using CONST:"]
    pub mod zptr {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start address of R result in PKCRAM with byte granularity: Least significant bits are ignored depending on PKC_CTRL.REDMUL setting. Most significant bits are ignored depending on available PKCRAM size."]
    pub mod rptr {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Length register, parameter set 2"]
pub mod pkc_len2 {
    #[doc = "Operand length: LEN defines the length of the operands and the result in bytes. The length of Y, Z and R depend furthermore on the selected calculation mode."]
    pub mod len {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loop counter for microcode pattern: MCLEN defines the length of the loop counter that can be used in L1 calculation mode, e.g. in MC opcode DecrTBNZ. For the hardcoded MC patterns Modular Multiplication (MC start address 0x00)"]
    pub mod mclen {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Universal pointer FUP program"]
pub mod pkc_uptr {
    #[doc = "Pointer to start address of PKC FUP program: PKC_UPTR needs to be defined before starting a universal pointer PKC calculation (L2) via PKC_CTRL.GOU. The pointer address needs to be valid and the memory space the pointer addresses needs to be enabled for PKC access by the system. Otherwise a security alarm is triggered (PKC_ACCESS_ERR.AHB is set)."]
    pub mod ptr {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Universal pointer FUP table"]
pub mod pkc_uptrt {
    #[doc = "Pointer to start address of PKC FUP table: PKC_UPTRT needs to be defined before starting a universal pointer PKC calculation (L2) via PKC_CTRL.GOU. The pointer address needs to be valid and the memory space the pointer addresses needs to be enabled for PKC access by the system. Otherwise a security alarm is triggered (PKC_ACCESS_ERR.AHB is set)."]
    pub mod ptr {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Universal pointer length"]
pub mod pkc_ulen {
    #[doc = "Length of universal pointer calculation: PKC_ULEN defines how many FUP program entries shall be processed for one L2 calculation started via PKC_CTRL.GOU. The FUP program entries include L0 calculations, L1 calculations and CRC entries for FUP program integrity protection."]
    pub mod len {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MC pattern data interface"]
pub mod pkc_mcdata {
    #[doc = "Microcode read/write data: This IP version does not support flexible MC patterns (only hard coded ones). Any read or write access to PKC_MCDATA triggers a security alarm (PKC_ACCESS_ERR.CTRL is set)."]
    pub mod mcdata {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PKC version register"]
pub mod pkc_version {
    #[doc = "native multiplier size and operand granularity"]
    pub mod mulsize {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "32-bit multiplier"]
            pub const VALUE_32B: u32 = 1;
            #[doc = "64-bit multiplier"]
            pub const VALUE_64B: u32 = 2;
            #[doc = "128-bit multiplier"]
            pub const VALUE_128B: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MC feature (L1 calculation) is available"]
    pub mod mcavail {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UP feature (L2 calculation) is available"]
    pub mod upavail {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UP cache is available"]
    pub mod upcacheavail {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "GF2 calculation modes are available"]
    pub mod gf2avail {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of parameter sets for real calculation"]
    pub mod paramnum {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SBX0 operation is available"]
    pub mod sbx0avail {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SBX1 operation is available"]
    pub mod sbx1avail {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SBX2 operation is available"]
    pub mod sbx2avail {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SBX3 operation is available"]
    pub mod sbx3avail {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Size of reconfigurable MC table in bytes"]
    pub mod mcreconf_size {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Software reset"]
pub mod pkc_soft_rst {
    #[doc = "Write 1 to reset module (0 has no effect). All running and pending PKC calculation are stopped. All PKC SFRs are reset except PKC_ACCESS_ERR."]
    pub mod soft_rst {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Access Error"]
pub mod pkc_access_err {
    #[doc = "APB Error: address not available"]
    pub mod apb_notav {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APB Error: Wrong access mode"]
    pub mod apb_wrgmd {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved for future erors on APB I/F"]
    pub mod reserved3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "APB Master that triggered first APB error (APB_WRGMD or APB_NOTAV)"]
    pub mod apb_master {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved for future erors on AHB I/F L2 Only"]
    pub mod reserved9 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Error: invalid AHB access L2 Only"]
    pub mod ahb {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved for future erors on AHB I/F L2 Only"]
    pub mod reserved15 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error in PKC coprocessor kernel"]
    pub mod pkcc {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error due to error detection circuitry"]
    pub mod fdet {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error in PKC software control"]
    pub mod ctrl {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error in L2 CRC check"]
    pub mod ucrc {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved for more block errors"]
    pub mod reserved31 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b11111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clear Access Error"]
pub mod pkc_access_err_clr {
    #[doc = "Write 1 to reset PKC_ACCESS_ERR SFR."]
    pub mod err_clr {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt enable clear"]
pub mod pkc_int_clr_enable {
    #[doc = "Write to clear PDONE interrupt enable flag (PKC_INT_ENABLE.EN_PDONE=0)."]
    pub mod en_pdone {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt enable set"]
pub mod pkc_int_set_enable {
    #[doc = "Write to set PDONE interrupt enable flag (PKC_INT_ENABLE.EN_PDONE=1)."]
    pub mod en_pdone {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt status"]
pub mod pkc_int_status {
    #[doc = "End-of-computation status flag: INT_PDONE is set after EACH single PKC L0 or L1 calculation. In case of a universal pointer calculation (L2) INT_PDONE is set at the end of the pipe calculation when PKC_ULEN has been decremented to zero and the final PKC calculation has completed."]
    pub mod int_pdone {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt enable"]
pub mod pkc_int_enable {
    #[doc = "PDONE interrupt enable flag: If EN_PDONE=1 an interrupt is triggered every time PKC_INT_STATUS.INT_PDONE is set. Otherwise the interrupt generation is suppressed."]
    pub mod en_pdone {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt status clear"]
pub mod pkc_int_clr_status {
    #[doc = "Write to clear End-of-computation status flag (PKC_INT_STATUS.INT_PDONE=0)."]
    pub mod int_pdone {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt status set"]
pub mod pkc_int_set_status {
    #[doc = "Write to set End-of-computation status flag (PKC_INT_STATUS.INT_PDONE=1) to trigger a PKC interrupt via software, e.g. for debug purposes."]
    pub mod int_pdone {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "reserved"]
    pub mod reserved31 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Module ID"]
pub mod pkc_module_id {
    #[doc = "Address space of the IP"]
    pub mod size {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minor revision"]
    pub mod minor_rev {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major revision"]
    pub mod major_rev {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Module ID"]
    pub mod id {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
