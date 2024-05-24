#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "USBDCD"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control"]
    pub CONTROL: crate::RWRegister<u32>,
    #[doc = "Clock"]
    pub CLOCK: crate::RWRegister<u32>,
    #[doc = "Status"]
    pub STATUS: crate::RWRegister<u32>,
    #[doc = "Signal Override"]
    pub SIGNAL_OVERRIDE: crate::RWRegister<u32>,
    #[doc = "TIMER0"]
    pub TIMER0: crate::RWRegister<u32>,
    #[doc = "TIMER1"]
    pub TIMER1: crate::RWRegister<u32>,
    #[doc = "TIMER2_BC11"]
    pub TIMER2_BC11: crate::RWRegister<u32>,
    #[doc = "TIMER2_BC12"]
    pub TIMER2_BC12: crate::RWRegister<u32>,
}
#[doc = "Control"]
pub mod CONTROL {
    #[doc = "Interrupt Acknowledge"]
    pub mod IACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not clear the interrupt."]
            pub const INT_NOCLEAR: u32 = 0;
            #[doc = "Clear the IF field (interrupt flag)."]
            pub const INT_CLEAR: u32 = 1;
        }
    }
    #[doc = "Interrupt Flag"]
    pub mod IF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No interrupt is pending."]
            pub const INT_PEND: u32 = 0;
            #[doc = "An interrupt is pending."]
            pub const INT_NOPEND: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Enable"]
    pub mod IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupts to the system."]
            pub const DIS_INT: u32 = 0;
            #[doc = "Enable interrupts to the system."]
            pub const EN_INT: u32 = 1;
        }
    }
    #[doc = "Battery Charging Revision 1.2 Compatibility"]
    pub mod BC12 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compatible with BC1.1"]
            pub const BC11: u32 = 0;
            #[doc = "Compatible with BC1.2 (default)"]
            pub const BC12: u32 = 1;
        }
    }
    #[doc = "Start Change Detection Sequence"]
    pub mod START {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not start the sequence. Writes of this value have no effect."]
            pub const NO_START: u32 = 0;
            #[doc = "Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
            pub const START: u32 = 1;
        }
    }
    #[doc = "Software Reset"]
    pub mod SR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not perform a software reset."]
            pub const NO_RESET: u32 = 0;
            #[doc = "Perform a software reset."]
            pub const SW_RESET: u32 = 1;
        }
    }
}
#[doc = "Clock"]
pub mod CLOCK {
    #[doc = "Unit of Measurement Encoding for Clock Speed"]
    pub mod CLOCK_UNIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "kHz Speed (between 4 kHz and 1023 kHz)"]
            pub const KHZ_CLK: u32 = 0;
            #[doc = "MHz Speed (between 1 MHz and 1023 MHz)"]
            pub const MHZ_CLK: u32 = 1;
        }
    }
    #[doc = "Numerical Value of Clock Speed in Binary"]
    pub mod CLOCK_SPEED {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status"]
pub mod STATUS {
    #[doc = "Charger Detection Sequence Results"]
    pub mod SEQ_RES {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "No results to report."]
            pub const NO_RESULT: u32 = 0;
            #[doc = "Attached to an SDP. Must comply with USB 2.0 by drawing only 2.5 mA (max) until connected."]
            pub const CONN_SDP: u32 = 1;
            #[doc = "Attached to a charging port. The exact meaning depends on the STATUS[SEQ_STAT] field (value 0: Attached to either a CDP or a DCP. The charger type detection has not completed. value 1: Attached to a CDP. The charger type detection has completed.)"]
            pub const CONN_CP: u32 = 2;
            #[doc = "Attached to a DCP."]
            pub const CONN_DCP: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Charger Detection Sequence Status"]
    pub mod SEQ_STAT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "The module is either not enabled, or the module is enabled but the data pins have not yet been detected."]
            pub const NO_DATA_PIN_CONN: u32 = 0;
            #[doc = "Data pin contact detection is complete."]
            pub const DATA_PIN_CONN: u32 = 1;
            #[doc = "Charging port detection is complete."]
            pub const CP_DET_DONE: u32 = 2;
            #[doc = "Charger type detection is complete."]
            pub const CT_DET_DONE: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Flag"]
    pub mod ERR {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No sequence errors."]
            pub const NO_SEQ_ERR: u32 = 0;
            #[doc = "Error in the detection sequence."]
            pub const SEQ_ERR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout Flag"]
    pub mod TO {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The detection sequence is not running for over 1 s."]
            pub const NO_TIMEOUT: u32 = 0;
            #[doc = "It is over 1 s since the data pin contact was detected and debounced."]
            pub const TIMEOUT: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Active Status Indicator"]
    pub mod ACTIVE {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The sequence is not running."]
            pub const SEQ_NOT_RUNNING: u32 = 0;
            #[doc = "The sequence is running."]
            pub const SEQ_RUNNING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Signal Override"]
pub mod SIGNAL_OVERRIDE {
    #[doc = "Phase Selection"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overrides. Field must remain at this value during normal USB data communication to prevent unexpected conditions on USB_DP and USB_DM pins. (Default)"]
            pub const NO_OVERRIDE: u32 = 0;
            #[doc = "Enables VDP_SRC voltage source for the USB_DP pin and IDM_SINK current source for the USB_DM pin."]
            pub const PRI_DET_OVERRIDE: u32 = 2;
            #[doc = "Enables VDM_SRC voltage source only."]
            pub const CDP_ADVERT: u32 = 4;
        }
    }
}
#[doc = "TIMER0"]
pub mod TIMER0 {
    #[doc = "Unit Connection Timer Elapse (in ms)"]
    pub mod TUNITCON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Initiation Time"]
    pub mod TSEQ_INIT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0 ms - 1023 ms"]
            pub const MS_0: u32 = 0;
            #[doc = "0 ms - 1023 ms"]
            pub const MS_1: u32 = 1;
            #[doc = "0 ms - 1023 ms"]
            pub const MS_2: u32 = 2;
            #[doc = "0 ms - 1023 ms"]
            pub const MS_3: u32 = 3;
            #[doc = "0 ms - 1023 ms"]
            pub const MS_4: u32 = 4;
            #[doc = "0 ms - 1023 ms"]
            pub const MS_5: u32 = 5;
            #[doc = "0 ms - 1023 ms"]
            pub const MS_6: u32 = 6;
            #[doc = "0 ms - 1023 ms"]
            pub const MS_7: u32 = 7;
            #[doc = "0 ms - 1023 ms"]
            pub const MS_8: u32 = 8;
            #[doc = "0 ms - 1023 ms"]
            pub const MS_9: u32 = 9;
        }
    }
}
#[doc = "TIMER1"]
pub mod TIMER1 {
    #[doc = "Time Period Comparator Enabled"]
    pub mod TVDPSRC_ON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 ms - 1023 ms"]
            pub const MS_1: u32 = 1;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_2: u32 = 2;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_3: u32 = 3;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_4: u32 = 4;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_5: u32 = 5;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_6: u32 = 6;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_7: u32 = 7;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_8: u32 = 8;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_9: u32 = 9;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_10: u32 = 10;
        }
    }
    #[doc = "Time Period to Debounce D+ Signal"]
    pub mod TDCD_DBNC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 ms - 1023 ms"]
            pub const MS_1: u32 = 1;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_2: u32 = 2;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_3: u32 = 3;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_4: u32 = 4;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_5: u32 = 5;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_6: u32 = 6;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_7: u32 = 7;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_8: u32 = 8;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_9: u32 = 9;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_10: u32 = 10;
        }
    }
}
#[doc = "TIMER2_BC11"]
pub mod TIMER2_BC11 {
    #[doc = "Time Before Check of D- Line"]
    pub mod CHECK_DM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 ms - 15 ms"]
            pub const MS_1: u32 = 1;
            #[doc = "1 ms - 15 ms"]
            pub const MS_2: u32 = 2;
            #[doc = "1 ms - 15 ms"]
            pub const MS_3: u32 = 3;
            #[doc = "1 ms - 15 ms"]
            pub const MS_4: u32 = 4;
            #[doc = "1 ms - 15 ms"]
            pub const MS_5: u32 = 5;
            #[doc = "1 ms - 15 ms"]
            pub const MS_6: u32 = 6;
            #[doc = "1 ms - 15 ms"]
            pub const MS_7: u32 = 7;
            #[doc = "1 ms - 15 ms"]
            pub const MS_8: u32 = 8;
            #[doc = "1 ms - 15 ms"]
            pub const MS_9: u32 = 9;
            #[doc = "1 ms - 15 ms"]
            pub const MS_10: u32 = 10;
        }
    }
    #[doc = "Time Period Before Enabling D+ Pullup"]
    pub mod TVDPSRC_CON {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 ms - 1023 ms"]
            pub const MS_1: u32 = 1;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_2: u32 = 2;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_3: u32 = 3;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_4: u32 = 4;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_5: u32 = 5;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_6: u32 = 6;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_7: u32 = 7;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_8: u32 = 8;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_9: u32 = 9;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_10: u32 = 10;
        }
    }
}
#[doc = "TIMER2_BC12"]
pub mod TIMER2_BC12 {
    #[doc = "TVDMSRC_ON"]
    pub mod TVDMSRC_ON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0 ms - 40 ms"]
            pub const MS_0: u32 = 0;
            #[doc = "0 ms - 40 ms"]
            pub const MS_1: u32 = 1;
            #[doc = "0 ms - 40 ms"]
            pub const MS_2: u32 = 2;
            #[doc = "0 ms - 40 ms"]
            pub const MS_3: u32 = 3;
            #[doc = "0 ms - 40 ms"]
            pub const MS_4: u32 = 4;
            #[doc = "0 ms - 40 ms"]
            pub const MS_5: u32 = 5;
            #[doc = "0 ms - 40 ms"]
            pub const MS_6: u32 = 6;
            #[doc = "0 ms - 40 ms"]
            pub const MS_7: u32 = 7;
            #[doc = "0 ms - 40 ms"]
            pub const MS_8: u32 = 8;
            #[doc = "0 ms - 40 ms"]
            pub const MS_9: u32 = 9;
        }
    }
    #[doc = "TWAIT_AFTER_PRD"]
    pub mod TWAIT_AFTER_PRD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 ms - 1023 ms"]
            pub const MS_1: u32 = 1;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_2: u32 = 2;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_3: u32 = 3;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_4: u32 = 4;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_5: u32 = 5;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_6: u32 = 6;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_7: u32 = 7;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_8: u32 = 8;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_9: u32 = 9;
            #[doc = "1 ms - 1023 ms"]
            pub const MS_10: u32 = 10;
        }
    }
}
