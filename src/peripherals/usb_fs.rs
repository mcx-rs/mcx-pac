#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "USBFS"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Peripheral ID"]
    pub PERID: crate::RWRegister<u8>,
    _reserved0: [u8; 0x3],
    #[doc = "Peripheral ID Complement"]
    pub IDCOMP: crate::RWRegister<u8>,
    _reserved1: [u8; 0x3],
    #[doc = "Peripheral Revision"]
    pub REV: crate::RWRegister<u8>,
    _reserved2: [u8; 0x3],
    #[doc = "Peripheral Additional Information"]
    pub ADDINFO: crate::RWRegister<u8>,
    _reserved3: [u8; 0x3],
    #[doc = "OTG Interrupt Status"]
    pub OTGISTAT: crate::RWRegister<u8>,
    _reserved4: [u8; 0x3],
    #[doc = "OTG Interrupt Control"]
    pub OTGICR: crate::RWRegister<u8>,
    _reserved5: [u8; 0x3],
    #[doc = "OTG Status"]
    pub OTGSTAT: crate::RWRegister<u8>,
    _reserved6: [u8; 0x3],
    #[doc = "OTG Control"]
    pub OTGCTL: crate::RWRegister<u8>,
    _reserved7: [u8; 0x63],
    #[doc = "Interrupt Status"]
    pub ISTAT: crate::RWRegister<u8>,
    _reserved8: [u8; 0x3],
    #[doc = "Interrupt Enable"]
    pub INTEN: crate::RWRegister<u8>,
    _reserved9: [u8; 0x3],
    #[doc = "Error Interrupt Status"]
    pub ERRSTAT: crate::RWRegister<u8>,
    _reserved10: [u8; 0x3],
    #[doc = "Error Interrupt Enable"]
    pub ERREN: crate::RWRegister<u8>,
    _reserved11: [u8; 0x3],
    #[doc = "Status"]
    pub STAT: crate::RWRegister<u8>,
    _reserved12: [u8; 0x3],
    #[doc = "Control"]
    pub CTL: crate::RWRegister<u8>,
    _reserved13: [u8; 0x3],
    #[doc = "Address"]
    pub ADDR: crate::RWRegister<u8>,
    _reserved14: [u8; 0x3],
    #[doc = "BDT Page 1"]
    pub BDTPAGE1: crate::RWRegister<u8>,
    _reserved15: [u8; 0x3],
    #[doc = "Frame Number Register Low"]
    pub FRMNUML: crate::RWRegister<u8>,
    _reserved16: [u8; 0x3],
    #[doc = "Frame Number Register High"]
    pub FRMNUMH: crate::RWRegister<u8>,
    _reserved17: [u8; 0x3],
    #[doc = "Token"]
    pub TOKEN: crate::RWRegister<u8>,
    _reserved18: [u8; 0x3],
    #[doc = "SOF Threshold"]
    pub SOFTHLD: crate::RWRegister<u8>,
    _reserved19: [u8; 0x3],
    #[doc = "BDT Page 2"]
    pub BDTPAGE2: crate::RWRegister<u8>,
    _reserved20: [u8; 0x3],
    #[doc = "BDT Page 3"]
    pub BDTPAGE3: crate::RWRegister<u8>,
    _reserved21: [u8; 0xb],
    #[doc = "no description available"]
    pub ENDPOINT: [endpoint::RegisterBlock; 16usize],
    #[doc = "USB Control"]
    pub USBCTRL: crate::RWRegister<u8>,
    _reserved22: [u8; 0x3],
    #[doc = "USB OTG Observe"]
    pub OBSERVE: crate::RWRegister<u8>,
    _reserved23: [u8; 0x3],
    #[doc = "USB OTG Control"]
    pub CONTROL: crate::RWRegister<u8>,
    _reserved24: [u8; 0x3],
    #[doc = "USB Transceiver Control 0"]
    pub USBTRC0: crate::RWRegister<u8>,
    _reserved25: [u8; 0x7],
    #[doc = "Frame Adjust"]
    pub USBFRMADJUST: crate::RWRegister<u8>,
    _reserved26: [u8; 0xf],
    #[doc = "Keep Alive Mode Control"]
    pub KEEP_ALIVE_CTRL: crate::RWRegister<u8>,
    _reserved27: [u8; 0x3],
    #[doc = "Keep Alive Mode Wakeup Control"]
    pub KEEP_ALIVE_WKCTRL: crate::RWRegister<u8>,
    _reserved28: [u8; 0x3],
    #[doc = "Miscellaneous Control"]
    pub MISCCTRL: crate::RWRegister<u8>,
    _reserved29: [u8; 0x3],
    #[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in IN Direction"]
    pub STALL_IL_DIS: crate::RWRegister<u8>,
    _reserved30: [u8; 0x3],
    #[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in IN Direction"]
    pub STALL_IH_DIS: crate::RWRegister<u8>,
    _reserved31: [u8; 0x3],
    #[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in OUT Direction"]
    pub STALL_OL_DIS: crate::RWRegister<u8>,
    _reserved32: [u8; 0x3],
    #[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in OUT Direction"]
    pub STALL_OH_DIS: crate::RWRegister<u8>,
    _reserved33: [u8; 0x3],
    #[doc = "USB Clock Recovery Control"]
    pub CLK_RECOVER_CTRL: crate::RWRegister<u8>,
    _reserved34: [u8; 0x3],
    #[doc = "FIRC Oscillator Enable"]
    pub CLK_RECOVER_IRC_EN: crate::RWRegister<u8>,
    _reserved35: [u8; 0xf],
    #[doc = "Clock Recovery Combined Interrupt Enable"]
    pub CLK_RECOVER_INT_EN: crate::RWRegister<u8>,
    _reserved36: [u8; 0x7],
    #[doc = "Clock Recovery Separated Interrupt Status"]
    pub CLK_RECOVER_INT_STATUS: crate::RWRegister<u8>,
}
#[doc = "Peripheral ID"]
pub mod PERID {
    #[doc = "Peripheral Identification"]
    pub mod ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Peripheral ID Complement"]
pub mod IDCOMP {
    #[doc = "Negative Peripheral ID"]
    pub mod NID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Peripheral Revision"]
pub mod REV {
    #[doc = "Revision"]
    pub mod REV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Peripheral Additional Information"]
pub mod ADDINFO {
    #[doc = "Host Mode Enable"]
    pub mod IEHOST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTG Interrupt Status"]
pub mod OTGISTAT {
    #[doc = "Line State Change Interrupt Flag"]
    pub mod LINE_STATE_CHG {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt did not occur"]
            pub const INT_NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "One Millisecond Timer Timeout Flag"]
    pub mod ONEMSEC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not timed out"]
            pub const INT_NO: u32 = 0;
            #[doc = "Timed out"]
            pub const INT_YES: u32 = 1;
        }
    }
}
#[doc = "OTG Interrupt Control"]
pub mod OTGICR {
    #[doc = "Line State Change Interrupt Enable"]
    pub mod LINESTATEEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_LINEST_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_LINEST_INT: u32 = 1;
        }
    }
    #[doc = "1-Millisecond Interrupt Enable"]
    pub mod ONEMSECEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_TIMER_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_TIMER_INT: u32 = 1;
        }
    }
}
#[doc = "OTG Status"]
pub mod OTGSTAT {
    #[doc = "Line State Stable"]
    pub mod LINESTATESTABLE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Unstable"]
            pub const LINEST_NOT_STABLE: u32 = 0;
            #[doc = "Stable"]
            pub const LINEST_STABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reserved for 1 ms count"]
    pub mod ONEMSEC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "OTG Control"]
pub mod OTGCTL {
    #[doc = "On-The-Go Pullup and Pulldown Resistor Enable"]
    pub mod OTGEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "If USBENSOFEN is 1 and HOSTMODEEN is 0 in the Control Register (CTL), then the D+ Data line pullup resistors are enabled. If HOSTMODEEN is 1, then the D+ and D- Data line pulldown resistors are engaged."]
            pub const CONFIG_RESISTORS_CTL: u32 = 0;
            #[doc = "Uses the pullup and pulldown controls in this register."]
            pub const CONFIG_RESISTORS_HERE: u32 = 1;
        }
    }
    #[doc = "D- Data Line Pulldown Resistor Enable"]
    pub mod DMLOW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_DM_PULLDOWN: u32 = 0;
            #[doc = "Enable"]
            pub const EN_DM_PULLDOWN: u32 = 1;
        }
    }
    #[doc = "D+ Data Line pulldown Resistor Enable"]
    pub mod DPLOW {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_DP_PULLDOWN: u32 = 0;
            #[doc = "Enable"]
            pub const EN_DP_PULLDOWN: u32 = 1;
        }
    }
    #[doc = "D+ Data Line Pullup Resistor Enable"]
    pub mod DPHIGH {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_DP_PULLUP: u32 = 0;
            #[doc = "Enable"]
            pub const EN_DP_PULLUP: u32 = 1;
        }
    }
}
#[doc = "Interrupt Status"]
pub mod ISTAT {
    #[doc = "USB Reset Flag"]
    pub mod USBRST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const INT_NO: u32 = 0;
            #[doc = "Detected"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "Error Flag"]
    pub mod ERROR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Error did not occur"]
            pub const INT_NO: u32 = 0;
            #[doc = "Error occurred"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "Start Of Frame (SOF) Token Flag"]
    pub mod SOFTOK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not receive"]
            pub const INT_NO: u32 = 0;
            #[doc = "Received"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "Current Token Processing Flag"]
    pub mod TOKDNE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not processed"]
            pub const INT_NO: u32 = 0;
            #[doc = "Processed"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "Sleep Flag"]
    pub mod SLEEP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt did not occur"]
            pub const INT_NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "Resume Flag"]
    pub mod RESUME {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt did not occur"]
            pub const INT_NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "Attach Interrupt Flag"]
    pub mod ATTACH {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const INT_NO: u32 = 0;
            #[doc = "Detected"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "Stall Interrupt Flag"]
    pub mod STALL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt did not occur"]
            pub const INT_NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const INT_YES: u32 = 1;
        }
    }
}
#[doc = "Interrupt Enable"]
pub mod INTEN {
    #[doc = "USBRST Interrupt Enable"]
    pub mod USBRSTEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_USBRST_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_USBRST_INT: u32 = 1;
        }
    }
    #[doc = "ERROR Interrupt Enable"]
    pub mod ERROREN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_ERROR_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_ERROR_INT: u32 = 1;
        }
    }
    #[doc = "SOFTOK Interrupt Enable"]
    pub mod SOFTOKEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_SOFTOK_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_SOFTOK_INT: u32 = 1;
        }
    }
    #[doc = "TOKDNE Interrupt Enable"]
    pub mod TOKDNEEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_TOKDNE_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_TOKDNE_INT: u32 = 1;
        }
    }
    #[doc = "SLEEP Interrupt Enable"]
    pub mod SLEEPEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_SLEEP_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_SLEEP_INT: u32 = 1;
        }
    }
    #[doc = "RESUME Interrupt Enable"]
    pub mod RESUMEEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_RESUME_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_RESUME_INT: u32 = 1;
        }
    }
    #[doc = "ATTACH Interrupt Enable"]
    pub mod ATTACHEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_ATTACH_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_ATTACH_INT: u32 = 1;
        }
    }
    #[doc = "STALL Interrupt Enable"]
    pub mod STALLEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_STALL_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_STALL_INT: u32 = 1;
        }
    }
}
#[doc = "Error Interrupt Status"]
pub mod ERRSTAT {
    #[doc = "PID Error Flag"]
    pub mod PIDERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not fail"]
            pub const INT_NO: u32 = 0;
            #[doc = "Failed"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "CRC5 Error or End of Frame Error Flag"]
    pub mod CRC5EOF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt did not occur"]
            pub const INT_NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "CRC16 Error Flag"]
    pub mod CRC16 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not rejected"]
            pub const INT_NO: u32 = 0;
            #[doc = "Rejected"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "Data Field Not 8 Bits Flag"]
    pub mod DFN8 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Integer number of bytes"]
            pub const INT_NO: u32 = 0;
            #[doc = "Not an integer number of bytes"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "Bus Turnaround Timeout Error Flag"]
    pub mod BTOERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not timed out"]
            pub const INT_NO: u32 = 0;
            #[doc = "Timed out"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "DMA Access Error Flag"]
    pub mod DMAERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt did not occur"]
            pub const INT_NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "BD Unavailable Error Flag"]
    pub mod OWNERR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt did not occur"]
            pub const INT_NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const INT_YES: u32 = 1;
        }
    }
    #[doc = "Bit Stuff Error Flag"]
    pub mod BTSERR {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Packet not rejected due to the error"]
            pub const INT_NO: u32 = 0;
            #[doc = "Packet rejected due to the error"]
            pub const INT_YES: u32 = 1;
        }
    }
}
#[doc = "Error Interrupt Enable"]
pub mod ERREN {
    #[doc = "PIDERR Interrupt Enable"]
    pub mod PIDERREN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_PIDERR_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_PIDERR_INT: u32 = 1;
        }
    }
    #[doc = "CRC5/EOF Interrupt Enable"]
    pub mod CRC5EOFEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_CRC5_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_CRC5_INT: u32 = 1;
        }
    }
    #[doc = "CRC16 Interrupt Enable"]
    pub mod CRC16EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_CRC16_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_CRC16_INT: u32 = 1;
        }
    }
    #[doc = "DFN8 (Data Field Not Integer Number of Bytes) Interrupt Enable"]
    pub mod DFN8EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_DFN8_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_DFN8_INT: u32 = 1;
        }
    }
    #[doc = "BTOERR (Bus Timeout Error) Interrupt Enable"]
    pub mod BTOERREN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_BTOERR_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_BTOERR_INT: u32 = 1;
        }
    }
    #[doc = "DMAERR Interrupt Enable"]
    pub mod DMAERREN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_DMAERR_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_DMAERR_INT: u32 = 1;
        }
    }
    #[doc = "OWNERR Interrupt Enable"]
    pub mod OWNERREN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_OWNERR_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_OWNERR_INT: u32 = 1;
        }
    }
    #[doc = "BTSERR (Bit Stuff Error) Interrupt Enable"]
    pub mod BTSERREN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_BTSERREN_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_BTSERREN_INT: u32 = 1;
        }
    }
}
#[doc = "Status"]
pub mod STAT {
    #[doc = "Odd Bank"]
    pub mod ODD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not in the odd bank"]
            pub const NOT_IN_ODD_BANK: u32 = 0;
            #[doc = "In the odd bank"]
            pub const ODD_BANK: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Indicator"]
    pub mod TX {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive"]
            pub const RX_TRANSACTION: u32 = 0;
            #[doc = "Transmit"]
            pub const TX_TRANSACTION: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Endpoint address"]
    pub mod ENDP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control"]
pub mod CTL {
    #[doc = "USB Enable"]
    pub mod USBENSOFEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_USB_SOF: u32 = 0;
            #[doc = "Enable"]
            pub const EN_USB_SOF: u32 = 1;
        }
    }
    #[doc = "Odd Reset"]
    pub mod ODDRST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Resume"]
    pub mod RESUME {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Host Mode Enable"]
    pub mod HOSTMODEEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "USBFS operates in Device mode."]
            pub const EN_DEVICE_MODE: u32 = 0;
            #[doc = "USBFS operates in Host mode. In Host mode, USBFS performs USB transactions under the programmed control of the host processor."]
            pub const EN_HOST_MODE: u32 = 1;
        }
    }
    #[doc = "Reset Signaling Enable"]
    pub mod RESET {
        pub const offset: u32 = 4;
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
    #[doc = "TXD Suspend And Token Busy"]
    pub mod TXSUSPENDTOKENBUSY {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Live USB Single-Ended Zero signal"]
    pub mod SE0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Live USB Differential Receiver JSTATE Signal"]
    pub mod JSTATE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Address"]
pub mod ADDR {
    #[doc = "USB Address"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low Speed Enable"]
    pub mod LSEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BDT Page 1"]
pub mod BDTPAGE1 {
    #[doc = "BDT Base Address"]
    pub mod BDTBA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frame Number Register Low"]
pub mod FRMNUML {
    #[doc = "Frame Number, Bits 0-7"]
    pub mod FRM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frame Number Register High"]
pub mod FRMNUMH {
    #[doc = "Frame Number, Bits 8-10"]
    pub mod FRM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Token"]
pub mod TOKEN {
    #[doc = "Token Endpoint Address"]
    pub mod TOKENENDPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Token Type"]
    pub mod TOKENPID {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OUT token. USBFS performs an OUT (TX) transaction."]
            pub const EN_OUT_TOKEN: u32 = 1;
            #[doc = "IN token. USBFS performs an IN (RX) transaction."]
            pub const EN_IN_TOKEN: u32 = 9;
            #[doc = "SETUP token. USBFS performs a SETUP (TX) transaction"]
            pub const EN_SETUP_TOKEN: u32 = 13;
        }
    }
}
#[doc = "SOF Threshold"]
pub mod SOFTHLD {
    #[doc = "SOF Count Threshold"]
    pub mod CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BDT Page 2"]
pub mod BDTPAGE2 {
    #[doc = "BDT Base Address"]
    pub mod BDTBA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "BDT Page 3"]
pub mod BDTPAGE3 {
    #[doc = "BDT Base Address"]
    pub mod BDTBA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod endpoint {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "Endpoint Control"]
        pub ENDPT: crate::RWRegister<u8>,
    }
    #[doc = "Endpoint Control"]
    pub mod ENDPT {
        #[doc = "Endpoint Handshaking Enable"]
        pub mod EPHSHK {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Endpoint Stalled"]
        pub mod EPSTALL {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Endpoint for TX transfers enable"]
        pub mod EPTXEN {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Endpoint for RX transfers enable"]
        pub mod EPRXEN {
            pub const offset: u32 = 3;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Control Transfer Disable"]
        pub mod EPCTLDIS {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Enable"]
                pub const ENABLE: u32 = 0;
                #[doc = "Disable"]
                pub const DISABLE: u32 = 1;
            }
        }
        #[doc = "Retry Disable"]
        pub mod RETRYDIS {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Retried NAK\'ed transactions in hardware."]
                pub const RETRIED: u32 = 0;
                #[doc = "Do not retry NAK\'ed transactions. When a transaction is NAK\'ed, the BDT PID field is updated with the NAK PID, and the TOKEN_DNE interrupt becomes 1."]
                pub const DO_NOT_RETRIED: u32 = 1;
            }
        }
        #[doc = "Host Without A Hub"]
        pub mod HOSTWOHUB {
            pub const offset: u32 = 7;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Connected using a hub (USBFS generates PRE_PID as required)"]
                pub const LS_THRU_HUB: u32 = 0;
                #[doc = "Connected directly to host without a hub, or was used to attach"]
                pub const LS_DIRECT_CONNECT: u32 = 1;
            }
        }
    }
}
#[doc = "USB Control"]
pub mod USBCTRL {
    #[doc = "DP and DM Lane Reversal Control"]
    pub mod DPDM_LANE_REVERSE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard USB DP and DM package pin assignment"]
            pub const DP_DM_STANDARD: u32 = 0;
            #[doc = "Reverse roles of USB DP and DM package pins"]
            pub const DP_DM_REVERSED: u32 = 1;
        }
    }
    #[doc = "Host-Mode-Only Low-Speed Device EOP Signaling"]
    pub mod HOST_LS_EOP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Full-speed device or a low-speed device through a hub"]
            pub const HOST_FS_RESUME_EOP: u32 = 0;
            #[doc = "Directly-connected low-speed device"]
            pub const HOST_LS_RESUME_EOP: u32 = 1;
        }
    }
    #[doc = "UART Select"]
    pub mod UARTSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "USB DP and DM external package pins are used for USB signaling."]
            pub const USB_MODE: u32 = 0;
            #[doc = "USB DP and DM external package pins are used for UART signaling."]
            pub const UART_MODE: u32 = 1;
        }
    }
    #[doc = "UART Signal Channel Select"]
    pub mod UARTCHLS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "USB DP and DM signals are used as UART TX/RX."]
            pub const UART_DP_TX: u32 = 0;
            #[doc = "USB DP and DM signals are used as UART RX/TX."]
            pub const UART_DM_TX: u32 = 1;
        }
    }
    #[doc = "Pulldown Enable"]
    pub mod PDE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable on D+ and D-"]
            pub const DIS_PULLDOWNS: u32 = 0;
            #[doc = "Enable on D+ and D-"]
            pub const EN_PULLDOWNS: u32 = 1;
        }
    }
    #[doc = "Suspend"]
    pub mod SUSP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not in Suspend state"]
            pub const XCVR_NOT_SUSPEND: u32 = 0;
            #[doc = "In Suspend state"]
            pub const XCVR_SUSPEND: u32 = 1;
        }
    }
}
#[doc = "USB OTG Observe"]
pub mod OBSERVE {
    #[doc = "D- Pulldown"]
    pub mod DMPD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Disabled"]
            pub const DM_PD_DIS_STAT: u32 = 0;
            #[doc = "Enabled"]
            pub const DM_PD_EN_STAT: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D+ Pulldown"]
    pub mod DPPD {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Disabled"]
            pub const DP_PD_DIS_STAT: u32 = 0;
            #[doc = "Enabled"]
            pub const DP_PD_EN_STAT: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "D+ Pullup"]
    pub mod DPPU {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Disabled"]
            pub const DP_PU_DIS_STAT: u32 = 0;
            #[doc = "Enabled"]
            pub const DP_PU_EN_STAT: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "USB OTG Control"]
pub mod CONTROL {
    #[doc = "VBUS Monitoring Source Select"]
    pub mod VBUS_SOURCE_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Resistive divider attached to a GPIO pin"]
            pub const RESISTIVE: u32 = 1;
        }
    }
    #[doc = "VBUS Session Valid status"]
    pub mod SESS_VLD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Below"]
            pub const SESS_VLD_LOW: u32 = 0;
            #[doc = "Above"]
            pub const SESS_VLD_HIGH: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DP Pullup in Non-OTG Device Mode"]
    pub mod DPPULLUPNONOTG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_DEVICE_DP_PU: u32 = 0;
            #[doc = "Enabled"]
            pub const EN_DEVICE_DP_PU: u32 = 1;
        }
    }
}
#[doc = "USB Transceiver Control 0"]
pub mod USBTRC0 {
    #[doc = "USB Asynchronous Interrupt"]
    pub mod USB_RESUME_INT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not generated"]
            pub const NO_ASYNC_INT: u32 = 0;
            #[doc = "Generated because of the USB asynchronous interrupt"]
            pub const SYNC_INT_GENERATED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Synchronous USB Interrupt Detect"]
    pub mod SYNC_DET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not detected"]
            pub const NO_SYNC_INT: u32 = 0;
            #[doc = "Detected"]
            pub const SYNC_INT_DETECTED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Combined USB Clock Recovery interrupt status"]
    pub mod USB_CLK_RECOVERY_INT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VREGIN Rising Edge Interrupt Detect"]
    pub mod VREDG_DET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not detected"]
            pub const NO_VREG_RE_INT: u32 = 0;
            #[doc = "Detected"]
            pub const VREG_RE_INT_DETECTED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "VREGIN Falling Edge Interrupt Detect"]
    pub mod VFEDG_DET {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not detected"]
            pub const NO_VREG_FE_INT: u32 = 0;
            #[doc = "Detected"]
            pub const VREG_FE_INT_DETECTED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Asynchronous Resume Interrupt Enable"]
    pub mod USBRESMEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_ASYNC_WAKEUP: u32 = 0;
            #[doc = "Enable"]
            pub const EN_ASYNC_WAKEUP: u32 = 1;
        }
    }
    #[doc = "VREGIN Status"]
    pub mod VREGIN_STS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "USB Reset"]
    pub mod USBRESET {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Normal USBFS operation"]
            pub const NORMAL_OPERATION: u32 = 0;
            #[doc = "Returns USBFS to its reset state"]
            pub const FORCE_HARD_RESET: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Frame Adjust"]
pub mod USBFRMADJUST {
    #[doc = "Frame Adjustment"]
    pub mod ADJ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Keep Alive Mode Control"]
pub mod KEEP_ALIVE_CTRL {
    #[doc = "Keep Alive Mode Enable"]
    pub mod KEEP_ALIVE_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Everything remains same as before."]
            pub const DE_ASSERTED: u32 = 0;
            #[doc = "USB shall enter USB_KEEP_ALIVE mode after asserting ipg_stop."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "OWN Bit Override Enable"]
    pub mod OWN_OVERRD_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stop Acknowledge Delay Enable"]
    pub mod STOP_ACK_DLY_EN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enter KEEP_ALIVE mode immediately when there is no USB AHB transfer."]
            pub const ENTER_KEEP_ALIVE_ON_IDLE: u32 = 0;
            #[doc = "Enter KEEP_ALIVE mode until the USB core is idle and there is no USB AHB transfer."]
            pub const ENTER_KEEP_ALIVE_IMMEDIATE: u32 = 1;
        }
    }
    #[doc = "Wakeup Request Enable"]
    pub mod WAKE_REQ_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_WAKE_REQUEST: u32 = 0;
            #[doc = "Enable"]
            pub const EN_WAKE_REQUEST: u32 = 1;
        }
    }
    #[doc = "Wakeup Interrupt Enable"]
    pub mod WAKE_INT_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Keep Alive Status"]
    pub mod KEEP_ALIVE_STS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not in Keep Alive mode"]
            pub const KEEP_ALIVE_DIS_STAT: u32 = 0;
            #[doc = "In Keep Alive mode"]
            pub const KEEP_ALIVE_EN_STAT: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Wakeup Interrupt Status Flag"]
    pub mod WAKE_INT_STS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt did not occur"]
            pub const INT_NO: u32 = 0;
            #[doc = "Interrupt occurred"]
            pub const INT_YES: u32 = 1;
        }
    }
}
#[doc = "Keep Alive Mode Wakeup Control"]
pub mod KEEP_ALIVE_WKCTRL {
    #[doc = "Token PID for the wakeup request"]
    pub mod WAKE_ON_THIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wake up after receiving OUT or SETUP token packet."]
            pub const WAKE_ON_OUT_SETUP: u32 = 1;
            #[doc = "Wake up after receiving SETUP token packet. All other values are reserved."]
            pub const WAKE_ON_SETUP_ONLY: u32 = 13;
        }
    }
    #[doc = "Endpoint address for the wakeup request"]
    pub mod WAKE_ENDPT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Miscellaneous Control"]
pub mod MISCCTRL {
    #[doc = "Dynamic SOF Threshold Compare mode"]
    pub mod SOFDYNTHLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "When the byte-times SOF threshold is reached"]
            pub const USE_DYN_SOF_THRESHOLD: u32 = 0;
            #[doc = "When 8 byte-times SOF threshold is reached or overstepped"]
            pub const USE_FIXED_SOF_THRESHOLD: u32 = 1;
        }
    }
    #[doc = "SOF_TOK Interrupt Generation Mode Select"]
    pub mod SOFBUSSET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "According to the SOF threshold value"]
            pub const SOF_TOK_INT_FROM_THRESHOLD: u32 = 0;
            #[doc = "When the SOF counter reaches 0"]
            pub const SOF_TOK_INT_COUNTER_0: u32 = 1;
        }
    }
    #[doc = "OWN Error Detect for ISO IN and ISO OUT Disable"]
    pub mod OWNERRISODIS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const DIS_OWN_ERROR_DETECT_ISO: u32 = 0;
            #[doc = "Disable"]
            pub const EN_OWN_ERROR_DETECT_ISO: u32 = 1;
        }
    }
    #[doc = "VREGIN Rising Edge Interrupt Enable"]
    pub mod VREDG_EN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_VREGIN_RE_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_VREGIN_RE_INT: u32 = 1;
        }
    }
    #[doc = "VREGIN Falling Edge Interrupt Enable"]
    pub mod VFEDG_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_VREGIN_FE_INT: u32 = 0;
            #[doc = "Enable"]
            pub const EN_VREGIN_FE_INT: u32 = 1;
        }
    }
    #[doc = "USB Peripheral Mode Stall Adjust Enable"]
    pub mod STL_ADJ_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "If ENDPTn[END_STALL] = 1, both IN and OUT directions for the associated endpoint stalls."]
            pub const STALL_BOTH_IN_OUT: u32 = 0;
            #[doc = "If ENDPTn[END_STALL] = 1, the STALL_xx_DIS registers control which directions for the associated endpoint stalls."]
            pub const STALL_SINGLE_DIRECTION: u32 = 1;
        }
    }
}
#[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in IN Direction"]
pub mod STALL_IL_DIS {
    #[doc = "Disable Endpoint 0 IN Direction"]
    pub mod STALL_I_DIS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP0_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP0_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 1 IN Direction"]
    pub mod STALL_I_DIS1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP1_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP1_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 2 IN Direction"]
    pub mod STALL_I_DIS2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP2_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP2_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 3 IN Direction"]
    pub mod STALL_I_DIS3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP3_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP3_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 4 IN Direction"]
    pub mod STALL_I_DIS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP4_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP4_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 5 IN Direction"]
    pub mod STALL_I_DIS5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP5_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP5_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 6 IN Direction"]
    pub mod STALL_I_DIS6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP6_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP6_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 7 IN Direction"]
    pub mod STALL_I_DIS7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP7_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP7_IN_STALL: u32 = 1;
        }
    }
}
#[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in IN Direction"]
pub mod STALL_IH_DIS {
    #[doc = "Disable Endpoint 8 IN Direction"]
    pub mod STALL_I_DIS8 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP8_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP8_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 9 IN Direction"]
    pub mod STALL_I_DIS9 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP9_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP9_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 10 IN Direction"]
    pub mod STALL_I_DIS10 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP10_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP10_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 11 IN Direction"]
    pub mod STALL_I_DIS11 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP11_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP11_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 12 IN Direction"]
    pub mod STALL_I_DIS12 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP12_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP12_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 13 IN Direction"]
    pub mod STALL_I_DIS13 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP13_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP13_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 14 IN Direction"]
    pub mod STALL_I_DIS14 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP14_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP14_IN_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 15 IN Direction"]
    pub mod STALL_I_DIS15 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP15_IN_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP15_IN_STALL: u32 = 1;
        }
    }
}
#[doc = "Peripheral Mode Stall Disable for Endpoints 7 to 0 in OUT Direction"]
pub mod STALL_OL_DIS {
    #[doc = "Disable Endpoint 0 OUT Direction"]
    pub mod STALL_O_DIS0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP0_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP0_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 1 OUT Direction"]
    pub mod STALL_O_DIS1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP1_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP1_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 2 OUT Direction"]
    pub mod STALL_O_DIS2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP2_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP2_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 3 OUT Direction"]
    pub mod STALL_O_DIS3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP3_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP3_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 4 OUT Direction"]
    pub mod STALL_O_DIS4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP4_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP4_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 5 OUT Direction"]
    pub mod STALL_O_DIS5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP5_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP5_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 6 OUT Direction"]
    pub mod STALL_O_DIS6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP6_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP6_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 7 OUT Direction"]
    pub mod STALL_O_DIS7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP7_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP7_OUT_STALL: u32 = 1;
        }
    }
}
#[doc = "Peripheral Mode Stall Disable for Endpoints 15 to 8 in OUT Direction"]
pub mod STALL_OH_DIS {
    #[doc = "Disable Endpoint 8 OUT Direction"]
    pub mod STALL_O_DIS8 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP8_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP8_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 9 OUT Direction"]
    pub mod STALL_O_DIS9 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP9_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP9_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 10 OUT Direction"]
    pub mod STALL_O_DIS10 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP10_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP10_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 11 OUT Direction"]
    pub mod STALL_O_DIS11 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP11_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP11_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable endpoint 12 OUT direction"]
    pub mod STALL_O_DIS12 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP12_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP12_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 13 OUT Direction"]
    pub mod STALL_O_DIS13 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP13_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP13_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 14 OUT Direction"]
    pub mod STALL_O_DIS14 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP14_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP14_OUT_STALL: u32 = 1;
        }
    }
    #[doc = "Disable Endpoint 15 OUT Direction"]
    pub mod STALL_O_DIS15 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const EN_EP15_OUT_STALL: u32 = 0;
            #[doc = "Disable"]
            pub const DIS_EP15_OUT_STALL: u32 = 1;
        }
    }
}
#[doc = "USB Clock Recovery Control"]
pub mod CLK_RECOVER_CTRL {
    #[doc = "Selects the source for the initial FIRC trim fine value used after a reset."]
    pub mod TRIM_INIT_VAL_SEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Mid-scale"]
            pub const INIT_TRIM_FINE_MID: u32 = 0;
            #[doc = "IFR"]
            pub const INIT_TRIM_FINE_IFR: u32 = 1;
        }
    }
    #[doc = "Restart from IFR Trim Value"]
    pub mod RESTART_IFRTRIM_EN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trim fine adjustment always works based on the previous updated trim fine value."]
            pub const LOAD_TRIM_FINE_MID: u32 = 0;
            #[doc = "Trim fine restarts from the IFR trim value whenever you detect bus_reset or bus_resume or deassert module enable."]
            pub const LOAD_TRIM_FINE_IFR: u32 = 1;
        }
    }
    #[doc = "Reset or Resume to Rough Phase Enable"]
    pub mod RESET_RESUME_ROUGH_EN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Always works in tracking phase after the first time rough phase, to track transition."]
            pub const KEEP_TRIM_FINE_ON_RESET: u32 = 0;
            #[doc = "Go back to rough stage whenever a bus reset or bus resume occurs."]
            pub const USE_IFR_TRIM_FINE_ON_RESET: u32 = 1;
        }
    }
    #[doc = "Crystal-Less USB Enable"]
    pub mod CLOCK_RECOVER_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_CLK_RECOVER: u32 = 0;
            #[doc = "Enable"]
            pub const EN_CLK_RECOVER: u32 = 1;
        }
    }
}
#[doc = "FIRC Oscillator Enable"]
pub mod CLK_RECOVER_IRC_EN {
    #[doc = "Fast IRC enable"]
    pub mod IRC_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS_IRC: u32 = 0;
            #[doc = "Enable"]
            pub const EN_IRC: u32 = 1;
        }
    }
}
#[doc = "Clock Recovery Combined Interrupt Enable"]
pub mod CLK_RECOVER_INT_EN {
    #[doc = "Overflow error interrupt enable"]
    pub mod OVF_ERROR_EN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The interrupt is masked"]
            pub const MASK_OVF_ERR_INT: u32 = 0;
            #[doc = "The interrupt is enabled"]
            pub const EN_OVF_ERR_INT: u32 = 1;
        }
    }
}
#[doc = "Clock Recovery Separated Interrupt Status"]
pub mod CLK_RECOVER_INT_STATUS {
    #[doc = "Overflow Error Interrupt Status Flag"]
    pub mod OVF_ERROR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt did not occur"]
            pub const INT_NO: u32 = 0;
            #[doc = "Unmasked interrupt occurred"]
            pub const INT_YES: u32 = 1;
        }
    }
}
