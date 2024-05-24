#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "FlexSPI"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module Control 0"]
    pub MCR0: crate::RWRegister<u32>,
    #[doc = "Module Control 1"]
    pub MCR1: crate::RWRegister<u32>,
    #[doc = "Module Control 2"]
    pub MCR2: crate::RWRegister<u32>,
    #[doc = "AHB Bus Control"]
    pub AHBCR: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub INTEN: crate::RWRegister<u32>,
    #[doc = "Interrupt"]
    pub INTR: crate::RWRegister<u32>,
    #[doc = "LUT Key"]
    pub LUTKEY: crate::RWRegister<u32>,
    #[doc = "LUT Control"]
    pub LUTCR: crate::RWRegister<u32>,
    #[doc = "AHB Receive Buffer %s Control 0"]
    pub AHBRXBUF_CR0: [crate::RWRegister<u32>; 8usize],
    _reserved0: [u8; 0x20],
    #[doc = "Flash Control 0"]
    pub FLASHCR0: [crate::RWRegister<u32>; 4usize],
    #[doc = "Flash Control 1"]
    pub FLSHCR1: [crate::RWRegister<u32>; 4usize],
    #[doc = "Flash Control 2"]
    pub FLSHCR2: [crate::RWRegister<u32>; 4usize],
    _reserved1: [u8; 0x4],
    #[doc = "Flash Control 4"]
    pub FLSHCR4: crate::RWRegister<u32>,
    _reserved2: [u8; 0x8],
    #[doc = "IP Control 0"]
    pub IPCR0: crate::RWRegister<u32>,
    #[doc = "IP Control 1"]
    pub IPCR1: crate::RWRegister<u32>,
    #[doc = "IP Control 2"]
    pub IPCR2: crate::RWRegister<u32>,
    _reserved3: [u8; 0x4],
    #[doc = "IP Command"]
    pub IPCMD: crate::RWRegister<u32>,
    #[doc = "Data Learning Pattern"]
    pub DLPR: crate::RWRegister<u32>,
    #[doc = "IP Receive FIFO Control"]
    pub IPRXFCR: crate::RWRegister<u32>,
    #[doc = "IP Transmit FIFO Control"]
    pub IPTXFCR: crate::RWRegister<u32>,
    #[doc = "DLL Control 0"]
    pub DLLCR: [crate::RWRegister<u32>; 2usize],
    _reserved4: [u8; 0x18],
    #[doc = "Status 0"]
    pub STS0: crate::RWRegister<u32>,
    #[doc = "Status 1"]
    pub STS1: crate::RWRegister<u32>,
    #[doc = "Status 2"]
    pub STS2: crate::RWRegister<u32>,
    #[doc = "AHB Suspend Status"]
    pub AHBSPNDSTS: crate::RWRegister<u32>,
    #[doc = "IP Receive FIFO Status"]
    pub IPRXFSTS: crate::RWRegister<u32>,
    #[doc = "IP Transmit FIFO Status"]
    pub IPTXFSTS: crate::RWRegister<u32>,
    _reserved5: [u8; 0x8],
    #[doc = "IP Receive FIFO Data x"]
    pub RFDR: [crate::RWRegister<u32>; 32usize],
    #[doc = "IP TX FIFO Data x"]
    pub TFDR: [crate::RWRegister<u32>; 32usize],
    #[doc = "Lookup Table x"]
    pub LUT: [crate::RWRegister<u32>; 64usize],
    _reserved6: [u8; 0x120],
    #[doc = "HADDR REMAP Start Address"]
    pub HADDRSTART: crate::RWRegister<u32>,
    #[doc = "HADDR REMAP END ADDR"]
    pub HADDREND: crate::RWRegister<u32>,
    #[doc = "HADDR Remap Offset"]
    pub HADDROFFSET: crate::RWRegister<u32>,
    #[doc = "IPED Function Control"]
    pub IPEDCTRL: crate::RWRegister<u32>,
    #[doc = "IPS Nonsecure Region 0 Start Address"]
    pub IPSNSZSTART0: crate::RWRegister<u32>,
    #[doc = "IPS Nonsecure Region 0 End Address"]
    pub IPSNSZEND0: crate::RWRegister<u32>,
    #[doc = "IPS Nonsecure Region 1 Start Address"]
    pub IPSNSZSTART1: crate::RWRegister<u32>,
    #[doc = "IPS Nonsecure Region 1 End Address"]
    pub IPSNSZEND1: crate::RWRegister<u32>,
    #[doc = "Receive Buffer Start Address of Region %s"]
    pub AHBBUFREGIONSTART: [crate::RWRegister<u32>; 4usize],
    #[doc = "Receive Buffer Region %s End Address"]
    pub AHBBUFREGIONEND: [crate::RWRegister<u32>; 4usize],
    _reserved7: [u8; 0x80],
    #[doc = "IPED context control 0"]
    pub IPEDCTXCTRL0: crate::RWRegister<u32>,
    #[doc = "IPED context control 1"]
    pub IPEDCTXCTRL1: crate::RWRegister<u32>,
    _reserved8: [u8; 0x18],
    #[doc = "Cluster IPEDCTX%s, containing IPEDCTX?IV0, IPEDCTX?IV1, IPEDCTX?START, IPEDCTX?END, IPEDCTX?AAD0, IPEDCTX?AAD1"]
    pub IPEDCTX: [ipedctx::RegisterBlock; 7usize],
}
#[doc = "Module Control 0"]
pub mod MCR0 {
    #[doc = "Software Reset"]
    pub mod SWRESET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No impact"]
            pub const VAL0: u32 = 0;
            #[doc = "Software reset"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Module Disable"]
    pub mod MDIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No impact"]
            pub const VAL0: u32 = 0;
            #[doc = "Module disable"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Sample Clock Source for Flash Reading"]
    pub mod RXCLKSRC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dummy Read strobe that FlexSPI generates, looped back internally"]
            pub const VAL0: u32 = 0;
            #[doc = "Dummy Read strobe that FlexSPI generates, looped back from DQS pad"]
            pub const VAL1: u32 = 1;
            #[doc = "SCLK output clock and looped back from SCLK pad"]
            pub const VAL2: u32 = 2;
            #[doc = "Flash-memory-provided read strobe and input from DQS pad"]
            pub const VAL3: u32 = 3;
        }
    }
    #[doc = "AHB Read Access to IP Receive FIFO Enable"]
    pub mod ARDFEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHB read access disabled. IP bus reads IP receive FIFO. AHB Bus read access to IP receive FIFO memory space produces bus error."]
            pub const VAL0: u32 = 0;
            #[doc = "AHB read access enabled. AHB bus reads IP receive FIFO. IP Bus read access to IP receive FIFO memory space returns data zero and causes no bus error."]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "AHB Write Access to IP Transmit FIFO Enable"]
    pub mod ATDFEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHB write access disabled. IP bus writes to IP transmit FIFO. AHB bus write access to IP transmit FIFO memory space produces bus error."]
            pub const VAL0: u32 = 0;
            #[doc = "AHB write access enabled. AHB bus writes to IP transmit FIFO. IP Bus write access to IP transmit FIFO memory space is ignored and causes no bus error."]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Serial Root Clock Divider"]
    pub mod SERCLKDIV {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divided by 1"]
            pub const VAL0: u32 = 0;
            #[doc = "Divided by 2"]
            pub const VAL1: u32 = 1;
            #[doc = "Divided by 3"]
            pub const VAL2: u32 = 2;
            #[doc = "Divided by 4"]
            pub const VAL3: u32 = 3;
            #[doc = "Divided by 5"]
            pub const VAL4: u32 = 4;
            #[doc = "Divided by 6"]
            pub const VAL5: u32 = 5;
            #[doc = "Divided by 7"]
            pub const VAL6: u32 = 6;
            #[doc = "Divided by 8"]
            pub const VAL7: u32 = 7;
        }
    }
    #[doc = "Half Speed Serial Flash Memory Access Enable"]
    pub mod HSEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VAL0: u32 = 0;
            #[doc = "Enable"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Doze Mode Enable"]
    pub mod DOZEEN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VAL0: u32 = 0;
            #[doc = "Enable"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Combination Mode Enable"]
    pub mod COMBINATIONEN {
        pub const offset: u32 = 13;
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
    #[doc = "SCLK Free-running Enable"]
    pub mod SCKFREERUNEN {
        pub const offset: u32 = 14;
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
    #[doc = "Data Learning Enable"]
    pub mod LEARNEN {
        pub const offset: u32 = 15;
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
    #[doc = "Timeout Wait Cycle for IP Command Grant"]
    pub mod IPGRANTWAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeouts Wait Cycle for AHB command Grant"]
    pub mod AHBGRANTWAIT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Module Control 1"]
pub mod MCR1 {
    #[doc = "AHB Bus Wait"]
    pub mod AHBBUSWAIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Sequence Wait"]
    pub mod SEQWAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Module Control 2"]
pub mod MCR2 {
    #[doc = "Clear AHB Buffer"]
    pub mod CLRAHBBUFOPT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not cleared automatically"]
            pub const VAL0: u32 = 0;
            #[doc = "Cleared automatically"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Clear Learn Phase Selection"]
    pub mod CLRLEARNPHASE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No impact"]
            pub const VAL0: u32 = 0;
            #[doc = "Reset sample clock phase selection to 0"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Same Device Enable"]
    pub mod SAMEDEVICEEN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "In Individual mode, FLSHA1CRx and FLSHA2CRx, FLSHB1CRx and FLSHB2CRx settings are applied to Flash A1, A2, B1, B2 separately. In Parallel mode, FLSHA1CRx register setting is applied to Flash A1 and B1, FLSHA2CRx register setting is applied to Flash A2 and B2. FLSHB1CRx and FLSHB2CRx register settings are ignored."]
            pub const INDIVIDUAL_PARALLEL: u32 = 0;
            #[doc = "FLSHA1CR0, FLSHA1CR1, and FLSHA1CR2 register settings are applied to Flash A1, A2, B1, B2. FLSHA2CRx, FLSHB1CRx, and FLSHB2CRx settings are ignored."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "SCLK Port B Differential Output"]
    pub mod SCKBDIFFOPT {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use B_SCLK pad as port B SCLK clock output. Port B flash memory access is available."]
            pub const VAL1: u32 = 0;
            #[doc = "Use B_SCLK pad as port A SCLK inverted clock output (Differential clock to A_SCLK). Port B flash memory access is not available."]
            pub const VAL0: u32 = 1;
        }
    }
    #[doc = "Port B Receiver Clock Source"]
    pub mod RXCLKSRC_B {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dummy read strobe that FlexSPI generates, looped back internally."]
            pub const VAL0: u32 = 0;
            #[doc = "Dummy read strobe that FlexSPI generates, looped back from DQS pad."]
            pub const VAL1: u32 = 1;
            #[doc = "SCLK output clock and looped back from SCLK pad"]
            pub const VAL2: u32 = 2;
            #[doc = "Flash-memory-provided read strobe and input from DQS pad"]
            pub const VAL3: u32 = 3;
        }
    }
    #[doc = "Sample Clock Source Different"]
    pub mod RX_CLK_SRC_DIFF {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use MCR0[RXCLKSRC] for Port A and Port B."]
            pub const VALUE0: u32 = 0;
            #[doc = "Use MCR0[RXCLKSRC] for Port A, and MCR2[RXCLKSRC_B] for Port B."]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "Resume Wait Duration"]
    pub mod RESUMEWAIT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Bus Control"]
pub mod AHBCR {
    #[doc = "AHB Parallel Mode Enable"]
    pub mod APAREN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Flash is accessed in Individual mode."]
            pub const INDIVIDUAL: u32 = 0;
            #[doc = "Flash is accessed in Parallel mode."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Clear AHB Receive Buffer"]
    pub mod CLRAHBRXBUF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No impact."]
            pub const VAL0: u32 = 0;
            #[doc = "Enable clear operation."]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Clear AHB Transmit Buffer"]
    pub mod CLRAHBTXBUF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No impact."]
            pub const VAL0: u32 = 0;
            #[doc = "Enable clear operation."]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Cacheable Read Access Enable"]
    pub mod CACHABLEEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. When an AHB bus cacheable read access occurs, FlexSPI does not check whether it hit the AHB transmit buffer."]
            pub const VAL0: u32 = 0;
            #[doc = "Enabled. When an AHB bus cacheable read access occurs, FlexSPI first checks whether the access hit the AHB transmit buffer."]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Bufferable Write Access Enable"]
    pub mod BUFFERABLEEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. For all AHB write accesses (bufferable or nonbufferable), FlexSPI returns AHB Bus Ready after transmitting all data and finishing command."]
            pub const VAL0: u32 = 0;
            #[doc = "Enabled. For AHB bufferable write access, FlexSPI returns AHB Bus Ready when the arbitrator grants the AHB command. FlexSPI does not wait for the AHB command to finish."]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "AHB Read Prefetch Enable"]
    pub mod PREFETCHEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "AHB Read Address Option"]
    pub mod READADDROPT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AHB read burst start address alignment is limited when flash memory is accessed in parallel mode or flash is word-addressable."]
            pub const VAL0: u32 = 0;
            #[doc = "AHB read burst start address alignment is not limited. FlexSPI fetches more data than the AHB burst requires for address alignment."]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "AHB Read Resume Disable"]
    pub mod RESUMEDISABLE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Suspended AHB read prefetch resumes when AHB is IDLE."]
            pub const VAL0: u32 = 0;
            #[doc = "Suspended AHB read prefetch does not resume once aborted,"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "AHB Read Size Alignment"]
    pub mod READSZALIGN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register settings such as PREFETCH_EN determine AHB read size."]
            pub const VAL0: u32 = 0;
            #[doc = "AHB read size to up size to 8 bytes aligned, no prefetching"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "AHB Boundary Alignment"]
    pub mod ALIGNMENT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No limit"]
            pub const BIT0: u32 = 0;
            #[doc = "1 KB"]
            pub const BIT1: u32 = 1;
            #[doc = "512 bytes"]
            pub const BIT2: u32 = 2;
            #[doc = "256 bytes"]
            pub const BIT3: u32 = 3;
        }
    }
}
#[doc = "Interrupt Enable"]
pub mod INTEN {
    #[doc = "IP-Triggered Command Sequences Execution Finished Interrupt Enable"]
    pub mod IPCMDDONEEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout Interrupt Enable"]
    pub mod IPCMDGEEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout Interrupt Enable."]
    pub mod AHBCMDGEEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "IP-Triggered Command Sequences Error Detected Interrupt Enable"]
    pub mod IPCMDERREN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "AHB-Triggered Command Sequences Error Detected Interrupt Enable"]
    pub mod AHBCMDERREN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "IP Receive FIFO Watermark Available Interrupt Enable"]
    pub mod IPRXWAEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "IP Transmit FIFO Watermark Empty Interrupt Enable"]
    pub mod IPTXWEEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "Data Learning Failed Interrupt Enable"]
    pub mod DATALEARNFAILEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "SCLK Stopped By Read Interrupt Enable"]
    pub mod SCKSTOPBYRDEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "SCLK Stopped By Write Interrupt Enable"]
    pub mod SCKSTOPBYWREN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "AHB Bus Timeout Interrupt Enable"]
    pub mod AHBBUSTIMEOUTEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "Sequence execution Timeout Interrupt Enable"]
    pub mod SEQTIMEOUTEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "IP Command Security Violation Interrupt Enable"]
    pub mod IPCMDSECUREVIOEN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "AHB Read GCM Error Interrupt Enable"]
    pub mod AHBGCMERREN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt or no impact"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const VALUE1: u32 = 1;
        }
    }
}
#[doc = "Interrupt"]
pub mod INTR {
    #[doc = "IP-Triggered Command Sequences Execution Finished"]
    pub mod IPCMDDONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout"]
    pub mod IPCMDGE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout"]
    pub mod AHBCMDGE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "IP-Triggered Command Sequences Error"]
    pub mod IPCMDERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "AHB-Triggered Command Sequences Error"]
    pub mod AHBCMDERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "IP Receive FIFO Watermark Available"]
    pub mod IPRXWA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "IP Transmit FIFO Watermark Empty"]
    pub mod IPTXWE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "Data Learning Failed"]
    pub mod DATALEARNFAIL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "SCLK Stopped Due To Full Receive FIFO"]
    pub mod SCKSTOPBYRD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "SCLK Stopped Due To Empty Transmit FIFO"]
    pub mod SCKSTOPBYWR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "AHB Bus Timeout"]
    pub mod AHBBUSTIMEOUT {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "Sequence Execution Timeout"]
    pub mod SEQTIMEOUT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "IP Command Security Violation"]
    pub mod IPCMDSECUREVIO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
    #[doc = "AHB Read GCM Error"]
    pub mod AHBGCMERR {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt condition has not occurred"]
            pub const NO_INTERRUPT: u32 = 0;
            #[doc = "Interrupt condition has occurred"]
            pub const INTERRUPT: u32 = 1;
        }
    }
}
#[doc = "LUT Key"]
pub mod LUTKEY {
    #[doc = "LUT Key"]
    pub mod KEY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "LUT Control"]
pub mod LUTCR {
    #[doc = "Lock LUT"]
    pub mod LOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LUT is unlocked (LUTCR[UNLOCK] must be 1)"]
            pub const VALUE0: u32 = 0;
            #[doc = "LUT is locked and cannot be written"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "Unlock LUT"]
    pub mod UNLOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LUT is locked (LUTCR[LOCK] must be 1)"]
            pub const VALUE0: u32 = 0;
            #[doc = "LUT is unlocked and can be written"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "LUT Protection"]
    pub mod PROTECT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not protected. All IPS controllers can access LUTCR and LUT memory."]
            pub const VALUE0: u32 = 0;
            #[doc = "Protected. Only secure IPS controller can change the value of LUTCR and write to LUT memory."]
            pub const VALUE1: u32 = 1;
        }
    }
}
#[doc = "AHB Receive Buffer %s Control 0"]
pub mod AHBRXBUF_CR0 {
    #[doc = "AHB Receive Buffer Size"]
    pub mod BUFSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller ID"]
    pub mod MSTRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Controller Read Priority"]
    pub mod PRIORITY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Receive Buffer Address Region Enable"]
    pub mod REGIONEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "AHB Read Prefetch Enable"]
    pub mod PREFETCHEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enabled when is enabled."]
            pub const VALUE1: u32 = 1;
        }
    }
}
#[doc = "Flash Control 0"]
pub mod FLASHCR0 {
    #[doc = "Flash Size in KB"]
    pub mod FLSHSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Address Shift Function control"]
    pub mod ADDRSHIFT {
        pub const offset: u32 = 29;
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
    #[doc = "AHB Write Access Split Function Enable"]
    pub mod SPLITWREN {
        pub const offset: u32 = 30;
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
    #[doc = "AHB Read Access Split Function Enable"]
    pub mod SPLITRDEN {
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
#[doc = "Flash Control 1"]
pub mod FLSHCR1 {
    #[doc = "Serial Flash CS Setup Time"]
    pub mod TCSS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Serial Flash CS Hold Time"]
    pub mod TCSH {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word-Addressable"]
    pub mod WA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte-addressable"]
            pub const VALUE0: u32 = 0;
            #[doc = "Word-addressable"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "Column Address Size"]
    pub mod CAS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Chip Select Interval Unit"]
    pub mod CSINTERVALUNIT {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 serial clock cycle"]
            pub const VAL0: u32 = 0;
            #[doc = "256 serial clock cycles"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Chip Select Interval"]
    pub mod CSINTERVAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control 2"]
pub mod FLSHCR2 {
    #[doc = "Sequence Index for AHB Read-Triggered Command in LUT"]
    pub mod ARDSEQID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Number for AHB Read-Triggered Command"]
    pub mod ARDSEQNUM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Index for AHB Write-Triggered Command"]
    pub mod AWRSEQID {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Number for AHB Write-Triggered Command"]
    pub mod AWRSEQNUM {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Write Wait"]
    pub mod AWRWAIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AWRWAIT Unit"]
    pub mod AWRWAITUNIT {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "2"]
            pub const VAL0: u32 = 0;
            #[doc = "8"]
            pub const VAL1: u32 = 1;
            #[doc = "32"]
            pub const VAL2: u32 = 2;
            #[doc = "128"]
            pub const VAL3: u32 = 3;
            #[doc = "512"]
            pub const VAL4: u32 = 4;
            #[doc = "2048"]
            pub const VAL5: u32 = 5;
            #[doc = "8192"]
            pub const VAL6: u32 = 6;
            #[doc = "32768"]
            pub const VAL7: u32 = 7;
        }
    }
    #[doc = "Clear Instruction Pointer"]
    pub mod CLRINSTRPTR {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Flash Control 4"]
pub mod FLSHCR4 {
    #[doc = "Write Mask Option 1"]
    pub mod WMOPT1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When writing to an external device, DQS pin is used as write mask. When flash memory is accessed in individual mode, AHB or IP write burst start address alignment is not limited."]
            pub const DISABLE: u32 = 0;
            #[doc = "When writing to an external device, DQS pin is not used as write mask. When flash memory is accessed in individual mode, AHB or IP write burst start address alignment is limited."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Write Mask Enable for Port A"]
    pub mod WMENA {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. When writing to external device, DQS(RWDS) pin is not driven."]
            pub const VAL0: u32 = 0;
            #[doc = "Enabled. When writing to external device, FlexSPI drives DQS(RWDS) pin as write mask output."]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Write Mask Enable for Port B"]
    pub mod WMENB {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. When writing to external device, DQS(RWDS) pin is not driven."]
            pub const VAL0: u32 = 0;
            #[doc = "Enabled. When writing to external device, FlexSPI drives DQS(RWDS) pin as write mask output."]
            pub const VAL1: u32 = 1;
        }
    }
}
#[doc = "IP Control 0"]
pub mod IPCR0 {
    #[doc = "Serial Flash Address"]
    pub mod SFAR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Control 1"]
pub mod IPCR1 {
    #[doc = "Flash Read/Program Data Size (in bytes) for IP command."]
    pub mod IDATSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Index in LUT for IP command."]
    pub mod ISEQID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Number for IP command: ISEQNUM+1."]
    pub mod ISEQNUM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parallel Mode Enable for IP Commands"]
    pub mod IPAREN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. Flash memory is accessed in Individual mode."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enabled. Flash memory is accessed in Parallel mode."]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "IP Control 2"]
pub mod IPCR2 {
    #[doc = "IP Command Blocking AHB Command Request Enable"]
    pub mod IPBLKAHBREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IP commands do not block AHB command requests."]
            pub const VALUE0: u32 = 0;
            #[doc = "IP commands block AHB command requests."]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "IP Command Blocking AHB Command Acknowledgment Enable"]
    pub mod IPBLKAHBACK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "IP commands do not block AHB command acknowledgment."]
            pub const VALUE0: u32 = 0;
            #[doc = "IP commands block AHB command acknowledgment."]
            pub const VALUE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP Command Blocking All AHB Command Enable"]
    pub mod IPBLKALLAHB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IP commands only block AHB commands that affect the IPED region."]
            pub const VALUE0: u32 = 0;
            #[doc = "IP commands block all AHB commands."]
            pub const VALUE1: u32 = 1;
        }
    }
}
#[doc = "IP Command"]
pub mod IPCMD {
    #[doc = "Command Trigger"]
    pub mod TRG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const VALUE0: u32 = 0;
            #[doc = "Start the IP command that the IPCR0 and IPCR1 registers define."]
            pub const VALUE1: u32 = 1;
        }
    }
}
#[doc = "Data Learning Pattern"]
pub mod DLPR {
    #[doc = "Data Learning Pattern"]
    pub mod DLP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Receive FIFO Control"]
pub mod IPRXFCR {
    #[doc = "Clear IP Receive FIFO"]
    pub mod CLRIPRXF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No function"]
            pub const VALUE0: u32 = 0;
            #[doc = "A clock cycle pulse clears all valid data entries in IP receive FIFO."]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "IP Receive FIFO Reading by DMA Enable"]
    pub mod RXDMAEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled. The processor reads the FIFO."]
            pub const VAL0: u32 = 0;
            #[doc = "Enabled. DMA reads the FIFO."]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "IP Receive FIFO Watermark Level"]
    pub mod RXWMRK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Transmit FIFO Control"]
pub mod IPTXFCR {
    #[doc = "Clear IP Transmit FIFO"]
    pub mod CLRIPTXF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No function"]
            pub const VALUE0: u32 = 0;
            #[doc = "A clock cycle pulse clears all valid data entries in the IP transmit FIFO."]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "Transmit FIFO DMA Enable"]
    pub mod TXDMAEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Processor"]
            pub const VAL0: u32 = 0;
            #[doc = "DMA"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Transmit Watermark Level"]
    pub mod TXWMRK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DLL Control 0"]
pub mod DLLCR {
    #[doc = "DLL Calibration Enable"]
    pub mod DLLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "DLL reset"]
    pub mod DLLRESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No function"]
            pub const VALUE0: u32 = 0;
            #[doc = "Force DLL reset."]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "Target Delay Line For Target"]
    pub mod SLVDLYTARGET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target Clock Delay Line Override Value Enable"]
    pub mod OVRDEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VALUE0: u32 = 0;
            #[doc = "Enable"]
            pub const VALUE1: u32 = 1;
        }
    }
    #[doc = "Target Clock Delay Line Override Value"]
    pub mod OVRDVAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status 0"]
pub mod STS0 {
    #[doc = "SEQ_CTL State Machine Idle"]
    pub mod SEQIDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not idle"]
            pub const VALUE0: u32 = 0;
            #[doc = "Idle"]
            pub const VALUE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ARB_CTL State Machine Idle"]
    pub mod ARBIDLE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not idle"]
            pub const VALUE0: u32 = 0;
            #[doc = "Idle"]
            pub const VALUE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ARB Command Source"]
    pub mod ARBCMDSRC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Trigger source is AHB read command."]
            pub const VAL0: u32 = 0;
            #[doc = "Trigger source is AHB write command."]
            pub const VAL1: u32 = 1;
            #[doc = "Trigger source is IP command (by writing 1 to IPCMD[TRG])."]
            pub const VAL2: u32 = 2;
            #[doc = "Trigger source is a suspended command that has resumed."]
            pub const VAL3: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Learning Phase Selection on Port A"]
    pub mod DATALEARNPHASEA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Learning Phase Selection on Port B"]
    pub mod DATALEARNPHASEB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status 1"]
pub mod STS1 {
    #[doc = "AHB Command Error ID"]
    pub mod AHBCMDERRID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Command Error Code"]
    pub mod AHBCMDERRCODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const VAL0: u32 = 0;
            #[doc = "AHB Write command with JMP_ON_CS instruction used in the sequence"]
            pub const VAL2: u32 = 2;
            #[doc = "Unknown instruction opcode in the sequence"]
            pub const VAL3: u32 = 3;
            #[doc = "DUMMY_SDR or DUMMY_RWDS_SDR instruction used in DDR sequence"]
            pub const VAL4: u32 = 4;
            #[doc = "DUMMY_DDR or DUMMY_RWDS_DDR instruction used in SDR sequence"]
            pub const VAL5: u32 = 5;
            #[doc = "Sequence execution timeout"]
            pub const VAL6: u32 = 14;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP Command Error ID"]
    pub mod IPCMDERRID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IP Command Error Code"]
    pub mod IPCMDERRCODE {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "No error"]
            pub const VAL0: u32 = 0;
            #[doc = "IP command with JMP_ON_CS instruction used in the sequence"]
            pub const VAL2: u32 = 2;
            #[doc = "Unknown instruction opcode in the sequence"]
            pub const VAL3: u32 = 3;
            #[doc = "DUMMY_SDR or DUMMY_RWDS_SDR instruction used in DDR sequence"]
            pub const VAL4: u32 = 4;
            #[doc = "DUMMY_DDR or DUMMY_RWDS_DDR instruction used in SDR sequence"]
            pub const VAL5: u32 = 5;
            #[doc = "Flash memory access start address exceeds entire flash address range (A1, A2, B1, and B2)"]
            pub const VAL6: u32 = 6;
            #[doc = "Sequence execution timeout"]
            pub const VAL7: u32 = 14;
            #[doc = "Flash boundary crossed"]
            pub const VAL8: u32 = 15;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status 2"]
pub mod STS2 {
    #[doc = "Flash A Sample Target Delay Line Locked"]
    pub mod ASLVLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not locked"]
            pub const VAL0: u32 = 0;
            #[doc = "Locked"]
            pub const VAL1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Locked"]
    pub mod AREFLOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not locked"]
            pub const VAL0: u32 = 0;
            #[doc = "Locked"]
            pub const VAL1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash A Sample Clock Target Delay Line Delay Cell Number"]
    pub mod ASLVSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Delay Cell Number"]
    pub mod AREFSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash B Sample Target Reference Delay Line Locked"]
    pub mod BSLVLOCK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not locked"]
            pub const VAL0: u32 = 0;
            #[doc = "Locked"]
            pub const VAL1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Locked"]
    pub mod BREFLOCK {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not locked"]
            pub const VAL0: u32 = 0;
            #[doc = "Locked"]
            pub const VAL1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash B Sample Clock Target Delay Line Delay Cell Number"]
    pub mod BSLVSEL {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Delay Cell Number"]
    pub mod BREFSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "AHB Suspend Status"]
pub mod AHBSPNDSTS {
    #[doc = "Active AHB Read Prefetch Suspended"]
    pub mod ACTIVE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No suspended AHB read prefetch command."]
            pub const VAL0: u32 = 0;
            #[doc = "An AHB read prefetch command sequence has been suspended."]
            pub const VAL1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "AHB Receive Buffer ID for Suspended Command Sequence"]
    pub mod BUFID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Left"]
    pub mod DATLFT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Receive FIFO Status"]
pub mod IPRXFSTS {
    #[doc = "Fill Level of IP Receive FIFO"]
    pub mod FILL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Data Counter"]
    pub mod RDCNTR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Transmit FIFO Status"]
pub mod IPTXFSTS {
    #[doc = "Fill Level of IP Transmit FIFO"]
    pub mod FILL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Data Counter"]
    pub mod WRCNTR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Receive FIFO Data x"]
pub mod RFDR {
    #[doc = "Receive Data"]
    pub mod RXDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP TX FIFO Data x"]
pub mod TFDR {
    #[doc = "Transmit Data"]
    pub mod TXDATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lookup Table x"]
pub mod LUT {
    #[doc = "OPERAND0"]
    pub mod OPERAND0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NUM_PADS0"]
    pub mod NUM_PADS0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OPCODE"]
    pub mod OPCODE0 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OPERAND1"]
    pub mod OPERAND1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NUM_PADS1"]
    pub mod NUM_PADS1 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "OPCODE1"]
    pub mod OPCODE1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HADDR REMAP Start Address"]
pub mod HADDRSTART {
    #[doc = "AHB Bus Address Remap Enable"]
    pub mod REMAPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HADDR REMAP Disabled"]
            pub const VAL0: u32 = 0;
            #[doc = "HADDR REMAP Enabled"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "HADDR Start Address"]
    pub mod ADDRSTART {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HADDR REMAP END ADDR"]
pub mod HADDREND {
    #[doc = "End Address of HADDR Remap Range"]
    pub mod ENDSTART {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "HADDR Remap Offset"]
pub mod HADDROFFSET {
    #[doc = "HADDR Offset"]
    pub mod ADDROFFSET {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPED Function Control"]
pub mod IPEDCTRL {
    #[doc = "IPED Mode Select"]
    pub mod CONFIG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fully pipelined"]
            pub const VAL0: u32 = 0;
            #[doc = "Not fully pipelined"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "IPED Encryption and Decryption Enable"]
    pub mod IPED_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VAL0: u32 = 0;
            #[doc = "Enable"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "IP Write IPED CTR Mode Encryption Enable"]
    pub mod IPWR_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VAL0: u32 = 0;
            #[doc = "Enable"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "AHB Write IPED CTR Mode Encryption Enable."]
    pub mod AHBWR_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VAL0: u32 = 0;
            #[doc = "Enable"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "AHB Read IPED CTR Mode Decryption Enable"]
    pub mod AHBRD_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VAL0: u32 = 0;
            #[doc = "Enable"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "IP Write GCM Mode Enable"]
    pub mod IPGCMWR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const VAL0: u32 = 0;
            #[doc = "Enabled"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "AHB Write IPED GCM Mode Encryption Enable"]
    pub mod AHGCMWR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VAL0: u32 = 0;
            #[doc = "Enable"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "AHB Read IPED GCM Mode Decryption Enable"]
    pub mod AHBGCMRD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const VAL0: u32 = 0;
            #[doc = "Enable"]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "IPED Protection"]
    pub mod IPED_PROTECT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No restrictions"]
            pub const VAL0: u32 = 0;
            #[doc = "Only privileged controllers can write IPED registers."]
            pub const VAL1: u32 = 1;
        }
    }
    #[doc = "Abort Current Decryption or Encryption"]
    pub mod IPED_SWRESET {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No function."]
            pub const VAL0: u32 = 0;
            #[doc = "Aborts current decryption or encryption and waits for the next start operation."]
            pub const VAL1: u32 = 1;
        }
    }
}
#[doc = "IPS Nonsecure Region 0 Start Address"]
pub mod IPSNSZSTART0 {
    #[doc = "Start Address of Nonsecure Region"]
    pub mod start_address {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPS Nonsecure Region 0 End Address"]
pub mod IPSNSZEND0 {
    #[doc = "End Address of Nonsecure Region"]
    pub mod end_address {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPS Nonsecure Region 1 Start Address"]
pub mod IPSNSZSTART1 {
    #[doc = "Start Address of Nonsecure Region"]
    pub mod start_address {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPS Nonsecure Region 1 End Address"]
pub mod IPSNSZEND1 {
    #[doc = "End Address of Nonsecure Region"]
    pub mod end_address {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Buffer Start Address of Region %s"]
pub mod AHBBUFREGIONSTART {
    #[doc = "Start Address of Prefetch Sub-Buffer Region"]
    pub mod START_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Buffer Region %s End Address"]
pub mod AHBBUFREGIONEND {
    #[doc = "End Address of Prefetch Sub-Buffer Region"]
    pub mod END_ADDRESS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPED context control 0"]
pub mod IPEDCTXCTRL0 {
    #[doc = "Context Register Freeze for Region 0"]
    pub mod CTX0_FREEZE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Register Freeze for Region 1"]
    pub mod CTX1_FREEZE0 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Register Freeze for Region 2"]
    pub mod CTX2_FREEZE0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Register Freeze for Region 3"]
    pub mod CTX3_FREEZE0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Register Freeze for Region 4"]
    pub mod CTX4_FREEZE0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Register Freeze for Region 5"]
    pub mod CTX5_FREEZE0 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Register Freeze for Region 6"]
    pub mod CTX6_FREEZE0 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IPED context control 1"]
pub mod IPEDCTXCTRL1 {
    #[doc = "Context Register Freeze for Region 0"]
    pub mod CTX0_FREEZE1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Register Freeze for Region 1"]
    pub mod CTX1_FREEZE1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Register Freeze for Region 2"]
    pub mod CTX2_FREEZE1 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Register Freeze for Region 3"]
    pub mod CTX3_FREEZE1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Register Freeze for Region 4"]
    pub mod CTX4_FREEZE1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Register Freeze for Region 5"]
    pub mod CTX5_FREEZE1 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Register Freeze for Region 6"]
    pub mod CTX6_FREEZE1 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod ipedctx {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "Cluster IPEDCTX%s, containing IPEDCTX?IV0, IPEDCTX?IV1, IPEDCTX?START, IPEDCTX?END, IPEDCTX?AAD0, IPEDCTX?AAD1"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "IPED Context0 IV0"]
        pub IV0: crate::RWRegister<u32>,
        #[doc = "IPED Context0 IV1"]
        pub IV1: crate::RWRegister<u32>,
        #[doc = "Start Address of Region"]
        pub START: crate::RWRegister<u32>,
        #[doc = "End Address of Region"]
        pub END: crate::RWRegister<u32>,
        #[doc = "IPED Context0 Additional Authenticated Data0"]
        pub AAD0: crate::RWRegister<u32>,
        #[doc = "IPED Context0 Additional Authenticated Data1"]
        pub AAD1: crate::RWRegister<u32>,
    }
    #[doc = "IPED Context0 IV0"]
    pub mod IV0 {
        #[doc = "Lowest 32 bits of IV for region 0."]
        pub mod CTX0_IV0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "IPED Context0 IV1"]
    pub mod IV1 {
        #[doc = "Highest 32 bits of IV for region 0."]
        pub mod CTX0_IV1 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "Start Address of Region"]
    pub mod START {
        #[doc = "GCM Mode Enable"]
        pub mod GCM {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Disabled. CTR mode is used."]
                pub const VALUE0: u32 = 0;
                #[doc = "Enabled. GCM mode is used."]
                pub const VALUE1: u32 = 1;
            }
        }
        #[doc = "AHB Bus Error Disable"]
        pub mod ahbbuserror_dis {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "AHB bus errors enabled"]
                pub const VALUE0: u32 = 0;
                #[doc = "AHB bus errors disabled"]
                pub const VALUE1: u32 = 1;
            }
        }
        #[doc = "Start Address"]
        pub mod start_address {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "End Address of Region"]
    pub mod END {
        #[doc = "End Address of IPED Region"]
        pub mod end_address {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "IPED Context0 Additional Authenticated Data0"]
    pub mod AAD0 {
        #[doc = "CTX AAD"]
        pub mod CTX0_AAD0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "IPED Context0 Additional Authenticated Data1"]
    pub mod AAD1 {
        #[doc = "CTX AAD"]
        pub mod CTX0_AAD1 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
