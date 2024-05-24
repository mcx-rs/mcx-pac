#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "ENET"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "MAC Configuration"]
    pub MAC_CONFIGURATION: crate::RWRegister<u32>,
    #[doc = "MAC Extended Configuration Register"]
    pub MAC_EXT_CONFIGURATION: crate::RWRegister<u32>,
    #[doc = "MAC Packet Filter"]
    pub MAC_PACKET_FILTER: crate::RWRegister<u32>,
    #[doc = "Watchdog Timeout"]
    pub MAC_WATCHDOG_TIMEOUT: crate::RWRegister<u32>,
    _reserved0: [u8; 0x40],
    #[doc = "MAC VLAN Tag Control"]
    pub MAC_VLAN_TAG_CTRL: crate::RWRegister<u32>,
    _reserved1: [u8; 0xc],
    #[doc = "VLAN Tag Inclusion or Replacement"]
    pub MAC_VLAN_INCL: crate::RWRegister<u32>,
    #[doc = "MAC Inner VLAN Tag Inclusion or Replacement"]
    pub MAC_INNER_VLAN_INCL: crate::RWRegister<u32>,
    _reserved2: [u8; 0x8],
    #[doc = "MAC Q0 Tx Flow Control"]
    pub MAC_Q0_TX_FLOW_CTRL: crate::RWRegister<u32>,
    _reserved3: [u8; 0x1c],
    #[doc = "MAC Rx Flow Control"]
    pub MAC_RX_FLOW_CTRL: crate::RWRegister<u32>,
    #[doc = "Receive Queue Control 4"]
    pub MAC_RXQ_CTRL4: crate::RWRegister<u32>,
    _reserved4: [u8; 0x8],
    #[doc = "Receive Queue Control 0"]
    pub MAC_RXQ_CTRL0: crate::RWRegister<u32>,
    #[doc = "Receive Queue Control 1"]
    pub MAC_RXQ_CTRL1: crate::RWRegister<u32>,
    #[doc = "Receive Queue Control 2"]
    pub MAC_RXQ_CTRL2: crate::RWRegister<u32>,
    _reserved5: [u8; 0x4],
    #[doc = "Interrupt Status"]
    pub MAC_INTERRUPT_STATUS: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub MAC_INTERRUPT_ENABLE: crate::RWRegister<u32>,
    #[doc = "Receive Transmit Status"]
    pub MAC_RX_TX_STATUS: crate::RWRegister<u32>,
    _reserved6: [u8; 0x4],
    #[doc = "PMT Control and Status"]
    pub MAC_PMT_CONTROL_STATUS: crate::RWRegister<u32>,
    #[doc = "Remote Wakeup Filter"]
    pub MAC_RWK_PACKET_FILTER: crate::RWRegister<u32>,
    _reserved7: [u8; 0x8],
    #[doc = "LPI Control and Status"]
    pub MAC_LPI_CONTROL_STATUS: crate::RWRegister<u32>,
    #[doc = "LPI Timers Control"]
    pub MAC_LPI_TIMERS_CONTROL: crate::RWRegister<u32>,
    #[doc = "Tx LPI Entry Timer Control"]
    pub MAC_LPI_ENTRY_TIMER: crate::RWRegister<u32>,
    #[doc = "One-microsecond Reference Timer"]
    pub MAC_ONEUS_TIC_COUNTER: crate::RWRegister<u32>,
    _reserved8: [u8; 0x30],
    #[doc = "MAC Version"]
    pub MAC_VERSION: crate::RWRegister<u32>,
    #[doc = "MAC Debug"]
    pub MAC_DEBUG: crate::RWRegister<u32>,
    _reserved9: [u8; 0x4],
    #[doc = "Hardware Features 0"]
    pub MAC_HW_FEATURE0: crate::RWRegister<u32>,
    #[doc = "Hardware Features 1"]
    pub MAC_HW_FEATURE1: crate::RWRegister<u32>,
    #[doc = "Hardware Features 2"]
    pub MAC_HW_FEATURE2: crate::RWRegister<u32>,
    #[doc = "Hardware Features 3"]
    pub MAC_HW_FEATURE3: crate::RWRegister<u32>,
    _reserved10: [u8; 0xd4],
    #[doc = "MDIO Address"]
    pub MAC_MDIO_ADDRESS: crate::RWRegister<u32>,
    #[doc = "MAC MDIO Data"]
    pub MAC_MDIO_DATA: crate::RWRegister<u32>,
    _reserved11: [u8; 0x28],
    #[doc = "CSR Software Control"]
    pub MAC_CSR_SW_CTRL: crate::RWRegister<u32>,
    _reserved12: [u8; 0xcc],
    #[doc = "MAC Address0 High"]
    pub MAC_ADDRESS0_HIGH: crate::RWRegister<u32>,
    #[doc = "MAC Address0 Low"]
    pub MAC_ADDRESS0_LOW: crate::RWRegister<u32>,
    _reserved13: [u8; 0x768],
    #[doc = "Indirect Access Control"]
    pub INDIR_ACCESS_CTRL: crate::RWRegister<u32>,
    #[doc = "Indirect Access Data"]
    pub INDIR_ACCESS_DATA: crate::RWRegister<u32>,
    _reserved14: [u8; 0x88],
    #[doc = "Timestamp Control"]
    pub MAC_TIMESTAMP_CONTROL: crate::RWRegister<u32>,
    #[doc = "Subsecond Increment"]
    pub MAC_SUB_SECOND_INCREMENT: crate::RWRegister<u32>,
    #[doc = "System Time Seconds"]
    pub MAC_SYSTEM_TIME_SECONDS: crate::RWRegister<u32>,
    #[doc = "System Time Nanoseconds"]
    pub MAC_SYSTEM_TIME_NANOSECONDS: crate::RWRegister<u32>,
    #[doc = "System Time Seconds Update"]
    pub MAC_SYSTEM_TIME_SECONDS_UPDATE: crate::RWRegister<u32>,
    #[doc = "System Time Nanoseconds Update"]
    pub MAC_SYSTEM_TIME_NANOSECONDS_UPDATE: crate::RWRegister<u32>,
    #[doc = "Timestamp Addend"]
    pub MAC_TIMESTAMP_ADDEND: crate::RWRegister<u32>,
    _reserved15: [u8; 0x4],
    #[doc = "Timestamp Status"]
    pub MAC_TIMESTAMP_STATUS: crate::RWRegister<u32>,
    _reserved16: [u8; 0xc],
    #[doc = "Transmit Timestamp Status Nanoseconds"]
    pub MAC_TX_TIMESTAMP_STATUS_NANOSECONDS: crate::RWRegister<u32>,
    #[doc = "Transmit Timestamp Status Seconds"]
    pub MAC_TX_TIMESTAMP_STATUS_SECONDS: crate::RWRegister<u32>,
    _reserved17: [u8; 0x20],
    #[doc = "Timestamp Ingress Correction Nanosecond"]
    pub MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND: crate::RWRegister<u32>,
    #[doc = "Timestamp Egress Correction Nanosecond"]
    pub MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND: crate::RWRegister<u32>,
    _reserved18: [u8; 0x8],
    #[doc = "Timestamp Ingress Latency"]
    pub MAC_TIMESTAMP_INGRESS_LATENCY: crate::RWRegister<u32>,
    #[doc = "Timestamp Egress Latency"]
    pub MAC_TIMESTAMP_EGRESS_LATENCY: crate::RWRegister<u32>,
    #[doc = "PPS Control"]
    pub MAC_PPS_CONTROL: crate::RWRegister<u32>,
    _reserved19: [u8; 0xc],
    #[doc = "PPS0 Target Time Seconds"]
    pub PPS0_TARGET_TIME_SECONDS: crate::RWRegister<u32>,
    #[doc = "PPS0 Target Time Nanoseconds"]
    pub PPS0_TARGET_TIME_NANOSECONDS: crate::RWRegister<u32>,
    _reserved20: [u8; 0x78],
    #[doc = "MTL Operation Mode"]
    pub MTL_OPERATION_MODE: crate::RWRegister<u32>,
    _reserved21: [u8; 0x1c],
    #[doc = "MTL Interrupt Status"]
    pub MTL_INTERRUPT_STATUS: crate::RWRegister<u32>,
    _reserved22: [u8; 0xc],
    #[doc = "Receive Queue and DMA Channel Mapping 0"]
    pub MTL_RXQ_DMA_MAP0: crate::RWRegister<u32>,
    _reserved23: [u8; 0xcc],
    #[doc = "Cluster QUEUE%s, containing MTL_TXQ?_OPERATION_MODE, MTL_TXQ?_UNDERFLOW, MTL_TXQ?_DEBUG, MTL_TXQ?_ETS_STATUS, MTL_TXQ?_QUANTUM_WEIGHT, MTL_Q?_INTERRUPT_CONTROL_STATUS, MTL_RXQ?_OPERATION_MODE, MTL_RXQ?_MISSED_PACKET_OVERFLOW_CNT, MTL_RXQ?_DEBUG, MTL_RXQ?_CONTROL"]
    pub QUEUE: [queue::RegisterBlock; 2usize],
    #[doc = "Queue 1 ETS Control"]
    pub MTL_TXQ1_ETS_CONTROL: crate::RWRegister<u32>,
    #[doc = "Queue 1 sendSlopeCredit"]
    pub MTL_TXQ1_SENDSLOPECREDIT: crate::RWRegister<u32>,
    #[doc = "Queue 1 hiCredit"]
    pub MTL_TXQ1_HICREDIT: crate::RWRegister<u32>,
    #[doc = "Queue 1 loCredit"]
    pub MTL_TXQ1_LOCREDIT: crate::RWRegister<u32>,
    _reserved24: [u8; 0x270],
    #[doc = "DMA Bus Mode"]
    pub DMA_MODE: crate::RWRegister<u32>,
    #[doc = "DMA System Bus Mode"]
    pub DMA_SYSBUS_MODE: crate::RWRegister<u32>,
    #[doc = "DMA Interrupt Status"]
    pub DMA_INTERRUPT_STATUS: crate::RWRegister<u32>,
    #[doc = "DMA Debug Status 0"]
    pub DMA_DEBUG_STATUS0: crate::RWRegister<u32>,
    _reserved25: [u8; 0xf0],
    #[doc = "Cluster DMA_CH%s, containing DMA_CH?_CONTROL, DMA_CH?_TX_CONTROL, DMA_CH?_RX_CONTROL, DMA_CH?_TXDESC_LIST_ADDRESS, DMA_CH?_RXDESC_LIST_ADDRESS, DMA_CH?_TXDESC_TAIL_POINTER, DMA_CH?_RXDESC_TAIL_POINTER, DMA_CH?_TXDESC_RING_LENGTH, DMA_CH?_RX_CONTROL2, DMA_CH?_INTERRUPT_ENABLE, DMA_CH?_RX_INTERRUPT_WATCHDOG_TIMER, DMA_CH?_SLOT_FUNCTION_CONTROL_STATUS, DMA_CH?_CURRENT_APP_TXDESC, DMA_CH?_CURRENT_APP_RXDESC, DMA_CH?_CURRENT_APP_TXBUFFER, DMA_CH?_CURRENT_APP_RXBUFFER, DMA_CH?_STATUS, DMA_CH?_MISS_FRAME_CNT, DMA_CH?_RX_ERI_CNT"]
    pub DMA_CH: [dma_ch::RegisterBlock; 2usize],
}
#[doc = "MAC Configuration"]
pub mod MAC_CONFIGURATION {
    #[doc = "Receiver Enable"]
    pub mod RE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receiver is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Transmitter Enable"]
    pub mod TE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmitter is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Preamble Length for Transmit packets"]
    pub mod PRELEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "7 bytes of preamble"]
            pub const M_7BYTES: u32 = 0;
            #[doc = "5 bytes of preamble"]
            pub const M_5BYTES: u32 = 1;
            #[doc = "3 bytes of preamble"]
            pub const M_3BYTES: u32 = 2;
        }
    }
    #[doc = "Deferral Check"]
    pub mod DC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Deferral check function is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Deferral check function is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Back-Off Limit"]
    pub mod BL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "k = min(n,10)"]
            pub const MIN_N_10: u32 = 0;
            #[doc = "k = min(n,8)"]
            pub const MIN_N_8: u32 = 1;
            #[doc = "k = min(n,4)"]
            pub const MIN_N_4: u32 = 2;
            #[doc = "k = min(n,1)"]
            pub const MIN_N_1: u32 = 3;
        }
    }
    #[doc = "Disable Retry"]
    pub mod DR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Retry"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable Retry"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Disable Carrier Sense During Transmission"]
    pub mod DCRS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Carrier Sense During Transmission"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable Carrier Sense During Transmission"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Disable Receive Own"]
    pub mod DO {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Receive Own"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable Receive Own"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Enable Carrier Sense Before Transmission in Full-Duplex Mode"]
    pub mod ECRSFD {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ECRSFD is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "ECRSFD is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Loopback Mode"]
    pub mod LM {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Loopback is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Loopback is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Duplex Mode"]
    pub mod DM {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Half-duplex mode"]
            pub const HDUPLX: u32 = 0;
            #[doc = "Full-duplex mode"]
            pub const FDUPLX: u32 = 1;
        }
    }
    #[doc = "Speed"]
    pub mod FES {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "10 Mbps when PS bit is 1 and 1 Gbps when PS bit is 0"]
            pub const M_10_1000M: u32 = 0;
            #[doc = "100 Mbps when PS bit is 1 and 2.5 Gbps when PS bit is 0"]
            pub const M_100_2500M: u32 = 1;
        }
    }
    #[doc = "Port Select"]
    pub mod PS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "For 1000 or 2500 Mbps operations"]
            pub const M_1000_2500M: u32 = 0;
            #[doc = "For 10 or 100 Mbps operations"]
            pub const M_10_100M: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Jumbo Packet Enable"]
    pub mod JE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Jumbo packet is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Jumbo packet is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Jabber Disable"]
    pub mod JD {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Jabber is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Jabber is disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Watchdog Disable"]
    pub mod WD {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Watchdog is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Watchdog is disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Automatic Pad or CRC Stripping"]
    pub mod ACS {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Automatic Pad or CRC Stripping is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Automatic Pad or CRC Stripping is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "CRC stripping for Type packets"]
    pub mod CST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CRC stripping for Type packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "CRC stripping for Type packets is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "IEEE 802.3as Support for 2K Packets"]
    pub mod S2KP {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Support upto 2K packet is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Support upto 2K packet is Enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Giant Packet Size Limit Control Enable"]
    pub mod GPSLCE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Giant Packet Size Limit Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Giant Packet Size Limit Control is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Inter-Packet Gap"]
    pub mod IPG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "96 bit times IPG"]
            pub const IPG96: u32 = 0;
            #[doc = "88 bit times IPG"]
            pub const IPG88: u32 = 1;
            #[doc = "80 bit times IPG"]
            pub const IPG80: u32 = 2;
            #[doc = "72 bit times IPG"]
            pub const IPG72: u32 = 3;
            #[doc = "64 bit times IPG"]
            pub const IPG64: u32 = 4;
            #[doc = "56 bit times IPG"]
            pub const IPG56: u32 = 5;
            #[doc = "48 bit times IPG"]
            pub const IPG48: u32 = 6;
            #[doc = "40 bit times IPG"]
            pub const IPG40: u32 = 7;
        }
    }
    #[doc = "Checksum Offload"]
    pub mod IPC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IP header/payload checksum checking is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "IP header/payload checksum checking is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Source Address Insertion or Replacement Control"]
    pub mod SARC {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "mti_sa_ctrl_i and ati_sa_ctrl_i input signals control the SA field generation"]
            pub const SA_CTRL_IN: u32 = 0;
            #[doc = "Contents of MAC Addr-0 inserted in SA field"]
            pub const MAC0_INS_SA: u32 = 2;
            #[doc = "Contents of MAC Addr-0 replaces SA field"]
            pub const MAC0_REP_SA: u32 = 3;
            #[doc = "Contents of MAC Addr-1 inserted in SA field"]
            pub const MAC1_INS_SA: u32 = 6;
            #[doc = "Contents of MAC Addr-1 replaces SA field"]
            pub const MAC1_REP_SA: u32 = 7;
        }
    }
}
#[doc = "MAC Extended Configuration Register"]
pub mod MAC_EXT_CONFIGURATION {
    #[doc = "Giant Packet Size Limit"]
    pub mod GPSL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Disable CRC Checking for Received Packets"]
    pub mod DCRCC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CRC Checking is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "CRC Checking is disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Slow Protocol Detection Enable"]
    pub mod SPEN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow Protocol Detection is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Slow Protocol Detection is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Unicast Slow Protocol Packet Detect"]
    pub mod USP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unicast Slow Protocol Packet Detection is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unicast Slow Protocol Packet Detection is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Packet Duplication Control"]
    pub mod PDC {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Packet Duplication Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Packet Duplication Control is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Extended Inter-Packet Gap Enable"]
    pub mod EIPGEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Extended Inter-Packet Gap is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Extended Inter-Packet Gap is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Extended Inter-Packet Gap"]
    pub mod EIPG {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Packet Filter"]
pub mod MAC_PACKET_FILTER {
    #[doc = "Promiscuous Mode"]
    pub mod PR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Promiscuous Mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Promiscuous Mode is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "DA Inverse Filtering"]
    pub mod DAIF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DA Inverse Filtering is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "DA Inverse Filtering is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Pass All Multicast"]
    pub mod PM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pass All Multicast is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Pass All Multicast is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Disable Broadcast Packets"]
    pub mod DBF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Broadcast Packets"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disable Broadcast Packets"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Pass Control Packets"]
    pub mod PCF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MAC filters all control packets from reaching the application"]
            pub const FLTR_ALL: u32 = 0;
            #[doc = "MAC forwards all control packets except Pause packets to the application even if they fail the address filter"]
            pub const FW_XCPT_PAU: u32 = 1;
            #[doc = "MAC forwards all control packets to the application even if they fail the address filter"]
            pub const FW_ALL: u32 = 2;
            #[doc = "MAC forwards the control packets that pass the Address filter"]
            pub const FW_PASS: u32 = 3;
        }
    }
    #[doc = "VLAN Tag Filter Enable"]
    pub mod VTFE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Tag Filter is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Tag Filter is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Receive All"]
    pub mod RA {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive All is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive All is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Watchdog Timeout"]
pub mod MAC_WATCHDOG_TIMEOUT {
    #[doc = "Watchdog Timeout"]
    pub mod WTO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2 KB"]
            pub const BF_2KBYTES: u32 = 0;
            #[doc = "3 KB"]
            pub const BF_3KBYTES: u32 = 1;
            #[doc = "4 KB"]
            pub const BF_4KBYTES: u32 = 2;
            #[doc = "5 KB"]
            pub const BF_5KBYTES: u32 = 3;
            #[doc = "6 KB"]
            pub const BF_6KBYTES: u32 = 4;
            #[doc = "7 KB"]
            pub const BF_7KBYTES: u32 = 5;
            #[doc = "8 KB"]
            pub const BF_8KBYTES: u32 = 6;
            #[doc = "9 KB"]
            pub const BF_9KBYTES: u32 = 7;
            #[doc = "10 KB"]
            pub const BF_10KBYTES: u32 = 8;
            #[doc = "11 KB"]
            pub const BF_11KBYTES: u32 = 9;
            #[doc = "12 KB"]
            pub const BF_12KBYTES: u32 = 10;
            #[doc = "13 KB"]
            pub const BF_13KBYTES: u32 = 11;
            #[doc = "14 KB"]
            pub const BF_14KBYTES: u32 = 12;
            #[doc = "15 KB"]
            pub const BF_15KBYTES: u32 = 13;
            #[doc = "16383 Bytes"]
            pub const BF_16383BYTES: u32 = 14;
        }
    }
    #[doc = "Programmable Watchdog Enable"]
    pub mod PWE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Programmable Watchdog is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Programmable Watchdog is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "MAC VLAN Tag Control"]
pub mod MAC_VLAN_TAG_CTRL {
    #[doc = "VLAN Tag Identifier for Receive Packets"]
    pub mod VL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable 12-Bit VLAN Tag Comparison"]
    pub mod ETV {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "12-bit VLAN Tag Comparison is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "12-bit VLAN Tag Comparison is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "VLAN Tag Inverse Match Enable"]
    pub mod VTIM {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Tag Inverse Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Tag Inverse Match is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable S-VLAN"]
    pub mod ESVL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "S-VLAN is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "S-VLAN is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Receive S-VLAN Match"]
    pub mod ERSVLM {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive S-VLAN Match is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive S-VLAN Match is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Disable VLAN Type Check"]
    pub mod DOVLTC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Type Check is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "VLAN Type Check is disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Enable VLAN Tag Stripping on Receive"]
    pub mod EVLS {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not strip"]
            pub const DONOT: u32 = 0;
            #[doc = "Strip if VLAN filter passes"]
            pub const IFPASS: u32 = 1;
            #[doc = "Strip if VLAN filter fails"]
            pub const IFFAIL: u32 = 2;
            #[doc = "Always strip"]
            pub const ALWAYS: u32 = 3;
        }
    }
    #[doc = "Enable VLAN Tag in Rx status"]
    pub mod EVLRXS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Tag in Rx status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Tag in Rx status is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Double VLAN Processing"]
    pub mod EDVLP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Double VLAN Processing is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Double VLAN Processing is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Inner VLAN Tag"]
    pub mod ERIVLT {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inner VLAN tag is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Inner VLAN tag is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Inner VLAN Tag Stripping on Receive"]
    pub mod EIVLS {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not strip"]
            pub const DONOT: u32 = 0;
            #[doc = "Strip if VLAN filter passes"]
            pub const IFPASS: u32 = 1;
            #[doc = "Strip if VLAN filter fails"]
            pub const IFFAIL: u32 = 2;
            #[doc = "Always strip"]
            pub const ALWAYS: u32 = 3;
        }
    }
    #[doc = "Enable Inner VLAN Tag in Rx Status"]
    pub mod EIVLRXS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inner VLAN Tag in Rx status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Inner VLAN Tag in Rx status is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "VLAN Tag Inclusion or Replacement"]
pub mod MAC_VLAN_INCL {
    #[doc = "VLAN Tag for Transmit Packets"]
    pub mod VLT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VLAN Tag Control in Transmit Packets"]
    pub mod VLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No VLAN tag deletion, insertion, or replacement"]
            pub const NONE: u32 = 0;
            #[doc = "VLAN tag deletion"]
            pub const DELETE: u32 = 1;
            #[doc = "VLAN tag insertion"]
            pub const INSERT: u32 = 2;
            #[doc = "VLAN tag replacement"]
            pub const REPLACE: u32 = 3;
        }
    }
    #[doc = "VLAN Priority Control"]
    pub mod VLP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Priority Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Priority Control is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "C-VLAN or S-VLAN"]
    pub mod CSVL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "C-VLAN type (0x8100) is inserted or replaced"]
            pub const C_VLAN: u32 = 0;
            #[doc = "S-VLAN type (0x88A8) is inserted or replaced"]
            pub const S_VLAN: u32 = 1;
        }
    }
    #[doc = "VLAN Tag Input"]
    pub mod VLTI {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Tag Input is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Tag Input is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Channel based tag insertion"]
    pub mod CBTI {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Channel based tag insertion is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Channel based tag insertion is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read write control"]
    pub mod RDWR {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read operation of indirect access"]
            pub const READ: u32 = 0;
            #[doc = "Write operation of indirect access"]
            pub const WRITE: u32 = 1;
        }
    }
    #[doc = "Busy"]
    pub mod BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Busy status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Busy status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Inner VLAN Tag Inclusion or Replacement"]
pub mod MAC_INNER_VLAN_INCL {
    #[doc = "VLAN Tag for Transmit Packets"]
    pub mod VLT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VLAN Tag Control in Transmit Packets"]
    pub mod VLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No VLAN tag deletion, insertion, or replacement"]
            pub const NONE: u32 = 0;
            #[doc = "VLAN tag deletion"]
            pub const DELETE: u32 = 1;
            #[doc = "VLAN tag insertion"]
            pub const INSERT: u32 = 2;
            #[doc = "VLAN tag replacement"]
            pub const REPLACE: u32 = 3;
        }
    }
    #[doc = "VLAN Priority Control"]
    pub mod VLP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Priority Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Priority Control is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "C-VLAN or S-VLAN"]
    pub mod CSVL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "C-VLAN type (0x8100) is inserted"]
            pub const CVLAN: u32 = 0;
            #[doc = "S-VLAN type (0x88A8) is inserted"]
            pub const SVLAN: u32 = 1;
        }
    }
    #[doc = "VLAN Tag Input"]
    pub mod VLTI {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN Tag Input is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN Tag Input is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "MAC Q0 Tx Flow Control"]
pub mod MAC_Q0_TX_FLOW_CTRL {
    #[doc = "Flow Control Busy or Backpressure Activate"]
    pub mod FCB_BPA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flow Control Busy or Backpressure Activate is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Flow Control Busy or Backpressure Activate is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Transmit Flow Control Enable"]
    pub mod TFE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Flow Control is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Pause Low Threshold"]
    pub mod PLT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pause Time minus 4 Slot Times (PT -4 slot times)"]
            pub const PT4: u32 = 0;
            #[doc = "Pause Time minus 28 Slot Times (PT -28 slot times)"]
            pub const PT28: u32 = 1;
            #[doc = "Pause Time minus 36 Slot Times (PT -36 slot times)"]
            pub const PT36: u32 = 2;
            #[doc = "Pause Time minus 144 Slot Times (PT -144 slot times)"]
            pub const PT144: u32 = 3;
            #[doc = "Pause Time minus 256 Slot Times (PT -256 slot times)"]
            pub const PT256: u32 = 4;
            #[doc = "Pause Time minus 512 Slot Times (PT -512 slot times)"]
            pub const PT512: u32 = 5;
        }
    }
    #[doc = "Disable Zero-Quanta Pause"]
    pub mod DZPQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Zero-Quanta Pause packet generation is enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Zero-Quanta Pause packet generation is disabled"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "Pause Time"]
    pub mod PT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Rx Flow Control"]
pub mod MAC_RX_FLOW_CTRL {
    #[doc = "Receive Flow Control Enable"]
    pub mod RFE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Flow Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Flow Control is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Unicast Pause Packet Detect"]
    pub mod UP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unicast Pause Packet Detect disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unicast Pause Packet Detect enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Receive Queue Control 4"]
pub mod MAC_RXQ_CTRL4 {
    #[doc = "Unicast Address Filter Fail Packets Queuing Enable."]
    pub mod UFFQE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unicast Address Filter Fail Packets Queuing is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Unicast Address Filter Fail Packets Queuing is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Unicast Address Filter Fail Packets Queue."]
    pub mod UFFQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multicast Address Filter Fail Packets Queuing Enable."]
    pub mod MFFQE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Multicast Address Filter Fail Packets Queuing is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Multicast Address Filter Fail Packets Queuing is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Multicast Address Filter Fail Packets Queue."]
    pub mod MFFQ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VLAN Tag Filter Fail Packets Queuing Enable"]
    pub mod VFFQE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "VLAN tag Filter Fail Packets Queuing is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "VLAN tag Filter Fail Packets Queuing is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "VLAN Tag Filter Fail Packets Queue"]
    pub mod VFFQ {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Queue Control 0"]
pub mod MAC_RXQ_CTRL0 {
    #[doc = "Receive Queue 0 Enable"]
    pub mod RXQ0EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue enabled for AV"]
            pub const EN_AV: u32 = 1;
            #[doc = "Queue enabled for DCB/Generic"]
            pub const EN_DCB_GEN: u32 = 2;
        }
    }
    #[doc = "Receive Queue 1 Enable"]
    pub mod RXQ1EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue enabled for AV"]
            pub const EN_AV: u32 = 1;
            #[doc = "Queue enabled for DCB/Generic"]
            pub const EN_DCB_GEN: u32 = 2;
        }
    }
}
#[doc = "Receive Queue Control 1"]
pub mod MAC_RXQ_CTRL1 {
    #[doc = "AV Untagged Control Packets Queue"]
    pub mod AVCPQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue 0"]
            pub const QUEUE0: u32 = 0;
            #[doc = "Receive Queue 1"]
            pub const QUEUE1: u32 = 1;
        }
    }
    #[doc = "PTP Packets Queue"]
    pub mod PTPQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue 0"]
            pub const QUEUE0: u32 = 0;
            #[doc = "Receive Queue 1"]
            pub const QUEUE1: u32 = 1;
        }
    }
    #[doc = "Untagged Packet Queue"]
    pub mod UPQ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue 0"]
            pub const QUEUE0: u32 = 0;
            #[doc = "Receive Queue 1"]
            pub const QUEUE1: u32 = 1;
        }
    }
    #[doc = "Multicast and Broadcast Queue"]
    pub mod MCBCQ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Queue 0"]
            pub const QUEUE0: u32 = 0;
            #[doc = "Receive Queue 1"]
            pub const QUEUE1: u32 = 1;
        }
    }
    #[doc = "Multicast and Broadcast Queue Enable"]
    pub mod MCBCQEN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Multicast and Broadcast Queue is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Multicast and Broadcast Queue is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Tagged AV Control Packets Queuing Enable."]
    pub mod TACPQE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tagged AV Control Packets Queuing is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Tagged AV Control Packets Queuing is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Tagged PTP over Ethernet Packets Queuing Control."]
    pub mod TPQC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OMCBCQ"]
    pub mod OMCBCQ {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "overriding MCBCQ priority disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "overriding MCBCQ priority enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Type Field Based Rx Queuing Enable"]
    pub mod TBRQE {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Queue Control 2"]
pub mod MAC_RXQ_CTRL2 {
    #[doc = "Priorities Selected in the Receive Queue 0"]
    pub mod PSRQ0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Priorities Selected in the Receive Queue 1"]
    pub mod PSRQ1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status"]
pub mod MAC_INTERRUPT_STATUS {
    #[doc = "PHY Interrupt"]
    pub mod PHYIS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "PHY Interrupt not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PHY Interrupt detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PMTIS"]
    pub mod PMTIS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "PMT Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PMT Interrupt status active"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPIIS"]
    pub mod LPIIS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "LPI Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "LPI Interrupt status active"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TSIS"]
    pub mod TSIS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Timestamp Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Interrupt status active"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Status Interrupt"]
    pub mod TXSTSIS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Interrupt status active"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Status Interrupt"]
    pub mod RXSTSIS {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Interrupt status active"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MDIO Interrupt Status"]
    pub mod MDIOIS {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "MDIO Interrupt status not active"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MDIO Interrupt status active"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable"]
pub mod MAC_INTERRUPT_ENABLE {
    #[doc = "PHY Interrupt Enable"]
    pub mod PHYIE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PHY Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "PHY Interrupt is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "PMT Interrupt Enable"]
    pub mod PMTIE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PMT Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "PMT Interrupt is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "LPI Interrupt Enable"]
    pub mod LPIIE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "LPI Interrupt is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Timestamp Interrupt Enable"]
    pub mod TSIE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp Interrupt is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Transmit Status Interrupt Enable"]
    pub mod TXSTSIE {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Status Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp Status Interrupt is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Receive Status Interrupt Enable"]
    pub mod RXSTSIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive Status Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive Status Interrupt is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "MDIO Interrupt Enable"]
    pub mod MDIOIE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MDIO Interrupt is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MDIO Interrupt is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Receive Transmit Status"]
pub mod MAC_RX_TX_STATUS {
    #[doc = "Transmit Jabber Timeout"]
    pub mod TJT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No Transmit Jabber Timeout"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Jabber Timeout occurred"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "No Carrier"]
    pub mod NCARR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Carrier is present"]
            pub const INACTIVE: u32 = 0;
            #[doc = "No carrier"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loss of Carrier"]
    pub mod LCARR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Carrier is present"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Loss of carrier"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Excessive Deferral"]
    pub mod EXDEF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No Excessive deferral"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Excessive deferral"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Late Collision"]
    pub mod LCOL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No collision"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Late collision is sensed"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Excessive Collisions"]
    pub mod EXCOL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No collision"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Excessive collision is sensed"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Watchdog Timeout"]
    pub mod RWT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No receive watchdog timeout"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive watchdog timed out"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PMT Control and Status"]
pub mod MAC_PMT_CONTROL_STATUS {
    #[doc = "Power Down"]
    pub mod PWRDWN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Power down is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Power down is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Magic Packet Enable"]
    pub mod MGKPKTEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Magic Packet is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Magic Packet is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Remote Wake-Up Packet Enable"]
    pub mod RWKPKTEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remote wake-up packet is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Remote wake-up packet is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Magic Packet Received"]
    pub mod MGKPRCVD {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No Magic packet is received"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Magic packet is received"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Wake-Up Packet Received"]
    pub mod RWKPRCVD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Remote wake-up packet is received"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Remote wake-up packet is received"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Global Unicast"]
    pub mod GLBLUCAST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Global unicast is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Global unicast is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Remote Wake-up Packet Forwarding Enable"]
    pub mod RWKPFE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remote Wake-up Packet Forwarding is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Remote Wake-up Packet Forwarding is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Remote Wake-up FIFO Pointer"]
    pub mod RWKPTR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Remote Wake-Up Packet Filter Register Pointer Reset"]
    pub mod RWKFILTRST {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Remote Wake-Up Packet Filter Register Pointer is not Reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Remote Wake-Up Packet Filter Register Pointer is Reset"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Remote Wakeup Filter"]
pub mod MAC_RWK_PACKET_FILTER {
    #[doc = "RWK Packet Filter"]
    pub mod WKUPFRMFTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LPI Control and Status"]
pub mod MAC_LPI_CONTROL_STATUS {
    #[doc = "Transmit LPI Entry"]
    pub mod TLPIEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit LPI entry not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit LPI entry detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit LPI Exit"]
    pub mod TLPIEX {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit LPI exit not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit LPI exit detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive LPI Entry"]
    pub mod RLPIEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive LPI entry not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive LPI entry detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive LPI Exit"]
    pub mod RLPIEX {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive LPI exit not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive LPI exit detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit LPI State"]
    pub mod TLPIST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit LPI state not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit LPI state detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive LPI State"]
    pub mod RLPIST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive LPI state not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive LPI state detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI Enable"]
    pub mod LPIEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI state is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "LPI state is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "PHY Link Status"]
    pub mod PLS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "link is down"]
            pub const DISABLE: u32 = 0;
            #[doc = "link is okay (UP)"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "LPI Tx Automate"]
    pub mod LPITXA {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI Tx Automate is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "LPI Tx Automate is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "LPI Timer Enable"]
    pub mod LPIATE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI Timer is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "LPI Timer is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "LPI Tx Clock Stop Enable"]
    pub mod LPITCSE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LPI Tx Clock Stop is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "LPI Tx Clock Stop is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "LPI Timers Control"]
pub mod MAC_LPI_TIMERS_CONTROL {
    #[doc = "LPI TW Timer"]
    pub mod TWT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LPI LS Timer"]
    pub mod LST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Tx LPI Entry Timer Control"]
pub mod MAC_LPI_ENTRY_TIMER {
    #[doc = "LPI Entry Timer"]
    pub mod LPIET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b11111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "One-microsecond Reference Timer"]
pub mod MAC_ONEUS_TIC_COUNTER {
    #[doc = "1US TIC Counter"]
    pub mod TIC_1US_CNTR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Version"]
pub mod MAC_VERSION {
    #[doc = "Synopsys-defined Version"]
    pub mod SNPSVER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "User-defined Version"]
    pub mod USERVER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Debug"]
pub mod MAC_DEBUG {
    #[doc = "MAC GMII or MII Receive Protocol Engine Status"]
    pub mod RPESTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "MAC GMII or MII Receive Protocol Engine Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MAC GMII or MII Receive Protocol Engine Status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC Receive Packet Controller FIFO Status"]
    pub mod RFCFCSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC GMII or MII Transmit Protocol Engine Status"]
    pub mod TPESTS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "MAC GMII or MII Transmit Protocol Engine Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MAC GMII or MII Transmit Protocol Engine Status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC Transmit Packet Controller Status"]
    pub mod TFCSTS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Idle state"]
            pub const IDLE: u32 = 0;
            #[doc = "Waiting for one of the following: Status of the previous packet OR IPG or back off period to be over"]
            pub const WAITING: u32 = 1;
            #[doc = "Generating and transmitting a Pause control packet (in full-duplex mode)"]
            pub const GEN_TX_PAU: u32 = 2;
            #[doc = "Transferring input packet for transmission"]
            pub const TRNSFR: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hardware Features 0"]
pub mod MAC_HW_FEATURE0 {
    #[doc = "10 or 100 Mbps Support"]
    pub mod MIISEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No 10 or 100 Mbps support"]
            pub const INACTIVE: u32 = 0;
            #[doc = "10 or 100 Mbps support"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "1000 Mbps Support"]
    pub mod GMIISEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No 1000 Mbps support"]
            pub const INACTIVE: u32 = 0;
            #[doc = "1000 Mbps support"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Half-duplex Support"]
    pub mod HDSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No Half-duplex support"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Half-duplex support"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PCS Registers (TBI, SGMII, or RTBI PHY interface)"]
    pub mod PCSSEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No PCS Registers (TBI, SGMII, or RTBI PHY interface)"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PCS Registers (TBI, SGMII, or RTBI PHY interface)"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VLAN Hash Filter Selected"]
    pub mod VLHASH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "VLAN Hash Filter not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "VLAN Hash Filter selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SMA (MDIO) Interface"]
    pub mod SMASEL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "SMA (MDIO) Interface not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "SMA (MDIO) Interface selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PMT Remote Wake-up Packet Enable"]
    pub mod RWKSEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "PMT Remote Wake-up Packet Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PMT Remote Wake-up Packet Enable option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PMT Magic Packet Enable"]
    pub mod MGKSEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "PMT Magic Packet Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PMT Magic Packet Enable option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RMON Module Enable"]
    pub mod MMCSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "RMON Module Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "RMON Module Enable option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ARP Offload Enabled"]
    pub mod ARPOFFSEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "ARP Offload Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "ARP Offload Enable option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IEEE 1588-2008 Timestamp Enabled"]
    pub mod TSSEL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "IEEE 1588-2008 Timestamp Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "IEEE 1588-2008 Timestamp Enable option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Energy Efficient Ethernet Enabled"]
    pub mod EEESEL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Energy Efficient Ethernet Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Energy Efficient Ethernet Enable option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Checksum Offload Enabled"]
    pub mod TXCOESEL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit Checksum Offload Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Checksum Offload Enable option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Checksum Offload Enabled"]
    pub mod RXCOESEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive Checksum Offload Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Receive Checksum Offload Enable option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC Addresses 1-31 Selected"]
    pub mod ADDMACADRSEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC Addresses 32-63 Selected"]
    pub mod MACADR32SEL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "MAC Addresses 32-63 Select option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MAC Addresses 32-63 Select option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC Addresses 64-127 Selected"]
    pub mod MACADR64SEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "MAC Addresses 64-127 Select option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MAC Addresses 64-127 Select option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timestamp System Time Source"]
    pub mod TSSTSSEL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Internal"]
            pub const INTRNL: u32 = 0;
            #[doc = "External"]
            pub const EXTRNL: u32 = 1;
            #[doc = "Both"]
            pub const BOTH: u32 = 2;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Source Address or VLAN Insertion Enable"]
    pub mod SAVLANINS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Source Address or VLAN Insertion Enable option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Source Address or VLAN Insertion Enable option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active PHY Selected"]
    pub mod ACTPHYSEL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "GMII or MII"]
            pub const GMII_MII: u32 = 0;
            #[doc = "RGMII"]
            pub const RGMII: u32 = 1;
            #[doc = "SGMII"]
            pub const SGMII: u32 = 2;
            #[doc = "TBI"]
            pub const TBI: u32 = 3;
            #[doc = "RMII"]
            pub const RMII: u32 = 4;
            #[doc = "RTBI"]
            pub const RTBI: u32 = 5;
            #[doc = "SMII"]
            pub const SMII: u32 = 6;
            #[doc = "RevMII"]
            pub const REVMIII: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hardware Features 1"]
pub mod MAC_HW_FEATURE1 {
    #[doc = "MTL Receive FIFO Size"]
    pub mod RXFIFOSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {
            #[doc = "128 bytes"]
            pub const M_128B: u32 = 0;
            #[doc = "256 bytes"]
            pub const M_256B: u32 = 1;
            #[doc = "512 bytes"]
            pub const M_512B: u32 = 2;
            #[doc = "1024 bytes"]
            pub const M_1024B: u32 = 3;
            #[doc = "2048 bytes"]
            pub const M_2048B: u32 = 4;
            #[doc = "4096 bytes"]
            pub const M_4096B: u32 = 5;
            #[doc = "8192 bytes"]
            pub const M_8192B: u32 = 6;
            #[doc = "16384 bytes"]
            pub const M_16384B: u32 = 7;
            #[doc = "32 KB"]
            pub const M_32KB: u32 = 8;
            #[doc = "64 KB"]
            pub const M_64KB: u32 = 9;
            #[doc = "128 KB"]
            pub const M_128KB: u32 = 10;
            #[doc = "256 KB"]
            pub const M_256KB: u32 = 11;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Single Port RAM Enable"]
    pub mod SPRAM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Single Port RAM feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Single Port RAM feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MTL Transmit FIFO Size"]
    pub mod TXFIFOSIZE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {
            #[doc = "128 bytes"]
            pub const M_128B: u32 = 0;
            #[doc = "256 bytes"]
            pub const M_256B: u32 = 1;
            #[doc = "512 bytes"]
            pub const M_512B: u32 = 2;
            #[doc = "1024 bytes"]
            pub const M_1024B: u32 = 3;
            #[doc = "2048 bytes"]
            pub const M_2048B: u32 = 4;
            #[doc = "4096 bytes"]
            pub const M_4096B: u32 = 5;
            #[doc = "8192 bytes"]
            pub const M_8192B: u32 = 6;
            #[doc = "16384 bytes"]
            pub const M_16384B: u32 = 7;
            #[doc = "32 KB"]
            pub const M_32KB: u32 = 8;
            #[doc = "64 KB"]
            pub const M_64KB: u32 = 9;
            #[doc = "128 KB"]
            pub const M_128KB: u32 = 10;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "One-Step Timestamping Enable"]
    pub mod OSTEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "One-Step Timestamping feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "One-Step Timestamping feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PTP Offload Enable"]
    pub mod PTOEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "PTP Offload feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "PTP Offload feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IEEE 1588 High Word Register Enable"]
    pub mod ADVTHWORD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "IEEE 1588 High Word Register option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "IEEE 1588 High Word Register option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Width."]
    pub mod ADDR64 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "32"]
            pub const M_32: u32 = 0;
            #[doc = "40"]
            pub const M_40: u32 = 1;
            #[doc = "48"]
            pub const M_48: u32 = 2;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DCB Feature Enable"]
    pub mod DCBEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "DCB Feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "DCB Feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Split Header Feature Enable"]
    pub mod SPHEN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Split Header Feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Split Header Feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TCP Segmentation Offload Enable"]
    pub mod TSOEN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "TCP Segmentation Offload Feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "TCP Segmentation Offload Feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Debug Registers Enable"]
    pub mod DBGMEMA {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "DMA Debug Registers option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "DMA Debug Registers option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AV Feature Enable"]
    pub mod AVSEL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "AV Feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "AV Feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx Side Only AV Feature Enable"]
    pub mod RAVSEL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Rx Side Only AV Feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Rx Side Only AV Feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "One Step for PTP over UDP/IP Feature Enable"]
    pub mod POUOST {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "One Step for PTP over UDP/IP Feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "One Step for PTP over UDP/IP Feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hash Table Size"]
    pub mod HASHTBLSZ {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "No hash table"]
            pub const NO_HT: u32 = 0;
            #[doc = "64"]
            pub const M_64: u32 = 1;
            #[doc = "128"]
            pub const M_128: u32 = 2;
            #[doc = "256"]
            pub const M_256: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Total number of L3 or L4 Filters"]
    pub mod L3L4FNUM {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "No L3 or L4 Filter"]
            pub const NOFILT: u32 = 0;
            #[doc = "1 L3 or L4 Filter"]
            pub const M_1FILT: u32 = 1;
            #[doc = "2 L3 or L4 Filters"]
            pub const M_2FILT: u32 = 2;
            #[doc = "3 L3 or L4 Filters"]
            pub const M_3FILT: u32 = 3;
            #[doc = "4 L3 or L4 Filters"]
            pub const M_4FILT: u32 = 4;
            #[doc = "5 L3 or L4 Filters"]
            pub const M_5FILT: u32 = 5;
            #[doc = "6 L3 or L4 Filters"]
            pub const M_6FILT: u32 = 6;
            #[doc = "7 L3 or L4 Filters"]
            pub const M_7FILT: u32 = 7;
            #[doc = "8 L3 or L4 Filters"]
            pub const M_8FILT: u32 = 8;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hardware Features 2"]
pub mod MAC_HW_FEATURE2 {
    #[doc = "Number of MTL Receive Queues"]
    pub mod RXQCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "1 MTL Rx Queue"]
            pub const M_1RXQ: u32 = 0;
            #[doc = "2 MTL Rx Queues"]
            pub const M_2RXQ: u32 = 1;
            #[doc = "3 MTL Rx Queues"]
            pub const M_3RXQ: u32 = 2;
            #[doc = "4 MTL Rx Queues"]
            pub const M_4RXQ: u32 = 3;
            #[doc = "5 MTL Rx Queues"]
            pub const M_5RXQ: u32 = 4;
            #[doc = "6 MTL Rx Queues"]
            pub const M_6RXQ: u32 = 5;
            #[doc = "7 MTL Rx Queues"]
            pub const M_7RXQ: u32 = 6;
            #[doc = "8 MTL Rx Queues"]
            pub const M_8RXQ: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of MTL Transmit Queues"]
    pub mod TXQCNT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "1 MTL Tx Queue"]
            pub const M_1TXQ: u32 = 0;
            #[doc = "2 MTL Tx Queues"]
            pub const M_2TXQ: u32 = 1;
            #[doc = "3 MTL Tx Queues"]
            pub const M_3TXQ: u32 = 2;
            #[doc = "4 MTL Tx Queues"]
            pub const M_4TXQ: u32 = 3;
            #[doc = "5 MTL Tx Queues"]
            pub const M_5TXQ: u32 = 4;
            #[doc = "6 MTL Tx Queues"]
            pub const M_6TXQ: u32 = 5;
            #[doc = "7 MTL Tx Queues"]
            pub const M_7TXQ: u32 = 6;
            #[doc = "8 MTL Tx Queues"]
            pub const M_8TXQ: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of DMA Receive Channels"]
    pub mod RXCHCNT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "1 MTL Rx Channel"]
            pub const M_1RXCH: u32 = 0;
            #[doc = "2 MTL Rx Channels"]
            pub const M_2RXCH: u32 = 1;
            #[doc = "3 MTL Rx Channels"]
            pub const M_3RXCH: u32 = 2;
            #[doc = "4 MTL Rx Channels"]
            pub const M_4RXCH: u32 = 3;
            #[doc = "5 MTL Rx Channels"]
            pub const M_5RXCH: u32 = 4;
            #[doc = "6 MTL Rx Channels"]
            pub const M_6RXCH: u32 = 5;
            #[doc = "7 MTL Rx Channels"]
            pub const M_7RXCH: u32 = 6;
            #[doc = "8 MTL Rx Channels"]
            pub const M_8RXCH: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rx DMA Descriptor Cache Size in terms of 16 bytes descriptors:"]
    pub mod RDCSZ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of DMA Transmit Channels"]
    pub mod TXCHCNT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "1 MTL Tx Channel"]
            pub const M_1TXCH: u32 = 0;
            #[doc = "2 MTL Tx Channels"]
            pub const M_2TXCH: u32 = 1;
            #[doc = "3 MTL Tx Channels"]
            pub const M_3TXCH: u32 = 2;
            #[doc = "4 MTL Tx Channels"]
            pub const M_4TXCH: u32 = 3;
            #[doc = "5 MTL Tx Channels"]
            pub const M_5TXCH: u32 = 4;
            #[doc = "6 MTL Tx Channels"]
            pub const M_6TXCH: u32 = 5;
            #[doc = "7 MTL Tx Channels"]
            pub const M_7TXCH: u32 = 6;
            #[doc = "8 MTL Tx Channels"]
            pub const M_8TXCH: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tx DMA Descriptor Cache Size in terms of 16 bytes descriptors:"]
    pub mod TDCSZ {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of PPS Outputs"]
    pub mod PPSOUTNUM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "No PPS output"]
            pub const NO_PPSO: u32 = 0;
            #[doc = "1 PPS output"]
            pub const M_1_PPSO: u32 = 1;
            #[doc = "2 PPS output"]
            pub const M_2_PPSO: u32 = 2;
            #[doc = "3 PPS output"]
            pub const M_3_PPSO: u32 = 3;
            #[doc = "4 PPS output"]
            pub const M_4_PPSO: u32 = 4;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Auxiliary Snapshot Inputs"]
    pub mod AUXSNAPNUM {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "No auxiliary input"]
            pub const NO_AUXI: u32 = 0;
            #[doc = "1 auxiliary input"]
            pub const M_1_AUXI: u32 = 1;
            #[doc = "2 auxiliary input"]
            pub const M_2_AUXI: u32 = 2;
            #[doc = "3 auxiliary input"]
            pub const M_3_AUXI: u32 = 3;
            #[doc = "4 auxiliary input"]
            pub const M_4_AUXI: u32 = 4;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hardware Features 3"]
pub mod MAC_HW_FEATURE3 {
    #[doc = "Number of Extended VLAN Tag Filters Enabled"]
    pub mod NRVF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "No Extended Rx VLAN Filters"]
            pub const NO_ERVLAN: u32 = 0;
            #[doc = "4 Extended Rx VLAN Filters"]
            pub const M_4_ERVLAN: u32 = 1;
            #[doc = "8 Extended Rx VLAN Filters"]
            pub const M_8_ERVLAN: u32 = 2;
            #[doc = "16 Extended Rx VLAN Filters"]
            pub const M_16_ERVLAN: u32 = 3;
            #[doc = "24 Extended Rx VLAN Filters"]
            pub const M_24_ERVLAN: u32 = 4;
            #[doc = "32 Extended Rx VLAN Filters"]
            pub const M_32_ERVLAN: u32 = 5;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Queue/Channel based VLAN tag insertion on Tx Enable"]
    pub mod CBTISEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Enable Queue/Channel based VLAN tag insertion on Tx feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Enable Queue/Channel based VLAN tag insertion on Tx feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Double VLAN Tag Processing Selected"]
    pub mod DVLAN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Double VLAN option is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Double VLAN option is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Broadcast/Multicast Packet Duplication"]
    pub mod PDUPSEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Broadcast/Multicast Packet Duplication feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Broadcast/Multicast Packet Duplication feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flexible Receive Parser Selected"]
    pub mod FRPSEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Flexible Receive Parser feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Flexible Receive Parser feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flexible Receive Parser Buffer size"]
    pub mod FRPBS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "64 Bytes"]
            pub const M_64BYTES: u32 = 0;
            #[doc = "128 Bytes"]
            pub const M_128BYTES: u32 = 1;
            #[doc = "256 Bytes"]
            pub const M_256BYTES: u32 = 2;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flexible Receive Parser Table Entries size"]
    pub mod FRPES {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "64 Entries"]
            pub const M_64ENTR: u32 = 0;
            #[doc = "128 Entries"]
            pub const M_128ENTR: u32 = 1;
            #[doc = "256 Entries"]
            pub const M_256ENTR: u32 = 2;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enhancements to Scheduled Traffic Enable"]
    pub mod ESTSEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Enable Enhancements to Scheduling Traffic feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Enable Enhancements to Scheduling Traffic feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Depth of the Gate Control List"]
    pub mod ESTDEP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "No Depth configured"]
            pub const NODEPTH: u32 = 0;
            #[doc = "64"]
            pub const DEPTH64: u32 = 1;
            #[doc = "128"]
            pub const DEPTH128: u32 = 2;
            #[doc = "256"]
            pub const DEPTH256: u32 = 3;
            #[doc = "512"]
            pub const DEPTH512: u32 = 4;
            #[doc = "1024"]
            pub const DEPTH1024: u32 = 5;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Width of the Time Interval field in the Gate Control List"]
    pub mod ESTWID {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Width not configured"]
            pub const NOWIDTH: u32 = 0;
            #[doc = "16"]
            pub const WIDTH16: u32 = 1;
            #[doc = "20"]
            pub const WIDTH20: u32 = 2;
            #[doc = "24"]
            pub const WIDTH24: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Preemption Enable"]
    pub mod FPESEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Frame Preemption Enable feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Frame Preemption Enable feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Based Scheduling Enable"]
    pub mod TBSSEL {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Time Based Scheduling Enable feature is not selected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Time Based Scheduling Enable feature is selected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Automotive Safety Package"]
    pub mod ASP {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "No Safety features selected"]
            pub const NONE: u32 = 0;
            #[doc = "Only \"ECC protection for external memory\" feature is selected"]
            pub const ECC_ONLY: u32 = 1;
            #[doc = "All the Automotive Safety features are selected without the \"Parity Port Enable for external interface\" feature"]
            pub const AS_NPPE: u32 = 2;
            #[doc = "All the Automotive Safety features are selected with the \"Parity Port Enable for external interface\" feature"]
            pub const AS_PPE: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MDIO Address"]
pub mod MAC_MDIO_ADDRESS {
    #[doc = "GMII Busy"]
    pub mod GB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GMII Busy is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "GMII Busy is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Clause 45 PHY Enable"]
    pub mod C45E {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Clause 45 PHY is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Clause 45 PHY is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "GMII Operation Command 0"]
    pub mod GOC_0 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GMII Operation Command 0 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "GMII Operation Command 0 is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "GMII Operation Command 1"]
    pub mod GOC_1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GMII Operation Command 1 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "GMII Operation Command 1 is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Skip Address Packet"]
    pub mod SKAP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Skip Address Packet is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Skip Address Packet is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "CR"]
    pub mod CR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NTC"]
    pub mod NTC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Register/Device Address"]
    pub mod RDA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Physical Layer Address"]
    pub mod PA {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Back to Back transactions"]
    pub mod BTB {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Back to Back transactions disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Back to Back transactions enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Preamble Suppression Enable"]
    pub mod PSE {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Preamble Suppression disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Preamble Suppression enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "MAC MDIO Data"]
pub mod MAC_MDIO_DATA {
    #[doc = "GMII Data"]
    pub mod GD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Register Address"]
    pub mod RA {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CSR Software Control"]
pub mod MAC_CSR_SW_CTRL {
    #[doc = "Register Clear on Write 1 Enable"]
    pub mod RCWE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register Clear on Write 1 is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Register Clear on Write 1 is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "MAC Address0 High"]
pub mod MAC_ADDRESS0_HIGH {
    #[doc = "MAC Address0[47:32]"]
    pub mod ADDRHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel Select"]
    pub mod DCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Enable"]
    pub mod AE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "INVALID : This bit must be always set to 1"]
            pub const DISABLE: u32 = 0;
            #[doc = "This bit is always set to 1"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MAC Address0 Low"]
pub mod MAC_ADDRESS0_LOW {
    #[doc = "MAC Address0[31:0]"]
    pub mod ADDRLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Indirect Access Control"]
pub mod INDIR_ACCESS_CTRL {
    #[doc = "Operation Busy."]
    pub mod OB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command type"]
    pub mod COM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write operation"]
            pub const WRITE: u32 = 0;
            #[doc = "Read operation"]
            pub const READ: u32 = 1;
        }
    }
    #[doc = "Auto increment"]
    pub mod AUTO {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Offset"]
    pub mod AOFF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Mode Select"]
    pub mod MSEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Indirect Access Data"]
pub mod INDIR_ACCESS_DATA {
    #[doc = "This field contains data to read/write for Indirect address access associated with MAC_Indir_Access_Ctrl"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Control"]
pub mod MAC_TIMESTAMP_CONTROL {
    #[doc = "Enable Timestamp"]
    pub mod TSENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Fine or Coarse Timestamp Update"]
    pub mod TSCFUPDT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Coarse method is used to update system timestamp"]
            pub const COARSE: u32 = 0;
            #[doc = "Fine method is used to update system timestamp"]
            pub const FINE: u32 = 1;
        }
    }
    #[doc = "Initialize Timestamp"]
    pub mod TSINIT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp is not initialized"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp is initialized"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Update Timestamp"]
    pub mod TSUPDT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp is not updated"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp is updated"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Timestamp Interrupt Trigger"]
    pub mod TSTRIG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Interrupt Trigger is not enabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp Interrupt Trigger is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Update Addend Register"]
    pub mod TSADDREG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Addend Register is not updated"]
            pub const DISABLE: u32 = 0;
            #[doc = "Addend Register is updated"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Timestamp for All Packets"]
    pub mod TSENALL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp for All Packets disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp for All Packets enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Timestamp Digital or Binary Rollover Control"]
    pub mod TSCTRLSSR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Digital or Binary Rollover Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp Digital or Binary Rollover Control is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable PTP Packet Processing for Version 2 Format"]
    pub mod TSVER2ENA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "PTP Packet Processing for Version 2 Format is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "PTP Packet Processing for Version 2 Format is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Processing of PTP over Ethernet Packets"]
    pub mod TSIPENA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing of PTP over Ethernet Packets is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Processing of PTP over Ethernet Packets is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Processing of PTP Packets Sent over IPv6-UDP"]
    pub mod TSIPV6ENA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing of PTP Packets Sent over IPv6-UDP is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Processing of PTP Packets Sent over IPv6-UDP is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Processing of PTP Packets Sent over IPv4-UDP"]
    pub mod TSIPV4ENA {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processing of PTP Packets Sent over IPv4-UDP is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Processing of PTP Packets Sent over IPv4-UDP is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Timestamp Snapshot for Event Messages"]
    pub mod TSEVNTENA {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timestamp Snapshot for Event Messages is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Timestamp Snapshot for Event Messages is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Enable Snapshot for Messages Relevant to Master"]
    pub mod TSMSTRENA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Snapshot for Messages Relevant to Master is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Snapshot for Messages Relevant to Master is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Select PTP packets for Taking Snapshots"]
    pub mod SNAPTYPSEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable MAC Address for PTP Packet Filtering"]
    pub mod TSENMACADDR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MAC Address for PTP Packet Filtering is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "MAC Address for PTP Packet Filtering is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "External System Time Input"]
    pub mod ESTI {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External System Time Input is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "External System Time Input is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Transmit Timestamp Status Mode"]
    pub mod TXTSSTSM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Timestamp Status Mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Timestamp Status Mode is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "AV 802."]
    pub mod AV8021ASMEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AV 802.1AS Mode is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "AV 802.1AS Mode is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Subsecond Increment"]
pub mod MAC_SUB_SECOND_INCREMENT {
    #[doc = "Sub-nanosecond Increment Value"]
    pub mod SNSINC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Time Seconds"]
pub mod MAC_SYSTEM_TIME_SECONDS {
    #[doc = "Timestamp Second"]
    pub mod TSS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Time Nanoseconds"]
pub mod MAC_SYSTEM_TIME_NANOSECONDS {
    #[doc = "Timestamp Sub Seconds"]
    pub mod TSSS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Time Seconds Update"]
pub mod MAC_SYSTEM_TIME_SECONDS_UPDATE {
    #[doc = "Timestamp Seconds"]
    pub mod TSS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "System Time Nanoseconds Update"]
pub mod MAC_SYSTEM_TIME_NANOSECONDS_UPDATE {
    #[doc = "Timestamp Sub Seconds"]
    pub mod TSSS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Add or Subtract Time"]
    pub mod ADDSUB {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Add time"]
            pub const ADD: u32 = 0;
            #[doc = "Subtract time"]
            pub const SUB: u32 = 1;
        }
    }
}
#[doc = "Timestamp Addend"]
pub mod MAC_TIMESTAMP_ADDEND {
    #[doc = "Timestamp Addend Register"]
    pub mod TSAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Status"]
pub mod MAC_TIMESTAMP_STATUS {
    #[doc = "Timestamp Seconds Overflow"]
    pub mod TSSOVF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Timestamp Seconds Overflow status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Seconds Overflow status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timestamp Target Time Reached"]
    pub mod TSTARGT0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Timestamp Target Time Reached status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Target Time Reached status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timestamp Target Time Error"]
    pub mod TSTRGTERR0 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Timestamp Target Time Error status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Timestamp Target Time Error status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Tx Timestamp Status Interrupt Status"]
    pub mod TXTSSIS {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Tx Timestamp Status Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Tx Timestamp Status Interrupt status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Timestamp Status Nanoseconds"]
pub mod MAC_TX_TIMESTAMP_STATUS_NANOSECONDS {
    #[doc = "Transmit Timestamp Status Low"]
    pub mod TXTSSLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXTSSMIS"]
    pub mod TXTSSMIS {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit Timestamp Status Missed status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Transmit Timestamp Status Missed status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Timestamp Status Seconds"]
pub mod MAC_TX_TIMESTAMP_STATUS_SECONDS {
    #[doc = "Transmit Timestamp Status High"]
    pub mod TXTSSHI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Ingress Correction Nanosecond"]
pub mod MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND {
    #[doc = "Timestamp Ingress Correction"]
    pub mod TSIC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Egress Correction Nanosecond"]
pub mod MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND {
    #[doc = "Timestamp Egress Correction"]
    pub mod TSEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Ingress Latency"]
pub mod MAC_TIMESTAMP_INGRESS_LATENCY {
    #[doc = "ITLSNS"]
    pub mod ITLSNS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ITLNS"]
    pub mod ITLNS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timestamp Egress Latency"]
pub mod MAC_TIMESTAMP_EGRESS_LATENCY {
    #[doc = "Egress Timestamp Latency, in sub-nanoseconds"]
    pub mod ETLSNS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Timestamp Latency, in nanoseconds"]
    pub mod ETLNS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS Control"]
pub mod MAC_PPS_CONTROL {
    #[doc = "PPS Output Frequency Control"]
    pub mod PPSCTRL_PPSCMD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS0 Target Time Seconds"]
pub mod PPS0_TARGET_TIME_SECONDS {
    #[doc = "PPS Target Time Seconds Register"]
    pub mod TSTRH0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "PPS0 Target Time Nanoseconds"]
pub mod PPS0_TARGET_TIME_NANOSECONDS {
    #[doc = "Target Time Low for PPS Register"]
    pub mod TTSL0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MTL Operation Mode"]
pub mod MTL_OPERATION_MODE {
    #[doc = "Drop Transmit Status"]
    pub mod DTXSTS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Drop Transmit Status is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Drop Transmit Status is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Receive Arbitration Algorithm"]
    pub mod RAA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Strict priority (SP)"]
            pub const SP: u32 = 0;
            #[doc = "Weighted Strict Priority (WSP)"]
            pub const WSP: u32 = 1;
        }
    }
    #[doc = "Tx Scheduling Algorithm"]
    pub mod SCHALG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WRR algorithm"]
            pub const WRR: u32 = 0;
            #[doc = "WFQ algorithm when DCB feature is selected.Otherwise, Reserved"]
            pub const WFQ: u32 = 1;
            #[doc = "DWRR algorithm when DCB feature is selected.Otherwise, Reserved"]
            pub const DWRR: u32 = 2;
            #[doc = "Strict priority algorithm"]
            pub const SP: u32 = 3;
        }
    }
    #[doc = "Counters Preset"]
    pub mod CNTPRST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counters Preset is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Counters Preset is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Counters Reset"]
    pub mod CNTCLR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counters are not reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "All counters are reset"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "MTL Interrupt Status"]
pub mod MTL_INTERRUPT_STATUS {
    #[doc = "Queue 0 Interrupt status"]
    pub mod Q0IS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Queue 0 Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Queue 0 Interrupt status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Queue 1 Interrupt status"]
    pub mod Q1IS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Queue 1 Interrupt status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Queue 1 Interrupt status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Queue and DMA Channel Mapping 0"]
pub mod MTL_RXQ_DMA_MAP0 {
    #[doc = "Queue 0 Mapped to DMA Channel"]
    pub mod Q0MDMACH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Queue 0 Enabled for DA-based DMA Channel Selection"]
    pub mod Q0DDMACH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue 0 disabled for DA-based DMA Channel Selection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue 0 enabled for DA-based DMA Channel Selection"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Queue 1 Mapped to DMA Channel"]
    pub mod Q1MDMACH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Queue 1 Enabled for DA-based DMA Channel Selection"]
    pub mod Q1DDMACH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Queue 1 disabled for DA-based DMA Channel Selection"]
            pub const DISABLE: u32 = 0;
            #[doc = "Queue 1 enabled for DA-based DMA Channel Selection"]
            pub const ENABLE: u32 = 1;
        }
    }
}
pub mod queue {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "Cluster QUEUE%s, containing MTL_TXQ?_OPERATION_MODE, MTL_TXQ?_UNDERFLOW, MTL_TXQ?_DEBUG, MTL_TXQ?_ETS_STATUS, MTL_TXQ?_QUANTUM_WEIGHT, MTL_Q?_INTERRUPT_CONTROL_STATUS, MTL_RXQ?_OPERATION_MODE, MTL_RXQ?_MISSED_PACKET_OVERFLOW_CNT, MTL_RXQ?_DEBUG, MTL_RXQ?_CONTROL"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Queue 0 Transmit Operation Mode"]
        pub MTL_TX_OPERATION_MODE: crate::RWRegister<u32>,
        #[doc = "Queue 0 Underflow Counter"]
        pub MTL_TX_UNDERFLOW: crate::RWRegister<u32>,
        #[doc = "Queue 0 Transmit Debug"]
        pub MTL_TX_DEBUG: crate::RWRegister<u32>,
        _reserved0: [u8; 0x8],
        #[doc = "Queue 0 ETS Status"]
        pub MTL_TX_ETS_STATUS: crate::RWRegister<u32>,
        #[doc = "Queue 0 Quantum or Weights"]
        pub MTL_TX_QUANTUM_WEIGHT: crate::RWRegister<u32>,
        _reserved1: [u8; 0x10],
        #[doc = "Queue 0 Interrupt Control Status"]
        pub MTL_TX_INTERRUPT_CONTROL_STATUS: crate::RWRegister<u32>,
        #[doc = "Queue 0 Receive Operation Mode"]
        pub MTL_RX_OPERATION_MODE: crate::RWRegister<u32>,
        #[doc = "Queue 0 Missed Packet and Overflow Counter"]
        pub MTL_RX_MISSED_PACKET_OVERFLOW_CNT: crate::RWRegister<u32>,
        #[doc = "Queue 0 Receive Debug"]
        pub MTL_RX_DEBUG: crate::RWRegister<u32>,
        #[doc = "Queue 0 Receive Control"]
        pub MTL_RX_CONTROL: crate::RWRegister<u32>,
    }
    #[doc = "Queue 0 Transmit Operation Mode"]
    pub mod MTL_TX_OPERATION_MODE {
        #[doc = "Flush Transmit Queue"]
        pub mod FTQ {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Flush Transmit Queue is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Flush Transmit Queue is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Transmit Store and Forward"]
        pub mod TSF {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Transmit Store and Forward is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Transmit Store and Forward is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Transmit Queue Enable"]
        pub mod TXQEN {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Not enabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Enable in AV mode (Reserved in non-AV)"]
                pub const EN_IF_AV: u32 = 1;
                #[doc = "Enabled"]
                pub const ENABLE: u32 = 2;
            }
        }
        #[doc = "Transmit Threshold Control"]
        pub mod TTC {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "32"]
                pub const M_32BYTES: u32 = 0;
                #[doc = "64"]
                pub const M_64BYTES: u32 = 1;
                #[doc = "96"]
                pub const M_96BYTES: u32 = 2;
                #[doc = "128"]
                pub const M_128BYTES: u32 = 3;
                #[doc = "192"]
                pub const M_192BYTES: u32 = 4;
                #[doc = "256"]
                pub const M_256BYTES: u32 = 5;
                #[doc = "384"]
                pub const M_384BYTES: u32 = 6;
                #[doc = "512"]
                pub const M_512BYTES: u32 = 7;
            }
        }
        #[doc = "Transmit Queue Size"]
        pub mod TQS {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Queue 0 Underflow Counter"]
    pub mod MTL_TX_UNDERFLOW {
        #[doc = "Underflow Packet Counter"]
        pub mod UFFRMCNT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Overflow Bit for Underflow Packet Counter"]
        pub mod UFCNTOVF {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Overflow not detected for Underflow Packet Counter"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Overflow detected for Underflow Packet Counter"]
                pub const ACTIVE: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Queue 0 Transmit Debug"]
    pub mod MTL_TX_DEBUG {
        #[doc = "Transmit Queue in Pause"]
        pub mod TXQPAUSED {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Transmit Queue in Pause status is not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Transmit Queue in Pause status is detected"]
                pub const ACTIVE: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "MTL Tx Queue Read Controller Status"]
        pub mod TRCSTS {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {
                #[doc = "Idle state"]
                pub const IDLE: u32 = 0;
                #[doc = "Read state (transferring data to the MAC transmitter)"]
                pub const READ: u32 = 1;
                #[doc = "Waiting for pending Tx Status from the MAC transmitter"]
                pub const WAIT: u32 = 2;
                #[doc = "Flushing the Tx queue because of the Packet Abort request from the MAC"]
                pub const FLUSH: u32 = 3;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "MTL Tx Queue Write Controller Status"]
        pub mod TWCSTS {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "MTL Tx Queue Write Controller status is not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "MTL Tx Queue Write Controller status is detected"]
                pub const ACTIVE: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "MTL Tx Queue Not Empty Status"]
        pub mod TXQSTS {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "MTL Tx Queue Not Empty status is not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "MTL Tx Queue Not Empty status is detected"]
                pub const ACTIVE: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "MTL Tx Status FIFO Full Status"]
        pub mod TXSTSFSTS {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "MTL Tx Status FIFO Full status is not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "MTL Tx Status FIFO Full status is detected"]
                pub const ACTIVE: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Number of Packets in the Transmit Queue"]
        pub mod PTXQ {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Number of Status Words in Tx Status FIFO of Queue"]
        pub mod STXSTSF {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Queue 0 ETS Status"]
    pub mod MTL_TX_ETS_STATUS {
        #[doc = "Average Bits per Slot"]
        pub mod ABS {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Queue 0 Quantum or Weights"]
    pub mod MTL_TX_QUANTUM_WEIGHT {
        #[doc = "Quantum or Weights"]
        pub mod ISCQW {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Queue 0 Interrupt Control Status"]
    pub mod MTL_TX_INTERRUPT_CONTROL_STATUS {
        #[doc = "Transmit Queue Underflow Interrupt Status"]
        pub mod TXUNFIS {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Transmit Queue Underflow Interrupt Status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Transmit Queue Underflow Interrupt Status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Average Bits Per Slot Interrupt Status"]
        pub mod ABPSIS {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Average Bits Per Slot Interrupt Status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Average Bits Per Slot Interrupt Status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Transmit Queue Underflow Interrupt Enable"]
        pub mod TXUIE {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Transmit Queue Underflow Interrupt Status is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Transmit Queue Underflow Interrupt Status is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Average Bits Per Slot Interrupt Enable"]
        pub mod ABPSIE {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Average Bits Per Slot Interrupt is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Average Bits Per Slot Interrupt is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Receive Queue Overflow Interrupt Status"]
        pub mod RXOVFIS {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Receive Queue Overflow Interrupt Status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Receive Queue Overflow Interrupt Status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Receive Queue Overflow Interrupt Enable"]
        pub mod RXOIE {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Receive Queue Overflow Interrupt is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Receive Queue Overflow Interrupt is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
    }
    #[doc = "Queue 0 Receive Operation Mode"]
    pub mod MTL_RX_OPERATION_MODE {
        #[doc = "Receive Queue Threshold Control"]
        pub mod RTC {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "64"]
                pub const M_64BYTE: u32 = 0;
                #[doc = "32"]
                pub const M_32BYTE: u32 = 1;
                #[doc = "96"]
                pub const M_96BYTE: u32 = 2;
                #[doc = "128"]
                pub const M_128BYTE: u32 = 3;
            }
        }
        #[doc = "Forward Undersized Good Packets"]
        pub mod FUP {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Forward Undersized Good Packets is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Forward Undersized Good Packets is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Forward Error Packets"]
        pub mod FEP {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Forward Error Packets is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Forward Error Packets is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Receive Queue Store and Forward"]
        pub mod RSF {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Receive Queue Store and Forward is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Receive Queue Store and Forward is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Disable Dropping of TCP/IP Checksum Error Packets"]
        pub mod DIS_TCP_EF {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Dropping of TCP/IP Checksum Error Packets is enabled"]
                pub const ENABLE: u32 = 0;
                #[doc = "Dropping of TCP/IP Checksum Error Packets is disabled"]
                pub const DISABLE: u32 = 1;
            }
        }
        #[doc = "Receive Queue Size"]
        pub mod RQS {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Queue 0 Missed Packet and Overflow Counter"]
    pub mod MTL_RX_MISSED_PACKET_OVERFLOW_CNT {
        #[doc = "Overflow Packet Counter"]
        pub mod OVFPKTCNT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Overflow Counter Overflow Bit"]
        pub mod OVFCNTOVF {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Overflow Counter overflow not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Overflow Counter overflow detected"]
                pub const ACTIVE: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Missed Packet Counter"]
        pub mod MISPKTCNT {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b11111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Missed Packet Counter Overflow Bit"]
        pub mod MISCNTOVF {
            pub const offset: u32 = 27;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Missed Packet Counter overflow not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Missed Packet Counter overflow detected"]
                pub const ACTIVE: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Queue 0 Receive Debug"]
    pub mod MTL_RX_DEBUG {
        #[doc = "MTL Rx Queue Write Controller Active Status"]
        pub mod RWCSTS {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "MTL Rx Queue Write Controller Active Status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "MTL Rx Queue Write Controller Active Status detected"]
                pub const ACTIVE: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "MTL Rx Queue Read Controller State"]
        pub mod RRCSTS {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {
                #[doc = "Idle state"]
                pub const IDLE: u32 = 0;
                #[doc = "Reading packet data"]
                pub const READ_DATA: u32 = 1;
                #[doc = "Reading packet status (or timestamp)"]
                pub const READ_STS: u32 = 2;
                #[doc = "Flushing the packet data and status"]
                pub const FLUSH: u32 = 3;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "MTL Rx Queue Fill-Level Status"]
        pub mod RXQSTS {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {
                #[doc = "Rx Queue empty"]
                pub const EMPTY: u32 = 0;
                #[doc = "Rx Queue fill-level below flow-control deactivate threshold"]
                pub const BLW_THR: u32 = 1;
                #[doc = "Rx Queue fill-level above flow-control activate threshold"]
                pub const ABV_THR: u32 = 2;
                #[doc = "Rx Queue full"]
                pub const FULL: u32 = 3;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Number of Packets in Receive Queue"]
        pub mod PRXQ {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b11111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Queue 0 Receive Control"]
    pub mod MTL_RX_CONTROL {
        #[doc = "Receive Queue Weight"]
        pub mod RXQ_WEGT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Receive Queue Packet Arbitration"]
        pub mod RXQ_FRM_ARBIT {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Receive Queue Packet Arbitration is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Receive Queue Packet Arbitration is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
    }
}
#[doc = "Queue 1 ETS Control"]
pub mod MTL_TXQ1_ETS_CONTROL {
    #[doc = "AV Algorithm"]
    pub mod AVALG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CBS Algorithm is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "CBS Algorithm is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Credit Control"]
    pub mod CC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Credit Control is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Credit Control is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Slot Count"]
    pub mod SLC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 slot"]
            pub const M_1_SLOT: u32 = 0;
            #[doc = "2 slots"]
            pub const M_2_SLOT: u32 = 1;
            #[doc = "4 slots"]
            pub const M_4_SLOT: u32 = 2;
            #[doc = "8 slots"]
            pub const M_8_SLOT: u32 = 3;
            #[doc = "16 slots"]
            pub const M_16_SLOT: u32 = 4;
        }
    }
}
#[doc = "Queue 1 sendSlopeCredit"]
pub mod MTL_TXQ1_SENDSLOPECREDIT {
    #[doc = "sendSlopeCredit Value"]
    pub mod SSC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 1 hiCredit"]
pub mod MTL_TXQ1_HICREDIT {
    #[doc = "hiCredit Value"]
    pub mod HC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Queue 1 loCredit"]
pub mod MTL_TXQ1_LOCREDIT {
    #[doc = "loCredit Value"]
    pub mod LC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Bus Mode"]
pub mod DMA_MODE {
    #[doc = "Software Reset"]
    pub mod SWR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Software Reset is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Software Reset is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "DMA Tx or Rx Arbitration Scheme"]
    pub mod DA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Weighted Round-Robin with Rx:Tx or Tx:Rx"]
            pub const WRR: u32 = 0;
            #[doc = "Fixed Priority"]
            pub const FP: u32 = 1;
        }
    }
    #[doc = "Transmit Arbitration Algorithm"]
    pub mod TAA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed priority"]
            pub const FP: u32 = 0;
            #[doc = "Weighted Strict Priority (WSP)"]
            pub const WSP: u32 = 1;
            #[doc = "Weighted Round-Robin (WRR)"]
            pub const WRR: u32 = 2;
        }
    }
    #[doc = "Transmit Priority"]
    pub mod TXPR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit Priority is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Transmit Priority is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Priority Ratio"]
    pub mod PR {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The priority ratio is 1:1"]
            pub const R_1_1: u32 = 0;
            #[doc = "The priority ratio is 2:1"]
            pub const R_2_1: u32 = 1;
            #[doc = "The priority ratio is 3:1"]
            pub const R_3_1: u32 = 2;
            #[doc = "The priority ratio is 4:1"]
            pub const R_4_1: u32 = 3;
            #[doc = "The priority ratio is 5:1"]
            pub const R_5_1: u32 = 4;
            #[doc = "The priority ratio is 6:1"]
            pub const R_6_1: u32 = 5;
            #[doc = "The priority ratio is 7:1"]
            pub const R_7_1: u32 = 6;
            #[doc = "The priority ratio is 8:1"]
            pub const R_8_1: u32 = 7;
        }
    }
}
#[doc = "DMA System Bus Mode"]
pub mod DMA_SYSBUS_MODE {
    #[doc = "Fixed Burst Length"]
    pub mod FB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fixed Burst Length is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Fixed Burst Length is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Address-Aligned Beats"]
    pub mod AAL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address-Aligned Beats is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Address-Aligned Beats is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Mixed Burst"]
    pub mod MB {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mixed Burst is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Mixed Burst is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Rebuild INCRx Burst"]
    pub mod RB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rebuild INCRx Burst is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Rebuild INCRx Burst is enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "DMA Interrupt Status"]
pub mod DMA_INTERRUPT_STATUS {
    #[doc = "DMA Channel 0 Interrupt Status"]
    pub mod DC0IS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "DMA Channel 0 Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "DMA Channel 0 Interrupt Status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel 1 Interrupt Status"]
    pub mod DC1IS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "DMA Channel 1 Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "DMA Channel 1 Interrupt Status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MTL Interrupt Status"]
    pub mod MTLIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "MTL Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MTL Interrupt Status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MAC Interrupt Status"]
    pub mod MACIS {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "MAC Interrupt Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "MAC Interrupt Status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DMA Debug Status 0"]
pub mod DMA_DEBUG_STATUS0 {
    #[doc = "AHB Master Status"]
    pub mod AXWHSTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "AXI Master Write Channel or AHB Master Status not detected"]
            pub const INACTIVE: u32 = 0;
            #[doc = "AXI Master Write Channel or AHB Master Status detected"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel 0 Receive Process State"]
    pub mod RPS0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "Stopped (Reset or Stop Receive Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Rx Transfer Descriptor)"]
            pub const RUN_FRTD: u32 = 1;
            #[doc = "Running (Waiting for Rx packet)"]
            pub const RUN_WRP: u32 = 3;
            #[doc = "Suspended (Rx Descriptor Unavailable)"]
            pub const SUSPND: u32 = 4;
            #[doc = "Running (Closing the Rx Descriptor)"]
            pub const RUN_CRD: u32 = 5;
            #[doc = "Timestamp write state"]
            pub const TSTMP: u32 = 6;
            #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)"]
            pub const RUN_TRP: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel 0 Transmit Process State"]
    pub mod TPS0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "Stopped (Reset or Stop Transmit Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Tx Transfer Descriptor)"]
            pub const RUN_FTTD: u32 = 1;
            #[doc = "Running (Waiting for status)"]
            pub const RUN_WS: u32 = 2;
            #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))"]
            pub const RUN_RDS: u32 = 3;
            #[doc = "Timestamp write state"]
            pub const TSTMP_WS: u32 = 4;
            #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)"]
            pub const SUSPND: u32 = 6;
            #[doc = "Running (Closing Tx Descriptor)"]
            pub const RUN_CTD: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel 1 Receive Process State"]
    pub mod RPS1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "Stopped (Reset or Stop Receive Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Rx Transfer Descriptor)"]
            pub const RUN_FRTD: u32 = 1;
            #[doc = "Running (Waiting for Rx packet)"]
            pub const RUN_WRP: u32 = 3;
            #[doc = "Suspended (Rx Descriptor Unavailable)"]
            pub const SUSPND: u32 = 4;
            #[doc = "Running (Closing the Rx Descriptor)"]
            pub const RUN_CRD: u32 = 5;
            #[doc = "Timestamp write state"]
            pub const TSTMP: u32 = 6;
            #[doc = "Running (Transferring the received packet data from the Rx buffer to the system memory)"]
            pub const RUN_TRP: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA Channel 1 Transmit Process State"]
    pub mod TPS1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "Stopped (Reset or Stop Transmit Command issued)"]
            pub const STOP: u32 = 0;
            #[doc = "Running (Fetching Tx Transfer Descriptor)"]
            pub const RUN_FTTD: u32 = 1;
            #[doc = "Running (Waiting for status)"]
            pub const RUN_WS: u32 = 2;
            #[doc = "Running (Reading Data from system memory buffer and queuing it to the Tx buffer (Tx FIFO))"]
            pub const RUN_RDS: u32 = 3;
            #[doc = "Timestamp write state"]
            pub const TSTMP_WS: u32 = 4;
            #[doc = "Suspended (Tx Descriptor Unavailable or Tx Buffer Underflow)"]
            pub const SUSPND: u32 = 6;
            #[doc = "Running (Closing Tx Descriptor)"]
            pub const RUN_CTD: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
}
pub mod dma_ch {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "Cluster DMA_CH%s, containing DMA_CH?_CONTROL, DMA_CH?_TX_CONTROL, DMA_CH?_RX_CONTROL, DMA_CH?_TXDESC_LIST_ADDRESS, DMA_CH?_RXDESC_LIST_ADDRESS, DMA_CH?_TXDESC_TAIL_POINTER, DMA_CH?_RXDESC_TAIL_POINTER, DMA_CH?_TXDESC_RING_LENGTH, DMA_CH?_RX_CONTROL2, DMA_CH?_INTERRUPT_ENABLE, DMA_CH?_RX_INTERRUPT_WATCHDOG_TIMER, DMA_CH?_SLOT_FUNCTION_CONTROL_STATUS, DMA_CH?_CURRENT_APP_TXDESC, DMA_CH?_CURRENT_APP_RXDESC, DMA_CH?_CURRENT_APP_TXBUFFER, DMA_CH?_CURRENT_APP_RXBUFFER, DMA_CH?_STATUS, DMA_CH?_MISS_FRAME_CNT, DMA_CH?_RX_ERI_CNT"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "DMA Channel 0 Control"]
        pub CONTROL: crate::RWRegister<u32>,
        #[doc = "DMA Channel 0 Transmit Control"]
        pub TX_CONTROL: crate::RWRegister<u32>,
        #[doc = "DMA Channel 0 Receive Control"]
        pub RX_CONTROL: crate::RWRegister<u32>,
        _reserved0: [u8; 0x8],
        #[doc = "Channel 0 Tx Descriptor List Address register"]
        pub TXDESC_LIST_ADDRESS: crate::RWRegister<u32>,
        _reserved1: [u8; 0x4],
        #[doc = "Channel 0 Rx Descriptor List Address register"]
        pub RXDESC_LIST_ADDRESS: crate::RWRegister<u32>,
        #[doc = "Channel 0 Tx Descriptor Tail Pointer"]
        pub TXDESC_TAIL_POINTER: crate::RWRegister<u32>,
        _reserved2: [u8; 0x4],
        #[doc = "Channel 0 Rx Descriptor Tail Pointer"]
        pub RXDESC_TAIL_POINTER: crate::RWRegister<u32>,
        #[doc = "Channel 0 Tx Descriptor Ring Length"]
        pub TXDESC_RING_LENGTH: crate::RWRegister<u32>,
        #[doc = "Channeli Receive Control"]
        pub TX_CONTROL2: crate::RWRegister<u32>,
        #[doc = "The Channeli Interrupt Enable register enables the interrupts reported by the Status register."]
        pub INTERRUPT_ENABLE: crate::RWRegister<u32>,
        #[doc = "Channel 0 Receive Interrupt Watchdog Timer"]
        pub RX_INTERRUPT_WATCHDOG_TIMER: crate::RWRegister<u32>,
        #[doc = "Channel 0 Slot Function Control and Status"]
        pub SLOT_FUNCTION_CONTROL_STATUS: crate::RWRegister<u32>,
        _reserved3: [u8; 0x4],
        #[doc = "Channel 0 Current Application Transmit Descriptor"]
        pub CURRENT_APP_TXDESC: crate::RWRegister<u32>,
        _reserved4: [u8; 0x4],
        #[doc = "Channel 0 Current Application Receive Descriptor"]
        pub CURRENT_APP_RXDESC: crate::RWRegister<u32>,
        _reserved5: [u8; 0x4],
        #[doc = "Channel 0 Current Application Transmit Buffer Address"]
        pub CURRENT_APP_TXBUFFER: crate::RWRegister<u32>,
        _reserved6: [u8; 0x4],
        #[doc = "Channel 0 Current Application Receive Buffer Address"]
        pub CURRENT_APP_RXBUFFER: crate::RWRegister<u32>,
        #[doc = "DMA Channel 0 Status"]
        pub STATUS: crate::RWRegister<u32>,
        #[doc = "Channel 0 Missed Frame Counter"]
        pub MISS_FRAME_CNT: crate::RWRegister<u32>,
        _reserved7: [u8; 0x4],
        #[doc = "Channel 0 Receive ERI Counter"]
        pub RX_ERI_CNT: crate::RWRegister<u32>,
    }
    #[doc = "DMA Channel 0 Control"]
    pub mod CONTROL {
        #[doc = "8xPBL mode"]
        pub mod PBLx8 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "8xPBL mode is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "8xPBL mode is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Descriptor Skip Length"]
        pub mod DSL {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "DMA Channel 0 Transmit Control"]
    pub mod TX_CONTROL {
        #[doc = "Start or Stop Transmission Command"]
        pub mod ST {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Stop Transmission Command"]
                pub const STOP: u32 = 0;
                #[doc = "Start Transmission Command"]
                pub const START: u32 = 1;
            }
        }
        #[doc = "Transmit Channel Weight"]
        pub mod TCW {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Operate on Second Packet"]
        pub mod OSF {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Operate on Second Packet disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Operate on Second Packet enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Transmit Programmable Burst Length"]
        pub mod TxPBL {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Early Transmit Interrupt Control"]
        pub mod ETIC {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Early Transmit Interrupt is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Early Transmit Interrupt is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
    }
    #[doc = "DMA Channel 0 Receive Control"]
    pub mod RX_CONTROL {
        #[doc = "Start or Stop Receive"]
        pub mod SR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Stop Receive"]
                pub const STOP: u32 = 0;
                #[doc = "Start Receive"]
                pub const START: u32 = 1;
            }
        }
        #[doc = "Receive Buffer size Low"]
        pub mod RBSZ_X_0 {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Receive Buffer size High"]
        pub mod RBSZ_13_Y {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Receive Programmable Burst Length"]
        pub mod RxPBL {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Early Receive Interrupt Control"]
        pub mod ERIC {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Early Receive Interrupt is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Early Receive Interrupt is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Rx Packet Flush."]
        pub mod RPF {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Rx Packet Flush is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Rx Packet Flush is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
    }
    #[doc = "Channel 0 Tx Descriptor List Address register"]
    pub mod TXDESC_LIST_ADDRESS {
        #[doc = "Start of Transmit List"]
        pub mod TDESLA {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel 0 Rx Descriptor List Address register"]
    pub mod RXDESC_LIST_ADDRESS {
        #[doc = "Start of Receive List"]
        pub mod RDESLA {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel 0 Tx Descriptor Tail Pointer"]
    pub mod TXDESC_TAIL_POINTER {
        #[doc = "Transmit Descriptor Tail Pointer"]
        pub mod TDTP {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel 0 Rx Descriptor Tail Pointer"]
    pub mod RXDESC_TAIL_POINTER {
        #[doc = "Receive Descriptor Tail Pointer"]
        pub mod RDTP {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel 0 Tx Descriptor Ring Length"]
    pub mod TXDESC_RING_LENGTH {
        #[doc = "Transmit Descriptor Ring Length"]
        pub mod TDRL {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channeli Receive Control"]
    pub mod TX_CONTROL2 {
        #[doc = "Receive Descriptor Ring Length"]
        pub mod RDRL {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Alternate Receive Buffer Size"]
        pub mod ARBS {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "The Channeli Interrupt Enable register enables the interrupts reported by the Status register."]
    pub mod INTERRUPT_ENABLE {
        #[doc = "Transmit Interrupt Enable"]
        pub mod TIE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Transmit Interrupt is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Transmit Interrupt is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Transmit Stopped Enable"]
        pub mod TXSE {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Transmit Stopped is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Transmit Stopped is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Transmit Buffer Unavailable Enable"]
        pub mod TBUE {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Transmit Buffer Unavailable is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Transmit Buffer Unavailable is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Receive Interrupt Enable"]
        pub mod RIE {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Receive Interrupt is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Receive Interrupt is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Receive Buffer Unavailable Enable"]
        pub mod RBUE {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Receive Buffer Unavailable is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Receive Buffer Unavailable is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Receive Stopped Enable"]
        pub mod RSE {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Receive Stopped is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Receive Stopped is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Receive Watchdog Timeout Enable"]
        pub mod RWTE {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Receive Watchdog Timeout is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Receive Watchdog Timeout is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Early Transmit Interrupt Enable"]
        pub mod ETIE {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Early Transmit Interrupt is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Early Transmit Interrupt is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Early Receive Interrupt Enable"]
        pub mod ERIE {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Early Receive Interrupt is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Early Receive Interrupt is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Fatal Bus Error Enable"]
        pub mod FBEE {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Fatal Bus Error is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Fatal Bus Error is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Context Descriptor Error Enable"]
        pub mod CDEE {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Context Descriptor Error is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Context Descriptor Error is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Abnormal Interrupt Summary Enable"]
        pub mod AIE {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Abnormal Interrupt Summary is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Abnormal Interrupt Summary is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Normal Interrupt Summary Enable"]
        pub mod NIE {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Normal Interrupt Summary is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Normal Interrupt Summary is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
    }
    #[doc = "Channel 0 Receive Interrupt Watchdog Timer"]
    pub mod RX_INTERRUPT_WATCHDOG_TIMER {
        #[doc = "Receive Interrupt Watchdog Timer Count"]
        pub mod RWT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Receive Interrupt Watchdog Timer Count Units"]
        pub mod RWTU {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel 0 Slot Function Control and Status"]
    pub mod SLOT_FUNCTION_CONTROL_STATUS {
        #[doc = "Enable Slot Comparison"]
        pub mod ESC {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Slot Comparison is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Slot Comparison is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Advance Slot Check"]
        pub mod ASC {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Advance Slot Check is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Advance Slot Check is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Slot Interval Value"]
        pub mod SIV {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Reference Slot Number"]
        pub mod RSN {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel 0 Current Application Transmit Descriptor"]
    pub mod CURRENT_APP_TXDESC {
        #[doc = "Application Transmit Descriptor Address Pointer"]
        pub mod CURTDESAPTR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel 0 Current Application Receive Descriptor"]
    pub mod CURRENT_APP_RXDESC {
        #[doc = "Application Receive Descriptor Address Pointer"]
        pub mod CURRDESAPTR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel 0 Current Application Transmit Buffer Address"]
    pub mod CURRENT_APP_TXBUFFER {
        #[doc = "Application Transmit Buffer Address Pointer"]
        pub mod CURTBUFAPTR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel 0 Current Application Receive Buffer Address"]
    pub mod CURRENT_APP_RXBUFFER {
        #[doc = "Application Receive Buffer Address Pointer"]
        pub mod CURRBUFAPTR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "DMA Channel 0 Status"]
    pub mod STATUS {
        #[doc = "Transmit Interrupt"]
        pub mod TI {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Transmit Interrupt status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Transmit Interrupt status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Transmit Process Stopped"]
        pub mod TPS {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Transmit Process Stopped status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Transmit Process Stopped status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Transmit Buffer Unavailable"]
        pub mod TBU {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Transmit Buffer Unavailable status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Transmit Buffer Unavailable status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Receive Interrupt"]
        pub mod RI {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Receive Interrupt status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Receive Interrupt status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Receive Buffer Unavailable"]
        pub mod RBU {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Receive Buffer Unavailable status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Receive Buffer Unavailable status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Receive Process Stopped"]
        pub mod RPS {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Receive Process Stopped status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Receive Process Stopped status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Receive Watchdog Timeout"]
        pub mod RWT {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Receive Watchdog Timeout status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Receive Watchdog Timeout status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Early Transmit Interrupt"]
        pub mod ETI {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Early Transmit Interrupt status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Early Transmit Interrupt status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Early Receive Interrupt"]
        pub mod ERI {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Early Receive Interrupt status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Early Receive Interrupt status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Fatal Bus Error"]
        pub mod FBE {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Fatal Bus Error status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Fatal Bus Error status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Context Descriptor Error"]
        pub mod CDE {
            pub const offset: u32 = 13;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Context Descriptor Error status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Context Descriptor Error status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Abnormal Interrupt Summary"]
        pub mod AIS {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Abnormal Interrupt Summary status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Abnormal Interrupt Summary status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Normal Interrupt Summary"]
        pub mod NIS {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Normal Interrupt Summary status not detected"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Normal Interrupt Summary status detected"]
                pub const ACTIVE: u32 = 1;
            }
        }
        #[doc = "Tx DMA Error Bits"]
        pub mod TEB {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Rx DMA Error Bits"]
        pub mod REB {
            pub const offset: u32 = 19;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel 0 Missed Frame Counter"]
    pub mod MISS_FRAME_CNT {
        #[doc = "Dropped Packet Counters"]
        pub mod MFC {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Overflow status of the MFC Counter"]
        pub mod MFCO {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "Miss Frame Counter overflow not occurred"]
                pub const INACTIVE: u32 = 0;
                #[doc = "Miss Frame Counter overflow occurred"]
                pub const ACTIVE: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel 0 Receive ERI Counter"]
    pub mod RX_ERI_CNT {
        #[doc = "ERI Counter"]
        pub mod ECNT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
