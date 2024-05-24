#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "CAN"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module Configuration Register"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "Control 1 Register"]
    pub CTRL1: crate::RWRegister<u32>,
    #[doc = "Free Running Timer"]
    pub TIMER: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "Rx Mailboxes Global Mask Register"]
    pub RXMGMASK: crate::RWRegister<u32>,
    #[doc = "Rx 14 Mask Register"]
    pub RX14MASK: crate::RWRegister<u32>,
    #[doc = "Rx 15 Mask Register"]
    pub RX15MASK: crate::RWRegister<u32>,
    #[doc = "Error Counter"]
    pub ECR: crate::RWRegister<u32>,
    #[doc = "Error and Status 1 Register"]
    pub ESR1: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "Interrupt Masks 1 Register"]
    pub IMASK1: crate::RWRegister<u32>,
    _reserved2: [u8; 0x4],
    #[doc = "Interrupt Flags 1 Register"]
    pub IFLAG1: crate::RWRegister<u32>,
    #[doc = "Control 2 Register"]
    pub CTRL2: crate::RWRegister<u32>,
    #[doc = "Error and Status 2 Register"]
    pub ESR2: crate::RWRegister<u32>,
    _reserved3: [u8; 0x8],
    #[doc = "CRC Register"]
    pub CRCR: crate::RWRegister<u32>,
    #[doc = "Legacy Rx FIFO Global Mask Register"]
    pub RXFGMASK: crate::RWRegister<u32>,
    #[doc = "Legacy Rx FIFO Information Register"]
    pub RXFIR: crate::RWRegister<u32>,
    #[doc = "CAN Bit Timing Register"]
    pub CBT: crate::RWRegister<u32>,
    _reserved4: [u8; 0x2c],
    #[doc = "Cluster MB%s, containing CS?,CS??, ID?,ID??, WORD0?,WORD0??, WORD1?,WORD1??"]
    pub MB: [mb::RegisterBlock; 32usize],
    _reserved5: [u8; 0x600],
    #[doc = "Rx Individual Mask Registers"]
    pub RXIMR: [crate::RWRegister<u32>; 32usize],
    _reserved6: [u8; 0x200],
    #[doc = "Pretended Networking Control 1 Register"]
    pub CTRL1_PN: crate::RWRegister<u32>,
    #[doc = "Pretended Networking Control 2 Register"]
    pub CTRL2_PN: crate::RWRegister<u32>,
    #[doc = "Pretended Networking Wake Up Match Register"]
    pub WU_MTC: crate::RWRegister<u32>,
    #[doc = "Pretended Networking ID Filter 1 Register"]
    pub FLT_ID1: crate::RWRegister<u32>,
    #[doc = "Pretended Networking DLC Filter Register"]
    pub FLT_DLC: crate::RWRegister<u32>,
    #[doc = "Pretended Networking Payload Low Filter 1 Register"]
    pub PL1_LO: crate::RWRegister<u32>,
    #[doc = "Pretended Networking Payload High Filter 1 Register"]
    pub PL1_HI: crate::RWRegister<u32>,
    #[doc = "Pretended Networking ID Filter 2 Register / ID Mask Register"]
    pub FLT_ID2_IDMASK: crate::RWRegister<u32>,
    #[doc = "Pretended Networking Payload Low Filter 2 Register / Payload Low Mask register"]
    pub PL2_PLMASK_LO: crate::RWRegister<u32>,
    #[doc = "Pretended Networking Payload High Filter 2 low order bits / Payload High Mask register"]
    pub PL2_PLMASK_HI: crate::RWRegister<u32>,
    _reserved7: [u8; 0x18],
    #[doc = "no description available"]
    pub WMB: [wmb::RegisterBlock; 4usize],
    _reserved8: [u8; 0x70],
    #[doc = "Enhanced CAN Bit Timing Prescalers"]
    pub EPRS: crate::RWRegister<u32>,
    #[doc = "Enhanced Nominal CAN Bit Timing"]
    pub ENCBT: crate::RWRegister<u32>,
    #[doc = "Enhanced Data Phase CAN bit Timing"]
    pub EDCBT: crate::RWRegister<u32>,
    #[doc = "Enhanced Transceiver Delay Compensation"]
    pub ETDC: crate::RWRegister<u32>,
    #[doc = "CAN FD Control Register"]
    pub FDCTRL: crate::RWRegister<u32>,
    #[doc = "CAN FD Bit Timing Register"]
    pub FDCBT: crate::RWRegister<u32>,
    #[doc = "CAN FD CRC Register"]
    pub FDCRC: crate::RWRegister<u32>,
    #[doc = "Enhanced Rx FIFO Control Register"]
    pub ERFCR: crate::RWRegister<u32>,
    #[doc = "Enhanced Rx FIFO Interrupt Enable Register"]
    pub ERFIER: crate::RWRegister<u32>,
    #[doc = "Enhanced Rx FIFO Status Register"]
    pub ERFSR: crate::RWRegister<u32>,
    _reserved9: [u8; 0x23e8],
    #[doc = "Enhanced Rx FIFO Filter Element"]
    pub ERFFEL: [crate::RWRegister<u32>; 32usize],
}
#[doc = "Module Configuration Register"]
pub mod MCR {
    #[doc = "Number Of The Last Message Buffer"]
    pub mod MAXMB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Acceptance Mode"]
    pub mod IDAM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Format A: One full ID (standard and extended) per ID filter table element."]
            pub const ONE_FULL_ID: u32 = 0;
            #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID filter table element."]
            pub const TWO_FULL_ID: u32 = 1;
            #[doc = "Format C: Four partial 8-bit standard IDs per ID filter table element."]
            pub const FOUR_PARTIAL_ID: u32 = 2;
            #[doc = "Format D: All frames rejected."]
            pub const ALL_FRAMES_REJECTED: u32 = 3;
        }
    }
    #[doc = "CAN FD operation enable"]
    pub mod FDEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CAN FD is disabled. FlexCAN is able to receive and transmit messages in CAN 2.0 format."]
            pub const CAN_FD_DISABLED: u32 = 0;
            #[doc = "CAN FD is enabled. FlexCAN is able to receive and transmit messages in both CAN FD and CAN 2.0 formats."]
            pub const CAN_FD_ENABLED: u32 = 1;
        }
    }
    #[doc = "Abort Enable"]
    pub mod AEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Abort disabled."]
            pub const ABORT_DISABLED: u32 = 0;
            #[doc = "Abort enabled."]
            pub const ABORT_ENABLED: u32 = 1;
        }
    }
    #[doc = "Local Priority Enable"]
    pub mod LPRIOEN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Local Priority disabled."]
            pub const LOCAL_PRIORITY_DISABLED: u32 = 0;
            #[doc = "Local Priority enabled."]
            pub const LOCAL_PRIORITY_ENABLED: u32 = 1;
        }
    }
    #[doc = "Pretended Networking Enable"]
    pub mod PNET_EN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pretended Networking mode is disabled."]
            pub const PN_DISABLED: u32 = 0;
            #[doc = "Pretended Networking mode is enabled."]
            pub const PN_ENABLED: u32 = 1;
        }
    }
    #[doc = "DMA Enable"]
    pub mod DMA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA feature for Legacy RX FIFO or Enhanced Rx FIFO disabled."]
            pub const ID1: u32 = 0;
            #[doc = "DMA feature for Legacy RX FIFO or Enhanced Rx FIFO enabled."]
            pub const ID3: u32 = 1;
        }
    }
    #[doc = "Individual Rx Masking And Queue Enable"]
    pub mod IRMQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
            pub const INDIVIDUAL_RX_MASKING_DISABLED: u32 = 0;
            #[doc = "Individual Rx masking and queue feature are enabled."]
            pub const INDIVIDUAL_RX_MASKING_ENABLED: u32 = 1;
        }
    }
    #[doc = "Self Reception Disable"]
    pub mod SRXDIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Self-reception enabled."]
            pub const SELF_RECEPTION_ENABLED: u32 = 0;
            #[doc = "Self-reception disabled."]
            pub const SELF_RECEPTION_DISABLED: u32 = 1;
        }
    }
    #[doc = "Wake Up Source"]
    pub mod WAKSRC {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexCAN uses the unfiltered Rx input to detect recessive to dominant edges on the CAN bus."]
            pub const UNFILTERED_RX_INPUT: u32 = 0;
            #[doc = "FlexCAN uses the filtered Rx input to detect recessive to dominant edges on the CAN bus."]
            pub const FILTERED_RX_INPUT: u32 = 1;
        }
    }
    #[doc = "Low-Power Mode Acknowledge"]
    pub mod LPMACK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "FlexCAN is not in a low-power mode."]
            pub const LOW_POWER_NO: u32 = 0;
            #[doc = "FlexCAN is in a low-power mode."]
            pub const LOW_POWER_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Warning Interrupt Enable"]
    pub mod WRNEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
            pub const TWRNINT_RWRNINT_INACTIVE: u32 = 0;
            #[doc = "TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
            pub const TWRNINT_RWRNINT_ACTIVE: u32 = 1;
        }
    }
    #[doc = "Self Wake Up"]
    pub mod SLFWAK {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexCAN Self Wake Up feature is disabled."]
            pub const SELF_WAKEUP_DISABLED: u32 = 0;
            #[doc = "FlexCAN Self Wake Up feature is enabled."]
            pub const SELF_WAKEUP_ENABLED: u32 = 1;
        }
    }
    #[doc = "Freeze Mode Acknowledge"]
    pub mod FRZACK {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "FlexCAN not in Freeze mode, prescaler running."]
            pub const FREEZE_MODE_NO: u32 = 0;
            #[doc = "FlexCAN in Freeze mode, prescaler stopped."]
            pub const FREEZE_MODE_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Soft Reset"]
    pub mod SOFTRST {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No reset request."]
            pub const SOFTRST_NO_RESET_REQUEST: u32 = 0;
            #[doc = "Resets the registers affected by soft reset."]
            pub const SOFTRST_RESET_REGISTERS: u32 = 1;
        }
    }
    #[doc = "Wake Up Interrupt Mask"]
    pub mod WAKMSK {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wake Up interrupt is disabled."]
            pub const WAKEUP_INTERRUPT_DISABLED: u32 = 0;
            #[doc = "Wake Up interrupt is enabled."]
            pub const WAKEUP_INTERRUPT_ENABLED: u32 = 1;
        }
    }
    #[doc = "FlexCAN Not Ready"]
    pub mod NOTRDY {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "FlexCAN module is either in Normal mode, Listen-Only mode, or Loop-Back mode."]
            pub const ID1: u32 = 0;
            #[doc = "FlexCAN module is either in Disable mode, Stop mode, or Freeze mode."]
            pub const ID4: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Halt FlexCAN"]
    pub mod HALT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Freeze mode request."]
            pub const HALT_DISABLE: u32 = 0;
            #[doc = "Enters Freeze mode if the FRZ bit is asserted."]
            pub const HALT_ENABLE: u32 = 1;
        }
    }
    #[doc = "Legacy Rx FIFO Enable"]
    pub mod RFEN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Legacy Rx FIFO not enabled."]
            pub const ID1: u32 = 0;
            #[doc = "Legacy Rx FIFO enabled."]
            pub const ID3: u32 = 1;
        }
    }
    #[doc = "Freeze Enable"]
    pub mod FRZ {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not enabled to enter Freeze mode."]
            pub const FREEZE_MODE_DISABLED: u32 = 0;
            #[doc = "Enabled to enter Freeze mode."]
            pub const FREEZE_MODE_ENABLED: u32 = 1;
        }
    }
    #[doc = "Module Disable"]
    pub mod MDIS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable the FlexCAN module."]
            pub const FLEXCAN_ENABLED: u32 = 0;
            #[doc = "Disable the FlexCAN module."]
            pub const FLEXCAN_DISABLED: u32 = 1;
        }
    }
}
#[doc = "Control 1 Register"]
pub mod CTRL1 {
    #[doc = "Propagation Segment"]
    pub mod PROPSEG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Listen-Only Mode"]
    pub mod LOM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Listen-Only mode is deactivated."]
            pub const LISTEN_ONLY_MODE_DISABLED: u32 = 0;
            #[doc = "FlexCAN module operates in Listen-Only mode."]
            pub const LISTEN_ONLY_MODE_ENABLED: u32 = 1;
        }
    }
    #[doc = "Lowest Buffer Transmitted First"]
    pub mod LBUF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Buffer with highest priority is transmitted first."]
            pub const HIGHEST_BUFFER_FIRST: u32 = 0;
            #[doc = "Lowest number buffer is transmitted first."]
            pub const LOWEST_BUFFER_FIRST: u32 = 1;
        }
    }
    #[doc = "Timer Sync"]
    pub mod TSYN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer sync feature disabled"]
            pub const TIMER_SYNC_DISABLED: u32 = 0;
            #[doc = "Timer sync feature enabled"]
            pub const TIMER_SYNC_ENABLED: u32 = 1;
        }
    }
    #[doc = "Bus Off Recovery"]
    pub mod BOFFREC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic recovering from Bus Off state enabled."]
            pub const AUTO_RECOVER_ENABLED: u32 = 0;
            #[doc = "Automatic recovering from Bus Off state disabled."]
            pub const AUTO_RECOVER_DISABLED: u32 = 1;
        }
    }
    #[doc = "CAN Bit Sampling"]
    pub mod SMP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Just one sample is used to determine the bit value."]
            pub const ONE_SAMPLE: u32 = 0;
            #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and two preceding samples; a majority rule is used."]
            pub const THREE_SAMPLE: u32 = 1;
        }
    }
    #[doc = "Rx Warning Interrupt Mask"]
    pub mod RWRNMSK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx Warning interrupt disabled."]
            pub const RX_WARNING_INT_DISABLED: u32 = 0;
            #[doc = "Rx Warning interrupt enabled."]
            pub const RX_WARNING_INT_ENABLED: u32 = 1;
        }
    }
    #[doc = "Tx Warning Interrupt Mask"]
    pub mod TWRNMSK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tx Warning interrupt disabled."]
            pub const TX_WARNING_INT_DISABLED: u32 = 0;
            #[doc = "Tx Warning interrupt enabled."]
            pub const TX_WARNING_INT_ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop Back Mode"]
    pub mod LPB {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Loop Back disabled."]
            pub const LOOPBACK_DISABLED: u32 = 0;
            #[doc = "Loop Back enabled."]
            pub const LOOPBACK_ENABLED: u32 = 1;
        }
    }
    #[doc = "Error Interrupt Mask"]
    pub mod ERRMSK {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error interrupt disabled."]
            pub const ERROR_INT_DISABLED: u32 = 0;
            #[doc = "Error interrupt enabled."]
            pub const ERROR_INT_ENABLED: u32 = 1;
        }
    }
    #[doc = "Bus Off Interrupt Mask"]
    pub mod BOFFMSK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus Off interrupt disabled."]
            pub const BUS_OFF_INT_DISABLED: u32 = 0;
            #[doc = "Bus Off interrupt enabled."]
            pub const BUS_OFF_INT_ENABLED: u32 = 1;
        }
    }
    #[doc = "Phase Segment 2"]
    pub mod PSEG2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Phase Segment 1"]
    pub mod PSEG1 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resync Jump Width"]
    pub mod RJW {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Prescaler Division Factor"]
    pub mod PRESDIV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Free Running Timer"]
pub mod TIMER {
    #[doc = "Timer Value"]
    pub mod TIMER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx Mailboxes Global Mask Register"]
pub mod RXMGMASK {
    #[doc = "Rx Mailboxes Global Mask Bits"]
    pub mod MG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx 14 Mask Register"]
pub mod RX14MASK {
    #[doc = "Rx Buffer 14 Mask Bits"]
    pub mod RX14M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rx 15 Mask Register"]
pub mod RX15MASK {
    #[doc = "Rx Buffer 15 Mask Bits"]
    pub mod RX15M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Counter"]
pub mod ECR {
    #[doc = "Transmit Error Counter"]
    pub mod TXERRCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Error Counter"]
    pub mod RXERRCNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Error Counter for fast bits"]
    pub mod TXERRCNT_FAST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Error Counter for fast bits"]
    pub mod RXERRCNT_FAST {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error and Status 1 Register"]
pub mod ESR1 {
    #[doc = "Wake-Up Interrupt"]
    pub mod WAKINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const DISABLE: u32 = 0;
            #[doc = "Indicates a recessive to dominant transition was received on the CAN bus."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Error Interrupt"]
    pub mod ERRINT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const DISABLE: u32 = 0;
            #[doc = "Indicates setting of any error bit in the Error and Status register."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Bus Off Interrupt"]
    pub mod BOFFINT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const DISABLE: u32 = 0;
            #[doc = "FlexCAN module entered Bus Off state."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "FlexCAN In Reception"]
    pub mod RX {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "FlexCAN is not receiving a message."]
            pub const DISABLE: u32 = 0;
            #[doc = "FlexCAN is receiving a message."]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fault Confinement State"]
    pub mod FLTCONF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Error Active"]
            pub const ERROR_ACTIVE: u32 = 0;
            #[doc = "Error Passive"]
            pub const ERROR_PASSIVE: u32 = 1;
            #[doc = "Bus Off"]
            pub const BUS_OFF: u32 = 2;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FlexCAN In Transmission"]
    pub mod TX {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "FlexCAN is not transmitting a message."]
            pub const TRANSMIT_MESSAGE_NO: u32 = 0;
            #[doc = "FlexCAN is transmitting a message."]
            pub const TRANSMIT_MESSAGE_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IDLE"]
    pub mod IDLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const CAN_BUS_NOT_IDLE: u32 = 0;
            #[doc = "CAN bus is now IDLE."]
            pub const CAN_BUS_IDLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Error Warning"]
    pub mod RXWRN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const RXERRCNT_LT_96: u32 = 0;
            #[doc = "RXERRCNT is greater than or equal to 96."]
            pub const RXERRCNT_GTE_96: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Error Warning"]
    pub mod TXWRN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const TXERRCNT_LT_96: u32 = 0;
            #[doc = "TXERRCNT is greater than or equal to 96."]
            pub const TXERRCNT_GTE_96: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stuffing Error"]
    pub mod STFERR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const STUFFING_ERROR_NO: u32 = 0;
            #[doc = "A stuffing error occurred since last read of this register."]
            pub const STUFFING_ERROR_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Form Error"]
    pub mod FRMERR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const FORM_ERROR_NO: u32 = 0;
            #[doc = "A Form Error occurred since last read of this register."]
            pub const FORM_ERROR_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Cyclic Redundancy Check Error"]
    pub mod CRCERR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const CRC_ERROR_NO: u32 = 0;
            #[doc = "A CRC error occurred since last read of this register."]
            pub const CRC_ERROR_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Acknowledge Error"]
    pub mod ACKERR {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const ACK_ERROR_NO: u32 = 0;
            #[doc = "An ACK error occurred since last read of this register."]
            pub const ACK_ERROR_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit0 Error"]
    pub mod BIT0ERR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const BIT0_ERROR_NO: u32 = 0;
            #[doc = "At least one bit sent as dominant is received as recessive."]
            pub const BIT0_ERROR_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit1 Error"]
    pub mod BIT1ERR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const BIT1_ERROR_NO: u32 = 0;
            #[doc = "At least one bit sent as recessive is received as dominant."]
            pub const BIT1_ERROR_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Warning Interrupt Flag"]
    pub mod RWRNINT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const RX_WARNING_INT_NO: u32 = 0;
            #[doc = "The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
            pub const RX_WARNING_INT_YES: u32 = 1;
        }
    }
    #[doc = "Tx Warning Interrupt Flag"]
    pub mod TWRNINT {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const TX_WARNING_INT_NO: u32 = 0;
            #[doc = "The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
            pub const TX_WARNING_INT_YES: u32 = 1;
        }
    }
    #[doc = "CAN Synchronization Status"]
    pub mod SYNCH {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "FlexCAN is not synchronized to the CAN bus."]
            pub const CAN_BUS_SYNC_NO: u32 = 0;
            #[doc = "FlexCAN is synchronized to the CAN bus."]
            pub const CAN_BUS_SYNC_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus Off Done Interrupt"]
    pub mod BOFFDONEINT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const BUS_OFF_NOT_DONE: u32 = 0;
            #[doc = "FlexCAN module has completed Bus Off process."]
            pub const BUS_OFF_DONE: u32 = 1;
        }
    }
    #[doc = "Error interrupt for errors detected in Data Phase of CAN FD frames with BRS bit set"]
    pub mod ERRINT_FAST {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence."]
            pub const ERRORS_DATA_PHASE_NO: u32 = 0;
            #[doc = "Indicates setting of any error bit detected in the data phase of CAN FD frames with the BRS bit set."]
            pub const ERRORS_DATA_PHASE_YES: u32 = 1;
        }
    }
    #[doc = "Error Overrun"]
    pub mod ERROVR {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Overrun has not occurred."]
            pub const OVERRUN_NOT_OCCURRED: u32 = 0;
            #[doc = "Overrun has occurred."]
            pub const OVERRUN_OCCURRED: u32 = 1;
        }
    }
    #[doc = "Stuffing Error in the Data Phase of CAN FD frames with the BRS bit set"]
    pub mod STFERR_FAST {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const STUFFING_ERROR_NO: u32 = 0;
            #[doc = "A stuffing error occurred since last read of this register."]
            pub const STUFFING_ERROR_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Form Error in the Data Phase of CAN FD frames with the BRS bit set"]
    pub mod FRMERR_FAST {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const FORM_ERROR_NO: u32 = 0;
            #[doc = "A form error occurred since last read of this register."]
            pub const FORM_ERROR_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Cyclic Redundancy Check Error in the CRC field of CAN FD frames with the BRS bit set"]
    pub mod CRCERR_FAST {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const CRC_ERROR_NO: u32 = 0;
            #[doc = "A CRC error occurred since last read of this register."]
            pub const CRC_ERROR_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit0 Error in the Data Phase of CAN FD frames with the BRS bit set"]
    pub mod BIT0ERR_FAST {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const BIT0_ERROR_NO: u32 = 0;
            #[doc = "At least one bit sent as dominant is received as recessive."]
            pub const BIT0_ERROR_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit1 Error in the Data Phase of CAN FD frames with the BRS bit set"]
    pub mod BIT1ERR_FAST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No such occurrence."]
            pub const BIT1_ERROR_NO: u32 = 0;
            #[doc = "At least one bit sent as recessive is received as dominant."]
            pub const BIT1_ERROR_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Masks 1 Register"]
pub mod IMASK1 {
    #[doc = "Buffer MBi Mask"]
    pub mod BUF31TO0M {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Flags 1 Register"]
pub mod IFLAG1 {
    #[doc = "Buffer MB0 Interrupt Or Clear Legacy FIFO bit"]
    pub mod BUF0I {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR[RFEN]=0."]
            pub const BUFFER_TX_RX_NOT_COMPLETE: u32 = 0;
            #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR[RFEN]=0."]
            pub const BUFFER_TX_RX_COMPLETE: u32 = 1;
        }
    }
    #[doc = "Buffer MBi Interrupt Or Reserved"]
    pub mod BUF4TO1I {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Buffer MB5 Interrupt Or Frames available in Legacy Rx FIFO"]
    pub mod BUF5I {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No occurrence of MB5 completing transmission/reception when MCR[RFEN]=0, or of frame(s) available in the Legacy FIFO, when MCR[RFEN]=1"]
            pub const ID1: u32 = 0;
            #[doc = "MB5 completed transmission/reception when MCR[RFEN]=0, or frame(s) available in the Legacy Rx FIFO when MCR[RFEN]=1. It generates a DMA request in case of MCR[RFEN] and MCR[DMA] are enabled."]
            pub const ID3: u32 = 1;
        }
    }
    #[doc = "Buffer MB6 Interrupt Or Legacy Rx FIFO Warning"]
    pub mod BUF6I {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No occurrence of MB6 completing transmission/reception when MCR[RFEN]=0, or of Legacy Rx FIFO almost full when MCR[RFEN]=1"]
            pub const ID1: u32 = 0;
            #[doc = "MB6 completed transmission/reception when MCR[RFEN]=0, or Legacy Rx FIFO almost full when MCR[RFEN]=1"]
            pub const ID3: u32 = 1;
        }
    }
    #[doc = "Buffer MB7 Interrupt Or Legacy Rx FIFO Overflow"]
    pub mod BUF7I {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No occurrence of MB7 completing transmission/reception when MCR[RFEN]=0, or of Legacy Rx FIFO overflow when MCR[RFEN]=1"]
            pub const ID1: u32 = 0;
            #[doc = "MB7 completed transmission/reception when MCR[RFEN]=0, or Legacy Rx FIFO overflow when MCR[RFEN]=1"]
            pub const ID3: u32 = 1;
        }
    }
    #[doc = "Buffer MBi Interrupt"]
    pub mod BUF31TO8I {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control 2 Register"]
pub mod CTRL2 {
    #[doc = "Edge Filter Disable"]
    pub mod EDFLTDIS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Edge filter is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Edge filter is disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "ISO CAN FD Enable"]
    pub mod ISOCANFDEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FlexCAN operates using the non-ISO CAN FD protocol."]
            pub const NON_ISO: u32 = 0;
            #[doc = "FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1:2015)."]
            pub const ISO: u32 = 1;
        }
    }
    #[doc = "Bit Timing Expansion enable"]
    pub mod BTE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CAN Bit timing expansion is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "CAN bit timing expansion is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Protocol Exception Enable"]
    pub mod PREXCEN {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Protocol exception is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Protocol exception is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    pub mod EACEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx mailbox filter\'s IDE bit is always compared and RTR is never compared despite mask bits."]
            pub const RTR_COMPARE_NO: u32 = 0;
            #[doc = "Enables the comparison of both Rx mailbox filter\'s IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
            pub const RTR_COMPARE_YES: u32 = 1;
        }
    }
    #[doc = "Remote Request Storing"]
    pub mod RRS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remote response frame is generated."]
            pub const REMOTE_RESPONSE_FRAME_NOT_GENERATED: u32 = 0;
            #[doc = "Remote request frame is stored."]
            pub const REMOTE_RESPONSE_FRAME_GENERATED: u32 = 1;
        }
    }
    #[doc = "Mailboxes Reception Priority"]
    pub mod MRP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Matching starts from Legacy Rx FIFO or Enhanced Rx FIFO and continues on mailboxes."]
            pub const ID1: u32 = 0;
            #[doc = "Matching starts from mailboxes and continues on Legacy Rx FIFO or Enhanced Rx FIFO."]
            pub const ID3: u32 = 1;
        }
    }
    #[doc = "Tx Arbitration Start Delay"]
    pub mod TASD {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number Of Legacy Rx FIFO Filters"]
    pub mod RFFN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus Off Done Interrupt Mask"]
    pub mod BOFFDONEMSK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bus off done interrupt disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Bus off done interrupt enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Error Interrupt Mask for errors detected in the data phase of fast CAN FD frames"]
    pub mod ERRMSK_FAST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ERRINT_FAST error interrupt disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "ERRINT_FAST error interrupt enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Error and Status 2 Register"]
pub mod ESR2 {
    #[doc = "Inactive Mailbox"]
    pub mod IMB {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "If ESR2[VPS] is asserted, the ESR2[LPTM] is not an inactive mailbox."]
            pub const INACTIVE_MAILBOX_NO: u32 = 0;
            #[doc = "If ESR2[VPS] is asserted, there is at least one inactive mailbox. LPTM content is the number of the first one."]
            pub const INACTIVE_MAILBOX_YES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Valid Priority Status"]
    pub mod VPS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Contents of IMB and LPTM are invalid."]
            pub const INVALID: u32 = 0;
            #[doc = "Contents of IMB and LPTM are valid."]
            pub const VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lowest Priority Tx Mailbox"]
    pub mod LPTM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CRC Register"]
pub mod CRCR {
    #[doc = "Transmitted CRC value"]
    pub mod TXCRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC Mailbox"]
    pub mod MBCRC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Legacy Rx FIFO Global Mask Register"]
pub mod RXFGMASK {
    #[doc = "Legacy Rx FIFO Global Mask Bits"]
    pub mod FGM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Legacy Rx FIFO Information Register"]
pub mod RXFIR {
    #[doc = "Identifier Acceptance Filter Hit Indicator"]
    pub mod IDHIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CAN Bit Timing Register"]
pub mod CBT {
    #[doc = "Extended Phase Segment 2"]
    pub mod EPSEG2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Phase Segment 1"]
    pub mod EPSEG1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Propagation Segment"]
    pub mod EPROPSEG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Resync Jump Width"]
    pub mod ERJW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Prescaler Division Factor"]
    pub mod EPRESDIV {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Timing Format Enable"]
    pub mod BTF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Extended bit time definitions disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Extended bit time definitions enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
}
pub mod mb {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "Cluster MB%s, containing CS?,CS??, ID?,ID??, WORD0?,WORD0??, WORD1?,WORD1??"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Message Buffer 0 CS Register"]
        pub CS: crate::RWRegister<u32>,
        #[doc = "Message Buffer 0 ID Register"]
        pub ID: crate::RWRegister<u32>,
        #[doc = "Message Buffer 0 WORD0 Register"]
        pub WORD0: crate::RWRegister<u32>,
        #[doc = "Message Buffer 0 WORD1 Register"]
        pub WORD1: crate::RWRegister<u32>,
    }
    #[doc = "Message Buffer 0 CS Register"]
    pub mod CS {
        #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
        pub mod TIME_STAMP {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Length of the data to be stored/transmitted."]
        pub mod DLC {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
        pub mod RTR {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "ID Extended. One/zero for extended/standard format frame."]
        pub mod IDE {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
        pub mod SRR {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
        pub mod CODE {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
        pub mod ESI {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
        pub mod BRS {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
        pub mod EDL {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Message Buffer 0 ID Register"]
    pub mod ID {
        #[doc = "Contains extended (LOW word) identifier of message buffer."]
        pub mod EXT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
        pub mod STD {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b11111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
        pub mod PRIO {
            pub const offset: u32 = 29;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Message Buffer 0 WORD0 Register"]
    pub mod WORD0 {
        #[doc = "Data byte 0 of Rx/Tx frame."]
        pub mod DATA_BYTE_3 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Data byte 1 of Rx/Tx frame."]
        pub mod DATA_BYTE_2 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Data byte 2 of Rx/Tx frame."]
        pub mod DATA_BYTE_1 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Data byte 3 of Rx/Tx frame."]
        pub mod DATA_BYTE_0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Message Buffer 0 WORD1 Register"]
    pub mod WORD1 {
        #[doc = "Data byte 0 of Rx/Tx frame."]
        pub mod DATA_BYTE_7 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Data byte 1 of Rx/Tx frame."]
        pub mod DATA_BYTE_6 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Data byte 2 of Rx/Tx frame."]
        pub mod DATA_BYTE_5 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Data byte 3 of Rx/Tx frame."]
        pub mod DATA_BYTE_4 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
#[doc = "Rx Individual Mask Registers"]
pub mod RXIMR {
    #[doc = "Individual Mask Bits"]
    pub mod MI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pretended Networking Control 1 Register"]
pub mod CTRL1_PN {
    #[doc = "Filtering Combination Selection"]
    pub mod FCS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Message ID filtering only"]
            pub const ID_FILTERING: u32 = 0;
            #[doc = "Message ID filtering and payload filtering"]
            pub const ID_PAYLOAD_FILTERING: u32 = 1;
            #[doc = "Message ID filtering occurring a specified number of times"]
            pub const ID_FILTERING_NUMBER: u32 = 2;
            #[doc = "Message ID filtering and payload filtering a specified number of times"]
            pub const ID_PAYLOAD_FILTERING_NUMBER: u32 = 3;
        }
    }
    #[doc = "ID Filtering Selection"]
    pub mod IDFS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match upon ID contents against an exact target value"]
            pub const MATCH_EXACT: u32 = 0;
            #[doc = "Match upon an ID value greater than or equal to a specified target value"]
            pub const MATCH_GTE: u32 = 1;
            #[doc = "Match upon an ID value smaller than or equal to a specified target value"]
            pub const MATCH_LTE: u32 = 2;
            #[doc = "Match upon an ID value inside a range, greater than or equal to a specified lower limit, and smaller than or equal to a specified upper limit"]
            pub const MATCH_RANGE: u32 = 3;
        }
    }
    #[doc = "Payload Filtering Selection"]
    pub mod PLFS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match upon a payload contents against an exact target value"]
            pub const MATCH_EXACT: u32 = 0;
            #[doc = "Match upon a payload value greater than or equal to a specified target value"]
            pub const MATCH_GTE: u32 = 1;
            #[doc = "Match upon a payload value smaller than or equal to a specified target value"]
            pub const MATCH_LTE: u32 = 2;
            #[doc = "Match upon a payload value inside a range, greater than or equal to a specified lower limit, and smaller than or equal to a specified upper limit"]
            pub const MATCH_RANGE: u32 = 3;
        }
    }
    #[doc = "Number of Messages Matching the Same Filtering Criteria"]
    pub mod NMATCH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Received message must match the predefined filtering criteria for ID and/or PL once before generating a wakeup event."]
            pub const MATCH_1: u32 = 1;
            #[doc = "Received message must match the predefined filtering criteria for ID and/or PL twice before generating a wakeup event."]
            pub const MATCH_2: u32 = 2;
            #[doc = "Received message must match the predefined filtering criteria for ID and/or PL 255 times before generating a wakeup event."]
            pub const MATCH_255: u32 = 255;
        }
    }
    #[doc = "Wake Up by Match Flag Mask Bit"]
    pub mod WUMF_MSK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wakeup match event is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Wakeup match event is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Wake Up by Timeout Flag Mask Bit"]
    pub mod WTOF_MSK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timeout wakeup event is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timeout wakeup event is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Pretended Networking Control 2 Register"]
pub mod CTRL2_PN {
    #[doc = "Timeout for No Message Matching the Filtering Criteria"]
    pub mod MATCHTO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pretended Networking Wake Up Match Register"]
pub mod WU_MTC {
    #[doc = "Number of Matches when in Pretended Networking"]
    pub mod MCOUNTER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wake Up by Match Flag Bit"]
    pub mod WUMF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No wakeup by match event detected"]
            pub const NO_MATCH: u32 = 0;
            #[doc = "Wakeup by match event detected"]
            pub const MATCH_: u32 = 1;
        }
    }
    #[doc = "Wake Up by Timeout Flag Bit"]
    pub mod WTOF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No wakeup by timeout event detected"]
            pub const NO_WAKEUP: u32 = 0;
            #[doc = "Wakeup by timeout event detected"]
            pub const WAKEUP: u32 = 1;
        }
    }
}
#[doc = "Pretended Networking ID Filter 1 Register"]
pub mod FLT_ID1 {
    #[doc = "ID Filter 1 for Pretended Networking filtering"]
    pub mod FLT_ID1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request Filter"]
    pub mod FLT_RTR {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reject remote frame (accept data frame)"]
            pub const REJECT: u32 = 0;
            #[doc = "Accept remote frame"]
            pub const ACCEPT: u32 = 1;
        }
    }
    #[doc = "ID Extended Filter"]
    pub mod FLT_IDE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Accept standard frame format"]
            pub const STANDARD: u32 = 0;
            #[doc = "Accept extended frame format"]
            pub const EXTENDED: u32 = 1;
        }
    }
}
#[doc = "Pretended Networking DLC Filter Register"]
pub mod FLT_DLC {
    #[doc = "Upper Limit for Length of Data Bytes Filter"]
    pub mod FLT_DLC_HI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lower Limit for Length of Data Bytes Filter"]
    pub mod FLT_DLC_LO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pretended Networking Payload Low Filter 1 Register"]
pub mod PL1_LO {
    #[doc = "Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to data byte 3."]
    pub mod Data_byte_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to data byte 2."]
    pub mod Data_byte_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to data byte 1."]
    pub mod Data_byte_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to data byte 0."]
    pub mod Data_byte_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pretended Networking Payload High Filter 1 Register"]
pub mod PL1_HI {
    #[doc = "Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to data byte 7."]
    pub mod Data_byte_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to data byte 6."]
    pub mod Data_byte_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to data byte 5."]
    pub mod Data_byte_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Filter 1 high order bits for Pretended Networking payload filtering corresponding to data byte 4."]
    pub mod Data_byte_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pretended Networking ID Filter 2 Register / ID Mask Register"]
pub mod FLT_ID2_IDMASK {
    #[doc = "ID Filter 2 for Pretended Networking Filtering / ID Mask Bits for Pretended Networking ID Filtering"]
    pub mod FLT_ID2_IDMASK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Transmission Request Mask Bit"]
    pub mod RTR_MSK {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding bit in the filter is \"don\'t care\""]
            pub const FRAME_TYPE_NO: u32 = 0;
            #[doc = "The corresponding bit in the filter is checked"]
            pub const FRAME_TYPE_YES: u32 = 1;
        }
    }
    #[doc = "ID Extended Mask Bit"]
    pub mod IDE_MSK {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The corresponding bit in the filter is \"don\'t care\""]
            pub const FRAME_FORMAT_NO: u32 = 0;
            #[doc = "The corresponding bit in the filter is checked"]
            pub const FRAME_FORMAT_YES: u32 = 1;
        }
    }
}
#[doc = "Pretended Networking Payload Low Filter 2 Register / Payload Low Mask register"]
pub mod PL2_PLMASK_LO {
    #[doc = "Payload Filter 2 low order bits / Payload Mask low order bits for Pretended Networking payload filtering corresponding to the data byte 3."]
    pub mod Data_byte_3 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Filter 2 low order bits / Payload Mask low order bits for Pretended Networking payload filtering corresponding to the data byte 2."]
    pub mod Data_byte_2 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Filter 2 low order bits / Payload Mask low order bits for Pretended Networking payload filtering corresponding to the data byte 1."]
    pub mod Data_byte_1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Filter 2 low order bits / Payload Mask low order bits for Pretended Networking payload filtering corresponding to the data byte 0."]
    pub mod Data_byte_0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pretended Networking Payload High Filter 2 low order bits / Payload High Mask register"]
pub mod PL2_PLMASK_HI {
    #[doc = "Payload Filter 2 high order bits / Payload Mask high order bits for Pretended Networking payload filtering corresponding to the data byte 7."]
    pub mod Data_byte_7 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Filter 2 high order bits / Payload Mask high order bits for Pretended Networking payload filtering corresponding to the data byte 6."]
    pub mod Data_byte_6 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Filter 2 high order bits / Payload Mask high order bits for Pretended Networking payload filtering corresponding to the data byte 5."]
    pub mod Data_byte_5 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Filter 2 high order bits / Payload Mask high order bits for Pretended Networking payload filtering corresponding to the data byte 4."]
    pub mod Data_byte_4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod wmb {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Wake Up Message Buffer register for C/S"]
        pub WMB_CS: crate::RWRegister<u32>,
        #[doc = "Wake Up Message Buffer Register for ID"]
        pub WMB_ID: crate::RWRegister<u32>,
        #[doc = "Wake Up Message Buffer Register for Data 0-3"]
        pub WMB_D03: crate::RWRegister<u32>,
        #[doc = "Wake Up Message Buffer Register Data 4-7"]
        pub WMB_D47: crate::RWRegister<u32>,
    }
    #[doc = "Wake Up Message Buffer register for C/S"]
    pub mod WMB_CS {
        #[doc = "Length of Data in Bytes"]
        pub mod DLC {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Remote Transmission Request Bit"]
        pub mod RTR {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Frame is data one (not remote)"]
                pub const NOT_REMOTE: u32 = 0;
                #[doc = "Frame is a remote one"]
                pub const REMOTE: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "ID Extended Bit"]
        pub mod IDE {
            pub const offset: u32 = 21;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Frame format is standard"]
                pub const STANDARD: u32 = 0;
                #[doc = "Frame format is extended"]
                pub const EXTENDED: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Substitute Remote Request"]
        pub mod SRR {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Wake Up Message Buffer Register for ID"]
    pub mod WMB_ID {
        #[doc = "Received ID under Pretended Networking mode"]
        pub mod ID {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Wake Up Message Buffer Register for Data 0-3"]
    pub mod WMB_D03 {
        #[doc = "Received payload corresponding to the data byte 3 under Pretended Networking mode"]
        pub mod Data_byte_3 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Received payload corresponding to the data byte 2 under Pretended Networking mode"]
        pub mod Data_byte_2 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Received payload corresponding to the data byte 1 under Pretended Networking mode"]
        pub mod Data_byte_1 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Received payload corresponding to the data byte 0 under Pretended Networking mode"]
        pub mod Data_byte_0 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Wake Up Message Buffer Register Data 4-7"]
    pub mod WMB_D47 {
        #[doc = "Received payload corresponding to the data byte 7 under Pretended Networking mode"]
        pub mod Data_byte_7 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Received payload corresponding to the data byte 6 under Pretended Networking mode"]
        pub mod Data_byte_6 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Received payload corresponding to the data byte 5 under Pretended Networking mode"]
        pub mod Data_byte_5 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Received payload corresponding to the data byte 4 under Pretended Networking mode"]
        pub mod Data_byte_4 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
#[doc = "Enhanced CAN Bit Timing Prescalers"]
pub mod EPRS {
    #[doc = "Extended Nominal Prescaler Division Factor"]
    pub mod ENPRESDIV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extended Data Phase Prescaler Division Factor"]
    pub mod EDPRESDIV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Enhanced Nominal CAN Bit Timing"]
pub mod ENCBT {
    #[doc = "Nominal Time Segment 1"]
    pub mod NTSEG1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Nominal Time Segment 2"]
    pub mod NTSEG2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Nominal Resynchronization Jump Width"]
    pub mod NRJW {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Enhanced Data Phase CAN bit Timing"]
pub mod EDCBT {
    #[doc = "Data Phase Segment 1"]
    pub mod DTSEG1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Phase Time Segment 2"]
    pub mod DTSEG2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Phase Resynchronization Jump Width"]
    pub mod DRJW {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Enhanced Transceiver Delay Compensation"]
pub mod ETDC {
    #[doc = "Enhanced Transceiver Delay Compensation Value"]
    pub mod ETDCVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transceiver Delay Compensation Fail"]
    pub mod ETDCFAIL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Measured loop delay is in range."]
            pub const IN_RANGE: u32 = 0;
            #[doc = "Measured loop delay is out of range."]
            pub const OUT_OF_RANGE: u32 = 1;
        }
    }
    #[doc = "Enhanced Transceiver Delay Compensation Offset"]
    pub mod ETDCOFF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transceiver Delay Measurement Disable"]
    pub mod TDMDIS {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TDC measurement is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "TDC measurement is disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Transceiver Delay Compensation Enable"]
    pub mod ETDCEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TDC is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "TDC is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "CAN FD Control Register"]
pub mod FDCTRL {
    #[doc = "Transceiver Delay Compensation Value"]
    pub mod TDCVAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transceiver Delay Compensation Offset"]
    pub mod TDCOFF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transceiver Delay Compensation Fail"]
    pub mod TDCFAIL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Measured loop delay is in range."]
            pub const IN_RANGE: u32 = 0;
            #[doc = "Measured loop delay is out of range."]
            pub const OUT_OF_RANGE: u32 = 1;
        }
    }
    #[doc = "Transceiver Delay Compensation Enable"]
    pub mod TDCEN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TDC is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "TDC is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Message Buffer Data Size for Region 0"]
    pub mod MBDSR0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Selects 8 bytes per message buffer."]
            pub const R0_8_BYTES: u32 = 0;
            #[doc = "Selects 16 bytes per message buffer."]
            pub const R0_16_BYTES: u32 = 1;
            #[doc = "Selects 32 bytes per message buffer."]
            pub const R0_32_BYTES: u32 = 2;
            #[doc = "Selects 64 bytes per message buffer."]
            pub const R0_64_BYTES: u32 = 3;
        }
    }
    #[doc = "Bit Rate Switch Enable"]
    pub mod FDRATE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
            pub const NOMINAL: u32 = 0;
            #[doc = "Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
            pub const BIT_RATE_SWITCHING: u32 = 1;
        }
    }
}
#[doc = "CAN FD Bit Timing Register"]
pub mod FDCBT {
    #[doc = "Fast Phase Segment 2"]
    pub mod FPSEG2 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fast Phase Segment 1"]
    pub mod FPSEG1 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fast Propagation Segment"]
    pub mod FPROPSEG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fast Resync Jump Width"]
    pub mod FRJW {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fast Prescaler Division Factor"]
    pub mod FPRESDIV {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CAN FD CRC Register"]
pub mod FDCRC {
    #[doc = "Extended Transmitted CRC value"]
    pub mod FD_TXCRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC Mailbox Number for FD_TXCRC"]
    pub mod FD_MBCRC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Enhanced Rx FIFO Control Register"]
pub mod ERFCR {
    #[doc = "Enhanced Rx FIFO Watermark"]
    pub mod ERFWM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Enhanced Rx FIFO Filter Elements"]
    pub mod NFE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Extended ID Filter Elements"]
    pub mod NEXIF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Last Word"]
    pub mod DMALW {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced Rx FIFO enable"]
    pub mod ERFEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enhanced Rx FIFO is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enhanced Rx FIFO is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Enhanced Rx FIFO Interrupt Enable Register"]
pub mod ERFIER {
    #[doc = "Enhanced Rx FIFO Data Available Interrupt Enable"]
    pub mod ERFDAIE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enhanced Rx FIFO Data Available interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enhanced Rx FIFO Data Available interrupt is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enhanced Rx FIFO Watermark Indication Interrupt Enable"]
    pub mod ERFWMIIE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enhanced Rx FIFO Watermark interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enhanced Rx FIFO Watermark interrupt is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enhanced Rx FIFO Overflow Interrupt Enable"]
    pub mod ERFOVFIE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enhanced Rx FIFO Overflow is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enhanced Rx FIFO Overflow is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enhanced Rx FIFO Underflow Interrupt Enable"]
    pub mod ERFUFWIE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enhanced Rx FIFO Underflow interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enhanced Rx FIFO Underflow interrupt is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Enhanced Rx FIFO Status Register"]
pub mod ERFSR {
    #[doc = "Enhanced Rx FIFO Elements"]
    pub mod ERFEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced Rx FIFO full"]
    pub mod ERFF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Enhanced Rx FIFO is not full"]
            pub const NOT_FULL: u32 = 0;
            #[doc = "Enhanced Rx FIFO is full"]
            pub const FULL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced Rx FIFO empty"]
    pub mod ERFE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Enhanced Rx FIFO is not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Enhanced Rx FIFO is empty"]
            pub const EMPTY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhanced Rx FIFO Clear"]
    pub mod ERFCLR {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Clear Enhanced Rx FIFO content"]
            pub const CLEAR: u32 = 1;
        }
    }
    #[doc = "Enhanced Rx FIFO Data Available"]
    pub mod ERFDA {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const NO_MESSAGE_STORED: u32 = 0;
            #[doc = "There is at least one message stored in Enhanced Rx FIFO"]
            pub const MESSAGE_STORED: u32 = 1;
        }
    }
    #[doc = "Enhanced Rx FIFO Watermark Indication"]
    pub mod ERFWMI {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const WATERMARK_NO: u32 = 0;
            #[doc = "The number of messages in FIFO is greater than the watermark"]
            pub const WATERMARK_YES: u32 = 1;
        }
    }
    #[doc = "Enhanced Rx FIFO Overflow"]
    pub mod ERFOVF {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "Enhanced Rx FIFO overflow"]
            pub const OVERFLOW: u32 = 1;
        }
    }
    #[doc = "Enhanced Rx FIFO Underflow"]
    pub mod ERFUFW {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No such occurrence"]
            pub const NO_UNDERFLOW: u32 = 0;
            #[doc = "Enhanced Rx FIFO underflow"]
            pub const UNDERFLOW: u32 = 1;
        }
    }
}
#[doc = "Enhanced Rx FIFO Filter Element"]
pub mod ERFFEL {
    #[doc = "Filter Element Bits"]
    pub mod FEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
