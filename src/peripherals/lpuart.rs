#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "LPUART"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RWRegister<u32>,
    #[doc = "Global"]
    pub GLOBAL: crate::RWRegister<u32>,
    #[doc = "Pin Configuration"]
    pub PINCFG: crate::RWRegister<u32>,
    #[doc = "Baud Rate"]
    pub BAUD: crate::RWRegister<u32>,
    #[doc = "Status"]
    pub STAT: crate::RWRegister<u32>,
    #[doc = "Control"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Data"]
    pub DATA: crate::RWRegister<u32>,
    #[doc = "Match Address"]
    pub MATCH: crate::RWRegister<u32>,
    #[doc = "MODEM IrDA"]
    pub MODIR: crate::RWRegister<u32>,
    #[doc = "FIFO"]
    pub FIFO: crate::RWRegister<u32>,
    #[doc = "Watermark"]
    pub WATER: crate::RWRegister<u32>,
    #[doc = "Data Read-Only"]
    pub DATARO: crate::RWRegister<u32>,
    _reserved0: [u8; 0xc],
    #[doc = "MODEM Control"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "MODEM Status"]
    pub MSR: crate::RWRegister<u32>,
    #[doc = "Receiver Extended Idle"]
    pub REIR: crate::RWRegister<u32>,
    #[doc = "Transmitter Extended Idle"]
    pub TEIR: crate::RWRegister<u32>,
    #[doc = "Half Duplex Control"]
    pub HDCR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "Timeout Control"]
    pub TOCR: crate::RWRegister<u32>,
    #[doc = "Timeout Status"]
    pub TOSR: crate::RWRegister<u32>,
    #[doc = "Timeout N"]
    pub TIMEOUT: [crate::RWRegister<u32>; 4usize],
    _reserved2: [u8; 0x190],
    #[doc = "Transmit Command Burst"]
    pub TCBR: [crate::RWRegister<u32>; 128usize],
    #[doc = "Transmit Data Burst"]
    pub TDBR: [crate::RWRegister<u32>; 256usize],
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Identification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {
            #[doc = "Standard feature set"]
            pub const STANDARD: u32 = 1;
            #[doc = "Standard feature set with MODEM and IrDA support"]
            pub const MODEM: u32 = 3;
            #[doc = "Enhanced feature set with full MODEM, IrDA, and enhanced idle detection"]
            pub const MODEM_IDLE: u32 = 7;
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
    #[doc = "Transmit FIFO Size"]
    pub mod TXFIFO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Size"]
    pub mod RXFIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Global"]
pub mod GLOBAL {
    #[doc = "Software Reset"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not reset"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Reset"]
            pub const RESET: u32 = 1;
        }
    }
}
#[doc = "Pin Configuration"]
pub mod PINCFG {
    #[doc = "Trigger Select"]
    pub mod TRGSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input trigger disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Input trigger used instead of the RXD pin input"]
            pub const TRG_RXD: u32 = 1;
            #[doc = "Input trigger used instead of the CTS_B pin input"]
            pub const TRG_CTS: u32 = 2;
            #[doc = "Input trigger used to modulate the TXD pin output, which (after TXINV configuration) is internally ANDed with the input trigger"]
            pub const TRG_TXD: u32 = 3;
        }
    }
}
#[doc = "Baud Rate"]
pub mod BAUD {
    #[doc = "Baud Rate Modulo Divisor"]
    pub mod SBR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stop Bit Number Select"]
    pub mod SBNS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "One stop bit"]
            pub const ONE: u32 = 0;
            #[doc = "Two stop bits"]
            pub const TWO: u32 = 1;
        }
    }
    #[doc = "RX Input Active Edge Interrupt Enable"]
    pub mod RXEDGIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables hardware interrupts from STAT[RXEDGIF]"]
            pub const DISABLE: u32 = 0;
            #[doc = "Requests hardware interrupts when STAT[RXEDGIF] is 1"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "LIN Break Detect Interrupt Enable"]
    pub mod LBKDIE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables hardware interrupts from STAT[LBKDIF] (uses polling)"]
            pub const DISABLE: u32 = 0;
            #[doc = "Requests hardware interrupt when STAT[LBKDIF] is 1"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Resynchronization Disable"]
    pub mod RESYNCDIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables resynchronization"]
            pub const RESYNC: u32 = 0;
            #[doc = "Disables resynchronization"]
            pub const NO_RESYNC: u32 = 1;
        }
    }
    #[doc = "Both Edge Sampling"]
    pub mod BOTHEDGE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver samples input data using the rising edge of the baud rate clock"]
            pub const DISABLED: u32 = 0;
            #[doc = "Receiver samples input data using the rising and falling edges of the baud rate clock"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Match Configuration"]
    pub mod MATCFG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address match wakeup"]
            pub const ADDR_MATCH: u32 = 0;
            #[doc = "Idle match wakeup"]
            pub const IDLE_MATCH: u32 = 1;
            #[doc = "Match on and match off"]
            pub const ONOFF_MATCH: u32 = 2;
            #[doc = "Enables RWU on data match and match on/off for the transmitter CTS input"]
            pub const RWU_MATCH: u32 = 3;
        }
    }
    #[doc = "Receiver Idle DMA Enable"]
    pub mod RIDMAE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "DMA request enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Receiver Full DMA Enable"]
    pub mod RDMAE {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables DMA request"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables DMA request"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Transmitter DMA Enable"]
    pub mod TDMAE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables DMA request"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables DMA request"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Oversampling Ratio (OSR)"]
    pub mod OSR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Results in an OSR of 16"]
            pub const DEFAULT: u32 = 0;
            #[doc = "Results in an OSR of 4 (requires BAUD[BOTHEDGE] to be 1)"]
            pub const OSR_4: u32 = 3;
            #[doc = "Results in an OSR of 5 (requires BAUD[BOTHEDGE] to be 1)"]
            pub const OSR_5: u32 = 4;
            #[doc = "Results in an OSR of 6 (requires BAUD[BOTHEDGE] to be 1)"]
            pub const OSR_6: u32 = 5;
            #[doc = "Results in an OSR of 7 (requires BAUD[BOTHEDGE] to be 1)"]
            pub const OSR_7: u32 = 6;
            #[doc = "Results in an OSR of 8"]
            pub const OSR_8: u32 = 7;
            #[doc = "Results in an OSR of 9"]
            pub const OSR_9: u32 = 8;
            #[doc = "Results in an OSR of 10"]
            pub const OSR_10: u32 = 9;
            #[doc = "Results in an OSR of 11"]
            pub const OSR_11: u32 = 10;
            #[doc = "Results in an OSR of 12"]
            pub const OSR_12: u32 = 11;
            #[doc = "Results in an OSR of 13"]
            pub const OSR_13: u32 = 12;
            #[doc = "Results in an OSR of 14"]
            pub const OSR_14: u32 = 13;
            #[doc = "Results in an OSR of 15"]
            pub const OSR_15: u32 = 14;
            #[doc = "Results in an OSR of 16"]
            pub const OSR_16: u32 = 15;
            #[doc = "Results in an OSR of 17"]
            pub const OSR_17: u32 = 16;
            #[doc = "Results in an OSR of 18"]
            pub const OSR_18: u32 = 17;
            #[doc = "Results in an OSR of 19"]
            pub const OSR_19: u32 = 18;
            #[doc = "Results in an OSR of 20"]
            pub const OSR_20: u32 = 19;
            #[doc = "Results in an OSR of 21"]
            pub const OSR_21: u32 = 20;
            #[doc = "Results in an OSR of 22"]
            pub const OSR_22: u32 = 21;
            #[doc = "Results in an OSR of 23"]
            pub const OSR_23: u32 = 22;
            #[doc = "Results in an OSR of 24"]
            pub const OSR_24: u32 = 23;
            #[doc = "Results in an OSR of 25"]
            pub const OSR_25: u32 = 24;
            #[doc = "Results in an OSR of 26"]
            pub const OSR_26: u32 = 25;
            #[doc = "Results in an OSR of 27"]
            pub const OSR_27: u32 = 26;
            #[doc = "Results in an OSR of 28"]
            pub const OSR_28: u32 = 27;
            #[doc = "Results in an OSR of 29"]
            pub const OSR_29: u32 = 28;
            #[doc = "Results in an OSR of 30"]
            pub const OSR_30: u32 = 29;
            #[doc = "Results in an OSR of 31"]
            pub const OSR_31: u32 = 30;
            #[doc = "Results in an OSR of 32"]
            pub const OSR_32: u32 = 31;
        }
    }
    #[doc = "10-Bit Mode Select"]
    pub mod M10 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters"]
            pub const DISABLED: u32 = 0;
            #[doc = "Receiver and transmitter use 10-bit data characters"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Match Address Mode Enable 2"]
    pub mod MAEN2 {
        pub const offset: u32 = 30;
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
    #[doc = "Match Address Mode Enable 1"]
    pub mod MAEN1 {
        pub const offset: u32 = 31;
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
}
#[doc = "Status"]
pub mod STAT {
    #[doc = "LIN Break Flag Enable"]
    pub mod LBKFE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables LIN break detect"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables LIN break detect"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Address Mark Enable"]
    pub mod AME {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address mark in character is MSB"]
            pub const DISABLED: u32 = 0;
            #[doc = "Address mark in character is the last bit before the stop bit (or parity bit when enabled)"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "MODEM Status Flag"]
    pub mod MSF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Field is 0"]
            pub const NOFLAG: u32 = 0;
            #[doc = "Field is 1"]
            pub const FLAG: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout Status Flag"]
    pub mod TSF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Field is 0"]
            pub const NOFLAG: u32 = 0;
            #[doc = "Field is 1"]
            pub const FLAG: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Match 2 Flag"]
    pub mod MA2F {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not equal to MA2"]
            pub const NOMATCH: u32 = 0;
            #[doc = "Equal to MA2"]
            pub const MATCH: u32 = 1;
        }
    }
    #[doc = "Match 1 Flag"]
    pub mod MA1F {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not equal to MA1"]
            pub const NOMATCH: u32 = 0;
            #[doc = "Equal to MA1"]
            pub const MATCH: u32 = 1;
        }
    }
    #[doc = "Parity Error Flag (PF)"]
    pub mod PF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No parity error detected"]
            pub const NOPARITY: u32 = 0;
            #[doc = "Parity error detected"]
            pub const PARITY: u32 = 1;
        }
    }
    #[doc = "Framing Error Flag (FE)"]
    pub mod FE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No framing error detected (this does not guarantee that the framing is correct)"]
            pub const NOERROR: u32 = 0;
            #[doc = "Framing error detected"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Noise Flag (NF)"]
    pub mod NF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No noise detected"]
            pub const NONOISE: u32 = 0;
            #[doc = "Noise detected"]
            pub const NOISE: u32 = 1;
        }
    }
    #[doc = "Receiver Overrun Flag"]
    pub mod OR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overrun"]
            pub const NO_OVERRUN: u32 = 0;
            #[doc = "Receive overrun (new LPUART data lost)"]
            pub const OVERRUN: u32 = 1;
        }
    }
    #[doc = "Idle Line Flag"]
    pub mod IDLE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No idle line detected"]
            pub const NOIDLE: u32 = 0;
            #[doc = "Idle line detected"]
            pub const IDLE: u32 = 1;
        }
    }
    #[doc = "Receive Data Register Full Flag"]
    pub mod RDRF {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Equal to or less than watermark"]
            pub const NO_RXDATA: u32 = 0;
            #[doc = "Greater than watermark"]
            pub const RXDATA: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmission Complete Flag"]
    pub mod TC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmitter active (sending data, a preamble, or a break)"]
            pub const ACTIVE: u32 = 0;
            #[doc = "Transmitter idle (transmission activity complete)"]
            pub const COMPLETE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Data Register Empty Flag"]
    pub mod TDRE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Greater than watermark"]
            pub const TXDATA: u32 = 0;
            #[doc = "Equal to or less than watermark"]
            pub const NO_TXDATA: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receiver Active Flag"]
    pub mod RAF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Idle, waiting for a start bit"]
            pub const IDLE: u32 = 0;
            #[doc = "Receiver active (RXD pin input not idle)"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LIN Break Detection Enable"]
    pub mod LBKDE {
        pub const offset: u32 = 25;
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
    #[doc = "Break Character Generation Length"]
    pub mod BRK13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "9 to 13 bit times"]
            pub const SHORT: u32 = 0;
            #[doc = "12 to 15 bit times"]
            pub const LONG: u32 = 1;
        }
    }
    #[doc = "Receive Wake Up Idle Detect"]
    pub mod RWUID {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "STAT[IDLE] does not become 1"]
            pub const IDLE_NOTSET: u32 = 0;
            #[doc = "STAT[IDLE] becomes 1"]
            pub const IDLE_SET: u32 = 1;
        }
    }
    #[doc = "Receive Data Inversion"]
    pub mod RXINV {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inverted"]
            pub const NOT_INVERTED: u32 = 0;
            #[doc = "Not inverted"]
            pub const INVERTED: u32 = 1;
        }
    }
    #[doc = "MSB First"]
    pub mod MSBF {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LSB"]
            pub const LSB_FIRST: u32 = 0;
            #[doc = "MSB"]
            pub const MSB_FIRST: u32 = 1;
        }
    }
    #[doc = "RXD Pin Active Edge Interrupt Flag"]
    pub mod RXEDGIF {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not occurred"]
            pub const NO_EDGE: u32 = 0;
            #[doc = "Occurred"]
            pub const EDGE: u32 = 1;
        }
    }
    #[doc = "LIN Break Detect Interrupt Flag"]
    pub mod LBKDIF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const NOT_DETECTED: u32 = 0;
            #[doc = "Detected"]
            pub const DETECTED: u32 = 1;
        }
    }
}
#[doc = "Control"]
pub mod CTRL {
    #[doc = "Parity Type"]
    pub mod PT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Even parity"]
            pub const EVEN: u32 = 0;
            #[doc = "Odd parity"]
            pub const ODD: u32 = 1;
        }
    }
    #[doc = "Parity Enable"]
    pub mod PE {
        pub const offset: u32 = 1;
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
    #[doc = "Idle Line Type Select"]
    pub mod ILT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "After the start bit"]
            pub const FROM_START: u32 = 0;
            #[doc = "After the stop bit"]
            pub const FROM_STOP: u32 = 1;
        }
    }
    #[doc = "Receiver Wakeup Method Select"]
    pub mod WAKE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configures CTRL[RWU] for idle-line wakeup"]
            pub const IDLE: u32 = 0;
            #[doc = "Configures CTRL[RWU] with address-mark wakeup"]
            pub const MARK: u32 = 1;
        }
    }
    #[doc = "9-Bit Or 8-Bit Mode Select"]
    pub mod M {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8-bit data characters"]
            pub const DATA8: u32 = 0;
            #[doc = "9-bit data characters"]
            pub const DATA9: u32 = 1;
        }
    }
    #[doc = "Receiver Source Select"]
    pub mod RSRC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal Loopback mode"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Single-wire mode"]
            pub const ONEWIRE: u32 = 1;
        }
    }
    #[doc = "Enables LPUART in Doze mode."]
    pub mod DOZEEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables"]
            pub const ENABLED: u32 = 0;
            #[doc = "Disables"]
            pub const DISABLED: u32 = 1;
        }
    }
    #[doc = "Loop Mode Select"]
    pub mod LOOPS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation: RXD and TXD use separate pins"]
            pub const NOFFECT: u32 = 0;
            #[doc = "Loop mode or Single-Wire mode"]
            pub const LOOPBACK: u32 = 1;
        }
    }
    #[doc = "Idle Configuration"]
    pub mod IDLECFG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const IDLE_1: u32 = 0;
            #[doc = "2"]
            pub const IDLE_2: u32 = 1;
            #[doc = "4"]
            pub const IDLE_4: u32 = 2;
            #[doc = "8"]
            pub const IDLE_8: u32 = 3;
            #[doc = "16"]
            pub const IDLE_16: u32 = 4;
            #[doc = "32"]
            pub const IDLE_32: u32 = 5;
            #[doc = "64"]
            pub const IDLE_64: u32 = 6;
            #[doc = "128"]
            pub const IDLE_128: u32 = 7;
        }
    }
    #[doc = "7-Bit Mode Select"]
    pub mod M7 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8-bit to 10-bit data characters"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "7-bit data characters"]
            pub const DATA7: u32 = 1;
        }
    }
    #[doc = "Match 2 (MA2F) Interrupt Enable"]
    pub mod MA2IE {
        pub const offset: u32 = 14;
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
    #[doc = "Match 1 (MA1F) Interrupt Enable"]
    pub mod MA1IE {
        pub const offset: u32 = 15;
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
    #[doc = "Send Break"]
    pub mod SBK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal transmitter operation"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Queue break character(s) to be sent"]
            pub const TX_BREAK: u32 = 1;
        }
    }
    #[doc = "Receiver Wakeup Control"]
    pub mod RWU {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal receiver operation"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "LPUART receiver in standby, waiting for a wakeup condition"]
            pub const RX_WAKEUP: u32 = 1;
        }
    }
    #[doc = "Receiver Enable"]
    pub mod RE {
        pub const offset: u32 = 18;
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
    #[doc = "Transmitter Enable"]
    pub mod TE {
        pub const offset: u32 = 19;
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
    #[doc = "Idle Line Interrupt Enable"]
    pub mod ILIE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables hardware interrupts from STAT[IDLE]; use polling"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables hardware interrupts when STAT[IDLE] = 1"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Receiver Interrupt Enable"]
    pub mod RIE {
        pub const offset: u32 = 21;
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
    #[doc = "Transmission Complete Interrupt Enable"]
    pub mod TCIE {
        pub const offset: u32 = 22;
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
    #[doc = "Transmit Interrupt Enable"]
    pub mod TIE {
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
    #[doc = "Parity Error Interrupt Enable"]
    pub mod PEIE {
        pub const offset: u32 = 24;
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
    #[doc = "Framing Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 25;
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
    #[doc = "Noise Error Interrupt Enable"]
    pub mod NEIE {
        pub const offset: u32 = 26;
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
    #[doc = "Overrun Interrupt Enable"]
    pub mod ORIE {
        pub const offset: u32 = 27;
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
    #[doc = "Transmit Data Inversion"]
    pub mod TXINV {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted"]
            pub const NOT_INVERTED: u32 = 0;
            #[doc = "Inverted"]
            pub const INVERTED: u32 = 1;
        }
    }
    #[doc = "TXD Pin Direction in Single-Wire Mode"]
    pub mod TXDIR {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "TXD pin is an input in Single-Wire mode"]
            pub const TX_INPUT: u32 = 0;
            #[doc = "TXD pin is an output in Single-Wire mode"]
            pub const TX_OUTPUT: u32 = 1;
        }
    }
    #[doc = "Receive Bit 9 Transmit Bit 8"]
    pub mod R9T8 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Bit 8 Transmit Bit 9"]
    pub mod R8T9 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data"]
pub mod DATA {
    #[doc = "Read Receive FIFO Bit 0 Or Write Transmit FIFO Bit 0"]
    pub mod R0T0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Receive FIFO Bit 1 Or Write Transmit FIFO Bit 1"]
    pub mod R1T1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Receive FIFO Bit 2 Or Write Transmit FIFO Bit 2"]
    pub mod R2T2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Receive FIFO Bit 3 Or Write Transmit FIFO Bit 3"]
    pub mod R3T3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Receive FIFO Bit 4 Or Write Transmit FIFO Bit 4"]
    pub mod R4T4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Receive FIFO Bit 5 Or Write Transmit FIFO Bit 5"]
    pub mod R5T5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Receive FIFO Bit 6 Or Write Transmit FIFO Bit 6"]
    pub mod R6T6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Receive FIFO Bit 7 Or Write Transmit FIFO Bit 7"]
    pub mod R7T7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Receive FIFO Bit 8 Or Write Transmit FIFO Bit 8"]
    pub mod R8T8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Receive FIFO Bit 9 Or Write Transmit FIFO Bit 9"]
    pub mod R9T9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LIN Break"]
    pub mod LINBRK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "LIN break not detected or LIN break detect circuitry disabled"]
            pub const NO_BREAK: u32 = 0;
            #[doc = "LIN break detected"]
            pub const BREAK: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Idle Line"]
    pub mod IDLINE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Received was not idle"]
            pub const NO_IDLE: u32 = 0;
            #[doc = "Receiver was idle"]
            pub const IDLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Buffer Empty"]
    pub mod RXEMPT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Contains valid data"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Contains invalid data and is empty"]
            pub const EMPTY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Frame Error Transmit Special Character"]
    pub mod FRETSC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Received without a frame error on reads or transmits a normal character on writes"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Received with a frame error on reads or transmits an idle or break character on writes"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Parity Error"]
    pub mod PARITYE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Received without a parity error"]
            pub const NO_PARITY: u32 = 0;
            #[doc = "Received with a parity error"]
            pub const PARITY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Noisy Data Received"]
    pub mod NOISY {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Received without noise"]
            pub const NO_NOISE: u32 = 0;
            #[doc = "Received with noise"]
            pub const NOISE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Match Address"]
pub mod MATCH {
    #[doc = "Match Address 1"]
    pub mod MA1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Match Address 2"]
    pub mod MA2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MODEM IrDA"]
pub mod MODIR {
    #[doc = "Transmitter CTS Enable"]
    pub mod TXCTSE {
        pub const offset: u32 = 0;
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
    #[doc = "Transmitter RTS Enable"]
    pub mod TXRTSE {
        pub const offset: u32 = 1;
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
    #[doc = "Transmitter RTS Polarity"]
    pub mod TXRTSPOL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter RTS is active low"]
            pub const LOW: u32 = 0;
            #[doc = "Transmitter RTS is active high"]
            pub const HIGH: u32 = 1;
        }
    }
    #[doc = "Receiver RTS Enable"]
    pub mod RXRTSE {
        pub const offset: u32 = 3;
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
    #[doc = "Transmit CTS Configuration"]
    pub mod TXCTSC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sampled at the start of each character"]
            pub const START: u32 = 0;
            #[doc = "Sampled when the transmitter is idle"]
            pub const IDLE: u32 = 1;
        }
    }
    #[doc = "Transmit CTS Source"]
    pub mod TXCTSSRC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CTS_B pin"]
            pub const CTS: u32 = 0;
            #[doc = "An internal connection to the receiver address match result"]
            pub const MATCH: u32 = 1;
        }
    }
    #[doc = "Receive RTS Configuration"]
    pub mod RTSWATER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmitter Narrow Pulse"]
    pub mod TNP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 / OSR"]
            pub const ONE_SAMPLE: u32 = 0;
            #[doc = "2 / OSR"]
            pub const TWO_SAMPLE: u32 = 1;
            #[doc = "3 / OSR"]
            pub const THREE_SAMPLE: u32 = 2;
            #[doc = "4 / OSR"]
            pub const FOUR_SAMPLE: u32 = 3;
        }
    }
    #[doc = "IR Enable"]
    pub mod IREN {
        pub const offset: u32 = 18;
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
}
#[doc = "FIFO"]
pub mod FIFO {
    #[doc = "Receive FIFO Buffer Depth"]
    pub mod RXFIFOSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "Receive FIFO buffer depth = 1 dataword"]
            pub const FIFO_1: u32 = 0;
            #[doc = "Receive FIFO buffer depth = 4 datawords"]
            pub const FIFO_4: u32 = 1;
            #[doc = "Receive FIFO buffer depth = 8 datawords"]
            pub const FIFO_8: u32 = 2;
            #[doc = "Receive FIFO buffer depth = 16 datawords"]
            pub const FIFO_16: u32 = 3;
            #[doc = "Receive FIFO buffer depth = 32 datawords"]
            pub const FIFO_32: u32 = 4;
            #[doc = "Receive FIFO buffer depth = 64 datawords"]
            pub const FIFO_64: u32 = 5;
            #[doc = "Receive FIFO buffer depth = 128 datawords"]
            pub const FIFO_128: u32 = 6;
            #[doc = "Receive FIFO buffer depth = 256 datawords"]
            pub const FIFO_256: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Enable"]
    pub mod RXFE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables; buffer depth is 1"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables; FIFO[RXFIFOSIZE] indicates the buffer depth"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Transmit FIFO Buffer Depth"]
    pub mod TXFIFOSIZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "Transmit FIFO buffer depth = 1 dataword"]
            pub const FIFO_1: u32 = 0;
            #[doc = "Transmit FIFO buffer depth = 4 datawords"]
            pub const FIFO_4: u32 = 1;
            #[doc = "Transmit FIFO buffer depth = 8 datawords"]
            pub const FIFO_8: u32 = 2;
            #[doc = "Transmit FIFO buffer depth = 16 datawords"]
            pub const FIFO_16: u32 = 3;
            #[doc = "Transmit FIFO buffer depth = 32 datawords"]
            pub const FIFO_32: u32 = 4;
            #[doc = "Transmit FIFO buffer depth = 64 datawords"]
            pub const FIFO_64: u32 = 5;
            #[doc = "Transmit FIFO buffer depth = 128 datawords"]
            pub const FIFO_128: u32 = 6;
            #[doc = "Transmit FIFO buffer depth = 256 datawords"]
            pub const FIFO_256: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit FIFO Enable"]
    pub mod TXFE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables; buffer depth is 1"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables; FIFO[TXFIFOSIZE] indicates the buffer depth"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Receive FIFO Underflow Interrupt Enable"]
    pub mod RXUFE {
        pub const offset: u32 = 8;
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
    #[doc = "Transmit FIFO Overflow Interrupt Enable"]
    pub mod TXOFE {
        pub const offset: u32 = 9;
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
    #[doc = "Receiver Idle Empty Enable"]
    pub mod RXIDEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables STAT[RDRF] to become 1 because of partially filled FIFO when the receiver is idle"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enables STAT[RDRF] to become 1 because of partially filled FIFO when the receiver is idle for one character"]
            pub const IDLE_1: u32 = 1;
            #[doc = "Enables STAT[RDRF] to become 1 because of partially filled FIFO when the receiver is idle for two characters"]
            pub const IDLE_2: u32 = 2;
            #[doc = "Enables STAT[RDRF] to become 1 because of partially filled FIFO when the receiver is idle for four characters"]
            pub const IDLE_4: u32 = 3;
            #[doc = "Enables STAT[RDRF] to become 1 because of partially filled FIFO when the receiver is idle for eight characters"]
            pub const IDLE_8: u32 = 4;
            #[doc = "Enables STAT[RDRF] to become 1 because of partially filled FIFO when the receiver is idle for 16 characters"]
            pub const IDLE_16: u32 = 5;
            #[doc = "Enables STAT[RDRF] to become 1 because of partially filled FIFO when the receiver is idle for 32 characters"]
            pub const IDLE_32: u32 = 6;
            #[doc = "Enables STAT[RDRF] to become 1 because of partially filled FIFO when the receiver is idle for 64 characters"]
            pub const IDLE_64: u32 = 7;
        }
    }
    #[doc = "Receive FIFO Flush"]
    pub mod RXFLUSH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "All data is flushed out"]
            pub const RXFIFO_RST: u32 = 1;
        }
    }
    #[doc = "Transmit FIFO Flush"]
    pub mod TXFLUSH {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "All data is flushed out"]
            pub const TXFIFO_RST: u32 = 1;
        }
    }
    #[doc = "Receiver FIFO Underflow Flag"]
    pub mod RXUF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No underflow"]
            pub const NO_UNDERFLOW: u32 = 0;
            #[doc = "Underflow"]
            pub const UNDERFLOW: u32 = 1;
        }
    }
    #[doc = "Transmitter FIFO Overflow Flag"]
    pub mod TXOF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overflow"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "Overflow"]
            pub const OVERFLOW: u32 = 1;
        }
    }
    #[doc = "Receive FIFO Or Buffer Empty"]
    pub mod RXEMPT {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit FIFO Or Buffer Empty"]
    pub mod TXEMPT {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Watermark"]
pub mod WATER {
    #[doc = "Transmit Watermark"]
    pub mod TXWATER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Counter"]
    pub mod TXCOUNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Watermark"]
    pub mod RXWATER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Counter"]
    pub mod RXCOUNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Read-Only"]
pub mod DATARO {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MODEM Control"]
pub mod MCR {
    #[doc = "Clear To Send"]
    pub mod CTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Data Set Ready"]
    pub mod DSR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Ring Indicator"]
    pub mod RIN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Data Carrier Detect"]
    pub mod DCD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Data Terminal Ready"]
    pub mod DTR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default state is logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Default state is logic zero"]
            pub const LOGIC_ZERO: u32 = 1;
        }
    }
    #[doc = "Request To Send"]
    pub mod RTS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Default state is logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Default state is logic zero"]
            pub const LOGIC_ZERO: u32 = 1;
        }
    }
}
#[doc = "MODEM Status"]
pub mod MSR {
    #[doc = "Delta Clear To Send"]
    pub mod DCTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not change state"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Changed state"]
            pub const CHANGE: u32 = 1;
        }
    }
    #[doc = "Delta Data Set Ready"]
    pub mod DDSR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not change state"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Changed state"]
            pub const CHANGE: u32 = 1;
        }
    }
    #[doc = "Delta Ring Indicator"]
    pub mod DRI {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not change state"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Changed state"]
            pub const CHANGE: u32 = 1;
        }
    }
    #[doc = "Delta Data Carrier Detect"]
    pub mod DDCD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not change state"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Changed state"]
            pub const CHANGE: u32 = 1;
        }
    }
    #[doc = "Clear To Send"]
    pub mod CTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The CTS_B pin is logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "The CTS_B pin is logic zero"]
            pub const LOGIC_ZERO: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Set Ready"]
    pub mod DSR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The DSR_B pin is logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "The DSR_B pin is logic zero"]
            pub const LOGIC_ZERO: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ring Indicator"]
    pub mod RIN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The RIN_B pin is logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "The RIN_B pin is logic zero"]
            pub const LOGIC_ZERO: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Carrier Detect"]
    pub mod DCD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The DCD_B pin is logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "The DCD_B pin is logic zero"]
            pub const LOGIC_ZERO: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Receiver Extended Idle"]
pub mod REIR {
    #[doc = "Idle Time"]
    pub mod IDTIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmitter Extended Idle"]
pub mod TEIR {
    #[doc = "Idle Time"]
    pub mod IDTIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Half Duplex Control"]
pub mod HDCR {
    #[doc = "Transmit Stall"]
    pub mod TXSTALL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Does not become busy"]
            pub const RX_ACTIVE: u32 = 1;
        }
    }
    #[doc = "Receive Select"]
    pub mod RXSEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RXD"]
            pub const PIN_RXD: u32 = 0;
            #[doc = "TXD"]
            pub const PIN_TXD: u32 = 1;
        }
    }
    #[doc = "Receive FIFO Write Mask"]
    pub mod RXWRMSK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not mask"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Masks"]
            pub const TX_RTS: u32 = 1;
        }
    }
    #[doc = "Receive Mask"]
    pub mod RXMSK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not mask"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Masks"]
            pub const TX_RTS: u32 = 1;
        }
    }
    #[doc = "RTS Extended"]
    pub mod RTSEXT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timeout Control"]
pub mod TOCR {
    #[doc = "Timeout Enable"]
    pub mod TOEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout Interrupt Enable"]
    pub mod TOIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timeout Status"]
pub mod TOSR {
    #[doc = "Timeout Zero"]
    pub mod TOZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout Flag"]
    pub mod TOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timeout N"]
pub mod TIMEOUT {
    #[doc = "Timeout Value"]
    pub mod TIMEOUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Idle Configuration"]
    pub mod CFG {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Becomes 1 after timeout characters are received"]
            pub const CNT_CHAR: u32 = 0;
            #[doc = "Becomes 1 when idle for timeout bit clocks"]
            pub const CNT_IDLE: u32 = 1;
            #[doc = "Becomes 1 when idle for timeout bit clocks following the next character"]
            pub const CNT_BUSY_IDLE: u32 = 2;
            #[doc = "Becomes 1 when idle for at least timeout bit clocks, but a new character is detected before the extended idle timeout is reached"]
            pub const CNT_CHAR_IDLE: u32 = 3;
        }
    }
}
#[doc = "Transmit Command Burst"]
pub mod TCBR {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Data Burst"]
pub mod TDBR {
    #[doc = "Data0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data1"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data2"]
    pub mod DATA2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data3"]
    pub mod DATA3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
