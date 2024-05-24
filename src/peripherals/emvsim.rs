#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "EMVSIM"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VER_ID: crate::RWRegister<u32>,
    #[doc = "Parameters"]
    pub PARAM: crate::RWRegister<u32>,
    #[doc = "Clock Configuration"]
    pub CLKCFG: crate::RWRegister<u32>,
    #[doc = "Baud Rate Divisor"]
    pub DIVISOR: crate::RWRegister<u32>,
    #[doc = "Control"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Interrupt Mask"]
    pub INT_MASK: crate::RWRegister<u32>,
    #[doc = "Receiver Threshold"]
    pub RX_THD: crate::RWRegister<u32>,
    #[doc = "Transmitter Threshold"]
    pub TX_THD: crate::RWRegister<u32>,
    #[doc = "Receive Status"]
    pub RX_STATUS: crate::RWRegister<u32>,
    #[doc = "Transmitter Status"]
    pub TX_STATUS: crate::RWRegister<u32>,
    #[doc = "Port Control and Status"]
    pub PCSR: crate::RWRegister<u32>,
    #[doc = "Receive Data Read Buffer"]
    pub RX_BUF: crate::RWRegister<u32>,
    #[doc = "Transmit Data Buffer"]
    pub TX_BUF: crate::RWRegister<u32>,
    #[doc = "Transmitter Guard ETU Value"]
    pub TX_GETU: crate::RWRegister<u32>,
    #[doc = "Character Wait Time Value"]
    pub CWT_VAL: crate::RWRegister<u32>,
    #[doc = "Block Wait Time Value"]
    pub BWT_VAL: crate::RWRegister<u32>,
    #[doc = "Block Guard Time Value"]
    pub BGT_VAL: crate::RWRegister<u32>,
    #[doc = "General Purpose Counter 0 Timeout Value"]
    pub GPCNT0_VAL: crate::RWRegister<u32>,
    #[doc = "General Purpose Counter 1 Timeout Value"]
    pub GPCNT1_VAL: crate::RWRegister<u32>,
}
#[doc = "Version ID"]
pub mod VER_ID {
    #[doc = "Version ID"]
    pub mod VER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parameters"]
pub mod PARAM {
    #[doc = "Receive FIFO Depth"]
    pub mod RX_FIFO_DEPTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit FIFO Depth"]
    pub mod TX_FIFO_DEPTH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Clock Configuration"]
pub mod CLKCFG {
    #[doc = "Clock Prescaler Value"]
    pub mod CLK_PRSC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General Purpose Counter 1 Clock Select"]
    pub mod GPCNT1_CLK_SEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable/reset"]
            pub const DISABLED: u32 = 0;
            #[doc = "Card clock"]
            pub const CARDCLK: u32 = 1;
            #[doc = "Receive clock"]
            pub const RXCLK: u32 = 2;
            #[doc = "ETU clock (transmit clock)"]
            pub const TXCLK: u32 = 3;
        }
    }
    #[doc = "General Purpose Counter 0 Clock Select"]
    pub mod GPCNT0_CLK_SEL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable/reset"]
            pub const DISABLED: u32 = 0;
            #[doc = "Card clock"]
            pub const CARDCLK: u32 = 1;
            #[doc = "Receive clock"]
            pub const RXCLK: u32 = 2;
            #[doc = "ETU clock (transmit clock)"]
            pub const TXCLK: u32 = 3;
        }
    }
}
#[doc = "Baud Rate Divisor"]
pub mod DIVISOR {
    #[doc = "Divisor (F/D) Value"]
    pub mod DIVISOR_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
            pub const INVALID_0: u32 = 0;
            #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
            pub const INVALID_1: u32 = 1;
            #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
            pub const INVALID_2: u32 = 2;
            #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
            pub const INVALID_3: u32 = 3;
            #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
            pub const INVALID_4: u32 = 4;
            #[doc = "Divisor value F/D"]
            pub const VALID_5: u32 = 5;
            #[doc = "Divisor value F/D"]
            pub const VALID_6: u32 = 6;
            #[doc = "Divisor value F/D"]
            pub const VALID_7: u32 = 7;
            #[doc = "Divisor value F/D"]
            pub const VALID_8: u32 = 8;
            #[doc = "Divisor value F/D"]
            pub const VALID_9: u32 = 9;
        }
    }
}
#[doc = "Control"]
pub mod CTRL {
    #[doc = "Inverse Convention"]
    pub mod IC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Direct"]
            pub const DIR_CONVENTION: u32 = 0;
            #[doc = "Inverse"]
            pub const INV_CONVENTION: u32 = 1;
        }
    }
    #[doc = "Initial Character Mode"]
    pub mod ICM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Auto NACK Enable"]
    pub mod ANACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Overrun NACK Enable"]
    pub mod ONACK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Flush Receiver"]
    pub mod FLSH_RX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal"]
            pub const NORMALOP: u32 = 0;
            #[doc = "Reset"]
            pub const RESETHOLD: u32 = 1;
        }
    }
    #[doc = "Flush Transmitter"]
    pub mod FLSH_TX {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal"]
            pub const NORMALOP: u32 = 0;
            #[doc = "Reset"]
            pub const RESETHOLD: u32 = 1;
        }
    }
    #[doc = "Software Reset"]
    pub mod SW_RST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal"]
            pub const NORMALOP: u32 = 0;
            #[doc = "Reset"]
            pub const RESETHOLD: u32 = 1;
        }
    }
    #[doc = "Kill Internal Clocks"]
    pub mod KILL_CLOCKS {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INCLK_ENABLED: u32 = 0;
            #[doc = "Disable"]
            pub const INCLK_DISABLED: u32 = 1;
        }
    }
    #[doc = "Doze Enable"]
    pub mod DOZE_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DOZE_GATE: u32 = 0;
            #[doc = "Enable"]
            pub const DOZE_NOGATE: u32 = 1;
        }
    }
    #[doc = "STOP Enable"]
    pub mod STOP_EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const STOP_ALL_CLKS: u32 = 0;
            #[doc = "Enable"]
            pub const ONLY_SCK_ON: u32 = 1;
        }
    }
    #[doc = "Receiver Enable"]
    pub mod RCV_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Transmitter Enable"]
    pub mod XMT_EN {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Receiver 11 ETU Mode Enable"]
    pub mod RCVR_11 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "12 ETU operation"]
            pub const RCVR_12: u32 = 0;
            #[doc = "11 ETU operation"]
            pub const RCVR_11: u32 = 1;
        }
    }
    #[doc = "Receive DMA Enable"]
    pub mod RX_DMA_EN {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not asserted"]
            pub const NO_DMAREAD_REQ: u32 = 0;
            #[doc = "Asserted"]
            pub const DMAREAD_REQ: u32 = 1;
        }
    }
    #[doc = "Transmit DMA Enable"]
    pub mod TX_DMA_EN {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not asserted"]
            pub const NO_DMAWRITE_REQ: u32 = 0;
            #[doc = "Asserted"]
            pub const DMAWRITE_REQ: u32 = 1;
        }
    }
    #[doc = "Invert CRC Output Value Bits"]
    pub mod INV_CRC_VAL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted"]
            pub const NO_INVERT: u32 = 0;
            #[doc = "Inverted"]
            pub const INVERT: u32 = 1;
        }
    }
    #[doc = "CRC Output Value Bit Reversal Or Flip Control"]
    pub mod CRC_OUT_FLIP {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not reversed"]
            pub const NOT_REVERSED: u32 = 0;
            #[doc = "Reversed"]
            pub const REVERSED: u32 = 1;
        }
    }
    #[doc = "CRC Input Byte\'s Bit Reversal Or Flip Control"]
    pub mod CRC_IN_FLIP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not reversed"]
            pub const NOT_REVERSED: u32 = 0;
            #[doc = "Reversed"]
            pub const REVERSED: u32 = 1;
        }
    }
    #[doc = "CWT Counter Enable"]
    pub mod CWT_EN {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "LRC Enable"]
    pub mod LRC_EN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "CRC Enable"]
    pub mod CRC_EN {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Transmit CRC or LRC Enable"]
    pub mod XMT_CRC_LRC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not transmit"]
            pub const NO_CRC_LRC_TX: u32 = 0;
            #[doc = "Transmit"]
            pub const CRC_LRC_TX: u32 = 1;
        }
    }
    #[doc = "Block Wait Time Counter Enable"]
    pub mod BWT_EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Interrupt Mask"]
pub mod INT_MASK {
    #[doc = "Receive Data Threshold Interrupt Mask"]
    pub mod RDT_IM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Transmit Complete Interrupt Mask"]
    pub mod TC_IM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Receive FIFO Overflow Interrupt Mask"]
    pub mod RFO_IM {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Early Transmit Complete Interrupt Mask"]
    pub mod ETC_IM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Transmit FIFO Empty Interrupt Mask"]
    pub mod TFE_IM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Transmit NACK Threshold Interrupt Mask"]
    pub mod TNACK_IM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Transmit FIFO Full Interrupt Mask"]
    pub mod TFF_IM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Transmit Data Threshold Interrupt Mask"]
    pub mod TDT_IM {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "General Purpose Timer 0 Timeout Interrupt Mask"]
    pub mod GPCNT0_IM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Character Wait Time Error Interrupt Mask"]
    pub mod CWT_ERR_IM {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_DISABLED: u32 = 1;
        }
    }
    #[doc = "Receiver NACK Threshold Interrupt Mask"]
    pub mod RNACK_IM {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Block Wait Time Error Interrupt Mask"]
    pub mod BWT_ERR_IM {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Block Guard Time Error Interrupt"]
    pub mod BGT_ERR_IM {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "General Purpose Counter 1 Timeout Interrupt Mask"]
    pub mod GPCNT1_IM {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Receive Data Interrupt Mask"]
    pub mod RX_DATA_IM {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Parity Error Interrupt Mask"]
    pub mod PEF_IM {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Masked"]
            pub const INT_MASKED: u32 = 1;
        }
    }
}
#[doc = "Receiver Threshold"]
pub mod RX_THD {
    #[doc = "Receiver Data Threshold Value"]
    pub mod RDT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receiver NACK Threshold Value"]
    pub mod RNCK_THD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmitter Threshold"]
pub mod TX_THD {
    #[doc = "Transmitter Data Threshold Value"]
    pub mod TDT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmitter NACK Threshold Value"]
    pub mod TNCK_THD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receive Status"]
pub mod RX_STATUS {
    #[doc = "Receive FIFO Overflow Flag"]
    pub mod RFO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overrun error"]
            pub const NO_OVERRUN: u32 = 0;
            #[doc = "Overrun error"]
            pub const OVERFLOW: u32 = 1;
        }
    }
    #[doc = "Receive Data Interrupt Flag"]
    pub mod RX_DATA {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No new byte"]
            pub const NO_BYTE_RX: u32 = 0;
            #[doc = "New byte"]
            pub const BYTE_RX: u32 = 1;
        }
    }
    #[doc = "Receive Data Threshold Interrupt Flag"]
    pub mod RDTF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Less than threshold"]
            pub const LESSTHAN_RXTHRESH: u32 = 0;
            #[doc = "Greater than or equal to threshold"]
            pub const GREATER_EQ_RXTHRESH: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LRC Check OK Flag"]
    pub mod LRC_OK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No match"]
            pub const LRC_NOTOK: u32 = 0;
            #[doc = "Match"]
            pub const LRC_OK: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CRC Check OK Flag"]
    pub mod CRC_OK {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Current CRC value does not match remainder."]
            pub const CRC_NOTOK: u32 = 0;
            #[doc = "Current calculated CRC value matches the expected result."]
            pub const CRC_OK: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Character Wait Time Error Flag"]
    pub mod CWT_ERR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No CWT violation"]
            pub const NO_CWT_ERR: u32 = 0;
            #[doc = "CWT violation"]
            pub const CWT_ERR: u32 = 1;
        }
    }
    #[doc = "Received NACK Threshold Error Flag"]
    pub mod RTE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Less than"]
            pub const LESSTHAN_NACKTHRESH: u32 = 0;
            #[doc = "Equal to"]
            pub const GREATER_EQ_NACKTHRESH: u32 = 1;
        }
    }
    #[doc = "Block Wait Time Error Flag"]
    pub mod BWT_ERR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not exceeded"]
            pub const BWT_ERR_NO: u32 = 0;
            #[doc = "Exceeded"]
            pub const BWT_ERR_YES: u32 = 1;
        }
    }
    #[doc = "Block Guard Time Error Flag"]
    pub mod BGT_ERR {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sufficient"]
            pub const BGT_ERR_SUFFICIENT: u32 = 0;
            #[doc = "Too small"]
            pub const BGT_ERR_TOOSMALL: u32 = 1;
        }
    }
    #[doc = "Parity Error Flag"]
    pub mod PEF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_PARITY_DETECT: u32 = 0;
            #[doc = "Error"]
            pub const PARITY_DETECT: u32 = 1;
        }
    }
    #[doc = "Frame Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_FEF_DETECT: u32 = 0;
            #[doc = "Error"]
            pub const FEF_DETECT: u32 = 1;
        }
    }
    #[doc = "Receive FIFO Write Pointer Value"]
    pub mod RX_WPTR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Byte Count"]
    pub mod RX_CNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "FIFO empty"]
            pub const FIFO_EMPTY: u32 = 0;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmitter Status"]
pub mod TX_STATUS {
    #[doc = "Transmit NACK Threshold Error Flag"]
    pub mod TNTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Threshold not reached"]
            pub const LESSTHAN_NACKTHRESH: u32 = 0;
            #[doc = "Threshold reached"]
            pub const GREATER_EQ_NACKTHRESH: u32 = 1;
        }
    }
    #[doc = "Transmit FIFO Empty Flag"]
    pub mod TFE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not empty"]
            pub const FIFO_EMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const FIFO_NOTEMPTY: u32 = 1;
        }
    }
    #[doc = "Early Transmit Complete Flag"]
    pub mod ETCF {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pending or incomplete"]
            pub const ETX_PENDING: u32 = 0;
            #[doc = "Complete"]
            pub const ETX_COMPLETE: u32 = 1;
        }
    }
    #[doc = "Transmit Complete Flag"]
    pub mod TCF {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pending or incomplete"]
            pub const TX_PENDING: u32 = 0;
            #[doc = "Complete"]
            pub const TX_COMPLETE: u32 = 1;
        }
    }
    #[doc = "Transmit FIFO Full Flag"]
    pub mod TFF {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not full"]
            pub const TX_FIFO_NOTFULL: u32 = 0;
            #[doc = "Full"]
            pub const TX_FIFO_FULL: u32 = 1;
        }
    }
    #[doc = "Transmit Data Threshold Flag"]
    pub mod TDTF {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Threshold exceeded or this field written to 0"]
            pub const LESSTHAN_TXTHRESH: u32 = 0;
            #[doc = "Threshold not exceeded"]
            pub const GREATER_EQ_TXTHRESH: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General Purpose Counter 0 Timeout Flag"]
    pub mod GPCNT0_TO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPCNT0 not reached, or flag cleared"]
            pub const GPCNT0_TO_NOTREACHED: u32 = 0;
            #[doc = "GPCNT0 reached"]
            pub const GPCNT0_TO_REACHED: u32 = 1;
        }
    }
    #[doc = "General Purpose Counter 1 Timeout Flag"]
    pub mod GPCNT1_TO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPCNT1 not reached, or flag cleared"]
            pub const GPCNT1_TO_NOTREACHED: u32 = 0;
            #[doc = "GPCNT1 reached"]
            pub const GPCNT1_TO_REACHED: u32 = 1;
        }
    }
    #[doc = "Transmit FIFO Read Pointer"]
    pub mod TX_RPTR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit FIFO Byte Count"]
    pub mod TX_CNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "FIFO empty"]
            pub const FIFO_EMPTY: u32 = 0;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Port Control and Status"]
pub mod PCSR {
    #[doc = "Auto Power Down Enable"]
    pub mod SAPD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Vcc Enable for Smart Card"]
    pub mod SVCC_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "VCC Enable Polarity Control"]
    pub mod VCCENP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active high"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Active low"]
            pub const ACTIVE_LOW: u32 = 1;
        }
    }
    #[doc = "Reset Smart Card"]
    pub mod SRST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Assert"]
            pub const ASSERTED: u32 = 0;
            #[doc = "Deassert"]
            pub const DE_ASSERTED: u32 = 1;
        }
    }
    #[doc = "Clock Enable for Smart Card"]
    pub mod SCEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Smart Card Clock Stop Polarity"]
    pub mod SCSP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic 0"]
            pub const SCSP_LOGIC0: u32 = 0;
            #[doc = "Logic 1"]
            pub const SCSP_LOGIC1: u32 = 1;
        }
    }
    #[doc = "Auto Power-Down Control"]
    pub mod SPD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Yes"]
            pub const POWERDOWN: u32 = 1;
        }
    }
    #[doc = "Smart Card Presence Detect Interrupt Mask"]
    pub mod SPDIM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0;
            #[doc = "Mask"]
            pub const INT_MASKED: u32 = 1;
        }
    }
    #[doc = "Smart Card Presence Detect Interrupt Flag"]
    pub mod SPDIF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No insertion or removal"]
            pub const NO_INSERT_REMOVE_DETECT: u32 = 0;
            #[doc = "Insertion or removal"]
            pub const INSERT_REMOVE_DETECT: u32 = 1;
        }
    }
    #[doc = "Smart Card Presence Detect Pin Status"]
    pub mod SPDP {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Logic low"]
            pub const LOGIC_LOW: u32 = 0;
            #[doc = "Logic high"]
            pub const LOGIC_HIGH: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SIM Presence Detect Edge Select"]
    pub mod SPDES {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Falling edge"]
            pub const FALLING_EDGE: u32 = 0;
            #[doc = "Rising edge"]
            pub const RISING_EDGE: u32 = 1;
        }
    }
}
#[doc = "Receive Data Read Buffer"]
pub mod RX_BUF {
    #[doc = "Receive Data Byte Read"]
    pub mod RX_BYTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Data Buffer"]
pub mod TX_BUF {
    #[doc = "Transmit Data Byte"]
    pub mod TX_BYTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmitter Guard ETU Value"]
pub mod TX_GETU {
    #[doc = "Transmitter Guard Time Value in ETU"]
    pub mod GETU {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Character Wait Time Value"]
pub mod CWT_VAL {
    #[doc = "Character Wait Time Value"]
    pub mod CWT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Block Wait Time Value"]
pub mod BWT_VAL {
    #[doc = "Block Wait Time Value"]
    pub mod BWT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Block Guard Time Value"]
pub mod BGT_VAL {
    #[doc = "Block Guard Time Value"]
    pub mod BGT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Counter 0 Timeout Value"]
pub mod GPCNT0_VAL {
    #[doc = "General Purpose Counter 0 Timeout Value"]
    pub mod GPCNT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "General Purpose Counter 1 Timeout Value"]
pub mod GPCNT1_VAL {
    #[doc = "General Purpose Counter 1 Timeout Value"]
    pub mod GPCNT1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
