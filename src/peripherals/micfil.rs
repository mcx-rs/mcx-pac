#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "MICFIL"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "MICFIL Control 1"]
    pub CTRL_1: crate::RWRegister<u32>,
    #[doc = "MICFIL Control 2"]
    pub CTRL_2: crate::RWRegister<u32>,
    #[doc = "MICFIL Status"]
    pub STAT: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "MICFIL FIFO Control"]
    pub FIFO_CTRL: crate::RWRegister<u32>,
    #[doc = "MICFIL FIFO Status"]
    pub FIFO_STAT: crate::RWRegister<u32>,
    _reserved1: [u8; 0xc],
    #[doc = "MICFIL Output Result"]
    pub DATACH: [crate::RWRegister<u32>; 4usize],
    _reserved2: [u8; 0x30],
    #[doc = "MICFIL DC Remover Control"]
    pub DC_CTRL: crate::RWRegister<u32>,
    #[doc = "MICFIL Output DC Remover Control"]
    pub DC_OUT_CTRL: crate::RWRegister<u32>,
    _reserved3: [u8; 0x8],
    #[doc = "MICFIL Range Control"]
    pub RANGE_CTRL: crate::RWRegister<u32>,
    _reserved4: [u8; 0x4],
    #[doc = "MICFIL Range Status"]
    pub RANGE_STAT: crate::RWRegister<u32>,
    #[doc = "Frame Synchronization Control"]
    pub FSYNC_CTRL: crate::RWRegister<u32>,
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RWRegister<u32>,
}
#[doc = "MICFIL Control 1"]
pub mod CTRL_1 {
    #[doc = "Channel 0 Enable"]
    pub mod CH0EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 1 Enable"]
    pub mod CH1EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 2 Enable"]
    pub mod CH2EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 3 Enable"]
    pub mod CH3EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Synchronization Enable"]
    pub mod FSYNCEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Decimation Filter Enable in Stop"]
    pub mod DECFILS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stops decimation filter"]
            pub const STOP: u32 = 0;
            #[doc = "Keeps decimation filter running"]
            pub const RUN: u32 = 1;
        }
    }
    #[doc = "Error Interruption Enable"]
    pub mod ERREN {
        pub const offset: u32 = 23;
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
    #[doc = "DMA Interrupt Selection"]
    pub mod DISEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables DMA and interrupt requests"]
            pub const ALL_DISABLED: u32 = 0;
            #[doc = "Enables DMA requests"]
            pub const DMAREQ_ENABLED: u32 = 1;
            #[doc = "Enables interrupt requests"]
            pub const INTREQ_ENABLED: u32 = 2;
        }
    }
    #[doc = "Module Enable in Debug"]
    pub mod DBGE {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables after completing the current frame"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables operation"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Software Reset"]
    pub mod SRES {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const NO_ACTION: u32 = 0;
            #[doc = "Software reset"]
            pub const SW_RESET: u32 = 1;
        }
    }
    #[doc = "Debug Mode"]
    pub mod DBG {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal"]
            pub const NORMAL: u32 = 0;
            #[doc = "Debug"]
            pub const DEBUG: u32 = 1;
        }
    }
    #[doc = "MICFIL Enable"]
    pub mod PDMIEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stops MICFIL operation"]
            pub const STOPPED: u32 = 0;
            #[doc = "Starts MICFIL operation"]
            pub const STARTED: u32 = 1;
        }
    }
    #[doc = "Doze Enable"]
    pub mod DOZEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Module Disable"]
    pub mod MDIS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal mode"]
            pub const NORMAL: u32 = 0;
            #[doc = "DLL mode"]
            pub const LOW_LEAKAGE: u32 = 1;
        }
    }
}
#[doc = "MICFIL Control 2"]
pub mod CTRL_2 {
    #[doc = "Clock Divider"]
    pub mod CLKDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Divider Disable"]
    pub mod CLKDIVDIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disables"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "CIC Decimation Rate"]
    pub mod CICOSR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Quality Mode"]
    pub mod QSEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Medium-Quality mode"]
            pub const MQ_MODE: u32 = 0;
            #[doc = "High-Quality mode"]
            pub const HQ_MODE: u32 = 1;
            #[doc = "Very-Low-Quality 2 mode"]
            pub const VLQ2_MODE: u32 = 4;
            #[doc = "Very-Low-Quality 1 mode"]
            pub const VLQ1_MODE: u32 = 5;
            #[doc = "Very-Low-Quality 0 mode"]
            pub const VLQ0_MODE: u32 = 6;
            #[doc = "Low-Quality mode"]
            pub const LQ_MODE: u32 = 7;
        }
    }
}
#[doc = "MICFIL Status"]
pub mod STAT {
    #[doc = "Channel 0 Output Data Flag"]
    pub mod CH0F {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not surpassed"]
            pub const WM_NOTREACHED: u32 = 0;
            #[doc = "Surpassed"]
            pub const WM_REACHED: u32 = 1;
        }
    }
    #[doc = "Channel 1 Output Data Flag"]
    pub mod CH1F {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not surpassed"]
            pub const WM_NOTREACHED: u32 = 0;
            #[doc = "Surpassed"]
            pub const WM_REACHED: u32 = 1;
        }
    }
    #[doc = "Channel 2 Output Data Flag"]
    pub mod CH2F {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not surpassed"]
            pub const WM_NOTREACHED: u32 = 0;
            #[doc = "Surpassed"]
            pub const WM_REACHED: u32 = 1;
        }
    }
    #[doc = "Channel 3 Output Data Flag"]
    pub mod CH3F {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not surpassed"]
            pub const WM_NOTREACHED: u32 = 0;
            #[doc = "Surpassed"]
            pub const WM_REACHED: u32 = 1;
        }
    }
    #[doc = "Busy Flag"]
    pub mod BSY_FIL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "MICFIL is stopped"]
            pub const STOPPED: u32 = 0;
            #[doc = "MICFIL is running"]
            pub const RUNNING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MICFIL FIFO Control"]
pub mod FIFO_CTRL {
    #[doc = "FIFO Watermark Control"]
    pub mod FIFOWMK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MICFIL FIFO Status"]
pub mod FIFO_STAT {
    #[doc = "FIFO Overflow Exception Flag for Channel 0"]
    pub mod FIFOOVF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO overflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Exception Flag for Channel 1"]
    pub mod FIFOOVF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO overflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Exception Flag for Channel 2"]
    pub mod FIFOOVF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO overflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "FIFO Overflow Exception Flag for Channel 3"]
    pub mod FIFOOVF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO overflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Exception Flag for Channel 0"]
    pub mod FIFOUND0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO underflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Exception Flag for Channel 1"]
    pub mod FIFOUND1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO underflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Exception Flag for Channel 2"]
    pub mod FIFOUND2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO underflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "FIFO Underflow Exception Flag for Channel 3"]
    pub mod FIFOUND3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by FIFO underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by FIFO underflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
}
#[doc = "MICFIL Output Result"]
pub mod DATACH {
    #[doc = "Channel n Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MICFIL DC Remover Control"]
pub mod DC_CTRL {
    #[doc = "Channel 0 DC Remover Configuration"]
    pub mod DCCONFIG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "20 Hz (PDM_CLK = 3.072 MHz)"]
            pub const DC_REM_20HZ: u32 = 0;
            #[doc = "13.3 Hz (PDM_CLK = 3.072 MHz)"]
            pub const DC_REM_13P3HZ: u32 = 1;
            #[doc = "40 Hz (PDM_CLK = 3.072 MHz)"]
            pub const DC_REM_40HZ: u32 = 2;
            #[doc = "DC remover is bypassed"]
            pub const DC_REM_BYPASS: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 1 DC Remover Configuration"]
    pub mod DCCONFIG1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "20 Hz (PDM_CLK = 3.072 MHz)"]
            pub const DC_REM_20HZ: u32 = 0;
            #[doc = "13.3 Hz (PDM_CLK = 3.072 MHz)"]
            pub const DC_REM_13P3HZ: u32 = 1;
            #[doc = "40 Hz (PDM_CLK = 3.072 MHz)"]
            pub const DC_REM_40HZ: u32 = 2;
            #[doc = "DC remover is bypassed"]
            pub const DC_REM_BYPASS: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 2 DC Remover Configuration"]
    pub mod DCCONFIG2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "20 Hz (PDM_CLK = 3.072 MHz)"]
            pub const DC_REM_20HZ: u32 = 0;
            #[doc = "13.3 Hz (PDM_CLK = 3.072 MHz)"]
            pub const DC_REM_13P3HZ: u32 = 1;
            #[doc = "40 Hz (PDM_CLK = 3.072 MHz)"]
            pub const DC_REM_40HZ: u32 = 2;
            #[doc = "DC remover is bypassed"]
            pub const DC_REM_BYPASS: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 3 DC Remover Configuration"]
    pub mod DCCONFIG3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "20 Hz (PDM_CLK = 3.072 MHz)"]
            pub const DC_REM_20HZ: u32 = 0;
            #[doc = "13.3 Hz (PDM_CLK = 3.072 MHz)"]
            pub const DC_REM_13P3HZ: u32 = 1;
            #[doc = "40 Hz (PDM_CLK = 3.072 MHz)"]
            pub const DC_REM_40HZ: u32 = 2;
            #[doc = "DC remover is bypassed"]
            pub const DC_REM_BYPASS: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MICFIL Output DC Remover Control"]
pub mod DC_OUT_CTRL {
    #[doc = "Channel 0 DC Remover Configuration"]
    pub mod DCCONFIG0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "20 Hz (FS = 48 kHz)"]
            pub const DC_REM_20HZ: u32 = 0;
            #[doc = "13.3 Hz (FS = 48 kHz)"]
            pub const DC_REM_13P3HZ: u32 = 1;
            #[doc = "40 Hz (FS = 48 kHz)"]
            pub const DC_REM_40HZ: u32 = 2;
            #[doc = "DC remover is bypassed"]
            pub const DC_REM_BYPASSED: u32 = 3;
        }
    }
    #[doc = "Channel 1 DC Remover Configuration"]
    pub mod DCCONFIG1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "20 Hz (FS = 48 kHz)"]
            pub const DC_REM_20HZ: u32 = 0;
            #[doc = "13.3 Hz (FS = 48 kHz)"]
            pub const DC_REM_13P3HZ: u32 = 1;
            #[doc = "40 Hz (FS = 48 kHz)"]
            pub const DC_REM_40HZ: u32 = 2;
            #[doc = "DC remover is bypassed"]
            pub const DC_REM_BYPASSED: u32 = 3;
        }
    }
    #[doc = "Channel 2 DC Remover Configuration"]
    pub mod DCCONFIG2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "20 Hz (FS = 48 kHz)"]
            pub const DC_REM_20HZ: u32 = 0;
            #[doc = "13.3 Hz (FS = 48 kHz)"]
            pub const DC_REM_13P3HZ: u32 = 1;
            #[doc = "40 Hz (FS = 48 kHz)"]
            pub const DC_REM_40HZ: u32 = 2;
            #[doc = "DC remover is bypassed"]
            pub const DC_REM_BYPASSED: u32 = 3;
        }
    }
    #[doc = "Channel 3 DC Remover Configuration"]
    pub mod DCCONFIG3 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "20 Hz (FS = 48 kHz)"]
            pub const DC_REM_20HZ: u32 = 0;
            #[doc = "13.3 Hz (FS = 48 kHz)"]
            pub const DC_REM_13P3HZ: u32 = 1;
            #[doc = "40 Hz (FS = 48 kHz)"]
            pub const DC_REM_40HZ: u32 = 2;
            #[doc = "DC remover is bypassed"]
            pub const DC_REM_BYPASSED: u32 = 3;
        }
    }
}
#[doc = "MICFIL Range Control"]
pub mod RANGE_CTRL {
    #[doc = "Channel 0 Range Adjustment"]
    pub mod RANGEADJ0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 1 Range Adjustment"]
    pub mod RANGEADJ1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 2 Range Adjustment"]
    pub mod RANGEADJ2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel 3 Range Adjustment"]
    pub mod RANGEADJ3 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MICFIL Range Status"]
pub mod RANGE_STAT {
    #[doc = "Channel 0 Range Overflow Error Flag"]
    pub mod RANGEOVF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range overflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "Channel 1 Range Overflow Error Flag"]
    pub mod RANGEOVF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range overflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "Channel 2 Range Overflow Error Flag"]
    pub mod RANGEOVF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range overflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "Channel 3 Range Overflow Error Flag"]
    pub mod RANGEOVF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range overflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range overflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "Channel 0 Range Underflow Error Flag"]
    pub mod RANGEUNF0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range underflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "Channel 1 Range Underflow Error Flag"]
    pub mod RANGEUNF1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range underflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "Channel 2 Range Underflow Error Flag"]
    pub mod RANGEUNF2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range underflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
    #[doc = "Channel 3 Range Underflow Error Flag"]
    pub mod RANGEUNF3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No exception by range underflow"]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "Exception by range underflow"]
            pub const EXCEPTION: u32 = 1;
        }
    }
}
#[doc = "Frame Synchronization Control"]
pub mod FSYNC_CTRL {
    #[doc = "Frame Synchronization Window Length"]
    pub mod FSYNCLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
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
#[doc = "Parameter"]
pub mod PARAM {
    #[doc = "Number of Microphone Pairs"]
    pub mod NPAIR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Pointer Width"]
    pub mod FIFO_PTRWID {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Filter Output Width"]
    pub mod FIL_OUT_WIDTH_24B {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "16 bits"]
            pub const WID16B: u32 = 0;
            #[doc = "24 bits"]
            pub const WID24B: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low-Power Decimation Filter"]
    pub mod LOW_POWER {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Disables"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input DC Remover Bypass"]
    pub mod DC_BYPASS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Active"]
            pub const DCACTIVE: u32 = 0;
            #[doc = "Disabled"]
            pub const DCBYPASSED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output DC Remover Bypass"]
    pub mod DC_OUT_BYPASS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Active"]
            pub const DCACTIVE: u32 = 0;
            #[doc = "Disabled"]
            pub const DCBYPASSED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
