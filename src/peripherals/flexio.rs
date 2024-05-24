#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "FLEXIO"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID Register"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameter Register"]
    pub PARAM: crate::RWRegister<u32>,
    #[doc = "FLEXIO Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Pin State Register"]
    pub PIN: crate::RWRegister<u32>,
    #[doc = "Shifter Status Register"]
    pub SHIFTSTAT: crate::RWRegister<u32>,
    #[doc = "Shifter Error Register"]
    pub SHIFTERR: crate::RWRegister<u32>,
    #[doc = "Timer Status Register"]
    pub TIMSTAT: crate::RWRegister<u32>,
    _reserved0: [u8; 0x4],
    #[doc = "Shifter Status Interrupt Enable"]
    pub SHIFTSIEN: crate::RWRegister<u32>,
    #[doc = "Shifter Error Interrupt Enable"]
    pub SHIFTEIEN: crate::RWRegister<u32>,
    #[doc = "Timer Interrupt Enable Register"]
    pub TIMIEN: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "Shifter Status DMA Enable"]
    pub SHIFTSDEN: crate::RWRegister<u32>,
    _reserved2: [u8; 0x4],
    #[doc = "Timer Status DMA Enable"]
    pub TIMERSDEN: crate::RWRegister<u32>,
    _reserved3: [u8; 0x4],
    #[doc = "Shifter State Register"]
    pub SHIFTSTATE: crate::RWRegister<u32>,
    _reserved4: [u8; 0x4],
    #[doc = "Trigger Status Register"]
    pub TRGSTAT: crate::RWRegister<u32>,
    #[doc = "External Trigger Interrupt Enable Register"]
    pub TRIGIEN: crate::RWRegister<u32>,
    #[doc = "Pin Status Register"]
    pub PINSTAT: crate::RWRegister<u32>,
    #[doc = "Pin Interrupt Enable Register"]
    pub PINIEN: crate::RWRegister<u32>,
    #[doc = "Pin Rising Edge Enable Register"]
    pub PINREN: crate::RWRegister<u32>,
    #[doc = "Pin Falling Edge Enable Register"]
    pub PINFEN: crate::RWRegister<u32>,
    #[doc = "Pin Output Data Register"]
    pub PINOUTD: crate::RWRegister<u32>,
    #[doc = "Pin Output Enable Register"]
    pub PINOUTE: crate::RWRegister<u32>,
    #[doc = "Pin Output Disable Register"]
    pub PINOUTDIS: crate::RWRegister<u32>,
    #[doc = "Pin Output Clear Register"]
    pub PINOUTCLR: crate::RWRegister<u32>,
    #[doc = "Pin Output Set Register"]
    pub PINOUTSET: crate::RWRegister<u32>,
    #[doc = "Pin Output Toggle Register"]
    pub PINOUTTOG: crate::RWRegister<u32>,
    _reserved5: [u8; 0x8],
    #[doc = "Shifter Control N Register"]
    pub SHIFTCTL: [crate::RWRegister<u32>; 8usize],
    _reserved6: [u8; 0x60],
    #[doc = "Shifter Configuration N Register"]
    pub SHIFTCFG: [crate::RWRegister<u32>; 8usize],
    _reserved7: [u8; 0xe0],
    #[doc = "Shifter Buffer N Register"]
    pub SHIFTBUF: [crate::RWRegister<u32>; 8usize],
    _reserved8: [u8; 0x60],
    #[doc = "Shifter Buffer N Bit Swapped Register"]
    pub SHIFTBUFBIS: [crate::RWRegister<u32>; 8usize],
    _reserved9: [u8; 0x60],
    #[doc = "Shifter Buffer N Byte Swapped Register"]
    pub SHIFTBUFBYS: [crate::RWRegister<u32>; 8usize],
    _reserved10: [u8; 0x60],
    #[doc = "Shifter Buffer N Bit Byte Swapped Register"]
    pub SHIFTBUFBBS: [crate::RWRegister<u32>; 8usize],
    _reserved11: [u8; 0x60],
    #[doc = "Timer Control N Register"]
    pub TIMCTL: [crate::RWRegister<u32>; 8usize],
    _reserved12: [u8; 0x60],
    #[doc = "Timer Configuration N Register"]
    pub TIMCFG: [crate::RWRegister<u32>; 8usize],
    _reserved13: [u8; 0x60],
    #[doc = "Timer Compare N Register"]
    pub TIMCMP: [crate::RWRegister<u32>; 8usize],
    _reserved14: [u8; 0x160],
    #[doc = "Shifter Buffer N Nibble Byte Swapped Register"]
    pub SHIFTBUFNBS: [crate::RWRegister<u32>; 8usize],
    _reserved15: [u8; 0x60],
    #[doc = "Shifter Buffer N Halfword Swapped Register"]
    pub SHIFTBUFHWS: [crate::RWRegister<u32>; 8usize],
    _reserved16: [u8; 0x60],
    #[doc = "Shifter Buffer N Nibble Swapped Register"]
    pub SHIFTBUFNIS: [crate::RWRegister<u32>; 8usize],
    _reserved17: [u8; 0x60],
    #[doc = "Shifter Buffer N Odd Even Swapped Register"]
    pub SHIFTBUFOES: [crate::RWRegister<u32>; 8usize],
    _reserved18: [u8; 0x60],
    #[doc = "Shifter Buffer N Even Odd Swapped Register"]
    pub SHIFTBUFEOS: [crate::RWRegister<u32>; 8usize],
    _reserved19: [u8; 0x60],
    #[doc = "Shifter Buffer N Halfword Byte Swapped Register"]
    pub SHIFTBUFHBS: [crate::RWRegister<u32>; 8usize],
}
#[doc = "Version ID Register"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {
            #[doc = "Standard features implemented."]
            pub const STANDARD: u32 = 0;
            #[doc = "Supports state, logic, and parallel modes."]
            pub const STATE_LOGIC_PARALLEL: u32 = 1;
            #[doc = "Supports pin control registers."]
            pub const PINCTRL: u32 = 2;
            #[doc = "Supports state, logic, and parallel modes, plus pin control registers."]
            pub const STATE_LOGIC_PARALLEL_PINCTRL: u32 = 3;
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
#[doc = "Parameter Register"]
pub mod PARAM {
    #[doc = "Shifter Number"]
    pub mod SHIFTER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Number"]
    pub mod TIMER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pin Number"]
    pub mod PIN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger Number"]
    pub mod TRIGGER {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FLEXIO Control Register"]
pub mod CTRL {
    #[doc = "FLEXIO Enable"]
    pub mod FLEXEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO module is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "FLEXIO module is enabled."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Software Reset"]
    pub mod SWRST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Software reset is disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Software reset is enabled. All FLEXIO registers except the Control Register are reset."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Fast Access"]
    pub mod FASTACC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Configures for normal register accesses to FLEXIO"]
            pub const NORMAL: u32 = 0;
            #[doc = "Configures for fast register accesses to FLEXIO"]
            pub const FAST: u32 = 1;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBGE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO is disabled in Debug modes."]
            pub const DISABLE: u32 = 0;
            #[doc = "FLEXIO is enabled in Debug modes."]
            pub const EMABLE: u32 = 1;
        }
    }
    #[doc = "Doze Enable"]
    pub mod DOZEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FLEXIO enabled in Doze modes."]
            pub const ENABLE: u32 = 0;
            #[doc = "FLEXIO disabled in Doze modes."]
            pub const DISABLE: u32 = 1;
        }
    }
}
#[doc = "Pin State Register"]
pub mod PIN {
    #[doc = "Pin Data Input"]
    pub mod PDI {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Status Register"]
pub mod SHIFTSTAT {
    #[doc = "Shifter Status Flag"]
    pub mod SSF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Error Register"]
pub mod SHIFTERR {
    #[doc = "Shifter Error Flags"]
    pub mod SEF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Status Register"]
pub mod TIMSTAT {
    #[doc = "Timer Status Flags"]
    pub mod TSF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Status Interrupt Enable"]
pub mod SHIFTSIEN {
    #[doc = "Shifter Status Interrupt Enable"]
    pub mod SSIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Error Interrupt Enable"]
pub mod SHIFTEIEN {
    #[doc = "Shifter Error Interrupt Enable"]
    pub mod SEIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Interrupt Enable Register"]
pub mod TIMIEN {
    #[doc = "Timer Status Interrupt Enable"]
    pub mod TEIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Status DMA Enable"]
pub mod SHIFTSDEN {
    #[doc = "Shifter Status DMA Enable"]
    pub mod SSDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Status DMA Enable"]
pub mod TIMERSDEN {
    #[doc = "Timer Status DMA Enable"]
    pub mod TSDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter State Register"]
pub mod SHIFTSTATE {
    #[doc = "Current State Pointer"]
    pub mod STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger Status Register"]
pub mod TRGSTAT {
    #[doc = "External Trigger Status Flags"]
    pub mod ETSF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External Trigger Interrupt Enable Register"]
pub mod TRIGIEN {
    #[doc = "External Trigger Interrupt Enable"]
    pub mod TRIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Status Register"]
pub mod PINSTAT {
    #[doc = "Pin Status Flags"]
    pub mod PSF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Interrupt Enable Register"]
pub mod PINIEN {
    #[doc = "Pin Status Interrupt Enable"]
    pub mod PSIE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Rising Edge Enable Register"]
pub mod PINREN {
    #[doc = "Pin Rising Edge"]
    pub mod PRE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Falling Edge Enable Register"]
pub mod PINFEN {
    #[doc = "Pin Falling Edge"]
    pub mod PFE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Output Data Register"]
pub mod PINOUTD {
    #[doc = "Output Data"]
    pub mod OUTD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Output Enable Register"]
pub mod PINOUTE {
    #[doc = "Output Enable"]
    pub mod OUTE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Output Disable Register"]
pub mod PINOUTDIS {
    #[doc = "Output Disable"]
    pub mod OUTDIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Output Clear Register"]
pub mod PINOUTCLR {
    #[doc = "Output Clear"]
    pub mod OUTCLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Output Set Register"]
pub mod PINOUTSET {
    #[doc = "Output Set"]
    pub mod OUTSET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Output Toggle Register"]
pub mod PINOUTTOG {
    #[doc = "Output Toggle"]
    pub mod OUTTOG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Control N Register"]
pub mod SHIFTCTL {
    #[doc = "Shifter Mode"]
    pub mod SMOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Receive mode. Captures the current shifter content into the SHIFTBUF on expiration of the timer."]
            pub const RECEIVE: u32 = 1;
            #[doc = "Transmit mode. Load SHIFTBUF contents into the shifter on expiration of the timer."]
            pub const TRANSMIT: u32 = 2;
            #[doc = "Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the timer."]
            pub const MATCHSTORE: u32 = 4;
            #[doc = "Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
            pub const MATCHCONT: u32 = 5;
            #[doc = "State mode. SHIFTBUF contents are used for storing programmable state attributes."]
            pub const STATE: u32 = 6;
            #[doc = "Logic mode. SHIFTBUF contents are used for implementing programmable logic lookup table."]
            pub const LOGIC: u32 = 7;
        }
    }
    #[doc = "Shifter Pin Polarity"]
    pub mod PINPOL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is active high"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Pin is active low"]
            pub const ACTIVE_LOW: u32 = 1;
        }
    }
    #[doc = "Shifter Pin Select"]
    pub mod PINSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Shifter Pin Configuration"]
    pub mod PINCFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Shifter pin output disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Shifter pin open-drain or bidirectional output enable"]
            pub const OPEND_BIDIROUTEN: u32 = 1;
            #[doc = "Shifter pin bidirectional output data"]
            pub const BIDIR_OUTDATA: u32 = 2;
            #[doc = "Shifter pin output"]
            pub const OUTPUT: u32 = 3;
        }
    }
    #[doc = "Timer Polarity"]
    pub mod TIMPOL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Shift on posedge of shift clock"]
            pub const POSEDGE: u32 = 0;
            #[doc = "Shift on negedge of shift clock"]
            pub const NEGEDGE: u32 = 1;
        }
    }
    #[doc = "Timer Select"]
    pub mod TIMSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Configuration N Register"]
pub mod SHIFTCFG {
    #[doc = "Shifter Start Bit"]
    pub mod SSTART {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start bit disabled for transmitter/receiver/match store. Transmitter loads data on enable."]
            pub const VALUE00: u32 = 0;
            #[doc = "Start bit disabled for transmitter/receiver/match store. Transmitter loads data on first shift."]
            pub const VALUE01: u32 = 1;
            #[doc = "Transmitter outputs start bit value 0 before loading data on first shift. If start bit is not 0, receiver/match store sets error flag."]
            pub const VALUE10: u32 = 2;
            #[doc = "Transmitter outputs start bit value 1 before loading data on first shift. If start bit is not 1, receiver/match store sets error flag."]
            pub const VALUE11: u32 = 3;
        }
    }
    #[doc = "Shifter Stop bit"]
    pub mod SSTOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop bit disabled for transmitter/receiver/match store"]
            pub const VALUE00: u32 = 0;
            #[doc = "Stop bit disabled for transmitter/receiver/match store. When timer is in stop condition, receiver/match store stores receive data on the configured shift edge."]
            pub const VALUE01: u32 = 1;
            #[doc = "Transmitter outputs stop bit value 0 on store. If stop bit is not 0, receiver/match store sets error flag. When timer is in stop condition, receiver/match stores also store receive data on the configured shift edge."]
            pub const VALUE10: u32 = 2;
            #[doc = "Transmitter outputs stop bit value 1 on store. If stop bit is not 1, receiver/match store sets error flag. When timer is in stop condition, receiver/match store also stores receive data on the configured shift edge."]
            pub const VALUE11: u32 = 3;
        }
    }
    #[doc = "Input Source"]
    pub mod INSRC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin"]
            pub const PIN: u32 = 0;
            #[doc = "Shifter N+1 Output"]
            pub const SHIFTER_NPLUS1: u32 = 1;
        }
    }
    #[doc = "Late Store"]
    pub mod LATST {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Shift register stores the pre-shift register state."]
            pub const PRESHIFT: u32 = 0;
            #[doc = "Shift register stores the post-shift register state."]
            pub const POSTSHIFT: u32 = 1;
        }
    }
    #[doc = "Shifter Size"]
    pub mod SSIZE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Shift register is 32-bit."]
            pub const WIDTH32: u32 = 0;
            #[doc = "Shift register is 24-bit."]
            pub const WIDTH24: u32 = 1;
        }
    }
    #[doc = "Parallel Width"]
    pub mod PWIDTH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Register"]
pub mod SHIFTBUF {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod SHIFTBUFBIS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFBIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod SHIFTBUFBYS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFBYS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod SHIFTBUFBBS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFBBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Control N Register"]
pub mod TIMCTL {
    #[doc = "Timer Mode"]
    pub mod TIMOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Dual 8-bit counters baud mode."]
            pub const DUAL8BIT_BAUD: u32 = 1;
            #[doc = "Dual 8-bit counters PWM high mode."]
            pub const DUAL8BIT_PWM_H: u32 = 2;
            #[doc = "Single 16-bit counter mode."]
            pub const SINGLE16BIT: u32 = 3;
            #[doc = "Single 16-bit counter disable mode."]
            pub const SINGLE16BIT_DISABLE: u32 = 4;
            #[doc = "Dual 8-bit counters word mode."]
            pub const DUAL8BIT_WORD: u32 = 5;
            #[doc = "Dual 8-bit counters PWM low mode."]
            pub const DUAL8BIT_PWM_L: u32 = 6;
            #[doc = "Single 16-bit input capture mode."]
            pub const SINGLE16BIT_IN_CAPTURE: u32 = 7;
        }
    }
    #[doc = "Timer One Time Operation"]
    pub mod ONETIM {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The timer enable event is generated as normal."]
            pub const NOT_BLOCKED: u32 = 0;
            #[doc = "The timer enable event is blocked unless timer status flag is clear."]
            pub const BLOCKED: u32 = 1;
        }
    }
    #[doc = "Timer Pin Input Select"]
    pub mod PININS {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer pin input and output are selected by PINSEL."]
            pub const PINSEL: u32 = 0;
            #[doc = "Timer pin input is selected by PINSEL+1. Timer pin output remains selected by PINSEL."]
            pub const PINSELPLUS1: u32 = 1;
        }
    }
    #[doc = "Timer Pin Polarity"]
    pub mod PINPOL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin is active high"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Pin is active low"]
            pub const ACTIVE_LOW: u32 = 1;
        }
    }
    #[doc = "Timer Pin Select"]
    pub mod PINSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Pin Configuration"]
    pub mod PINCFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer pin output disabled"]
            pub const OUTDISABLE: u32 = 0;
            #[doc = "Timer pin open-drain or bidirectional output enable"]
            pub const OPEND_BIDIROUTEN: u32 = 1;
            #[doc = "Timer pin bidirectional output data"]
            pub const BIDIR_OUTDATA: u32 = 2;
            #[doc = "Timer pin output"]
            pub const OUTPUT: u32 = 3;
        }
    }
    #[doc = "Trigger Source"]
    pub mod TRGSRC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External trigger selected"]
            pub const EXT_TRIG: u32 = 0;
            #[doc = "Internal trigger selected"]
            pub const INTERNAL_TRIG: u32 = 1;
        }
    }
    #[doc = "Trigger Polarity"]
    pub mod TRGPOL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger active high"]
            pub const ACTIVE_HIGH: u32 = 0;
            #[doc = "Trigger active low"]
            pub const ACTIVE_LOW: u32 = 1;
        }
    }
    #[doc = "Trigger Select"]
    pub mod TRGSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Configuration N Register"]
pub mod TIMCFG {
    #[doc = "Timer Start Bit"]
    pub mod TSTART {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Start bit disabled"]
            pub const DISABLE: u32 = 0;
            #[doc = "Start bit enabled"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Timer Stop Bit"]
    pub mod TSTOP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop bit disabled"]
            pub const STOP_DISABLE: u32 = 0;
            #[doc = "Stop bit is enabled on timer compare"]
            pub const ENABLE_TMRCMP: u32 = 1;
            #[doc = "Stop bit is enabled on timer disable"]
            pub const ENABLE_TMRDISABLE: u32 = 2;
            #[doc = "Stop bit is enabled on timer compare and timer disable"]
            pub const ENABLE_TMR_CMP_DIS: u32 = 3;
        }
    }
    #[doc = "Timer Enable"]
    pub mod TIMENA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer always enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Timer enabled on Timer N-1 enable"]
            pub const TMR_NMINUS1_EN: u32 = 1;
            #[doc = "Timer enabled on Trigger high"]
            pub const TMR_TRIGHI_EN: u32 = 2;
            #[doc = "Timer enabled on Trigger high and Pin high"]
            pub const TMR_TRIG_PIN_HI_EN: u32 = 3;
            #[doc = "Timer enabled on Pin rising edge"]
            pub const TMR_PINRISE_EN: u32 = 4;
            #[doc = "Timer enabled on Pin rising edge and Trigger high"]
            pub const TMR_PINRISE_TRIGHI_EN: u32 = 5;
            #[doc = "Timer enabled on Trigger rising edge"]
            pub const TMR_TRIGRISE_EN: u32 = 6;
            #[doc = "Timer enabled on Trigger rising or falling edge"]
            pub const TMR_TRIGEDGE_EN: u32 = 7;
        }
    }
    #[doc = "Timer Disable"]
    pub mod TIMDIS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer never disabled"]
            pub const NEVER: u32 = 0;
            #[doc = "Timer disabled on Timer N-1 disable"]
            pub const TMR_NMINUS1: u32 = 1;
            #[doc = "Timer disabled on timer compare (upper 8-bits match and decrement)"]
            pub const TMR_CMP: u32 = 2;
            #[doc = "Timer disabled on timer compare (upper 8-bits match and decrement) and trigger low"]
            pub const TMR_CMP_TRIGLOW: u32 = 3;
            #[doc = "Timer disabled on pin rising or falling edge"]
            pub const PIN_EDGE: u32 = 4;
            #[doc = "Timer disabled on pin rising or falling edge provided trigger is high"]
            pub const PIN_EDGE_TRIGHI: u32 = 5;
            #[doc = "Timer disabled on trigger falling edge"]
            pub const TRIG_FALLEDGE: u32 = 6;
        }
    }
    #[doc = "Timer Reset"]
    pub mod TIMRST {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer never reset"]
            pub const NEVER: u32 = 0;
            #[doc = "Timer reset on timer output high."]
            pub const TMR_OUT_HI: u32 = 1;
            #[doc = "Timer reset on timer pin equal to timer output"]
            pub const PIN_EQ_TMR_OUT: u32 = 2;
            #[doc = "Timer reset on timer trigger equal to timer output"]
            pub const TRIG_EQ_TMR_OUT: u32 = 3;
            #[doc = "Timer reset on timer pin rising edge"]
            pub const PIN_RISE_EDGE: u32 = 4;
            #[doc = "Timer reset on trigger rising edge"]
            pub const TRIG_RISE_EDGE: u32 = 6;
            #[doc = "Timer reset on trigger rising or falling edge"]
            pub const TRIG_EDGE: u32 = 7;
        }
    }
    #[doc = "Timer Decrement"]
    pub mod TIMDEC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Decrement counter on FLEXIO clock. Shift clock equals timer output."]
            pub const FLEXIO_CLK_SHIFTCLK_TMR_OUT: u32 = 0;
            #[doc = "Decrement counter on trigger input (both edges). Shift clock equals timer output."]
            pub const TRIG_EDGE_SHIFTCLK_TMR_OUT: u32 = 1;
            #[doc = "Decrement counter on pin input (both edges). Shift clock equals pin input."]
            pub const PIN_EDGE_SHIFTCLK_TMR_OUT: u32 = 2;
            #[doc = "Decrement counter on trigger input (both edges). Shift clock equals trigger input."]
            pub const TRIG_EDGE_SHIFTCLK_TRIG_IN: u32 = 3;
            #[doc = "Decrement counter on FLEXIO clock divided by 16. Shift clock equals timer output."]
            pub const FLEXIO_CLK_DIV16_SHIFTCLK_TMR_OUT: u32 = 4;
            #[doc = "Decrement counter on FLEXIO clock divided by 256. Shift clock equals timer output."]
            pub const FLEXIO_CLK_DIV256_SHIFTCLK_TMR_OUT: u32 = 5;
            #[doc = "Decrement counter on pin input (rising edge). Shift clock equals pin input."]
            pub const PIN_RISE_SHIFTCLK_PIN_IN: u32 = 6;
            #[doc = "Decrement counter on trigger input (rising edge). Shift clock equals trigger input."]
            pub const TRIG_RISE_SHIFTCLK_TRIG_IN: u32 = 7;
        }
    }
    #[doc = "Timer Output"]
    pub mod TIMOUT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
            pub const ONE: u32 = 0;
            #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
            pub const ZERO: u32 = 1;
            #[doc = "Timer output is logic one when enabled and on timer reset"]
            pub const ONE_TMRRESET: u32 = 2;
            #[doc = "Timer output is logic zero when enabled and on timer reset"]
            pub const ZERO_TMRRESET: u32 = 3;
        }
    }
}
#[doc = "Timer Compare N Register"]
pub mod TIMCMP {
    #[doc = "Timer Compare Value"]
    pub mod CMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Nibble Byte Swapped Register"]
pub mod SHIFTBUFNBS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFNBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Halfword Swapped Register"]
pub mod SHIFTBUFHWS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFHWS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Nibble Swapped Register"]
pub mod SHIFTBUFNIS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFNIS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Odd Even Swapped Register"]
pub mod SHIFTBUFOES {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFOES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Even Odd Swapped Register"]
pub mod SHIFTBUFEOS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFEOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Shifter Buffer N Halfword Byte Swapped Register"]
pub mod SHIFTBUFHBS {
    #[doc = "Shift Buffer"]
    pub mod SHIFTBUFHBS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
