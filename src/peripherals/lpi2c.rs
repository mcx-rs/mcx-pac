#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "LPI2C"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RWRegister<u32>,
    _reserved0: [u8; 0x8],
    #[doc = "Controller Control"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "Controller Status"]
    pub MSR: crate::RWRegister<u32>,
    #[doc = "Controller Interrupt Enable"]
    pub MIER: crate::RWRegister<u32>,
    #[doc = "Controller DMA Enable"]
    pub MDER: crate::RWRegister<u32>,
    #[doc = "Controller Configuration 0"]
    pub MCFGR0: crate::RWRegister<u32>,
    #[doc = "Controller Configuration 1"]
    pub MCFGR1: crate::RWRegister<u32>,
    #[doc = "Controller Configuration 2"]
    pub MCFGR2: crate::RWRegister<u32>,
    #[doc = "Controller Configuration 3"]
    pub MCFGR3: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "Controller Data Match"]
    pub MDMR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x4],
    #[doc = "Controller Clock Configuration 0"]
    pub MCCR0: crate::RWRegister<u32>,
    _reserved3: [u8; 0x4],
    #[doc = "Controller Clock Configuration 1"]
    pub MCCR1: crate::RWRegister<u32>,
    _reserved4: [u8; 0x4],
    #[doc = "Controller FIFO Control"]
    pub MFCR: crate::RWRegister<u32>,
    #[doc = "Controller FIFO Status"]
    pub MFSR: crate::RWRegister<u32>,
    #[doc = "Controller Transmit Data"]
    pub MTDR: crate::RWRegister<u32>,
    _reserved5: [u8; 0xc],
    #[doc = "Controller Receive Data"]
    pub MRDR: crate::RWRegister<u32>,
    _reserved6: [u8; 0x4],
    #[doc = "Controller Receive Data Read Only"]
    pub MRDROR: crate::RWRegister<u32>,
    _reserved7: [u8; 0x94],
    #[doc = "Target Control"]
    pub SCR: crate::RWRegister<u32>,
    #[doc = "Target Status"]
    pub SSR: crate::RWRegister<u32>,
    #[doc = "Target interrupt enable"]
    pub SIER: crate::RWRegister<u32>,
    #[doc = "Target DMA Enable"]
    pub SDER: crate::RWRegister<u32>,
    #[doc = "Target Configuration 0"]
    pub SCFGR0: crate::RWRegister<u32>,
    #[doc = "Target Configuration 1"]
    pub SCFGR1: crate::RWRegister<u32>,
    #[doc = "Target Configuration 2"]
    pub SCFGR2: crate::RWRegister<u32>,
    _reserved8: [u8; 0x14],
    #[doc = "Target Address Match"]
    pub SAMR: crate::RWRegister<u32>,
    _reserved9: [u8; 0xc],
    #[doc = "Target Address Status"]
    pub SASR: crate::RWRegister<u32>,
    #[doc = "Target Transmit ACK"]
    pub STAR: crate::RWRegister<u32>,
    _reserved10: [u8; 0x8],
    #[doc = "Target Transmit Data"]
    pub STDR: crate::RWRegister<u32>,
    _reserved11: [u8; 0xc],
    #[doc = "Target Receive Data"]
    pub SRDR: crate::RWRegister<u32>,
    _reserved12: [u8; 0x4],
    #[doc = "Target Receive Data Read Only"]
    pub SRDROR: crate::RWRegister<u32>,
    _reserved13: [u8; 0x84],
    #[doc = "Controller Transmit Command Burst"]
    pub MTCBR: [crate::RWRegister<u32>; 128usize],
    #[doc = "Transmit Data Burst"]
    pub MTDBR: [crate::RWRegister<u32>; 253usize],
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {
            #[doc = "Controller only, with standard feature set"]
            pub const MASTER_ONLY: u32 = 2;
            #[doc = "Controller and target, with standard feature set"]
            pub const MASTER_AND_SLAVE: u32 = 3;
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
    #[doc = "Controller Transmit FIFO Size"]
    pub mod MTXFIFO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controller Receive FIFO Size"]
    pub mod MRXFIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Control"]
pub mod MCR {
    #[doc = "Controller Enable"]
    pub mod MEN {
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
            #[doc = "Not reset"]
            pub const NOT_RESET: u32 = 0;
            #[doc = "Reset"]
            pub const RESET: u32 = 1;
        }
    }
    #[doc = "Doze mode enable"]
    pub mod DOZEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Controller is enabled in doze mode"]
            pub const ENABLED: u32 = 0;
            #[doc = "Controller is disabled in doze mode"]
            pub const DISABLED: u32 = 1;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBGEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Controller is disabled in debug mode"]
            pub const DISABLED: u32 = 0;
            #[doc = "Controller is enabled in debug mode"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Reset Transmit FIFO"]
    pub mod RTF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Transmit FIFO is reset"]
            pub const RESET: u32 = 1;
        }
    }
    #[doc = "Reset Receive FIFO"]
    pub mod RRF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Receive FIFO is reset"]
            pub const RESET: u32 = 1;
        }
    }
}
#[doc = "Controller Status"]
pub mod MSR {
    #[doc = "Transmit Data Flag"]
    pub mod TDF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit data is not requested"]
            pub const DISABLED: u32 = 0;
            #[doc = "Transmit data is requested"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Data Flag"]
    pub mod RDF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive data is not ready"]
            pub const DISABLED: u32 = 0;
            #[doc = "Receive data is ready"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End Packet Flag"]
    pub mod EPF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Controller has not generated a STOP or Repeated START condition"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Controller has generated a STOP or Repeated START condition"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "STOP Detect Flag"]
    pub mod SDF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Controller has not generated a STOP condition"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Controller has generated a STOP condition"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "NACK Detect Flag"]
    pub mod NDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unexpected NACK was not detected"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Unexpected NACK was detected"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Arbitration Lost Flag"]
    pub mod ALF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Controller has not lost arbitration"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Controller has lost arbitration"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "FIFO Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Controller sending or receiving data without a START condition"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Pin Low Timeout Flag"]
    pub mod PLTF {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pin low timeout has not occurred or is disabled"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Pin low timeout has occurred"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Data Match Flag"]
    pub mod DMF {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Have not received matching data"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Have received matching data"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "START Flag"]
    pub mod STF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "START condition not detected."]
            pub const NO_FLAG: u32 = 0;
            #[doc = "START condition detected."]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Controller Busy Flag"]
    pub mod MBF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Idle"]
            pub const IDLE: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus Busy Flag"]
    pub mod BBF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Idle"]
            pub const IDLE: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Interrupt Enable"]
pub mod MIER {
    #[doc = "Transmit Data Interrupt Enable"]
    pub mod TDIE {
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
    #[doc = "Receive Data Interrupt Enable"]
    pub mod RDIE {
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
    #[doc = "End Packet Interrupt Enable"]
    pub mod EPIE {
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
    #[doc = "STOP Detect Interrupt Enable"]
    pub mod SDIE {
        pub const offset: u32 = 9;
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
    #[doc = "NACK Detect Interrupt Enable"]
    pub mod NDIE {
        pub const offset: u32 = 10;
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
    #[doc = "Arbitration Lost Interrupt Enable"]
    pub mod ALIE {
        pub const offset: u32 = 11;
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
    #[doc = "FIFO Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 12;
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
    #[doc = "Pin Low Timeout Interrupt Enable"]
    pub mod PLTIE {
        pub const offset: u32 = 13;
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
    #[doc = "Data Match Interrupt Enable"]
    pub mod DMIE {
        pub const offset: u32 = 14;
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
    #[doc = "START Interrupt Enable"]
    pub mod STIE {
        pub const offset: u32 = 15;
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
#[doc = "Controller DMA Enable"]
pub mod MDER {
    #[doc = "Transmit Data DMA Enable"]
    pub mod TDDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Receive Data DMA Enable"]
    pub mod RDDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Controller Configuration 0"]
pub mod MCFGR0 {
    #[doc = "Host request enable"]
    pub mod HREN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Host request input is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Host request input is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Host request polarity"]
    pub mod HRPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active low"]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "Active high"]
            pub const ACTIVE_HIGH: u32 = 1;
        }
    }
    #[doc = "Host request select"]
    pub mod HRSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Host request input is pin HREQ"]
            pub const DISABLED: u32 = 0;
            #[doc = "Host request input is input trigger"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Host request direction"]
    pub mod HRDIR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HREQ pin is input (for LPI2C controller)"]
            pub const INPUT: u32 = 0;
            #[doc = "HREQ pin is output (for LPI2C target)"]
            pub const OUTPUT: u32 = 1;
        }
    }
    #[doc = "Circular FIFO enable"]
    pub mod CIRFIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Circular FIFO is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Circular FIFO is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Receive data match only"]
    pub mod RDMO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Received data is stored in the receive FIFO"]
            pub const DISABLED: u32 = 0;
            #[doc = "Received data is discarded unless the Data Match Flag (MSR[DMF]) is 1."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Relaxed Mode"]
    pub mod RELAX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal transfer"]
            pub const NORMAL_TRANSFER: u32 = 0;
            #[doc = "Relaxed transfer"]
            pub const RELAXED_TRANSFER: u32 = 1;
        }
    }
    #[doc = "Abort Transfer"]
    pub mod ABORT {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal transfer"]
            pub const DISABLED: u32 = 0;
            #[doc = "Abort existing transfer and do not start new transfer"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Controller Configuration 1"]
pub mod MCFGR1 {
    #[doc = "Prescaler"]
    pub mod PRESCALE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_BY_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_BY_2: u32 = 1;
            #[doc = "Divide by 4"]
            pub const DIVIDE_BY_4: u32 = 2;
            #[doc = "Divide by 8"]
            pub const DIVIDE_BY_8: u32 = 3;
            #[doc = "Divide by 16"]
            pub const DIVIDE_BY_16: u32 = 4;
            #[doc = "Divide by 32"]
            pub const DIVIDE_BY_32: u32 = 5;
            #[doc = "Divide by 64"]
            pub const DIVIDE_BY_64: u32 = 6;
            #[doc = "Divide by 128"]
            pub const DIVIDE_BY_128: u32 = 7;
        }
    }
    #[doc = "Automatic STOP Generation"]
    pub mod AUTOSTOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLED: u32 = 0;
            #[doc = "STOP condition is automatically generated when the transmit FIFO is empty and the LPI2C controller is busy"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Ignore NACK"]
    pub mod IGNACK {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLED: u32 = 0;
            #[doc = "LPI2C controller treats a received NACK as if it (NACK) was an ACK and the NACK Detect Flag is never written 1."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Timeout Configuration"]
    pub mod TIMECFG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MSR[PLTF] becomes 1 if SCL is low for longer than the configured timeout."]
            pub const IF_SCL_LOW: u32 = 0;
            #[doc = "MSR[PLTF] becomes 1 if either SCL or SDA is low for longer than the configured timeout."]
            pub const IF_SCL_OR_SDA_LOW: u32 = 1;
        }
    }
    #[doc = "STOP Configuration"]
    pub mod STOPCFG {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MSR[SDF] asserts on any STOP condition generated by LPI2C controller."]
            pub const ANY_STOP: u32 = 0;
            #[doc = "MSR[SDF] asserts on last STOP condition before LPI2C controller is idle (that is, the transmit FIFO is empty at the time of the STOP condition)."]
            pub const LAST_STOP: u32 = 1;
        }
    }
    #[doc = "START Configuration"]
    pub mod STARTCFG {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MSR[STF] asserts on START condition provided both I2C bus and LPI2C controller are idle (that is, any non-repeated START condition initiated by any other controller on the bus but not the LPI2C controller)."]
            pub const BOTH_I2C_AND_LPI2C_IDLE: u32 = 0;
            #[doc = "MSR[STF] asserts on START condition provided I2C bus is idle (that is, any non-repeated START condition initiated by any controller on the bus including the LPI2C controller)."]
            pub const I2C_IDLE: u32 = 1;
        }
    }
    #[doc = "Match Configuration"]
    pub mod MATCFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Match is enabled (first data word equals MDMR[MATCH0] OR MDMR[MATCH1])"]
            pub const FIRST_DATA_WORD_EQUALS_MATCH0_OR_MATCH1: u32 = 2;
            #[doc = "Match is enabled (any data word equals MDMR[MATCH0] OR MDMR[MATCH1])"]
            pub const ANY_DATA_WORD_EQUALS_MATCH0_OR_MATCH1: u32 = 3;
            #[doc = "Match is enabled (first data word equals MDMR[MATCH0] AND second data word equals MDMR[MATCH1)"]
            pub const FIRST_DATA_WORD_MATCH0_AND_SECOND_DATA_WORD_MATCH1: u32 = 4;
            #[doc = "Match is enabled (any data word equals MDMR[MATCH0] AND next data word equals MDMR[MATCH1)"]
            pub const ANY_DATA_WORD_MATCH0_NEXT_DATA_WORD_MATCH1: u32 = 5;
            #[doc = "Match is enabled (first data word AND MDMR[MATCH1] equals MDMR[MATCH0] AND MDMR[MATCH1])"]
            pub const FIRST_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1: u32 = 6;
            #[doc = "Match is enabled (any data word AND MDMR[MATCH1] equals MDMR[MATCH0] AND MDMR[MATCH1])"]
            pub const ANY_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1: u32 = 7;
        }
    }
    #[doc = "Pin Configuration"]
    pub mod PINCFG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Two-pin open drain mode"]
            pub const OPEN_DRAIN_2_PIN: u32 = 0;
            #[doc = "Two-pin output only mode (ultra-fast mode)"]
            pub const OUTPUT_2_PIN_ONLY: u32 = 1;
            #[doc = "Two-pin push-pull mode"]
            pub const PUSH_PULL_2_PIN: u32 = 2;
            #[doc = "Four-pin push-pull mode"]
            pub const PUSH_PULL_4_PIN: u32 = 3;
            #[doc = "Two-pin open drain mode with separate LPI2C target"]
            pub const OPEN_DRAIN_2_PIN_W_LPI2C_SLAVE: u32 = 4;
            #[doc = "Two-pin output only mode (ultra-fast mode) with separate LPI2C target"]
            pub const OUTPUT_2_PIN_ONLY_W_LPI2C_SLAVE: u32 = 5;
            #[doc = "Two-pin push-pull mode with separate LPI2C target"]
            pub const PUSH_PULL_2_PIN_W_LPI2C_SLAVE: u32 = 6;
            #[doc = "Four-pin push-pull mode (inverted outputs)"]
            pub const PUSH_PULL_4_PIN_W_LPI2C_SLAVE: u32 = 7;
        }
    }
}
#[doc = "Controller Configuration 2"]
pub mod MCFGR2 {
    #[doc = "Bus Idle Timeout"]
    pub mod BUSIDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch Filter SCL"]
    pub mod FILTSCL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch Filter SDA"]
    pub mod FILTSDA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Configuration 3"]
pub mod MCFGR3 {
    #[doc = "Pin low timeout"]
    pub mod PINLOW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Data Match"]
pub mod MDMR {
    #[doc = "Match 0 Value"]
    pub mod MATCH0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Match 1 Value"]
    pub mod MATCH1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Clock Configuration 0"]
pub mod MCCR0 {
    #[doc = "Clock Low Period"]
    pub mod CLKLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock High Period"]
    pub mod CLKHI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setup Hold Delay"]
    pub mod SETHOLD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Valid Delay"]
    pub mod DATAVD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Clock Configuration 1"]
pub mod MCCR1 {
    #[doc = "Clock Low Period"]
    pub mod CLKLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock High Period"]
    pub mod CLKHI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setup Hold Delay"]
    pub mod SETHOLD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Valid Delay"]
    pub mod DATAVD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller FIFO Control"]
pub mod MFCR {
    #[doc = "Transmit FIFO Watermark"]
    pub mod TXWATER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Watermark"]
    pub mod RXWATER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller FIFO Status"]
pub mod MFSR {
    #[doc = "Transmit FIFO Count"]
    pub mod TXCOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Count"]
    pub mod RXCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Transmit Data"]
pub mod MTDR {
    #[doc = "Transmit Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Data"]
    pub mod CMD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Transmit DATA[7:0]"]
            pub const TRANSMIT_DATA_7_THROUGH_0: u32 = 0;
            #[doc = "Receive (DATA[7:0] + 1) bytes"]
            pub const RECEIVE_DATA_7_THROUGH_0_PLUS_ONE: u32 = 1;
            #[doc = "Generate STOP condition"]
            pub const GENERATE_STOP_CONDITION: u32 = 2;
            #[doc = "Receive and discard (DATA[7:0] + 1) bytes"]
            pub const RECEIVE_AND_DISCARD_DATA_7_THROUGH_0_PLUS_ONE: u32 = 3;
            #[doc = "Generate (repeated) START and transmit address in DATA[7:0]"]
            pub const GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0: u32 = 4;
            #[doc = "Generate (repeated) START and transmit address in DATA[7:0]. This transfer expects a NACK to be returned."]
            pub const GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_EXPECT_NACK: u32 = 5;
            #[doc = "Generate (repeated) START and transmit address in DATA[7:0] using high-speed mode"]
            pub const GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE: u32 = 6;
            #[doc = "Generate (repeated) START and transmit address in DATA[7:0] using high-speed mode. This transfer expects a NACK to be returned."]
            pub const GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE_EXPECT_NACK: u32 = 7;
        }
        pub mod RW {}
    }
}
#[doc = "Controller Receive Data"]
pub mod MRDR {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive FIFO is not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Receive FIFO is empty"]
            pub const EMPTY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Receive Data Read Only"]
pub mod MRDROR {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive FIFO is not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Receive FIFO is empty"]
            pub const EMPTY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Control"]
pub mod SCR {
    #[doc = "Target Enable"]
    pub mod SEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I2C Target mode is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "I2C Target mode is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Software reset"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Target mode logic is not reset"]
            pub const NOT_RESET: u32 = 0;
            #[doc = "Target mode logic is reset"]
            pub const RESET: u32 = 1;
        }
    }
    #[doc = "Filter enable"]
    pub mod FILTEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable digital filter and output delay counter for target mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable digital filter and output delay counter for target mode"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Filter doze enable"]
    pub mod FILTDZ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Filter remains enabled in Doze mode"]
            pub const FILTER_ENABLED: u32 = 0;
            #[doc = "Filter is disabled in Doze mode"]
            pub const FILTER_DISABLED: u32 = 1;
        }
    }
    #[doc = "Reset transmit FIFO"]
    pub mod RTF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Transmit Data Register is now empty."]
            pub const NOW_EMPTY: u32 = 1;
        }
    }
    #[doc = "Reset Receive FIFO"]
    pub mod RRF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Receive Data Register is now empty."]
            pub const NOW_EMPTY: u32 = 1;
        }
    }
}
#[doc = "Target Status"]
pub mod SSR {
    #[doc = "Transmit data flag"]
    pub mod TDF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit data not requested"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Transmit data is requested"]
            pub const FLAG: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive data flag"]
    pub mod RDF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive data is not ready"]
            pub const NOT_READY: u32 = 0;
            #[doc = "Receive data is ready"]
            pub const READY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address valid flag"]
    pub mod AVF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Address Status Register is not valid"]
            pub const NOT_VALID: u32 = 0;
            #[doc = "Address Status Register is valid"]
            pub const VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit ACK flag"]
    pub mod TAF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit ACK/NACK is not required"]
            pub const NOT_REQUIRED: u32 = 0;
            #[doc = "Transmit ACK/NACK is required"]
            pub const REQUIRED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Repeated start flag"]
    pub mod RSF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Target has not detected a Repeated START condition"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Target has detected a Repeated START condition"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "STOP detect flag"]
    pub mod SDF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Target has not detected a STOP condition"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Target has detected a STOP condition"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Bit error flag"]
    pub mod BEF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Target has not detected a bit error"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Target has detected a bit error"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "FIFO error flag"]
    pub mod FEF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "FIFO underflow or overflow was not detected"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "FIFO underflow or overflow was detected"]
            pub const FLAG: u32 = 1;
        }
    }
    #[doc = "Address match 0 flag"]
    pub mod AM0F {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "ADDR0 matching address not received"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "ADDR0 matching address received"]
            pub const FLAG: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address match 1 flag"]
    pub mod AM1F {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "ADDR1 or ADDR0/ADDR1 range matching address not received"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "ADDR1 or ADDR0/ADDR1 range matching address received"]
            pub const FLAG: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "General call flag"]
    pub mod GCF {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Target has not detected the General Call Address or the General Call Address is disabled"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Target has detected the General Call Address"]
            pub const FLAG: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SMBus alert response flag"]
    pub mod SARF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "SMBus alert response is disabled or not detected"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "SMBus alert response is enabled and detected"]
            pub const FLAG: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target busy flag"]
    pub mod SBF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Idle"]
            pub const IDLE: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus busy flag"]
    pub mod BBF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Idle"]
            pub const IDLE: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target interrupt enable"]
pub mod SIER {
    #[doc = "Transmit data interrupt enable"]
    pub mod TDIE {
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
    #[doc = "Receive data interrupt enable"]
    pub mod RDIE {
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
    #[doc = "Address valid interrupt enable"]
    pub mod AVIE {
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
    #[doc = "Transmit ACK interrupt enable"]
    pub mod TAIE {
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
    #[doc = "Repeated start interrupt enable"]
    pub mod RSIE {
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
    #[doc = "STOP detect interrupt enable"]
    pub mod SDIE {
        pub const offset: u32 = 9;
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
    #[doc = "Bit error interrupt enable"]
    pub mod BEIE {
        pub const offset: u32 = 10;
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
    #[doc = "FIFO error interrupt enable"]
    pub mod FEIE {
        pub const offset: u32 = 11;
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
    #[doc = "Address match 0 interrupt enable"]
    pub mod AM0IE {
        pub const offset: u32 = 12;
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
    #[doc = "Address match 1 interrupt enable"]
    pub mod AM1IE {
        pub const offset: u32 = 13;
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
    #[doc = "General call interrupt enable"]
    pub mod GCIE {
        pub const offset: u32 = 14;
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
    #[doc = "SMBus alert response interrupt enable"]
    pub mod SARIE {
        pub const offset: u32 = 15;
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
#[doc = "Target DMA Enable"]
pub mod SDER {
    #[doc = "Transmit data DMA enable"]
    pub mod TDDE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Receive data DMA enable"]
    pub mod RDDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Address valid DMA enable"]
    pub mod AVDE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Repeated start DMA enable"]
    pub mod RSDE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Stop detect DMA enable"]
    pub mod SDDE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA request is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "DMA request is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Target Configuration 0"]
pub mod SCFGR0 {
    #[doc = "Read Request"]
    pub mod RDREQ {
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
    #[doc = "Read Acknowledge"]
    pub mod RDACK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Read Request not acknowledged"]
            pub const NOT_ACKNOWLEDGED: u32 = 0;
            #[doc = "Read Request acknowledged"]
            pub const ACKNOWLEDGED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Configuration 1"]
pub mod SCFGR1 {
    #[doc = "Address SCL stall"]
    pub mod ADRSTALL {
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
    #[doc = "RX SCL stall"]
    pub mod RXSTALL {
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
    #[doc = "Transmit data SCL stall"]
    pub mod TXDSTALL {
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
    #[doc = "ACK SCL stall"]
    pub mod ACKSTALL {
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
    #[doc = "Receive NACK"]
    pub mod RXNACK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ACK/NACK always written 1 by TXNACK"]
            pub const SET_BY_TXNACK: u32 = 0;
            #[doc = "NACK always generated on address overrun or receive data overrun, otherwise ACK/NACK is written 1 by TXNACK."]
            pub const ALWAYS_GENERATED_ON_ADDRESS_OR_RECEIVE_DATA_OVERRUN: u32 = 1;
        }
    }
    #[doc = "General call enable"]
    pub mod GCEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "General Call address is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "General Call address is enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "SMBus alert enable"]
    pub mod SAEN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables match on SMBus Alert"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables match on SMBus Alert"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Transmit flag configuration"]
    pub mod TXCFG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MSR[TDF] becomes 1 only during a target-transmit transfer when the Transmit Data register is empty"]
            pub const ASSERTS_DURING_SLAVE_TRANSMIT_TRANSFER_WHEN_TX_DATA_EMPTY: u32 = 0;
            #[doc = "MSR[TDF] becomes 1 whenever the Transmit Data register is empty"]
            pub const ASSERTS_WHEN_TX_DATA_EMPTY: u32 = 1;
        }
    }
    #[doc = "Receive Data Configuration"]
    pub mod RXCFG {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reading the Receive Data register returns received data and writes 0 to the Receive Data flag."]
            pub const RETURNS_RECEIVED_DATA_AND_CLEARS_RX_DATA_FLAG: u32 = 0;
            #[doc = "Reading the Receive Data register when the Address Valid flag (SSR[AVF]) is 1, returns the Address Status register and writes 0 to SSR[AVF]. Reading the Receive Data register when SSR[AVF] is 0, returns received data and writes 0 to the Receive Data flag (MSR[RDF])."]
            pub const WHEN_ADDRESS_VALID_FLAG_SET_RETURNS_ADDRESS_STATUS_AND_CLEARS_ADDRESS_VALID_FLAG: u32 = 1;
        }
    }
    #[doc = "Ignore NACK"]
    pub mod IGNACK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Target ends transfer when NACK is detected"]
            pub const ENDS_TRANSFER_ON_NACK: u32 = 0;
            #[doc = "Target does not end transfer when NACK detected"]
            pub const DOES_NOT_END_TRANSFER_ON_NACK: u32 = 1;
        }
    }
    #[doc = "High-speed mode enable"]
    pub mod HSMEN {
        pub const offset: u32 = 13;
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
    #[doc = "Address configuration"]
    pub mod ADDRCFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address match 0 (7-bit)"]
            pub const ADDRESS_MATCH0_7_BIT: u32 = 0;
            #[doc = "Address match 0 (10-bit)"]
            pub const ADDRESS_MATCH0_10_BIT: u32 = 1;
            #[doc = "Address match 0 (7-bit) or Address match 1 (7-bit)"]
            pub const ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_7_BIT: u32 = 2;
            #[doc = "Address match 0 (10-bit) or Address match 1 (10-bit)"]
            pub const ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_10_BIT: u32 = 3;
            #[doc = "Address match 0 (7-bit) or Address match 1 (10-bit)"]
            pub const ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_10_BIT: u32 = 4;
            #[doc = "Address match 0 (10-bit) or Address match 1 (7-bit)"]
            pub const ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_7_BIT: u32 = 5;
            #[doc = "From Address match 0 (7-bit) to Address match 1 (7-bit)"]
            pub const FROM_ADDRESS_MATCH0_7_BIT_TO_ADDRESS_MATCH1_7_BIT: u32 = 6;
            #[doc = "From Address match 0 (10-bit) to Address match 1 (10-bit)"]
            pub const FROM_ADDRESS_MATCH0_10_BIT_TO_ADDRESS_MATCH1_10_BIT: u32 = 7;
        }
    }
    #[doc = "Receive all"]
    pub mod RXALL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive all disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Receive all enabled"]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Repeated start configuration"]
    pub mod RSCFG {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Any repeated START condition following an address match"]
            pub const ANY_REPEATED_START_AFTER_ADDRESS_MATCH: u32 = 0;
            #[doc = "Any repeated START condition"]
            pub const ANY_REPEATED_START: u32 = 1;
        }
    }
    #[doc = "Stop Detect Configuration"]
    pub mod SDCFG {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Any STOP condition following an address match"]
            pub const ANY_STOP_AFTER_ADDRESS_MATCH: u32 = 0;
            #[doc = "Any STOP condition"]
            pub const ANY_STOP: u32 = 1;
        }
    }
}
#[doc = "Target Configuration 2"]
pub mod SCFGR2 {
    #[doc = "Clock hold time"]
    pub mod CLKHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data valid delay"]
    pub mod DATAVD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch filter SCL"]
    pub mod FILTSCL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch filter SDA"]
    pub mod FILTSDA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Address Match"]
pub mod SAMR {
    #[doc = "Address 0 value"]
    pub mod ADDR0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address 1 value"]
    pub mod ADDR1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Address Status"]
pub mod SASR {
    #[doc = "Received Address"]
    pub mod RADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address not valid"]
    pub mod ANV {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Received Address (RADDR) is valid"]
            pub const VALID: u32 = 0;
            #[doc = "Received Address (RADDR) is not valid"]
            pub const NOT_VALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Transmit ACK"]
pub mod STAR {
    #[doc = "Transmit NACK"]
    pub mod TXNACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write a Transmit ACK for each received word"]
            pub const TRANSMIT_ACK: u32 = 0;
            #[doc = "Write a Transmit NACK for each received word"]
            pub const TRANSMIT_NACK: u32 = 1;
        }
    }
}
#[doc = "Target Transmit Data"]
pub mod STDR {
    #[doc = "Transmit data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Receive Data"]
pub mod SRDR {
    #[doc = "Receive data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Received address"]
    pub mod RADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The Receive Data Register is not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "The Receive Data Register is empty"]
            pub const EMPTY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start of frame"]
    pub mod SOF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not the first data word since a (repeated) START or STOP condition"]
            pub const NOT_FIRST_DATA_WORD: u32 = 0;
            #[doc = "Is the first data word since a (repeated) START or STOP condition"]
            pub const FIRST_DATA_WORD: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Receive Data Read Only"]
pub mod SRDROR {
    #[doc = "Receive data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Received address"]
    pub mod RADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "The Receive Data Register is not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "The Receive Data Register is empty"]
            pub const EMPTY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start of frame"]
    pub mod SOF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Is not the first data word since a (repeated) START or STOP condition"]
            pub const NOT_FIRST_DATA_WORD: u32 = 0;
            #[doc = "Is the first data word since a (repeated) START or STOP condition"]
            pub const FIRST_DATA_WORD: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Transmit Command Burst"]
pub mod MTCBR {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command"]
    pub mod CMD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Data Burst"]
pub mod MTDBR {
    #[doc = "Data"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data"]
    pub mod DATA2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data"]
    pub mod DATA3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
