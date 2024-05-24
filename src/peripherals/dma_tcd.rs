#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "DMA TCD"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Cluster CH%s, containing CH?_CSR,CH??_CSR, CH?_ES,CH??_ES, CH?_INT,CH??_INT, CH?_SBR,CH??_SBR, CH?_PRI,CH??_PRI, CH?_MUX,CH??_MUX"]
    pub CH: [ch::RegisterBlock; 16usize],
    #[doc = "Cluster TCD%s, containing TCD?_SADDR,TCD??_SADDR, TCD?_SOFF,TCD??_SOFF, TCD?_ATTR,TCD??_ATTR, TCD?_NBYTES_MLOFFNO,TCD??_NBYTES_MLOFFNO, TCD?_NBYTES_MLOFFYES,TCD??_NBYTES_MLOFFYES, TCD?_SLAST_SDA,TCD??_SLAST_SDA, TCD?_DADDR,TCD??_DADDR, TCD?_DOFF,TCD??_DOFF, TCD?_CITER_ELINKNO,TCD??_CITER_ELINKNO, TCD?_CITER_ELINKYES,TCD??_CITER_ELINKYES, TCD?_DLAST_SGA,TCD??_DLAST_SGA, TCD?_CSR,TCD??_CSR, TCD?_BITER_ELINKNO,TCD??_BITER_ELINKNO, TCD?_BITER_ELINKYES,TCD??_BITER_ELINKYES"]
    pub TCD: [tcd::RegisterBlock; 16usize],
}
pub mod ch {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "Cluster CH%s, containing CH?_CSR,CH??_CSR, CH?_ES,CH??_ES, CH?_INT,CH??_INT, CH?_SBR,CH??_SBR, CH?_PRI,CH??_PRI, CH?_MUX,CH??_MUX"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Channel Control and Status"]
        pub CSR: crate::RWRegister<u32>,
        #[doc = "Channel Error Status"]
        pub ES: crate::RWRegister<u32>,
        #[doc = "Channel Interrupt Status"]
        pub INT: crate::RWRegister<u32>,
        #[doc = "Channel System Bus"]
        pub SBR: crate::RWRegister<u32>,
        #[doc = "Channel Priority"]
        pub PRI: crate::RWRegister<u32>,
        #[doc = "Channel Multiplexor Configuration"]
        pub MUX: crate::RWRegister<u32>,
    }
    #[doc = "Channel Control and Status"]
    pub mod CSR {
        #[doc = "Enable DMA Request"]
        pub mod ERQ {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "DMA hardware request signal for corresponding channel disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "DMA hardware request signal for corresponding channel enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Enable Asynchronous DMA Request"]
        pub mod EARQ {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disable asynchronous DMA request for the channel"]
                pub const DISABLE: u32 = 0;
                #[doc = "Enable asynchronous DMA request for the channel"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Enable Error Interrupt"]
        pub mod EEI {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Error signal for corresponding channel does not generate error interrupt"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "Assertion of error signal for corresponding channel generates error interrupt request"]
                pub const ERROR: u32 = 1;
            }
        }
        #[doc = "Enable Buffered Writes"]
        pub mod EBW {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Buffered writes on system bus disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Buffered writes on system bus enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Channel Done"]
        pub mod DONE {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Channel Active"]
        pub mod ACTIVE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Channel Error Status"]
    pub mod ES {
        #[doc = "Destination Bus Error"]
        pub mod DBE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "No destination bus error"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "Last recorded error was bus error on destination write"]
                pub const ERROR: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Source Bus Error"]
        pub mod SBE {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "No source bus error"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "Last recorded error was bus error on source read"]
                pub const ERROR: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Scatter/Gather Configuration Error"]
        pub mod SGE {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "No scatter/gather configuration error"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "Last recorded error was a configuration error detected in the TCDn_DLAST_SGA field"]
                pub const ERROR: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "NBYTES/CITER Configuration Error"]
        pub mod NCE {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "No NBYTES/CITER configuration error"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "Last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields"]
                pub const ERROR: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Destination Offset Error"]
        pub mod DOE {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "No destination offset configuration error"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "Last recorded error was a configuration error detected in the TCDn_DOFF field"]
                pub const ERROR: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Destination Address Error"]
        pub mod DAE {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "No destination address configuration error"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "Last recorded error was a configuration error detected in the TCDn_DADDR field"]
                pub const ERROR: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Source Offset Error"]
        pub mod SOE {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "No source offset configuration error"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "Last recorded error was a configuration error detected in the TCDn_SOFF field"]
                pub const ERROR: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Source Address Error"]
        pub mod SAE {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {
                #[doc = "No source address configuration error"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "Last recorded error was a configuration error detected in the TCDn_SADDR field"]
                pub const ERROR: u32 = 1;
            }
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Error In Channel"]
        pub mod ERR {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "An error in this channel has not occurred"]
                pub const NO_ERROR: u32 = 0;
                #[doc = "An error in this channel has occurred"]
                pub const ERROR: u32 = 1;
            }
        }
    }
    #[doc = "Channel Interrupt Status"]
    pub mod INT {
        #[doc = "Interrupt Request"]
        pub mod INT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Interrupt request for corresponding channel cleared"]
                pub const INTERRUPT_CLEARED: u32 = 0;
                #[doc = "Interrupt request for corresponding channel active"]
                pub const INTERRUPT_ACTIVE: u32 = 1;
            }
        }
    }
    #[doc = "Channel System Bus"]
    pub mod SBR {
        #[doc = "Master ID"]
        pub mod MID {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Security Level"]
        pub mod SEC {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Nonsecure protection level for DMA transfers"]
                pub const NONSECURE_PROTECTION: u32 = 0;
                #[doc = "Secure protection level for DMA transfers"]
                pub const SECURE_PROTECTION: u32 = 1;
            }
        }
        #[doc = "Privileged Access Level"]
        pub mod PAL {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "User protection level for DMA transfers"]
                pub const USER_PROTECTION: u32 = 0;
                #[doc = "Privileged protection level for DMA transfers"]
                pub const PRIVILEGED_PROTECTION: u32 = 1;
            }
        }
        #[doc = "Enable Master ID Replication"]
        pub mod EMI {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Master ID replication is disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Master ID replication is enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
    }
    #[doc = "Channel Priority"]
    pub mod PRI {
        #[doc = "Arbitration Priority Level"]
        pub mod APL {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Disable Preempt Ability"]
        pub mod DPA {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel can suspend a lower-priority channel"]
                pub const SUSPEND: u32 = 0;
                #[doc = "Channel cannot suspend any other channel, regardless of channel priority"]
                pub const CANNOT_SUSPEND: u32 = 1;
            }
        }
        #[doc = "Enable Channel Preemption"]
        pub mod ECP {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel cannot be suspended by a higher-priority channel\'s service request"]
                pub const CANNOT_SUSPEND: u32 = 0;
                #[doc = "Channel can be temporarily suspended by a higher-priority channel\'s service request"]
                pub const SUSPEND: u32 = 1;
            }
        }
    }
    #[doc = "Channel Multiplexor Configuration"]
    pub mod MUX {
        #[doc = "Service Request Source"]
        pub mod SRC {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
pub mod tcd {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "Cluster TCD%s, containing TCD?_SADDR,TCD??_SADDR, TCD?_SOFF,TCD??_SOFF, TCD?_ATTR,TCD??_ATTR, TCD?_NBYTES_MLOFFNO,TCD??_NBYTES_MLOFFNO, TCD?_NBYTES_MLOFFYES,TCD??_NBYTES_MLOFFYES, TCD?_SLAST_SDA,TCD??_SLAST_SDA, TCD?_DADDR,TCD??_DADDR, TCD?_DOFF,TCD??_DOFF, TCD?_CITER_ELINKNO,TCD??_CITER_ELINKNO, TCD?_CITER_ELINKYES,TCD??_CITER_ELINKYES, TCD?_DLAST_SGA,TCD??_DLAST_SGA, TCD?_CSR,TCD??_CSR, TCD?_BITER_ELINKNO,TCD??_BITER_ELINKNO, TCD?_BITER_ELINKYES,TCD??_BITER_ELINKYES"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "TCD Source Address"]
        pub SADDR: crate::RWRegister<u32>,
        #[doc = "TCD Signed Source Address Offset"]
        pub SOFF: crate::RWRegister<u16>,
        #[doc = "TCD Transfer Attributes"]
        pub ATTR: crate::RWRegister<u16>,
        #[doc = "TCD Transfer Size Without Minor Loop Offsets"]
        pub NBYTES_MLOFFNO: crate::RWRegister<u32>,
        #[doc = "TCD Transfer Size with Minor Loop Offsets"]
        pub NBYTES_MLOFFYES: crate::RWRegister<u32>,
        #[doc = "TCD Last Source Address Adjustment / Store DADDR Address"]
        pub SLAST_SDA: crate::RWRegister<u32>,
        #[doc = "TCD Destination Address"]
        pub DADDR: crate::RWRegister<u32>,
        #[doc = "TCD Signed Destination Address Offset"]
        pub DOFF: crate::RWRegister<u16>,
        #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled)"]
        pub CITER_ELINKNO: crate::RWRegister<u16>,
        #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Enabled)"]
        pub CITER_ELINKYES: crate::RWRegister<u16>,
        #[doc = "TCD Last Destination Address Adjustment / Scatter Gather Address"]
        pub DLAST_SGA: crate::RWRegister<u32>,
        #[doc = "TCD Control and Status"]
        pub CSR: crate::RWRegister<u16>,
        #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Disabled)"]
        pub BITER_ELINKNO: crate::RWRegister<u16>,
        #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Enabled)"]
        pub BITER_ELINKYES: crate::RWRegister<u16>,
    }
    #[doc = "TCD Source Address"]
    pub mod SADDR {
        #[doc = "Source Address"]
        pub mod SADDR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Signed Source Address Offset"]
    pub mod SOFF {
        #[doc = "Source Address Signed Offset"]
        pub mod SOFF {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Transfer Attributes"]
    pub mod ATTR {
        #[doc = "Destination Data Transfer Size"]
        pub mod DSIZE {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Destination Address Modulo"]
        pub mod DMOD {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b11111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Source Data Transfer Size"]
        pub mod SSIZE {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "8-bit"]
                pub const EIGHT_BIT: u32 = 0;
                #[doc = "16-bit"]
                pub const SIXTEEN_BIT: u32 = 1;
                #[doc = "32-bit"]
                pub const THIRTYTWO_BIT: u32 = 2;
                #[doc = "64-bit"]
                pub const SIXTYFOUR_BIT: u32 = 3;
                #[doc = "16-byte"]
                pub const SIXTEEN_BYTE: u32 = 4;
                #[doc = "32-byte"]
                pub const THIRTYTWO_BYTE: u32 = 5;
            }
        }
        #[doc = "Source Address Modulo"]
        pub mod SMOD {
            pub const offset: u32 = 11;
            pub const mask: u32 = 0b11111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Source address modulo feature disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Source address modulo feature enabled for any non-zero value [1-31]"]
                pub const ENABLE: u32 = 1;
            }
        }
    }
    #[doc = "TCD Transfer Size Without Minor Loop Offsets"]
    pub mod NBYTES_MLOFFNO {
        #[doc = "Number of Bytes To Transfer Per Service Request"]
        pub mod NBYTES {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Destination Minor Loop Offset Enable"]
        pub mod DMLOE {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Minor loop offset not applied to DADDR"]
                pub const OFFSET_NOT_APPLIED: u32 = 0;
                #[doc = "Minor loop offset applied to DADDR"]
                pub const OFFSET_APPLIED: u32 = 1;
            }
        }
        #[doc = "Source Minor Loop Offset Enable"]
        pub mod SMLOE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Minor loop offset not applied to SADDR"]
                pub const OFFSET_NOT_APPLIED: u32 = 0;
                #[doc = "Minor loop offset applied to SADDR"]
                pub const OFFSET_APPLIED: u32 = 1;
            }
        }
    }
    #[doc = "TCD Transfer Size with Minor Loop Offsets"]
    pub mod NBYTES_MLOFFYES {
        #[doc = "Number of Bytes To Transfer Per Service Request"]
        pub mod NBYTES {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Minor Loop Offset"]
        pub mod MLOFF {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b11111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Destination Minor Loop Offset Enable"]
        pub mod DMLOE {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Minor loop offset not applied to DADDR"]
                pub const OFFSET_NOT_APPLIED: u32 = 0;
                #[doc = "Minor loop offset applied to DADDR"]
                pub const OFFSET_APPLIED: u32 = 1;
            }
        }
        #[doc = "Source Minor Loop Offset Enable"]
        pub mod SMLOE {
            pub const offset: u32 = 31;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Minor loop offset not applied to SADDR"]
                pub const OFFSET_NOT_APPLIED: u32 = 0;
                #[doc = "Minor loop offset applied to SADDR"]
                pub const OFFSET_APPLIED: u32 = 1;
            }
        }
    }
    #[doc = "TCD Last Source Address Adjustment / Store DADDR Address"]
    pub mod SLAST_SDA {
        #[doc = "Last Source Address Adjustment / Store DADDR Address"]
        pub mod SLAST_SDA {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Destination Address"]
    pub mod DADDR {
        #[doc = "Destination Address"]
        pub mod DADDR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    pub mod DOFF {
        #[doc = "Destination Address Signed Offset"]
        pub mod DOFF {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled)"]
    pub mod CITER_ELINKNO {
        #[doc = "Current Major Iteration Count"]
        pub mod CITER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Enable Link"]
        pub mod ELINK {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel-to-channel linking disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Channel-to-channel linking enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
    }
    #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Enabled)"]
    pub mod CITER_ELINKYES {
        #[doc = "Current Major Iteration Count"]
        pub mod CITER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Minor Loop Link Channel Number"]
        pub mod LINKCH {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Enable Link"]
        pub mod ELINK {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel-to-channel linking disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Channel-to-channel linking enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
    }
    #[doc = "TCD Last Destination Address Adjustment / Scatter Gather Address"]
    pub mod DLAST_SGA {
        #[doc = "Last Destination Address Adjustment / Scatter Gather Address"]
        pub mod DLAST_SGA {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "TCD Control and Status"]
    pub mod CSR {
        #[doc = "Channel Start"]
        pub mod START {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel not explicitly started"]
                pub const CHANNEL_NOT_STARTED: u32 = 0;
                #[doc = "Channel explicitly started via a software-initiated service request"]
                pub const CHANNEL_STARTED: u32 = 1;
            }
        }
        #[doc = "Enable Interrupt If Major count complete"]
        pub mod INTMAJOR {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "End-of-major loop interrupt disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "End-of-major loop interrupt enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Enable Interrupt If Major Counter Half-complete"]
        pub mod INTHALF {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Halfway point interrupt disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Halfway point interrupt enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Disable Request"]
        pub mod DREQ {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No operation"]
                pub const CHANNEL_NOT_AFFECTED: u32 = 0;
                #[doc = "Clear the ERQ field to 0 upon major loop completion, thus disabling hardware service requests"]
                pub const ERQ_FIELD_CLEAR: u32 = 1;
            }
        }
        #[doc = "Enable Scatter/Gather Processing"]
        pub mod ESG {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Current channel\'s TCD is normal format"]
                pub const NORMAL_FORMAT: u32 = 0;
                #[doc = "Current channel\'s TCD specifies scatter/gather format."]
                pub const SCATTER_GATHER_FORMAT: u32 = 1;
            }
        }
        #[doc = "Enable Link When Major Loop Complete"]
        pub mod MAJORELINK {
            pub const offset: u32 = 5;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel-to-channel linking disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Channel-to-channel linking enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Enable End-Of-Packet Processing"]
        pub mod EEOP {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "End-of-packet operation disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "End-of-packet hardware input signal enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Enable Store Destination Address"]
        pub mod ESDA {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Ability to store destination address to system memory disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Ability to store destination address to system memory enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
        #[doc = "Major Loop Link Channel Number"]
        pub mod MAJORLINKCH {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Bandwidth Control"]
        pub mod BWC {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "No eDMA engine stalls"]
                pub const NO_STALL: u32 = 0;
                #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
                pub const ENGINE_STALLS_FOUR: u32 = 2;
                #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
                pub const ENGINE_STALLS_EIGHT: u32 = 3;
            }
        }
    }
    #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Disabled)"]
    pub mod BITER_ELINKNO {
        #[doc = "Starting Major Iteration Count"]
        pub mod BITER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Enables Link"]
        pub mod ELINK {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel-to-channel linking disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Channel-to-channel linking enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
    }
    #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Enabled)"]
    pub mod BITER_ELINKYES {
        #[doc = "Starting Major Iteration Count"]
        pub mod BITER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Link Channel Number"]
        pub mod LINKCH {
            pub const offset: u32 = 9;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Enable Link"]
        pub mod ELINK {
            pub const offset: u32 = 15;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Channel-to-channel linking disabled"]
                pub const DISABLE: u32 = 0;
                #[doc = "Channel-to-channel linking enabled"]
                pub const ENABLE: u32 = 1;
            }
        }
    }
}
