#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "SAI"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RWRegister<u32>,
    #[doc = "Transmit Control"]
    pub TCSR: crate::RWRegister<u32>,
    #[doc = "Transmit Configuration 1"]
    pub TCR1: crate::RWRegister<u32>,
    #[doc = "Transmit Configuration 2"]
    pub TCR2: crate::RWRegister<u32>,
    #[doc = "Transmit Configuration 3"]
    pub TCR3: crate::RWRegister<u32>,
    #[doc = "Transmit Configuration 4"]
    pub TCR4: crate::RWRegister<u32>,
    #[doc = "Transmit Configuration 5"]
    pub TCR5: crate::RWRegister<u32>,
    #[doc = "Transmit Data"]
    pub TDR: [crate::RWRegister<u32>; 2usize],
    _reserved0: [u8; 0x18],
    #[doc = "Transmit FIFO"]
    pub TFR: [crate::RWRegister<u32>; 2usize],
    _reserved1: [u8; 0x18],
    #[doc = "Transmit Mask"]
    pub TMR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x24],
    #[doc = "Receive Control"]
    pub RCSR: crate::RWRegister<u32>,
    #[doc = "Receive Configuration 1"]
    pub RCR1: crate::RWRegister<u32>,
    #[doc = "Receive Configuration 2"]
    pub RCR2: crate::RWRegister<u32>,
    #[doc = "Receive Configuration 3"]
    pub RCR3: crate::RWRegister<u32>,
    #[doc = "Receive Configuration 4"]
    pub RCR4: crate::RWRegister<u32>,
    #[doc = "Receive Configuration 5"]
    pub RCR5: crate::RWRegister<u32>,
    #[doc = "Receive Data"]
    pub RDR: [crate::RWRegister<u32>; 2usize],
    _reserved3: [u8; 0x18],
    #[doc = "Receive FIFO"]
    pub RFR: [crate::RWRegister<u32>; 2usize],
    _reserved4: [u8; 0x18],
    #[doc = "Receive Mask"]
    pub RMR: crate::RWRegister<u32>,
    _reserved5: [u8; 0x1c],
    #[doc = "MCLK Control"]
    pub MCR: crate::RWRegister<u32>,
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {
            #[doc = "Standard feature set."]
            pub const STD: u32 = 0;
        }
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
    #[doc = "Number of Datalinks"]
    pub mod DATALINE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Size"]
    pub mod FIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Size"]
    pub mod FRAME {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Control"]
pub mod TCSR {
    #[doc = "FIFO Request DMA Enable"]
    pub mod FRDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the DMA request."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the DMA request."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FIFO Warning DMA Enable"]
    pub mod FWDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the DMA warning."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the DMA warning."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FIFO Request Interrupt Enable"]
    pub mod FRIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    pub mod FWIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FIFO Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Sync Error Interrupt Enable"]
    pub mod SEIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables interrupt."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Word Start Interrupt Enable"]
    pub mod WSIE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables interrupt."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FIFO Request Flag"]
    pub mod FRF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit FIFO watermark has not been reached."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Transmit FIFO watermark has been reached."]
            pub const FLAG: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Warning Flag"]
    pub mod FWF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No enabled transmit FIFO is empty."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled transmit FIFO is empty."]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit underrun not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Transmit underrun detected."]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Sync Error Flag"]
    pub mod SEF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sync error not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Frame sync error detected."]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Word Start Flag"]
    pub mod WSF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start of word not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Start of word detected."]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Software Reset"]
    pub mod SR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLE: u32 = 0;
            #[doc = "Software reset"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FIFO Reset"]
    pub mod FR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "FIFO reset."]
            pub const RESET: u32 = 1;
        }
    }
    #[doc = "Bit Clock Enable"]
    pub mod BCE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit bit clock is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit bit clock is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBGE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter is disabled in Debug mode, after completing the current frame."]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmitter is enabled in Debug mode."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Stop Enable"]
    pub mod STOPE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter disabled in Stop mode."]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmitter enabled in Stop mode."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Transmitter Enable"]
    pub mod TE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmitter is enabled, or transmitter has been disabled and has not yet reached end of frame."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Transmit Configuration 1"]
pub mod TCR1 {
    #[doc = "Transmit FIFO Watermark"]
    pub mod TFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 FIFO word"]
            pub const MIN: u32 = 0;
            #[doc = "2 FIFO words"]
            pub const TWO: u32 = 1;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_2: u32 = 2;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_3: u32 = 3;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_4: u32 = 4;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_5: u32 = 5;
            #[doc = "(TFW +1) FIFO words"]
            pub const WATERMARK_VALUE_6: u32 = 6;
            #[doc = "8 FIFO words"]
            pub const MAX: u32 = 7;
        }
    }
}
#[doc = "Transmit Configuration 2"]
pub mod TCR2 {
    #[doc = "Bit Clock Divide"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Clock Bypass"]
    pub mod BYP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal bit clock is generated from bit clock divider."]
            pub const DISABLE: u32 = 0;
            #[doc = "Internal bit clock is divide-by-one of the audio master clock."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Bit Clock Direction"]
    pub mod BCD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock is generated externally in Slave mode."]
            pub const EXT_IN_SLAVE: u32 = 0;
            #[doc = "Bit clock is generated internally in Master mode."]
            pub const INT_IN_MASTER: u32 = 1;
        }
    }
    #[doc = "Bit Clock Polarity"]
    pub mod BCP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
            pub const ACTIVE_LOW: u32 = 1;
        }
    }
    #[doc = "MCLK Select"]
    pub mod MSEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus Clock selected."]
            pub const BUS_CLOCK: u32 = 0;
            #[doc = "Master Clock (MCLK) 1 option selected."]
            pub const MCLK1: u32 = 1;
            #[doc = "Master Clock (MCLK) 2 option selected."]
            pub const MCLK2: u32 = 2;
            #[doc = "Master Clock (MCLK) 3 option selected."]
            pub const MCLK3: u32 = 3;
        }
    }
    #[doc = "Bit Clock Input"]
    pub mod BCI {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const DISABLE: u32 = 0;
            #[doc = "Internal logic is clocked as if bit clock was externally generated."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Bit Clock Swap"]
    pub mod BCS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the normal bit clock source."]
            pub const DISABLE: u32 = 0;
            #[doc = "Swap the bit clock source."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Synchronous Mode"]
    pub mod SYNC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asynchronous mode"]
            pub const ASYNC: u32 = 0;
            #[doc = "Synchronous with receiver"]
            pub const SYNC_W_RX: u32 = 1;
            #[doc = "Synchronous with another SAI transmitter"]
            pub const SYNC_W_TX: u32 = 2;
            #[doc = "Synchronous with another SAI receiver"]
            pub const SYNC_W_ANOTHER_SAI_RX: u32 = 3;
        }
    }
}
#[doc = "Transmit Configuration 3"]
pub mod TCR3 {
    #[doc = "Word Flag Configuration"]
    pub mod WDFL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Channel Enable"]
    pub mod TCE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel FIFO Reset"]
    pub mod CFR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Configuration 4"]
pub mod TCR4 {
    #[doc = "Frame Sync Direction"]
    pub mod FSD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame sync is generated externally in Slave mode."]
            pub const EXT_IN_SLAVE_MODE: u32 = 0;
            #[doc = "Frame sync is generated internally in Master mode."]
            pub const INT_IN_MASTER_MODE: u32 = 1;
        }
    }
    #[doc = "Frame Sync Polarity"]
    pub mod FSP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame sync is active high."]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Frame sync is active low."]
            pub const ACTIVE_LOW: u32 = 1;
        }
    }
    #[doc = "On Demand Mode"]
    pub mod ONDEM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal frame sync is generated continuously."]
            pub const CONTINUOUS_FRAME_SYNC: u32 = 0;
            #[doc = "Internal frame sync is generated when the FIFO warning flag is 0."]
            pub const ON_DEMAND_FRAME_SYNC: u32 = 1;
        }
    }
    #[doc = "Frame Sync Early"]
    pub mod FSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame sync asserts with the first bit of the frame."]
            pub const DISABLE: u32 = 0;
            #[doc = "Frame sync asserts one bit before the first bit of the frame."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "MSB First"]
    pub mod MF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LSB is transmitted first."]
            pub const DISABLE: u32 = 0;
            #[doc = "MSB is transmitted first."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Channel Mode"]
    pub mod CHMOD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TDM mode, transmit data pins are 3-stated when slots are masked or channels are disabled."]
            pub const TDM_MODE: u32 = 0;
            #[doc = "Output mode, transmit data pins are never 3-stated and output zero when slots are masked or channels are disabled."]
            pub const OUTPUT_MODE: u32 = 1;
        }
    }
    #[doc = "Sync Width"]
    pub mod SYWD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame size"]
    pub mod FRSZ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Packing Mode"]
    pub mod FPACK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO packing is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "8-bit FIFO packing is enabled."]
            pub const EIGHT_BIT_FIFO_PACKING: u32 = 2;
            #[doc = "16-bit FIFO packing is enabled."]
            pub const SIXTEEN_BIT_FIFO_PACKING: u32 = 3;
        }
    }
    #[doc = "FIFO Combine Mode"]
    pub mod FCOMB {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO Combine mode disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "FIFO Combine mode enabled on FIFO reads (from transmit shift registers)."]
            pub const ENABLED_ON_FIFO_READS: u32 = 1;
            #[doc = "FIFO Combine mode enabled on FIFO writes (by software)."]
            pub const ENABLED_ON_FIFO_WRITES: u32 = 2;
            #[doc = "FIFO Combine mode enabled on FIFO reads (from transmit shift registers) and writes (by software)."]
            pub const ENABLED_ON_FIFO_READS_WRITES: u32 = 3;
        }
    }
    #[doc = "FIFO Continue on Error"]
    pub mod FCONT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On FIFO error, SAI continues from the start of the next frame after the FIFO error flag has been cleared."]
            pub const DISABLE: u32 = 0;
            #[doc = "On FIFO error, SAI continues from the same word that caused the FIFO error to become 1 after the FIFO warning flag returns to 0."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Transmit Configuration 5"]
pub mod TCR5 {
    #[doc = "First Bit Shifted"]
    pub mod FBT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit index is 0."]
            pub const INDEX0: u32 = 0;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_1: u32 = 1;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_2: u32 = 2;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_3: u32 = 3;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_4: u32 = 4;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_5: u32 = 5;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_6: u32 = 6;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_7: u32 = 7;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_8: u32 = 8;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_9: u32 = 9;
            #[doc = "Bit index is 31."]
            pub const INDEX31: u32 = 31;
        }
    }
    #[doc = "Word 0 Width"]
    pub mod W0W {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8 bits per word"]
            pub const EIGHT: u32 = 7;
            #[doc = "9 bits per word"]
            pub const NINE: u32 = 8;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_9: u32 = 9;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_10: u32 = 10;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_11: u32 = 11;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_12: u32 = 12;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_13: u32 = 13;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_14: u32 = 14;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_15: u32 = 15;
            #[doc = "(W0W value + 1) bits per word"]
            pub const TEN_THIRTYONE_16: u32 = 16;
            #[doc = "32 bits per word"]
            pub const MAX: u32 = 31;
        }
    }
    #[doc = "Word N Width"]
    pub mod WNW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8 bits per word"]
            pub const EIGHT: u32 = 7;
            #[doc = "9 bits per word"]
            pub const NINE: u32 = 8;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_9: u32 = 9;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_10: u32 = 10;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_11: u32 = 11;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_12: u32 = 12;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_13: u32 = 13;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_14: u32 = 14;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_15: u32 = 15;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_16: u32 = 16;
            #[doc = "32 bits per word"]
            pub const MAX: u32 = 31;
        }
    }
}
#[doc = "Transmit Data"]
pub mod TDR {
    #[doc = "Transmit Data Register"]
    pub mod TDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit FIFO"]
pub mod TFR {
    #[doc = "Read FIFO Pointer"]
    pub mod RFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write FIFO Pointer"]
    pub mod WFP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Channel Pointer"]
    pub mod WCP {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No effect"]
            pub const DISABLE: u32 = 0;
            #[doc = "FIFO Combine mode is enabled for FIFO writes and this FIFO will be written on the next FIFO write."]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Mask"]
pub mod TMR {
    #[doc = "Transmit Word Mask"]
    pub mod TWM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Word N is enabled."]
            pub const WORD_N_ENABLED: u32 = 0;
            #[doc = "Word N is masked. The transmit data pins are 3-stated or drive zero when masked."]
            pub const WORD_N_MASKED: u32 = 1;
        }
    }
}
#[doc = "Receive Control"]
pub mod RCSR {
    #[doc = "FIFO Request DMA Enable"]
    pub mod FRDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the DMA request."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the DMA request."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FIFO Warning DMA Enable"]
    pub mod FWDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables DMA warnings."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables DMA warnings."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FIFO Request Interrupt Enable"]
    pub mod FRIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    pub mod FWIE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FIFO Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables the interrupt."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Sync Error Interrupt Enable"]
    pub mod SEIE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables interrupt."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Word Start Interrupt Enable"]
    pub mod WSIE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables interrupt."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables interrupt."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FIFO Request Flag"]
    pub mod FRF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive FIFO watermark not reached."]
            pub const BELOW_WATERMARK: u32 = 0;
            #[doc = "Receive FIFO watermark has been reached."]
            pub const WATERMARK_REACHED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Warning Flag"]
    pub mod FWF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No enabled receive FIFO is full."]
            pub const NOT_FULL: u32 = 0;
            #[doc = "Enabled receive FIFO is full."]
            pub const FULL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive overflow not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Receive overflow detected."]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Sync Error Flag"]
    pub mod SEF {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sync error not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Frame sync error detected."]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Word Start Flag"]
    pub mod WSF {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start of word not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Start of word detected."]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Software Reset"]
    pub mod SR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Software reset."]
            pub const SW_RESET: u32 = 1;
        }
    }
    #[doc = "FIFO Reset"]
    pub mod FR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "FIFO reset."]
            pub const FIFO_RESET: u32 = 1;
        }
    }
    #[doc = "Bit Clock Enable"]
    pub mod BCE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive bit clock is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive bit clock is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBGE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver is disabled in Debug mode, after completing the current frame."]
            pub const DISABLE: u32 = 0;
            #[doc = "Receiver is enabled in Debug mode."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Stop Enable"]
    pub mod STOPE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver disabled in Stop mode."]
            pub const DISABLE: u32 = 0;
            #[doc = "Receiver enabled in Stop mode."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Receiver Enable"]
    pub mod RE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Receive Configuration 1"]
pub mod RCR1 {
    #[doc = "Receive FIFO Watermark"]
    pub mod RFW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 FIFO word"]
            pub const MIN: u32 = 0;
            #[doc = "2 FIFO words"]
            pub const TWO: u32 = 1;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_2: u32 = 2;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_3: u32 = 3;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_4: u32 = 4;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_5: u32 = 5;
            #[doc = "(RFW value + 1) FIFO words"]
            pub const WATERMARK_6: u32 = 6;
            #[doc = "8 FIFO words"]
            pub const MAX: u32 = 7;
        }
    }
}
#[doc = "Receive Configuration 2"]
pub mod RCR2 {
    #[doc = "Bit Clock Divide"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Clock Bypass"]
    pub mod BYP {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal bit clock is generated from bit clock divider."]
            pub const DISABLE: u32 = 0;
            #[doc = "Internal bit clock is divide-by-one of the audio master clock."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Bit Clock Direction"]
    pub mod BCD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock is generated externally in Slave mode."]
            pub const EXT_SLAVE_MODE: u32 = 0;
            #[doc = "Bit clock is generated internally in Master mode."]
            pub const INT_MASTER_MODE: u32 = 1;
        }
    }
    #[doc = "Bit Clock Polarity"]
    pub mod BCP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
            pub const ACTIVE_LOW: u32 = 1;
        }
    }
    #[doc = "MCLK Select"]
    pub mod MSEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus Clock selected."]
            pub const BUS_CLOCK: u32 = 0;
            #[doc = "Master Clock (MCLK) 1 option selected."]
            pub const MCLK1: u32 = 1;
            #[doc = "Master Clock (MCLK) 2 option selected."]
            pub const MCLK2: u32 = 2;
            #[doc = "Master Clock (MCLK) 3 option selected."]
            pub const MCLK3: u32 = 3;
        }
    }
    #[doc = "Bit Clock Input"]
    pub mod BCI {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Internal logic is clocked as if bit clock was externally generated."]
            pub const CLOCKED_AS_IF_EXT_GENERATED: u32 = 1;
        }
    }
    #[doc = "Bit Clock Swap"]
    pub mod BCS {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use the normal bit clock source."]
            pub const NORMAL: u32 = 0;
            #[doc = "Swap the bit clock source."]
            pub const SWAP_BIT_CLK_SOURCE: u32 = 1;
        }
    }
    #[doc = "Synchronous Mode"]
    pub mod SYNC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asynchronous mode"]
            pub const ASYNC: u32 = 0;
            #[doc = "Synchronous with transmitter"]
            pub const SYNC_W_TX: u32 = 1;
            #[doc = "Synchronous with another SAI receiver"]
            pub const SYNC_W_ANOTHER_SAI_RX: u32 = 2;
            #[doc = "Synchronous with another SAI transmitter"]
            pub const SYNC_W_ANOTHER_SAI_TX: u32 = 3;
        }
    }
}
#[doc = "Receive Configuration 3"]
pub mod RCR3 {
    #[doc = "Word Flag Configuration"]
    pub mod WDFL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Word 1"]
            pub const WORD_1: u32 = 0;
            #[doc = "Word 2"]
            pub const WORD_2: u32 = 1;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_2: u32 = 2;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_3: u32 = 3;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_4: u32 = 4;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_5: u32 = 5;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_6: u32 = 6;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_7: u32 = 7;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_8: u32 = 8;
            #[doc = "Word (WDFL value + 1)"]
            pub const WORD_N_9: u32 = 9;
            #[doc = "Word 32"]
            pub const WORD_MAX: u32 = 31;
        }
    }
    #[doc = "Receive Channel Enable"]
    pub mod RCE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel FIFO Reset"]
    pub mod CFR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Configuration 4"]
pub mod RCR4 {
    #[doc = "Frame Sync Direction"]
    pub mod FSD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame Sync is generated externally in Slave mode."]
            pub const EXT_SLAVE_MODE: u32 = 0;
            #[doc = "Frame Sync is generated internally in Master mode."]
            pub const INT_MASTER_MODE: u32 = 1;
        }
    }
    #[doc = "Frame Sync Polarity"]
    pub mod FSP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame sync is active high."]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Frame sync is active low."]
            pub const ACTIVE_LOW: u32 = 1;
        }
    }
    #[doc = "On Demand Mode"]
    pub mod ONDEM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal frame sync is generated continuously."]
            pub const DISABLE: u32 = 0;
            #[doc = "Internal frame sync is generated when the FIFO warning flag is 0."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Frame Sync Early"]
    pub mod FSE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Frame sync asserts with the first bit of the frame."]
            pub const DISABLE: u32 = 0;
            #[doc = "Frame sync asserts one bit before the first bit of the frame."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "MSB First"]
    pub mod MF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LSB is received first."]
            pub const DISABLE: u32 = 0;
            #[doc = "MSB is received first."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Sync Width"]
    pub mod SYWD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 bit-clock cycle"]
            pub const MIN: u32 = 0;
            #[doc = "2 bit-clock cycle"]
            pub const TWO_CLOCKS: u32 = 1;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_2: u32 = 2;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_3: u32 = 3;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_4: u32 = 4;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_5: u32 = 5;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_6: u32 = 6;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_7: u32 = 7;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_8: u32 = 8;
            #[doc = "(SYWD value + 1) bit-clock cycle"]
            pub const N_CLOCKS_9: u32 = 9;
            #[doc = "32 bit-clock cycle"]
            pub const THIRTYTWO_CLOCKS: u32 = 31;
        }
    }
    #[doc = "Frame Size"]
    pub mod FRSZ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 word per frame"]
            pub const ONE_WORD: u32 = 0;
            #[doc = "2 words per frame"]
            pub const TWO_WORDS: u32 = 1;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_2: u32 = 2;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_3: u32 = 3;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_4: u32 = 4;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_5: u32 = 5;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_6: u32 = 6;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_7: u32 = 7;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_8: u32 = 8;
            #[doc = "(FRSZ value + 1) words per frame"]
            pub const N_WORDS_9: u32 = 9;
            #[doc = "32 words per frame"]
            pub const MAX_WORDS: u32 = 31;
        }
    }
    #[doc = "FIFO Packing Mode"]
    pub mod FPACK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO packing is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "8-bit FIFO packing is enabled"]
            pub const EIGHT_BIT_PACKING: u32 = 2;
            #[doc = "16-bit FIFO packing is enabled"]
            pub const SIXTEEN_BIT_PACKING: u32 = 3;
        }
    }
    #[doc = "FIFO Combine Mode"]
    pub mod FCOMB {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO Combine mode disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "FIFO Combine mode enabled on FIFO writes (from receive shift registers)."]
            pub const ENA_ON_FIFO_WRITES: u32 = 1;
            #[doc = "FIFO Combine mode enabled on FIFO reads (by software)."]
            pub const ENA_ON_FIFO_READS: u32 = 2;
            #[doc = "FIFO Combine mode enabled on FIFO writes (from receive shift registers) and reads (by software)."]
            pub const ENA_ON_FIFO_WRITES_READS: u32 = 3;
        }
    }
    #[doc = "FIFO Continue on Error"]
    pub mod FCONT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "On FIFO error, SAI continues from the start of the next frame after the FIFO error flag returns to 0."]
            pub const DISABLE: u32 = 0;
            #[doc = "On FIFO error, SAI continues from the same word that caused the FIFO error to become 1 after the FIFO warning flag returns to 0."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Receive Configuration 5"]
pub mod RCR5 {
    #[doc = "First Bit Shifted"]
    pub mod FBT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit index is 0."]
            pub const INDEX0: u32 = 0;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_1: u32 = 1;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_2: u32 = 2;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_3: u32 = 3;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_4: u32 = 4;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_5: u32 = 5;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_6: u32 = 6;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_7: u32 = 7;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_8: u32 = 8;
            #[doc = "Bit index is FBT value."]
            pub const INDEX_9: u32 = 9;
            #[doc = "Bit index is 31."]
            pub const INDEX31: u32 = 31;
        }
    }
    #[doc = "Word 0 Width"]
    pub mod W0W {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 bit per word"]
            pub const MIN: u32 = 0;
            #[doc = "2 bits per word"]
            pub const TWO: u32 = 1;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_2: u32 = 2;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_3: u32 = 3;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_4: u32 = 4;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_5: u32 = 5;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_6: u32 = 6;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_7: u32 = 7;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_8: u32 = 8;
            #[doc = "(W0W value + 1) bits per word"]
            pub const THREE_THIRTYONE_9: u32 = 9;
            #[doc = "32 bits per word"]
            pub const MAX: u32 = 31;
        }
    }
    #[doc = "Word N Width"]
    pub mod WNW {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8 bits per word"]
            pub const EIGHT: u32 = 7;
            #[doc = "9 bits per word"]
            pub const NINE: u32 = 8;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_9: u32 = 9;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_10: u32 = 10;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_11: u32 = 11;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_12: u32 = 12;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_13: u32 = 13;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_14: u32 = 14;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_15: u32 = 15;
            #[doc = "(WNW value + 1) bits per word"]
            pub const TEN_THIRTYONE_16: u32 = 16;
            #[doc = "32 bits per word"]
            pub const MAX: u32 = 31;
        }
    }
}
#[doc = "Receive Data"]
pub mod RDR {
    #[doc = "Receive Data Register"]
    pub mod RDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive FIFO"]
pub mod RFR {
    #[doc = "Read FIFO Pointer"]
    pub mod RFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Channel Pointer"]
    pub mod RCP {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No effect."]
            pub const DISABLE: u32 = 0;
            #[doc = "FIFO Combine mode is enabled for FIFO reads and this FIFO will be read on the next FIFO read."]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write FIFO Pointer"]
    pub mod WFP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Mask"]
pub mod RMR {
    #[doc = "Receive Word Mask"]
    pub mod RWM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Word N is enabled."]
            pub const WORD_N_ENABLED: u32 = 0;
            #[doc = "Word N is masked."]
            pub const WORD_N_MASKED: u32 = 1;
        }
    }
}
#[doc = "MCLK Control"]
pub mod MCR {
    #[doc = "MCLK Post Divide"]
    pub mod DIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MCLK Post Divide Enable"]
    pub mod DIVEN {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output on MCLK signal pin is the audio master clock."]
            pub const DISABLE: u32 = 0;
            #[doc = "Output on MCLK signal pin is a post-divided version of audio master clock."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "MCLK Select"]
    pub mod MSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Master Clock (MCLK) 1 option selected."]
            pub const MCLK1: u32 = 0;
            #[doc = "Master Clock (MCLK) 2 option selected."]
            pub const MCLK2: u32 = 2;
            #[doc = "Master Clock (MCLK) 3 option selected."]
            pub const MCLK3: u32 = 3;
        }
    }
    #[doc = "MCLK Output Enable"]
    pub mod MOE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MCLK signal pin is an input."]
            pub const INPUT: u32 = 0;
            #[doc = "MCLK signal pin is an output."]
            pub const OUTPUT: u32 = 1;
        }
    }
}
