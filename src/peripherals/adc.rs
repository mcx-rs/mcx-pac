#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "ADC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID Register"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameter Register"]
    pub PARAM: crate::RWRegister<u32>,
    _reserved0: [u8; 0x8],
    #[doc = "Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub STAT: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register"]
    pub IE: crate::RWRegister<u32>,
    #[doc = "DMA Enable Register"]
    pub DE: crate::RWRegister<u32>,
    #[doc = "Configuration Register"]
    pub CFG: crate::RWRegister<u32>,
    #[doc = "Pause Register"]
    pub PAUSE: crate::RWRegister<u32>,
    _reserved1: [u8; 0xc],
    #[doc = "Software Trigger Register"]
    pub SWTRIG: crate::RWRegister<u32>,
    #[doc = "Trigger Status Register"]
    pub TSTAT: crate::RWRegister<u32>,
    _reserved2: [u8; 0x4],
    #[doc = "Offset Trim Register"]
    pub OFSTRIM: crate::RWRegister<u32>,
    _reserved3: [u8; 0x5c],
    #[doc = "Trigger Control Register"]
    pub TCTRL: [crate::RWRegister<u32>; 4usize],
    _reserved4: [u8; 0x30],
    #[doc = "FIFO Control Register"]
    pub FCTRL: [crate::RWRegister<u32>; 2usize],
    _reserved5: [u8; 0x8],
    #[doc = "Gain Calibration Control"]
    pub GCC: [crate::RWRegister<u32>; 2usize],
    #[doc = "Gain Calculation Result"]
    pub GCR: [crate::RWRegister<u32>; 2usize],
    #[doc = "Command Low Buffer Register"]
    pub CMDL1: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH1: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL2: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH2: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL3: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH3: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL4: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH4: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL5: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH5: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL6: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH6: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL7: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH7: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL8: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH8: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL9: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH9: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL10: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH10: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL11: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH11: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL12: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH12: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL13: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH13: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL14: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH14: crate::RWRegister<u32>,
    #[doc = "Command Low Buffer Register"]
    pub CMDL15: crate::RWRegister<u32>,
    #[doc = "Command High Buffer Register"]
    pub CMDH15: crate::RWRegister<u32>,
    _reserved6: [u8; 0x88],
    #[doc = "Compare Value Register"]
    pub CV: [crate::RWRegister<u32>; 15usize],
    _reserved7: [u8; 0xc4],
    #[doc = "Data Result FIFO Register"]
    pub RESFIFO: [crate::RWRegister<u32>; 2usize],
    _reserved8: [u8; 0xf8],
    #[doc = "Calibration General A-Side Registers"]
    pub CAL_GAR: [crate::RWRegister<u32>; 33usize],
    _reserved9: [u8; 0x7c],
    #[doc = "Calibration General B-Side Registers"]
    pub CAL_GBR: [crate::RWRegister<u32>; 33usize],
}
#[doc = "Version ID Register"]
pub mod VERID {
    #[doc = "Resolution"]
    pub mod RES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Up to 13-bit differential or 12-bit single-ended resolution supported."]
            pub const MAX_13_BIT: u32 = 0;
            #[doc = "Up to 16-bit differential or 16-bit single-ended resolution supported. CMDLn[MODE] available for selecting the resolution of conversions for the associated command."]
            pub const MAX_16_BIT: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Differential Supported"]
    pub mod DIFFEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not supported"]
            pub const DIFFERENTIAL_NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported. CMDLn[CTYPE] controls fields implemented."]
            pub const DIFFERENTIAL_SUPPORTED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multiple Vref Implemented"]
    pub mod MVI {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Single VREFH input supported."]
            pub const MULTIPLE_REF_NOT_SUPPORTED: u32 = 0;
            #[doc = "Multiple VREFH inputs supported."]
            pub const MULTIPLE_REF_SUPPORTED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Scale Width"]
    pub mod CSW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "Not supported."]
            pub const CSCALE_NOT_SUPPORTED: u32 = 0;
            #[doc = "Supported with one-bit CSCALE control field."]
            pub const BIT_WIDTH_1: u32 = 1;
            #[doc = "Supported with six-bit CSCALE control field."]
            pub const BIT_WIDTH_6: u32 = 6;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Voltage Reference 1 Range Control Bit Implemented"]
    pub mod VR1RNGI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Range control not required."]
            pub const REF1_FIXED_VOLTAGE_RANGE: u32 = 0;
            #[doc = "Range control required."]
            pub const REF1_SELECTABLE_VOLTAGE_RANGE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Internal ADC Clock Implemented"]
    pub mod IADCKI {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not implemented"]
            pub const INTERNAL_CLK_NOT_AVAILABLE: u32 = 0;
            #[doc = "Implemented"]
            pub const INTERNAL_CLK_AVAILABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Calibration Function Implemented"]
    pub mod CALOFSI {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not implemented"]
            pub const CAL_FUNCTION_NOT_AVAILABLE: u32 = 0;
            #[doc = "Implemented"]
            pub const CAL_FUNCTION_AVAILABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Single-Ended Outputs Supported"]
    pub mod NUM_SEC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "One"]
            pub const SINGLE_CONVERTOR: u32 = 0;
            #[doc = "Two"]
            pub const DUAL_CONVERTOR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of FIFOs"]
    pub mod NUM_FIFO {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "N/A"]
            pub const NO_FIFO_IMPLEMENTED: u32 = 0;
            #[doc = "One"]
            pub const CNT_1: u32 = 1;
            #[doc = "Two"]
            pub const CNT_2: u32 = 2;
            #[doc = "Three"]
            pub const CNT_3: u32 = 3;
            #[doc = "Four"]
            pub const CNT_4: u32 = 4;
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
    #[doc = "Trigger Number"]
    pub mod TRIG_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result FIFO Depth"]
    pub mod FIFOSIZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {
            #[doc = "2"]
            pub const ENTRIES_2: u32 = 1;
            #[doc = "4"]
            pub const ENTRIES_4: u32 = 4;
            #[doc = "8"]
            pub const ENTRIES_8: u32 = 8;
            #[doc = "16"]
            pub const ENTRIES_16: u32 = 16;
            #[doc = "32"]
            pub const ENTRIES_32: u32 = 32;
            #[doc = "64"]
            pub const ENTRIES_64: u32 = 64;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compare Value Number"]
    pub mod CV_NUM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Buffer Number"]
    pub mod CMD_NUM {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control Register"]
pub mod CTRL {
    #[doc = "ADC Enable"]
    pub mod ADCEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Software Reset"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADC logic is not reset."]
            pub const RELEASED_FROM_RESET: u32 = 0;
            #[doc = "ADC logic is reset."]
            pub const HELD_IN_RESET: u32 = 1;
        }
    }
    #[doc = "Doze Enable"]
    pub mod DOZEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADC is enabled in low-power mode."]
            pub const ENABLED: u32 = 0;
            #[doc = "ADC is disabled in low-power mode."]
            pub const DISABLED: u32 = 1;
        }
    }
    #[doc = "Auto-Calibration Request"]
    pub mod CAL_REQ {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No request made."]
            pub const NO_CALIBRATION_REQUEST: u32 = 0;
            #[doc = "Request has been made."]
            pub const CALIBRATION_REQUEST_PENDING: u32 = 1;
        }
    }
    #[doc = "Offset Calibration Request"]
    pub mod CALOFS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Calibration function disabled"]
            pub const NO_ACTIVE_OFFSET_CALIBRATION_REQUEST: u32 = 0;
            #[doc = "Request for offset calibration function"]
            pub const OFFSET_CALIBRATION_REQUEST_PENDING: u32 = 1;
        }
    }
    #[doc = "Reset FIFO 0"]
    pub mod RSTFIFO0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const NO_ACTION: u32 = 0;
            #[doc = "FIFO 0 is reset."]
            pub const TRIGGER_RESET: u32 = 1;
        }
    }
    #[doc = "Reset FIFO 1"]
    pub mod RSTFIFO1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect."]
            pub const NO_ACTION: u32 = 0;
            #[doc = "FIFO 1 is reset."]
            pub const TRIGGER_RESET: u32 = 1;
        }
    }
    #[doc = "Auto-Calibration Averages"]
    pub mod CAL_AVGS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion."]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2 conversions averaged."]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4 conversions averaged."]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8 conversions averaged."]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16 conversions averaged."]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32 conversions averaged."]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64 conversions averaged."]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128 conversions averaged."]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256 conversions averaged."]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512 conversions averaged."]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024 conversions averaged."]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
}
#[doc = "Status Register"]
pub mod STAT {
    #[doc = "Result FIFO 0 Ready Flag"]
    pub mod RDY0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not above watermark"]
            pub const BELOW_THRESHOLD: u32 = 0;
            #[doc = "Above watermark"]
            pub const ABOVE_THRESHOLD: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result FIFO 0 Overflow Flag"]
    pub mod FOF0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No result FIFO 0 overflow has occurred since the last time that the flag was cleared."]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "At least one result FIFO 0 overflow has occurred since the last time that the flag was cleared."]
            pub const OVERFLOW_DETECTED: u32 = 1;
        }
    }
    #[doc = "Result FIFO1 Ready Flag"]
    pub mod RDY1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not above watermark"]
            pub const BELOW_THRESHOLD: u32 = 0;
            #[doc = "Above watermark"]
            pub const ABOVE_THRESHOLD: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Result FIFO1 Overflow Flag"]
    pub mod FOF1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No result FIFO1 overflow has occurred since the last time that the flag was cleared."]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "At least one result FIFO1 overflow has occurred since the last time that the flag was cleared."]
            pub const OVERFLOW_DETECTED: u32 = 1;
        }
    }
    #[doc = "Interrupt Flag For High-Priority Trigger Exception"]
    pub mod TEXC_INT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No trigger exceptions have occurred."]
            pub const NO_EXCEPTION: u32 = 0;
            #[doc = "A trigger exception has occurred and is pending acknowledgment."]
            pub const EXCEPTION_DETECTED: u32 = 1;
        }
    }
    #[doc = "Interrupt Flag For Trigger Completion"]
    pub mod TCOMP_INT {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Either IE[TCOMP_IE] = 0, or no trigger sequences have run to completion."]
            pub const FLAG_CLEAR: u32 = 0;
            #[doc = "Trigger sequence has been completed and all data is stored in the associated FIFO."]
            pub const COMPLETION_DETECTED: u32 = 1;
        }
    }
    #[doc = "Calibration Ready"]
    pub mod CAL_RDY {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Calibration is incomplete or has not been run."]
            pub const NOT_SET: u32 = 0;
            #[doc = "ADC is calibrated."]
            pub const HARDWARE_CAL_STEP_COMPLETED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADC Active"]
    pub mod ADC_ACTIVE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "ADC is idle. There are no pending triggers to service and no active commands are being processed."]
            pub const NOT_ACTIVE: u32 = 0;
            #[doc = "ADC is processing a conversion, running through the power-up delay, or servicing a trigger."]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger Active"]
    pub mod TRGACT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
            pub const TRIG_0: u32 = 0;
            #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
            pub const TRIG_1: u32 = 1;
            #[doc = "Command (sequence) associated with Trigger 2 currently being executed."]
            pub const TRIG_2: u32 = 2;
            #[doc = "Command (sequence) associated with Trigger 3 currently being executed."]
            pub const TRIG_3: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Active"]
    pub mod CMDACT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "No command currently in progress."]
            pub const NO_COMMAND_ACTIVE: u32 = 0;
            #[doc = "Command 1 currently being executed."]
            pub const COMMAND_1: u32 = 1;
            #[doc = "Command 2 currently being executed."]
            pub const COMMAND_2: u32 = 2;
            #[doc = "Associated command number currently being executed."]
            pub const COMMAND_X_3: u32 = 3;
            #[doc = "Associated command number currently being executed."]
            pub const COMMAND_X_4: u32 = 4;
            #[doc = "Associated command number currently being executed."]
            pub const COMMAND_X_5: u32 = 5;
            #[doc = "Associated command number currently being executed."]
            pub const COMMAND_X_6: u32 = 6;
            #[doc = "Associated command number currently being executed."]
            pub const COMMAND_X_7: u32 = 7;
            #[doc = "Associated command number currently being executed."]
            pub const COMMAND_X_8: u32 = 8;
            #[doc = "Associated command number currently being executed."]
            pub const COMMAND_X_9: u32 = 9;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable Register"]
pub mod IE {
    #[doc = "FIFO 0 Watermark Interrupt Enable"]
    pub mod FWMIE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Result FIFO 0 Overflow Interrupt Enable"]
    pub mod FOFIE0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "FIFO1 Watermark Interrupt Enable"]
    pub mod FWMIE1 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Result FIFO1 Overflow Interrupt Enable"]
    pub mod FOFIE1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Trigger Exception Interrupt Enable"]
    pub mod TEXC_IE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Trigger Completion Interrupt Enable"]
    pub mod TCOMP_IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "All disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
            pub const TRIGGER_0_COMPLETE_ENABLED: u32 = 1;
            #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
            pub const TRIGGER_1_COMPLETE_ENABLED: u32 = 2;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_3: u32 = 3;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_4: u32 = 4;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_5: u32 = 5;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_6: u32 = 6;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_7: u32 = 7;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_8: u32 = 8;
            #[doc = "Associated trigger completion interrupts are enabled."]
            pub const TRIGGER_X_COMPLETE_ENABLED_9: u32 = 9;
            #[doc = "All enabled"]
            pub const ALL_TRIGGER_COMPLETES_ENABLED: u32 = 15;
        }
    }
}
#[doc = "DMA Enable Register"]
pub mod DE {
    #[doc = "FIFO 0 Watermark DMA Enable"]
    pub mod FWMDE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "FIFO1 Watermark DMA Enable"]
    pub mod FWMDE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Configuration Register"]
pub mod CFG {
    #[doc = "ADC Trigger Priority Control"]
    pub mod TPRICTRL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Current conversion is aborted and the new command specified by the trigger is started."]
            pub const ABORT_CURRENT_ON_PRIORITY: u32 = 0;
            #[doc = "Current command is stopped after completing the current conversion. If averaging is enabled, the averaging loop is completed. CMDHn[LOOP] is ignored and the higher-priority trigger is serviced."]
            pub const FINISH_CURRENT_ON_PRIORITY: u32 = 1;
            #[doc = "Current command is completed (averaging, looping, compare) before servicing the higher-priority trigger."]
            pub const FINISH_SEQUENCE_ON_PRIORITY: u32 = 2;
        }
    }
    #[doc = "Power Configuration Select"]
    pub mod PWRSEL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low power"]
            pub const LOWEST: u32 = 0;
            #[doc = "High power"]
            pub const HIGHEST: u32 = 2;
        }
    }
    #[doc = "Voltage Reference Selection"]
    pub mod REFSEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Option 1"]
            pub const OPTION_1: u32 = 0;
            #[doc = "Option 2"]
            pub const OPTION_2: u32 = 1;
            #[doc = "Option 3"]
            pub const OPTION_3: u32 = 2;
        }
    }
    #[doc = "Trigger Resume Enable"]
    pub mod TRES {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not automatically resumed or restarted"]
            pub const DISABLED: u32 = 0;
            #[doc = "Automatically resumed or restarted"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Trigger Command Resume"]
    pub mod TCMDRES {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger sequence automatically restarted."]
            pub const DISABLED: u32 = 0;
            #[doc = "Trigger sequence resumed from the command that was executed prior to the exception."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "High-Priority Trigger Exception Disable"]
    pub mod HPT_EXDI {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0;
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 1;
        }
    }
    #[doc = "Power-up Delay"]
    pub mod PUDLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADC Analog Pre-Enable"]
    pub mod PWREN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADC analog circuits are only enabled while conversions are active. Analog startup delays affect performance."]
            pub const NOT_PRE_ENABLED: u32 = 0;
            #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays, at the cost of higher DC current consumption. A single power-up delay (CFG[PUDLY]) is executed immediately once PWREN is set. No detected triggers begin ADC operation until the power-up delay time has passed. After this initial delay expires, the analog circuits remain pre-enabled, and no additional delays are executed."]
            pub const PRE_ENABLED: u32 = 1;
        }
    }
}
#[doc = "Pause Register"]
pub mod PAUSE {
    #[doc = "Pause Delay"]
    pub mod PAUSEDLY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pause Enable"]
    pub mod PAUSEEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Software Trigger Register"]
pub mod SWTRIG {
    #[doc = "Software Trigger 0"]
    pub mod SWT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No trigger 0 event generated."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 0 event generated."]
            pub const INITIATE_TRIGGER_0: u32 = 1;
        }
    }
    #[doc = "Software Trigger 1"]
    pub mod SWT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No trigger 1 event generated."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 1 event generated."]
            pub const INITIATE_TRIGGER_1: u32 = 1;
        }
    }
    #[doc = "Software Trigger 2"]
    pub mod SWT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No trigger 2 event generated."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 2 event generated."]
            pub const INITIATE_TRIGGER_2: u32 = 1;
        }
    }
    #[doc = "Software Trigger 3"]
    pub mod SWT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No trigger 3 event generated."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 3 event generated."]
            pub const INITIATE_TRIGGER_3: u32 = 1;
        }
    }
}
#[doc = "Trigger Status Register"]
pub mod TSTAT {
    #[doc = "Trigger Exception Number"]
    pub mod TEXC_NUM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No triggers have been interrupted by a high-priority exception."]
            pub const NO_EXCEPTIONS: u32 = 0;
            #[doc = "Trigger 0 has been interrupted by a high-priority exception."]
            pub const BIT0_MEANS_TRIGGER_0_INTERRUPTED: u32 = 1;
            #[doc = "Trigger 1 has been interrupted by a high-priority exception."]
            pub const BIT1_MEANS_TRIGGER_1_INTERRUPTED: u32 = 2;
            #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_3: u32 = 3;
            #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_4: u32 = 4;
            #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_5: u32 = 5;
            #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_6: u32 = 6;
            #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_7: u32 = 7;
            #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_8: u32 = 8;
            #[doc = "Associated trigger sequence has interrupted by a high-priority exception."]
            pub const SET_BITS_INDICATE_TRIGGER_X_INTERRUPTED_9: u32 = 9;
            #[doc = "Every trigger sequence has been interrupted by a high-priority exception."]
            pub const ALL_BITS_SET_INDICATE_ALL_TRIGGERS_INTERRUPTED: u32 = 15;
        }
    }
    #[doc = "Trigger Completion Flag"]
    pub mod TCOMP_FLAG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No triggers have been completed. Trigger completion interrupts are disabled."]
            pub const NO_TRIGGER: u32 = 0;
            #[doc = "Trigger 0 has been completed and trigger 0 has enabled completion interrupts."]
            pub const BIT0_MEANS_TRIGGER_0_COMPLETED: u32 = 1;
            #[doc = "Trigger 1 has been completed and trigger 1 has enabled completion interrupts."]
            pub const BIT1_MEANS_TRIGGER_1_COMPLETED: u32 = 2;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_3: u32 = 3;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_4: u32 = 4;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_5: u32 = 5;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_6: u32 = 6;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_7: u32 = 7;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_8: u32 = 8;
            #[doc = "Associated trigger sequence has completed and has enabled completion interrupts."]
            pub const SET_BITS_INDICATE_TRIGGER_X_COMPLETED_9: u32 = 9;
            #[doc = "Every trigger sequence has been completed and every trigger has enabled completion interrupts."]
            pub const ALL_BITS_SET_INDICATE_ALL_TRIGGERS_COMPLETED: u32 = 15;
        }
    }
}
#[doc = "Offset Trim Register"]
pub mod OFSTRIM {
    #[doc = "Trim for Offset"]
    pub mod OFSTRIM_A {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trim for Offset"]
    pub mod OFSTRIM_B {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Trigger Control Register"]
pub mod TCTRL {
    #[doc = "Trigger Enable"]
    pub mod HTEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SAR Result Destination for Channel A"]
    pub mod FIFO_SEL_A {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO 0"]
            pub const STORE_TO_FIFO0: u32 = 0;
            #[doc = "FIFO 1"]
            pub const STORE_TO_FIFO1: u32 = 1;
        }
    }
    #[doc = "SAR Result Destination for Channel B"]
    pub mod FIFO_SEL_B {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO 0"]
            pub const STORE_TO_FIFO0: u32 = 0;
            #[doc = "FIFO 1"]
            pub const STORE_TO_FIFO1: u32 = 1;
        }
    }
    #[doc = "Trigger Priority Setting"]
    pub mod TPRI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Highest priority, Level 1"]
            pub const HIGHEST_PRIORITY: u32 = 0;
            #[doc = "Set to corresponding priority level."]
            pub const CORRESPONDING_LOWER_PRIORITY_1: u32 = 1;
            #[doc = "Set to corresponding priority level."]
            pub const CORRESPONDING_LOWER_PRIORITY_2: u32 = 2;
            #[doc = "Lowest priority, Level 4"]
            pub const LOWEST_PRIORITY: u32 = 3;
        }
    }
    #[doc = "Trigger Resync"]
    pub mod RSYNC {
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
    #[doc = "Trigger Delay Select"]
    pub mod TDLY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger Command Select"]
    pub mod TCMD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "CMD1"]
            pub const EXECUTE_CMD1: u32 = 1;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_2: u32 = 2;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_3: u32 = 3;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_4: u32 = 4;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_5: u32 = 5;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_6: u32 = 6;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_7: u32 = 7;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_8: u32 = 8;
            #[doc = "Corresponding CMD is executed"]
            pub const EXECUTE_CORRESPONDING_CMD_9: u32 = 9;
            #[doc = "CMD15"]
            pub const EXECUTE_CMD15: u32 = 15;
        }
    }
}
#[doc = "FIFO Control Register"]
pub mod FCTRL {
    #[doc = "Result FIFO Counter"]
    pub mod FCOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Watermark Level Selection"]
    pub mod FWMARK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Gain Calibration Control"]
pub mod GCC {
    #[doc = "Gain Calibration Value"]
    pub mod GAIN_CAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gain Calibration Value Valid"]
    pub mod RDY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Invalid"]
            pub const GAIN_CAL_NOT_VALID: u32 = 0;
            #[doc = "Valid"]
            pub const HARDWARE_CAL_ROUTINE_COMPLETED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Gain Calculation Result"]
pub mod GCR {
    #[doc = "Gain Calculation Result"]
    pub mod GCALR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Gain Calculation Ready"]
    pub mod RDY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Invalid"]
            pub const NOT_VALID: u32 = 0;
            #[doc = "Valid"]
            pub const VALID: u32 = 1;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL1 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH1 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL2 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH2 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL3 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH3 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL4 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH4 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL5 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH5 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL6 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH6 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL7 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH7 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL8 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH8 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL9 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH9 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL10 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH10 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL11 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH11 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL12 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH12 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL13 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH13 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL14 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH14 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Command Low Buffer Register"]
pub mod CMDL15 {
    #[doc = "Input Channel Select"]
    pub mod ADCH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CH0A or CH0B or CH0A/CH0B pair."]
            pub const SELECT_CH0: u32 = 0;
            #[doc = "CH1A or CH1B or CH1A/CH1B pair."]
            pub const SELECT_CH1: u32 = 1;
            #[doc = "CH2A or CH2B or CH2A/CH2B pair."]
            pub const SELECT_CH2: u32 = 2;
            #[doc = "CH3A or CH3B or CH3A/CH3B pair."]
            pub const SELECT_CH3: u32 = 3;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_4: u32 = 4;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_5: u32 = 5;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_6: u32 = 6;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_7: u32 = 7;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_8: u32 = 8;
            #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
            pub const SELECT_CORRESPONDING_CHANNEL_9: u32 = 9;
            #[doc = "CH30A or CH30B or CH30A/CH30B pair."]
            pub const SELECT_CH30: u32 = 30;
            #[doc = "CH31A or CH31B or CH31A/CH31B pair."]
            pub const SELECT_CH31: u32 = 31;
        }
    }
    #[doc = "Conversion Type"]
    pub mod CTYPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single-Ended mode. Only A-side channel is converted."]
            pub const SINGLE_ENDED_A_SIDE_CHANNEL: u32 = 0;
            #[doc = "Single-Ended mode. Only B-side channel is converted."]
            pub const SINGLE_ENDED_B_SIDE_CHANNEL: u32 = 1;
            #[doc = "Differential mode. A-B."]
            pub const DIFFERENTIAL_A_MINUS_B: u32 = 2;
            #[doc = "Dual-Single-Ended mode. Both A-side and B-side channels are converted independently."]
            pub const DUAL_A_AND_B: u32 = 3;
        }
    }
    #[doc = "Select Resolution of Conversions"]
    pub mod MODE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard resolution. Single-ended 12-bit conversion; differential 13-bit conversion with 2\'s complement output."]
            pub const DATA_12_BITS: u32 = 0;
            #[doc = "High resolution. Single-ended 16-bit conversion; differential 16-bit conversion with 2\'s complement output."]
            pub const DATA_16_BITS: u32 = 1;
        }
    }
    #[doc = "Alternate Channel B Input Channel Select"]
    pub mod ALTB_ADCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select CH0B"]
            pub const SELECT_CH0B: u32 = 0;
            #[doc = "Select CH1B"]
            pub const SELECT_CH1B: u32 = 1;
            #[doc = "Select CH2B"]
            pub const SELECT_CH2B: u32 = 2;
            #[doc = "Select CH3B"]
            pub const SELECT_CH3B: u32 = 3;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_4: u32 = 4;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_5: u32 = 5;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_6: u32 = 6;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_7: u32 = 7;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_8: u32 = 8;
            #[doc = "Select corresponding channel CHnB"]
            pub const SELECT_CORRESPONDING_CHNB_9: u32 = 9;
            #[doc = "Select CH30B"]
            pub const SELECT_CH30B: u32 = 30;
            #[doc = "Select CH31B"]
            pub const SELECT_CH31B: u32 = 31;
        }
    }
    #[doc = "Alternate Channel B Select Enable"]
    pub mod ALTBEN {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ALTBEN_ADCH disabled. Channel-A and Channel-B inputs are selected based on ADCH settings."]
            pub const DISABLED: u32 = 0;
            #[doc = "ALTBEN_ADCH enabled. Channel-A inputs are selected by ADCH setting and Channel-B inputs are selected by ALTB_ADCH setting."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Command High Buffer Register"]
pub mod CMDH15 {
    #[doc = "Compare Function Enable"]
    pub mod CMPEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED_ALWAYS_STORE_RESULT: u32 = 0;
            #[doc = "Enabled. Store on true."]
            pub const COMPARE_RESULT_STORE_IF_TRUE: u32 = 2;
            #[doc = "Enabled. Repeat channel acquisition (sample, convert, and compare) until true."]
            pub const COMPARE_RESULT_KEEP_CONVERTING_UNTIL_TRUE_STORE_IF_TRUE: u32 = 3;
        }
    }
    #[doc = "Wait for Trigger Assertion Before Execution"]
    pub mod WAIT_TRIG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Command executes automatically."]
            pub const DISABLED: u32 = 0;
            #[doc = "Active trigger must be asserted again before executing this command."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Loop with Increment"]
    pub mod LWI {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Sample Time Select"]
    pub mod STS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Minimum sample time of 3.5 ADCK cycles."]
            pub const SAMPLE_3P5: u32 = 0;
            #[doc = "5.5 ADCK cycles"]
            pub const SAMPLE_5P5: u32 = 1;
            #[doc = "7.5 ADCK cycles"]
            pub const SAMPLE_7P5: u32 = 2;
            #[doc = "11.5 ADCK cycles"]
            pub const SAMPLE_11P5: u32 = 3;
            #[doc = "19.5 ADCK cycles"]
            pub const SAMPLE_19P5: u32 = 4;
            #[doc = "35.5 ADCK cycles"]
            pub const SAMPLE_35P5: u32 = 5;
            #[doc = "67.5 ADCK cycles"]
            pub const SAMPLE_67P5: u32 = 6;
            #[doc = "131.5 ADCK cycles"]
            pub const SAMPLE_131P5: u32 = 7;
        }
    }
    #[doc = "Hardware Average Select"]
    pub mod AVGS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single conversion"]
            pub const NO_AVERAGE: u32 = 0;
            #[doc = "2"]
            pub const AVERAGE_2: u32 = 1;
            #[doc = "4"]
            pub const AVERAGE_4: u32 = 2;
            #[doc = "8"]
            pub const AVERAGE_8: u32 = 3;
            #[doc = "16"]
            pub const AVERAGE_16: u32 = 4;
            #[doc = "32"]
            pub const AVERAGE_32: u32 = 5;
            #[doc = "64"]
            pub const AVERAGE_64: u32 = 6;
            #[doc = "128"]
            pub const AVERAGE_128: u32 = 7;
            #[doc = "256"]
            pub const AVERAGE_256: u32 = 8;
            #[doc = "512"]
            pub const AVERAGE_512: u32 = 9;
            #[doc = "1024"]
            pub const AVERAGE_1024: u32 = 10;
        }
    }
    #[doc = "Loop Count Select"]
    pub mod LOOP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Looping not enabled. Command executes one time."]
            pub const CMD_EXEC_1X: u32 = 0;
            #[doc = "Loop one time. Command executes two times."]
            pub const CMD_EXEC_2X: u32 = 1;
            #[doc = "Loop two times. Command executes three times."]
            pub const CMD_EXEC_3X: u32 = 2;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_3: u32 = 3;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_4: u32 = 4;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_5: u32 = 5;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_6: u32 = 6;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_7: u32 = 7;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_8: u32 = 8;
            #[doc = "Loop corresponding number of times. Command executes LOOP + 1 times."]
            pub const CMD_EXECUTES_CORRESPONDING_TIMES_9: u32 = 9;
            #[doc = "Loop 15 times. Command executes 16 times."]
            pub const CMD_EXEC_15X: u32 = 15;
        }
    }
    #[doc = "Next Command Select"]
    pub mod NEXT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
            pub const NO_NEXT_CMD_TERMINATE_ON_FINISH: u32 = 0;
            #[doc = "CMD1"]
            pub const DO_CMD1_NEXT: u32 = 1;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_2: u32 = 2;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_3: u32 = 3;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_4: u32 = 4;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_5: u32 = 5;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_6: u32 = 6;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_7: u32 = 7;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_8: u32 = 8;
            #[doc = "Select corresponding CMD command buffer register as next command"]
            pub const DO_CORRESPONDING_CMD_NEXT_9: u32 = 9;
            #[doc = "CMD15"]
            pub const DO_CMD15_NEXT: u32 = 15;
        }
    }
}
#[doc = "Compare Value Register"]
pub mod CV {
    #[doc = "Compare Value Low"]
    pub mod CVL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compare Value High"]
    pub mod CVH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Result FIFO Register"]
pub mod RESFIFO {
    #[doc = "Data Result"]
    pub mod D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Trigger Source"]
    pub mod TSRC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Trigger source 0"]
            pub const TRIGGER_0: u32 = 0;
            #[doc = "Trigger source 1"]
            pub const TRIGGER_1: u32 = 1;
            #[doc = "Corresponding trigger source initiated this conversion."]
            pub const CORRESPONDING_TRIGGER_2: u32 = 2;
            #[doc = "Trigger source 3"]
            pub const TRIGGER_3: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Loop Count Value"]
    pub mod LOOPCNT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "Result is from initial conversion in command."]
            pub const RESULT_1: u32 = 0;
            #[doc = "Result is from second conversion in command."]
            pub const RESULT_2: u32 = 1;
            #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
            pub const CORRESPONDING_RESULT_2: u32 = 2;
            #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
            pub const CORRESPONDING_RESULT_3: u32 = 3;
            #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
            pub const CORRESPONDING_RESULT_4: u32 = 4;
            #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
            pub const CORRESPONDING_RESULT_5: u32 = 5;
            #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
            pub const CORRESPONDING_RESULT_6: u32 = 6;
            #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
            pub const CORRESPONDING_RESULT_7: u32 = 7;
            #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
            pub const CORRESPONDING_RESULT_8: u32 = 8;
            #[doc = "Result is from (LOOPCNT + 1) conversion in command."]
            pub const CORRESPONDING_RESULT_9: u32 = 9;
            #[doc = "Result is from 16th conversion in command."]
            pub const RESULT_16: u32 = 15;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Buffer Source"]
    pub mod CMDSRC {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "Not a valid value CMDSRC value for a data word in RESFIFO. 0h is only found in the initial FIFO state, prior to the storage of an ADC conversion result into a RESFIFO buffer."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "CMD1"]
            pub const CMD1: u32 = 1;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_2: u32 = 2;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_3: u32 = 3;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_4: u32 = 4;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_5: u32 = 5;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_6: u32 = 6;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_7: u32 = 7;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_8: u32 = 8;
            #[doc = "Corresponding command buffer used as control settings for this conversion."]
            pub const CORRESPONDING_CMD_9: u32 = 9;
            #[doc = "CMD15"]
            pub const CMD15: u32 = 15;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Entry is Valid"]
    pub mod VALID {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "FIFO is empty. Discard any read from RESFIFO."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "FIFO contains data. FIFO record read from RESFIFO is valid."]
            pub const VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Calibration General A-Side Registers"]
pub mod CAL_GAR {
    #[doc = "Calibration General A Side Register Element"]
    pub mod CAL_GAR_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Calibration General B-Side Registers"]
pub mod CAL_GBR {
    #[doc = "Calibration General B Side Register Element"]
    pub mod CAL_GBR_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
