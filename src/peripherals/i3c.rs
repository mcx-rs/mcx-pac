#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "I3C"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Controller Configuration"]
    pub MCONFIG: crate::RWRegister<u32>,
    #[doc = "Target Configuration"]
    pub SCONFIG: crate::RWRegister<u32>,
    #[doc = "Target Status"]
    pub SSTATUS: crate::RWRegister<u32>,
    #[doc = "Target Control"]
    pub SCTRL: crate::RWRegister<u32>,
    #[doc = "Target Interrupt Set"]
    pub SINTSET: crate::RWRegister<u32>,
    #[doc = "Target Interrupt Clear"]
    pub SINTCLR: crate::RWRegister<u32>,
    #[doc = "Target Interrupt Mask"]
    pub SINTMASKED: crate::RWRegister<u32>,
    #[doc = "Target Errors and Warnings"]
    pub SERRWARN: crate::RWRegister<u32>,
    #[doc = "Target DMA Control"]
    pub SDMACTRL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x8],
    #[doc = "Target Data Control"]
    pub SDATACTRL: crate::RWRegister<u32>,
    #[doc = "Target Write Data Byte"]
    pub SWDATAB: crate::RWRegister<u32>,
    #[doc = "Target Write Data Byte End"]
    pub SWDATABE: crate::RWRegister<u32>,
    #[doc = "Target Write Data Half-word"]
    pub SWDATAH: crate::RWRegister<u32>,
    #[doc = "Target Write Data Half-word End"]
    pub SWDATAHE: crate::RWRegister<u32>,
    #[doc = "Target Read Data Byte"]
    pub SRDATAB: crate::RWRegister<u32>,
    _reserved1: [u8; 0x4],
    #[doc = "Target Read Data Halfword"]
    pub SRDATAH: crate::RWRegister<u32>,
    _reserved2: [u8; 0x8],
    #[doc = "Target Write Data Byte"]
    pub SWDATAB1: crate::RWRegister<u32>,
    _reserved3: [u8; 0x4],
    #[doc = "Target Capabilities 2"]
    pub SCAPABILITIES2: crate::RWRegister<u32>,
    #[doc = "Target Capabilities"]
    pub SCAPABILITIES: crate::RWRegister<u32>,
    #[doc = "Target Dynamic Address"]
    pub SDYNADDR: crate::RWRegister<u32>,
    #[doc = "Target Maximum Limits"]
    pub SMAXLIMITS: crate::RWRegister<u32>,
    #[doc = "Target ID Part Number"]
    pub SIDPARTNO: crate::RWRegister<u32>,
    #[doc = "Target ID Extension"]
    pub SIDEXT: crate::RWRegister<u32>,
    #[doc = "Target Vendor ID"]
    pub SVENDORID: crate::RWRegister<u32>,
    #[doc = "Target Time Control Clock"]
    pub STCCLOCK: crate::RWRegister<u32>,
    #[doc = "Target Message Map Address"]
    pub SMSGMAPADDR: crate::RWRegister<u32>,
    _reserved4: [u8; 0x4],
    #[doc = "Controller Control"]
    pub MCTRL: crate::RWRegister<u32>,
    #[doc = "Controller Status"]
    pub MSTATUS: crate::RWRegister<u32>,
    #[doc = "Controller In-band Interrupt Registry and Rules"]
    pub MIBIRULES: crate::RWRegister<u32>,
    #[doc = "Controller Interrupt Set"]
    pub MINTSET: crate::RWRegister<u32>,
    #[doc = "Controller Interrupt Clear"]
    pub MINTCLR: crate::RWRegister<u32>,
    #[doc = "Controller Interrupt Mask"]
    pub MINTMASKED: crate::RWRegister<u32>,
    #[doc = "Controller Errors and Warnings"]
    pub MERRWARN: crate::RWRegister<u32>,
    #[doc = "Controller DMA Control"]
    pub MDMACTRL: crate::RWRegister<u32>,
    _reserved5: [u8; 0x8],
    #[doc = "Controller Data Control"]
    pub MDATACTRL: crate::RWRegister<u32>,
    #[doc = "Controller Write Data Byte"]
    pub MWDATAB: crate::RWRegister<u32>,
    #[doc = "Controller Write Data Byte End"]
    pub MWDATABE: crate::RWRegister<u32>,
    #[doc = "Controller Write Data Halfword"]
    pub MWDATAH: crate::RWRegister<u32>,
    #[doc = "Controller Write Data Halfword End"]
    pub MWDATAHE: crate::RWRegister<u32>,
    #[doc = "Controller Read Data Byte"]
    pub MRDATAB: crate::RWRegister<u32>,
    _reserved6: [u8; 0x4],
    #[doc = "Controller Read Data Halfword"]
    pub MRDATAH: crate::RWRegister<u32>,
    #[doc = "Controller Write Byte Data 1(to bus)"]
    pub MWDATAB1: crate::RWRegister<u32>,
    #[doc = "Controller Write Message Control in SDR mode"]
    pub MWMSG_SDR_CONTROL: crate::RWRegister<u32>,
    #[doc = "Controller Write Message Data in SDR mode"]
    pub MWMSG_SDR_DATA: crate::RWRegister<u32>,
    #[doc = "Controller Read Message in SDR mode"]
    pub MRMSG_SDR: crate::RWRegister<u32>,
    #[doc = "Controller Write Message in DDR mode: First Control Word"]
    pub MWMSG_DDR_CONTROL: crate::RWRegister<u32>,
    #[doc = "Controller Write Message in DDR mode Control 2"]
    pub MWMSG_DDR_CONTROL2: crate::RWRegister<u32>,
    #[doc = "Controller Write Message Data in DDR mode"]
    pub MWMSG_DDR_DATA: crate::RWRegister<u32>,
    #[doc = "Controller Read Message in DDR mode"]
    pub MRMSG_DDR: crate::RWRegister<u32>,
    #[doc = "Controller Dynamic Address"]
    pub MDYNADDR: crate::RWRegister<u32>,
    _reserved7: [u8; 0x2c],
    #[doc = "Map Feature Control 0"]
    pub SMAPCTRL0: crate::RWRegister<u32>,
    _reserved8: [u8; 0x20],
    #[doc = "Extended IBI Data 1"]
    pub IBIEXT1: crate::RWRegister<u32>,
    #[doc = "Extended IBI Data 2"]
    pub IBIEXT2: crate::RWRegister<u32>,
    _reserved9: [u8; 0xeb4],
    #[doc = "Target Module ID"]
    pub SID: crate::RWRegister<u32>,
}
#[doc = "Controller Configuration"]
pub mod MCONFIG {
    #[doc = "Controller Enable"]
    pub mod MSTENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CONTROLLER_OFF"]
            pub const MASTER_OFF: u32 = 0;
            #[doc = "CONTROLLER_ON"]
            pub const MASTER_ON: u32 = 1;
            #[doc = "CONTROLLER_CAPABLE"]
            pub const MASTER_CAPABLE: u32 = 2;
            #[doc = "I2C_CONTROLLER_MODE"]
            pub const I2C_MASTER_MODE: u32 = 3;
        }
    }
    #[doc = "Disable Timeout"]
    pub mod DISTO {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Timeout enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Timeout disabled, if timeout is configured"]
            pub const DISABLE: u32 = 1;
        }
    }
    #[doc = "High-Keeper"]
    pub mod HKEEP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NONE"]
            pub const NONE: u32 = 0;
            #[doc = "WIRED_IN"]
            pub const WIRED_IN: u32 = 1;
            #[doc = "PASSIVE_SDA"]
            pub const PASSIVE_SDA: u32 = 2;
            #[doc = "PASSIVE_ON_SDA_SCL"]
            pub const PASSIVE_ON_SDA_SCL: u32 = 3;
        }
    }
    #[doc = "Open Drain Stop"]
    pub mod ODSTOP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable open-drain stop. ODSTOP must be disabled when sending an HDR exit pattern."]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable open-drain stop. STOP is emitted at open-drain speeds even for I3C messages. In legacy devices, this feature can ensure that the legacy devices see the STOP."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Push-Pull Baud Rate"]
    pub mod PPBAUD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Push-Pull Low"]
    pub mod PPLOW {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Open Drain Baud Rate"]
    pub mod ODBAUD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Open Drain High Push-Pull"]
    pub mod ODHPP {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ODHPP disabled. Open-Drain SCL High half-clock period is the same as the Open-Drain Low SCL half-period."]
            pub const DISABLE: u32 = 0;
            #[doc = "ODHPP enabled. Open-Drain High SCL half-lock period is one PPBAUD count for I3C messages. This setting is faster (and works for I3C devices). Any legacy I2C devices on the bus will not see the SCL High at all (less than the spike filter period)."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Skew"]
    pub mod SKEW {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2C Baud Rate"]
    pub mod I2CBAUD {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Configuration"]
pub mod SCONFIG {
    #[doc = "Target Enable"]
    pub mod SLVENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Target ignores the I2C or I3C bus"]
            pub const DISABLE: u32 = 0;
            #[doc = "Target can operate on the I2C or I3C bus"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Not Acknowledge"]
    pub mod NACK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Always NACK disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Always NACK enable. The target rejects all requests to it, except for a Common Command Code (CCC) broadcast. NACK = 1 should be used with caution, because the controller may decide that the target is missing, if NACK is overused."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Match START or STOP"]
    pub mod MATCHSS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match START or STOP disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Match START or STOP enable. START and STOP sticky SSTATUS bits only become 1 when SSTATUS[MATCHED] is 1. This setting allows START and STOP to be used to detect the end of a message to/from this target."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Ignore TE0/TE1 Errors"]
    pub mod S0IGNORE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not ignore TE0/TE1 errors"]
            pub const DISABLE: u32 = 0;
            #[doc = "Ignore TE0/TE1 errors. Target does not detect TE0 or TE1 errors, so it does not lock up waiting on an Exit Pattern. This setting should only be used when the bus does not use HDR mode."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Double Data Rate OK"]
    pub mod DDROK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not allow HDR-DDR messaging."]
            pub const DISABLE: u32 = 0;
            #[doc = "Allow HDR-DDR messaging."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "ID random"]
    pub mod IDRAND {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SIDPARTNO[PARTNO] is a part number and an instance."]
            pub const DISABLE: u32 = 0;
            #[doc = "SIDPARTNO[PARTNO] is a random value."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Offline"]
    pub mod OFFLINE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enables wait to ensure the bus is not in HDR mode."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Bus Available Match"]
    pub mod BAMATCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Static Address"]
    pub mod SADDR {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Status"]
pub mod SSTATUS {
    #[doc = "Status Not Stop"]
    pub mod STNOTSTOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "I3C module is in a STOP condition."]
            pub const STOPPED: u32 = 0;
            #[doc = "The bus is busy (has activity)."]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status message"]
    pub mod STMSG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Bus target not listening or responding."]
            pub const IDLE: u32 = 0;
            #[doc = "This bus target is listening to the bus traffic or responding."]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status Common Command Code Handler"]
    pub mod STCCCH {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No CCC message is being handled."]
            pub const IDLE: u32 = 0;
            #[doc = "A CCC message is being handled automatically."]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status Request Read"]
    pub mod STREQRD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "REQ in process is not an SDR read from this target."]
            pub const IDLE: u32 = 0;
            #[doc = "The REQ in process is an SDR read from this target, or an In-Band Interrupt (IBI) is being pushed out."]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status Request Write"]
    pub mod STREQWR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "REQ in process is not SDR write data from the controller."]
            pub const IDLE: u32 = 0;
            #[doc = "REQ in process is SDR write data from the controller to this bus target (or all I3C targets), but not in ENTDAA mode."]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status Dynamic Address Assignment"]
    pub mod STDAA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not in ENTDAA mode."]
            pub const NOT_IN_ENTDAA: u32 = 0;
            #[doc = "I3C bus is in Enter Dynamic Address Assignment (ENTDAA) mode, regardless of whether this bus target has a Dynamic Address or not."]
            pub const IN_ENTDAA: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Status High Data Rate"]
    pub mod STHDR {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "I3C bus not in HDR-DDR mode"]
            pub const NOT_IN_HDR_DDR: u32 = 0;
            #[doc = "The I3C bus is in HDR-DDR mode , regardless of whether HDR mode is supported by this module or not, and regardless of whether the message is to this module or to some other module."]
            pub const IN_HDR_DDR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start"]
    pub mod START {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No START seen."]
            pub const START_NOT_DETECTED: u32 = 0;
            #[doc = "A START or repeated START was seen after the START bit was last cleared."]
            pub const START_DETECTED: u32 = 1;
        }
    }
    #[doc = "Matched"]
    pub mod MATCHED {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No header matched."]
            pub const NOT_MATCHED: u32 = 0;
            #[doc = "An incoming header matched the I3C Dynamic or I2C Static address of this device (if any) since the bus was last cleared."]
            pub const MATCHED: u32 = 1;
        }
    }
    #[doc = "Stop"]
    pub mod STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No STOP detected."]
            pub const NO_STOP_DETECTED: u32 = 0;
            #[doc = "Stopped state detected. A STOP state was present on the bus since the bus was last cleared."]
            pub const STOP_DETECTED: u32 = 1;
        }
    }
    #[doc = "Received Message Pending"]
    pub mod RX_PEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No received message is pending."]
            pub const NO_MSG_PENDING: u32 = 0;
            #[doc = "Received message is pending."]
            pub const MSG_PENDING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Buffer Is Not Full"]
    pub mod TXNOTFULL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit buffer full"]
            pub const FULL: u32 = 0;
            #[doc = "Transmit buffer not full"]
            pub const NOT_FULL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Dynamic Address Change"]
    pub mod DACHG {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No DA change detected."]
            pub const NO_CHANGE_DETECTED: u32 = 0;
            #[doc = "DA change detected. The target DA has been assigned, re-assigned, or reset (lost) and is now in the state of being valid or none."]
            pub const CHANGE_DETECTED: u32 = 1;
        }
    }
    #[doc = "Common Command Code"]
    pub mod CCC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No CCC received."]
            pub const NO_CCC_RECEIVED: u32 = 0;
            #[doc = "CCC received."]
            pub const CCC_RECEIVED: u32 = 1;
        }
    }
    #[doc = "Error Warning"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Data Rate Command Match"]
    pub mod HDRMATCH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HDR command did not match."]
            pub const NO_MATCH: u32 = 0;
            #[doc = "HDR command matched the I3C Dynamic Address of this device."]
            pub const MATCH: u32 = 1;
        }
    }
    #[doc = "Common Command Code Handled"]
    pub mod CHANDLED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "CCC handling not in progress."]
            pub const NOT_HANDLED: u32 = 0;
            #[doc = "CCC handling in progress."]
            pub const HANDLED: u32 = 1;
        }
    }
    #[doc = "Event"]
    pub mod EVENT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No event has occurred."]
            pub const NO_EVENT: u32 = 0;
            #[doc = "An IBI, CR, or HJ has occurred."]
            pub const EVENT: u32 = 1;
        }
    }
    #[doc = "Event Details"]
    pub mod EVDET {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "NONE"]
            pub const NONE: u32 = 0;
            #[doc = "NO_REQUEST"]
            pub const NO_REQUEST: u32 = 1;
            #[doc = "NACKED"]
            pub const NACKED: u32 = 2;
            #[doc = "ACKED"]
            pub const ACKED: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In-Band Interrupts Are Disabled"]
    pub mod IBIDIS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "In-Band Interrupts not disabled"]
            pub const INTERRUPTS_ENABLED: u32 = 0;
            #[doc = "In-Band Interrupts disabled"]
            pub const INTERRUPTS_DISABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controller Requests Are Disabled"]
    pub mod MRDIS {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Controller Requests not disabled"]
            pub const MR_ENABLED: u32 = 0;
            #[doc = "Controller Requests disabled"]
            pub const MR_DISABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hot-Join Disabled"]
    pub mod HJDIS {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Hot-Join not disabled"]
            pub const MR_ENABLED: u32 = 0;
            #[doc = "Hot-Join disabled"]
            pub const MR_DISABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Activity State from Common Command Codes (CCC)"]
    pub mod ACTSTATE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "NO_LATENCY"]
            pub const NO_LATENCY: u32 = 0;
            #[doc = "LATENCY_1MS"]
            pub const LATENCY_1MS: u32 = 1;
            #[doc = "LATENCY_100MS"]
            pub const LATENCY_100MS: u32 = 2;
            #[doc = "LATENCY_10S"]
            pub const LATENCY_10S: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Control"]
    pub mod TIMECTRL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "NO_TIME_CONTROL"]
            pub const NO_TIME_CONTROL: u32 = 0;
            #[doc = "SYNC_MODE"]
            pub const SYNC: u32 = 1;
            #[doc = "ASYNC_MODE"]
            pub const ASYNC_MODE: u32 = 2;
            #[doc = "BOTHSYNCASYNC"]
            pub const BOTHSYNCASYNC: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Control"]
pub mod SCTRL {
    #[doc = "Event"]
    pub mod EVENT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NORMAL_MODE"]
            pub const NORMAL_MODE: u32 = 0;
            #[doc = "IBI"]
            pub const IBI: u32 = 1;
            #[doc = "CONTROLLER_REQUEST"]
            pub const MASTER_REQUEST: u32 = 2;
            #[doc = "HOT_JOIN_REQUEST"]
            pub const HOT_JOIN_REQUEST: u32 = 3;
        }
    }
    #[doc = "Extended Data"]
    pub mod EXTDATA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Extended data disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Extended data enabled. After IBIDATA is emitted, extended data is taken from IBIEXT1 and IBIEXT2 if configured."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "In-Band Interrupt Data"]
    pub mod IBIDATA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pending Interrupt"]
    pub mod PENDINT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Activity State of Target"]
    pub mod ACTSTATE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Vendor Information"]
    pub mod VENDINFO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Interrupt Set"]
pub mod SINTSET {
    #[doc = "Start Interrupt Enable"]
    pub mod START {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable START interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable START interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Match interrupt enable"]
    pub mod MATCHED {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable match interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable match interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Stop Interrupt Enable"]
    pub mod STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable STOP interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable STOP interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Receive Interrupt Enable"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable Receive interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable Receive interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Transmit Interrupt Enable"]
    pub mod TXSEND {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable Transmit interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable Transmit interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Dynamic Address Change Interrupt Enable"]
    pub mod DACHG {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable DA Change interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable DA Change interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "CCC (that was not handled by I3C module) Interrupt Enable"]
    pub mod CCC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable CCC interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable CCC interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Error or Warning Interrupt Enable"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable error or warning interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable error or warning interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Double Data Rate Interrupt Enable"]
    pub mod DDRMATCHED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable DDR interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable DDR interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Common Command Code (CCC) Interrupt Enable"]
    pub mod CHANDLED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable CCC Handled interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable CCC Handled interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event Interrupt Enable"]
    pub mod EVENT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable Event interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable Event interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Target Interrupt Clear"]
pub mod SINTCLR {
    #[doc = "START Interrupt Enable Clear"]
    pub mod START {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MATCHED Interrupt Enable Clear"]
    pub mod MATCHED {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "STOP Interrupt Enable Clear"]
    pub mod STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXPEND Interrupt Enable Clear"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXSEND Interrupt Enable Clear"]
    pub mod TXSEND {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DACHG Interrupt Enable Clear"]
    pub mod DACHG {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CCC Interrupt Enable Clear"]
    pub mod CCC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ERRWARN Interrupt Enable Clear"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DDRMATCHED Interrupt Enable Clear"]
    pub mod DDRMATCHED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHANDLED Interrupt Enable Clear"]
    pub mod CHANDLED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EVENT Interrupt Enable Clear"]
    pub mod EVENT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Interrupt Mask"]
pub mod SINTMASKED {
    #[doc = "START interrupt mask"]
    pub mod START {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MATCHED Interrupt Mask"]
    pub mod MATCHED {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "STOP Interrupt Mask"]
    pub mod STOP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXPEND Interrupt Mask"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXSEND Interrupt Mask"]
    pub mod TXSEND {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DACHG Interrupt Mask"]
    pub mod DACHG {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CCC Interrupt Mask"]
    pub mod CCC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ERRWARN Interrupt Mask"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DDRMATCHED Interrupt Mask"]
    pub mod DDRMATCHED {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHANDLED Interrupt Mask"]
    pub mod CHANDLED {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EVENT Interrupt Mask"]
    pub mod EVENT {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Errors and Warnings"]
pub mod SERRWARN {
    #[doc = "Overrun Error"]
    pub mod ORUN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overrun error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Overrun error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Underrun Error"]
    pub mod URUN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No underrun error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Underrun error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Underrun and Not Acknowledged (NACKED) Error"]
    pub mod URUNNACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No underrun and not acknowledged error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Underrun and not acknowledged error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Terminated Error"]
    pub mod TERM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No terminated error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Terminated error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Invalid Start Error"]
    pub mod INVSTART {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No invalid start error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Invalid start error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "SDR Parity Error"]
    pub mod SPAR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No SDR Parity error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "SDR Parity error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "HDR Parity Error"]
    pub mod HPAR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No HDR Parity error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "HDR Parity error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "HDR-DDR CRC Error"]
    pub mod HCRC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No HDR-DDR CRC error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "HDR-DDR CRC error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "TE0 or TE1 Error"]
    pub mod S0S1 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No TE0 or TE1 error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "TE0 or TE1 error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Over-read Error"]
    pub mod OREAD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Over-read error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Over-read error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Over-write Error"]
    pub mod OWRITE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Overwrite error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Overwrite error"]
            pub const ERROR: u32 = 1;
        }
    }
}
#[doc = "Target DMA Control"]
pub mod SDMACTRL {
    #[doc = "DMA Read (From-bus) Trigger"]
    pub mod DMAFB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA not used"]
            pub const NOT_USED: u32 = 0;
            #[doc = "DMA is enabled for one frame"]
            pub const ENABLE_ONE_FRAME: u32 = 1;
            #[doc = "DMA enabled until turned off"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "DMA Write (To-bus) Trigger"]
    pub mod DMATB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA not used"]
            pub const NOT_USED: u32 = 0;
            #[doc = "DMA enabled for one frame (ended by DMA or terminated)"]
            pub const ENABLE_ONE_FRAME: u32 = 1;
            #[doc = "DMA enabled until turned off"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "Width of DMA Operations"]
    pub mod DMAWIDTH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte"]
            pub const BYTE_0: u32 = 0;
            #[doc = "Byte"]
            pub const BYTE_1: u32 = 1;
            #[doc = "Half word (16 bits)"]
            pub const HALF_WORD: u32 = 2;
        }
    }
}
#[doc = "Target Data Control"]
pub mod SDATACTRL {
    #[doc = "Flush the To-bus Buffer or FIFO"]
    pub mod FLUSHTB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flush the From-bus Buffer or FIFO"]
    pub mod FLUSHFB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unlock"]
    pub mod UNLOCK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "RXTRIG and TXTRIG fields cannot be changed on a write."]
            pub const DISABLED: u32 = 0;
            #[doc = "RXTRIG and TXTRIG fields can be changed on a write."]
            pub const ENABLED: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Transmit Trigger Level"]
    pub mod TXTRIG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger when empty"]
            pub const TRIGGREMPTY: u32 = 0;
            #[doc = "Trigger when 1/4 full or less"]
            pub const TRIGGRONEFOURTH: u32 = 1;
            #[doc = "Trigger when 1/2 full or less"]
            pub const TRIGGRONEHALF: u32 = 2;
            #[doc = "Default. Trigger when 1 less than full or less"]
            pub const TRIGGRONELESS: u32 = 3;
        }
    }
    #[doc = "Receive Trigger Level"]
    pub mod RXTRIG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger when not empty"]
            pub const TRIGGRNOTEMPTY: u32 = 0;
            #[doc = "Trigger when 1/4 or more full"]
            pub const TRIGGRONEFOURTH: u32 = 1;
            #[doc = "Trigger when 1/2 or more full"]
            pub const TRIGGRONEHALF: u32 = 2;
            #[doc = "Trigger when 3/4 or more full"]
            pub const TRIGGRTHREEFOURTHS: u32 = 3;
        }
    }
    #[doc = "Count of Bytes in Transmit"]
    pub mod TXCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count of Bytes in Receive"]
    pub mod RXCOUNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Is Full"]
    pub mod TXFULL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not full"]
            pub const TXISNOTFULL: u32 = 0;
            #[doc = "Full"]
            pub const TXISFULL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Is Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not empty"]
            pub const RXISNOTEMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const RXISEMPTY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Write Data Byte"]
pub mod SWDATAB {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End"]
    pub mod END {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Not the end. There are more bytes in the message."]
            pub const NOT_END: u32 = 0;
            #[doc = "End. This bit marks the last byte of the message."]
            pub const END: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "End Also"]
    pub mod END_ALSO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Not the end. There are more bytes in the message."]
            pub const NOT_END: u32 = 0;
            #[doc = "End. This bit marks the last byte of the message."]
            pub const END: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Target Write Data Byte End"]
pub mod SWDATABE {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Write Data Half-word"]
pub mod SWDATAH {
    #[doc = "Data 0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data 1"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End of message"]
    pub mod END {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Not the end. There are more bytes in the message."]
            pub const NOT_END: u32 = 0;
            #[doc = "End. This bit marks the last byte of the message."]
            pub const END: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Target Write Data Half-word End"]
pub mod SWDATAHE {
    #[doc = "Data 0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data 1"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Read Data Byte"]
pub mod SRDATAB {
    #[doc = "Data 0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Read Data Halfword"]
pub mod SRDATAH {
    #[doc = "The first byte read from the target"]
    pub mod LSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The second byte read from the target"]
    pub mod MSB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Write Data Byte"]
pub mod SWDATAB1 {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Capabilities 2"]
pub mod SCAPABILITIES2 {
    #[doc = "Map Count"]
    pub mod MAPCNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2C 10-bit Address"]
    pub mod I2C10B {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Does not support 10-bit I2C address"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supports 10-bit I2C address"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2C Software Reset"]
    pub mod I2CRST {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Does not support I2C software reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supports I2C software reset"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "I2C Device ID"]
    pub mod I2CDEVID {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Does not support I2C device ID"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supports I2C device ID"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In-Band Interrupt EXTDATA"]
    pub mod IBIEXT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Does not support IBIEXT"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supports IBIEXT"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In-Band Interrupt Extended Register"]
    pub mod IBIXREG {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Does not support extended registers for IBIs"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supports extended registers for IBIs"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target Reset"]
    pub mod SLVRST {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Does not support Target Reset"]
            pub const DISABLE: u32 = 0;
            #[doc = "Supports Target Reset"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Group"]
    pub mod GROUP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Does not supports v1.1 Group addressing"]
            pub const NOTSUPPORTED: u32 = 0;
            #[doc = "Supports one group"]
            pub const ONE: u32 = 1;
            #[doc = "Supports two groups"]
            pub const TWO: u32 = 2;
            #[doc = "Supports three groups"]
            pub const THREE: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Supports SETAASA"]
    pub mod AASA {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Does not support SETAASA"]
            pub const NOTSUPPORTED: u32 = 0;
            #[doc = "Supports SETAASA"]
            pub const SUPPORTED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target-Target(s)-Tunnel Subscriber Capable"]
    pub mod SSTSUB {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not subscriber capable"]
            pub const NOTSUPPORTED: u32 = 0;
            #[doc = "Subscriber capable"]
            pub const SUPPORTED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target-Target(s)-Tunnel Write Capable"]
    pub mod SSTWR {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not write capable"]
            pub const NOTSUPPORTED: u32 = 0;
            #[doc = "Write capable"]
            pub const SUPPORTED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Capabilities"]
pub mod SCAPABILITIES {
    #[doc = "ID 48b Handler"]
    pub mod IDENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Application"]
            pub const APPLICATION: u32 = 0;
            #[doc = "Hardware"]
            pub const HW: u32 = 1;
            #[doc = "Hardware, but the I3C module instance handles ID 48b"]
            pub const HW_BUT: u32 = 2;
            #[doc = "A part number register (PARTNO)"]
            pub const PARTNO: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID Register"]
    pub mod IDREG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "All ID register features below are disabled."]
            pub const ALL_DISABLED: u32 = 0;
            #[doc = "ID Instance is a register, and is used if there is no PARTNO register."]
            pub const ID_INSTANCE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Data Rate Support"]
    pub mod HDRSUPP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "No HDR modes supported"]
            pub const NO_HDR: u32 = 0;
            #[doc = "Double Data Rate mode supported"]
            pub const DDR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controller"]
    pub mod MASTER {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not supported"]
            pub const MASTERNOTSUPPORTED: u32 = 0;
            #[doc = "Supported"]
            pub const MASTERSUPPORTED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Static Address"]
    pub mod SADDR {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "No static address"]
            pub const NO_STATIC: u32 = 0;
            #[doc = "Static address is fixed in hardware"]
            pub const STATIC: u32 = 1;
            #[doc = "Hardware controls the static address dynamically (for example, from the pin strap)"]
            pub const HW_CONTROL: u32 = 2;
            #[doc = "SCONFIG register supplies the static address"]
            pub const CONFIG: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Common Command Codes Handling"]
    pub mod CCCHANDLE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {
            #[doc = "All handling features below are disabled."]
            pub const ALL_DISABLED: u32 = 0;
            #[doc = "The block (I3C module) manages events, activities, status, HDR, and if enabled for it, ID and static-address-related items."]
            pub const BLOCK_HANDLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In-Band Interrupts, Controller Requests, Hot-Join Events"]
    pub mod IBI_MR_HJ {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {
            #[doc = "Application cannot generate IBI, CR, or HJ."]
            pub const ALL_DISABLED: u32 = 0;
            #[doc = "Application can generate an IBI."]
            pub const IBI: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Time Control"]
    pub mod TIMECTRL {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No time control enabled"]
            pub const NO_TIME_CONTROL_TYPE: u32 = 0;
            #[doc = "At least one time-control type supported"]
            pub const ATLEAST1_TIME_CONTROL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External FIFO"]
    pub mod EXTFIFO {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "No external FIFO is available"]
            pub const NO_EXT_FIFO: u32 = 0;
            #[doc = "Standard available or free external FIFO"]
            pub const STD_EXT_FIFO: u32 = 1;
            #[doc = "Request track external FIFO"]
            pub const REQUEST_EXT_FIFO: u32 = 2;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Transmit"]
    pub mod FIFOTX {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Two"]
            pub const FIFO_2BYTE: u32 = 0;
            #[doc = "Four"]
            pub const FIFO_4BYTE: u32 = 1;
            #[doc = "Eight"]
            pub const FIFO_8BYTE: u32 = 2;
            #[doc = "16 or larger"]
            pub const FIFO_16BYTE: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Receive"]
    pub mod FIFORX {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Two or three"]
            pub const FIFO_2BYTE: u32 = 0;
            #[doc = "Four"]
            pub const FIFO_4BYTE: u32 = 1;
            #[doc = "Eight"]
            pub const FIFO_8BYTE: u32 = 2;
            #[doc = "16 or larger"]
            pub const FIFO_16BYTE: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupts"]
    pub mod INT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not supported"]
            pub const INTERRUPTSNO: u32 = 0;
            #[doc = "Supported"]
            pub const INTERRUPTSYES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Direct Memory Access"]
    pub mod DMA {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not supported"]
            pub const DMANO: u32 = 0;
            #[doc = "Supported"]
            pub const DMAYES: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Dynamic Address"]
pub mod SDYNADDR {
    #[doc = "Dynamic Address Valid"]
    pub mod DAVALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DANOTASSIGNED: a Dynamic Address is not assigned"]
            pub const DANOTASSIGNED: u32 = 0;
            #[doc = "DAASSIGNED: a Dynamic Address is assigned"]
            pub const DAASSIGNED: u32 = 1;
        }
    }
    #[doc = "Dynamic Address"]
    pub mod DADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Map a Static Address"]
    pub mod MAPSA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "10bit Static Address"]
    pub mod SA10B {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Key"]
    pub mod KEY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Maximum Limits"]
pub mod SMAXLIMITS {
    #[doc = "Maximum Read Length"]
    pub mod MAXRD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum Write Length"]
    pub mod MAXWR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target ID Part Number"]
pub mod SIDPARTNO {
    #[doc = "Part number"]
    pub mod PARTNO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target ID Extension"]
pub mod SIDEXT {
    #[doc = "Device Characteristic Register"]
    pub mod DCR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus Characteristics Register"]
    pub mod BCR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Vendor ID"]
pub mod SVENDORID {
    #[doc = "Vendor ID"]
    pub mod VID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Time Control Clock"]
pub mod STCCLOCK {
    #[doc = "Clock Accuracy"]
    pub mod ACCURACY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock Frequency"]
    pub mod FREQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Message Map Address"]
pub mod SMSGMAPADDR {
    #[doc = "Matched Address Index"]
    pub mod MAPLAST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Last Static Address Matched"]
    pub mod LASTSTATIC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "I3C dynamic address"]
            pub const I3C: u32 = 0;
            #[doc = "I2C static address"]
            pub const I2C: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Matched Previous Address Index 1"]
    pub mod MAPLASTM1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Matched Previous Index 2"]
    pub mod MAPLASTM2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Control"]
pub mod MCTRL {
    #[doc = "Request"]
    pub mod REQUEST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NONE"]
            pub const NONE: u32 = 0;
            #[doc = "EMITSTARTADDR"]
            pub const EMITSTARTADDR: u32 = 1;
            #[doc = "EMITSTOP"]
            pub const EMITSTOP: u32 = 2;
            #[doc = "IBIACKNACK"]
            pub const IBIACKNACK: u32 = 3;
            #[doc = "PROCESSDAA"]
            pub const PROCESSDAA: u32 = 4;
            #[doc = "Force Exit and Target Reset"]
            pub const FORCEEXIT: u32 = 6;
            #[doc = "AUTOIBI"]
            pub const AUTOIBI: u32 = 7;
        }
    }
    #[doc = "Bus Type with EmitStartAddr"]
    pub mod TYPE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "I3C"]
            pub const I3C: u32 = 0;
            #[doc = "I2C"]
            pub const I2C: u32 = 1;
            #[doc = "DDR"]
            pub const DDR: u32 = 2;
        }
    }
    #[doc = "In-Band Interrupt Response"]
    pub mod IBIRESP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ACK (acknowledge)"]
            pub const ACK: u32 = 0;
            #[doc = "NACK (reject)"]
            pub const NACK: u32 = 1;
            #[doc = "Acknowledge with mandatory byte"]
            pub const ACK_WITH_MANDATORY: u32 = 2;
            #[doc = "Manual"]
            pub const MANUAL: u32 = 3;
        }
    }
    #[doc = "Direction"]
    pub mod DIR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write"]
            pub const DIRWRITE: u32 = 0;
            #[doc = "Read"]
            pub const DIRREAD: u32 = 1;
        }
    }
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Terminate Counter"]
    pub mod RDTERM {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Status"]
pub mod MSTATUS {
    #[doc = "State Of The Controller"]
    pub mod STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "IDLE"]
            pub const IDLE: u32 = 0;
            #[doc = "SLVREQ"]
            pub const SLVREQ: u32 = 1;
            #[doc = "MSGSDR"]
            pub const MSGSDR: u32 = 2;
            #[doc = "NORMACT"]
            pub const NORMACT: u32 = 3;
            #[doc = "MSGDDR"]
            pub const DDR: u32 = 4;
            #[doc = "DAA"]
            pub const DAA: u32 = 5;
            #[doc = "IBIACK"]
            pub const IBIACK: u32 = 6;
            #[doc = "IBIRCV"]
            pub const IBIRCV: u32 = 7;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Between"]
    pub mod BETWEEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Inactive"]
            pub const INACTIVE: u32 = 0;
            #[doc = "Active"]
            pub const ACTIVE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Not Acknowledged"]
    pub mod NACKED {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not NACKed"]
            pub const NOT_NACKED: u32 = 0;
            #[doc = "NACKed (not acknowledged)"]
            pub const NACKED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In-Band Interrupt (IBI) Type"]
    pub mod IBITYPE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "NONE"]
            pub const NONE: u32 = 0;
            #[doc = "In-Band Interrupt"]
            pub const IBI: u32 = 1;
            #[doc = "Controller Request"]
            pub const MR: u32 = 2;
            #[doc = "Hot-Join"]
            pub const HJ: u32 = 3;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Target Start"]
    pub mod SLVSTART {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Target not requesting START"]
            pub const NOT_START: u32 = 0;
            #[doc = "Target requesting START"]
            pub const START: u32 = 1;
        }
    }
    #[doc = "Controller Control Done"]
    pub mod MCTRLDONE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not done"]
            pub const NOT_DONE: u32 = 0;
            #[doc = "Done"]
            pub const DONE: u32 = 1;
        }
    }
    #[doc = "Complete"]
    pub mod COMPLETE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not complete"]
            pub const NOT_COMPLETE: u32 = 0;
            #[doc = "Complete"]
            pub const COMPLETE: u32 = 1;
        }
    }
    #[doc = "RXPEND"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No receive message pending"]
            pub const IDLE: u32 = 0;
            #[doc = "Receive message pending"]
            pub const PENDING: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX Buffer or FIFO Not Full"]
    pub mod TXNOTFULL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive buffer or FIFO full"]
            pub const FULL: u32 = 0;
            #[doc = "Receive buffer or FIFO not full"]
            pub const NOTFULL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "In-Band Interrupt (IBI) Won"]
    pub mod IBIWON {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No IBI arbitration won"]
            pub const NOT_WON: u32 = 0;
            #[doc = "IBI arbitration won"]
            pub const WON: u32 = 1;
        }
    }
    #[doc = "Error Or Warning"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No error or warning"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error or warning"]
            pub const ERROR: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Module Is Now Controller"]
    pub mod NOWMASTER {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Module has not become controller"]
            pub const NOT_MASTER: u32 = 0;
            #[doc = "Module has become controller"]
            pub const MASTER: u32 = 1;
        }
    }
    #[doc = "IBI Address"]
    pub mod IBIADDR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller In-band Interrupt Registry and Rules"]
pub mod MIBIRULES {
    #[doc = "ADDR0"]
    pub mod ADDR0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADDR1"]
    pub mod ADDR1 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADDR2"]
    pub mod ADDR2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADDR3"]
    pub mod ADDR3 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ADDR4"]
    pub mod ADDR4 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Most Significant Address Bit Is 0"]
    pub mod MSB0 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MSB is not 0."]
            pub const DISABLE: u32 = 0;
            #[doc = "For all I3C dynamic addresses, MSB is 0."]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "No IBI byte"]
    pub mod NOBYTE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "With mandatory IBI byte"]
            pub const IBIBYTE: u32 = 0;
            #[doc = "Without mandatory IBI byte"]
            pub const NO_IBIBYTE: u32 = 1;
        }
    }
}
#[doc = "Controller Interrupt Set"]
pub mod MINTSET {
    #[doc = "Target Start Interrupt Enable"]
    pub mod SLVSTART {
        pub const offset: u32 = 8;
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
    #[doc = "Controller Control Done Interrupt Enable"]
    pub mod MCTRLDONE {
        pub const offset: u32 = 9;
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
    #[doc = "Completed Message Interrupt Enable"]
    pub mod COMPLETE {
        pub const offset: u32 = 10;
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
    #[doc = "Rx Pending Interrupt Enable"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TX buffer/FIFO is not full interrupt enable"]
    pub mod TXNOTFULL {
        pub const offset: u32 = 12;
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
    #[doc = "In-Band Interrupt (IBI) Won Interrupt Enable"]
    pub mod IBIWON {
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
    #[doc = "Error or Warning (ERRWARN) Interrupt Enable"]
    pub mod ERRWARN {
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
    #[doc = "Now Controller (now this I3C module is a controller) Interrupt Enable"]
    pub mod NOWMASTER {
        pub const offset: u32 = 19;
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
#[doc = "Controller Interrupt Clear"]
pub mod MINTCLR {
    #[doc = "SLVSTART Interrupt Enable Clear"]
    pub mod SLVSTART {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Corresponding interrupt enable becomes 0"]
            pub const CLEAR: u32 = 1;
        }
    }
    #[doc = "MCTRLDONE Interrupt Enable Clear"]
    pub mod MCTRLDONE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Corresponding interrupt enable becomes 0"]
            pub const CLEAR: u32 = 1;
        }
    }
    #[doc = "COMPLETE Interrupt Enable Clear"]
    pub mod COMPLETE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Corresponding interrupt enable becomes 0"]
            pub const CLEAR: u32 = 1;
        }
    }
    #[doc = "RXPEND Interrupt Enable Clear"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Corresponding interrupt enable becomes 0"]
            pub const CLEAR: u32 = 1;
        }
    }
    #[doc = "TXNOTFULL Interrupt Enable Clear"]
    pub mod TXNOTFULL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Corresponding interrupt enable becomes 0"]
            pub const CLEAR: u32 = 1;
        }
    }
    #[doc = "IBIWON Interrupt Enable Clear"]
    pub mod IBIWON {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Corresponding interrupt enable becomes 0"]
            pub const CLEAR: u32 = 1;
        }
    }
    #[doc = "ERRWARN Interrupt Enable Clear"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Corresponding interrupt enable becomes 0"]
            pub const CLEAR: u32 = 1;
        }
    }
    #[doc = "NOWCONTROLLER Interrupt Enable Clear"]
    pub mod NOWMASTER {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NONE: u32 = 0;
            #[doc = "Corresponding interrupt enable becomes 0"]
            pub const CLEAR: u32 = 1;
        }
    }
}
#[doc = "Controller Interrupt Mask"]
pub mod MINTMASKED {
    #[doc = "SLVSTART Interrupt Mask"]
    pub mod SLVSTART {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Interrupt not enabled and/or not active"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Interrupt enabled and active"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MCTRLDONE Interrupt Mask"]
    pub mod MCTRLDONE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Interrupt not enabled and/or not active"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Interrupt enabled and active"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "COMPLETE Interrupt Mask"]
    pub mod COMPLETE {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Interrupt not enabled and/or not active"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Interrupt enabled and active"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RXPEND Interrupt Mask"]
    pub mod RXPEND {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TXNOTFULL Interrupt Mask"]
    pub mod TXNOTFULL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Interrupt not enabled and/or not active"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Interrupt enabled and active"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "IBIWON Interrupt Mask"]
    pub mod IBIWON {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Interrupt not enabled and/or not active"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Interrupt enabled and active"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ERRWARN Interrupt Mask"]
    pub mod ERRWARN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Interrupt not enabled and/or not active"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Interrupt enabled and active"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NOWCONTROLLER Interrupt Mask"]
    pub mod NOWMASTER {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Interrupt not enabled and/or not active"]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Interrupt enabled and active"]
            pub const ENABLED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Errors and Warnings"]
pub mod MERRWARN {
    #[doc = "Underrun error"]
    pub mod URUN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Not Acknowledge Error"]
    pub mod NACK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Write Abort Error"]
    pub mod WRABT {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Terminate Error"]
    pub mod TERM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "High Data Rate Parity"]
    pub mod HPAR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "High Data Rate CRC Error"]
    pub mod HCRC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Over-read Error"]
    pub mod OREAD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Over-write Error"]
    pub mod OWRITE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Message Error"]
    pub mod MSGERR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Invalid Request Error"]
    pub mod INVREQ {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Timeout Error"]
    pub mod TIMEOUT {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error"]
            pub const ERROR: u32 = 1;
        }
    }
}
#[doc = "Controller DMA Control"]
pub mod MDMACTRL {
    #[doc = "DMA From Bus"]
    pub mod DMAFB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA is not used"]
            pub const NOT_USED: u32 = 0;
            #[doc = "Enable DMA for one frame"]
            pub const ENABLE_ONE_FRAME: u32 = 1;
            #[doc = "Enable DMA until DMA is turned off"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "DMA To Bus"]
    pub mod DMATB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DMA is not used"]
            pub const NOT_USED: u32 = 0;
            #[doc = "Enable DMA for one frame (ended by DMA or Terminated)"]
            pub const ENABLE_ONE_FRAME: u32 = 1;
            #[doc = "Enable DMA until DMA is turned off"]
            pub const ENABLE: u32 = 2;
        }
    }
    #[doc = "DMA Width"]
    pub mod DMAWIDTH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte"]
            pub const BYTE_0: u32 = 0;
            #[doc = "Byte"]
            pub const BYTE_1: u32 = 1;
            #[doc = "Halfword (16 bits)"]
            pub const HALF_WORD: u32 = 2;
        }
    }
}
#[doc = "Controller Data Control"]
pub mod MDATACTRL {
    #[doc = "Flush To-bus Buffer or FIFO"]
    pub mod FLUSHTB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No action"]
            pub const NO_ACTION: u32 = 0;
            #[doc = "Flush the buffer"]
            pub const FLUSH: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Flush From-bus Buffer or FIFO"]
    pub mod FLUSHFB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "No action"]
            pub const NO_ACTION: u32 = 0;
            #[doc = "Flush the buffer"]
            pub const FLUSH: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Unlock"]
    pub mod UNLOCK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Locked. RXTRIG and TXTRIG fields cannot be changed on a write."]
            pub const DISABLED: u32 = 0;
            #[doc = "Unlocked. RXTRIG and TXTRIG fields can be changed on a write."]
            pub const ENABLED: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Transmit Trigger Level"]
    pub mod TXTRIG {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger when empty"]
            pub const EMPTY: u32 = 0;
            #[doc = "Trigger when 1/4 full or less"]
            pub const QUARTER_OR_LESS: u32 = 1;
            #[doc = "Trigger when 1/2 full or less"]
            pub const HALF_OR_LESS: u32 = 2;
            #[doc = "Default. Trigger when 1 less than full or less"]
            pub const FULL_OR_LESS: u32 = 3;
        }
    }
    #[doc = "Receive Trigger Level"]
    pub mod RXTRIG {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Trigger when not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Trigger when 1/4 full or more"]
            pub const QUARTER_OR_MORE: u32 = 1;
            #[doc = "Trigger when 1/2 full or more"]
            pub const HALF_OR_MORE: u32 = 2;
            #[doc = "Trigger when 3/4 full or more"]
            pub const THREE_QUARTER_OR_MORE: u32 = 3;
        }
    }
    #[doc = "Transmit Byte Count"]
    pub mod TXCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Byte Count"]
    pub mod RXCOUNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Is Full"]
    pub mod TXFULL {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Transmit FIFO or buffer is not yet full."]
            pub const NOT_FULL: u32 = 0;
            #[doc = "Transmit FIFO or buffer is full."]
            pub const FULL: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Is Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Receive FIFO or buffer is not yet empty."]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Receive FIFO or buffer is empty."]
            pub const EMPTY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Write Data Byte"]
pub mod MWDATAB {
    #[doc = "Data Byte"]
    pub mod VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End of Message"]
    pub mod END {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Not the end. More bytes are assumed to be in the message."]
            pub const NOT_END: u32 = 0;
            #[doc = "End. The END bit marks the last byte of the message."]
            pub const END: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "End of Message Also"]
    pub mod END_ALSO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Not the end. More bytes are assumed to be in the message."]
            pub const NOT_END: u32 = 0;
            #[doc = "End. The END bit marks the last byte of the message."]
            pub const END: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Controller Write Data Byte End"]
pub mod MWDATABE {
    #[doc = "Data"]
    pub mod VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Write Data Halfword"]
pub mod MWDATAH {
    #[doc = "Data Byte 0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Byte 1"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End of message"]
    pub mod END {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Not the end. More bytes are assumed to be in the message."]
            pub const NOT_END: u32 = 0;
            #[doc = "End. The END bit marks the last byte of the message."]
            pub const END: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Controller Write Data Halfword End"]
pub mod MWDATAHE {
    #[doc = "Data Byte 0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Byte 1"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Read Data Byte"]
pub mod MRDATAB {
    #[doc = "Value"]
    pub mod VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Read Data Halfword"]
pub mod MRDATAH {
    #[doc = "LSB"]
    pub mod LSB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "MSB"]
    pub mod MSB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Write Byte Data 1(to bus)"]
pub mod MWDATAB1 {
    #[doc = "Value"]
    pub mod VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Write Message Control in SDR mode"]
pub mod MWMSG_SDR_CONTROL {
    #[doc = "Direction"]
    pub mod DIR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Write"]
            pub const WRITE: u32 = 0;
            #[doc = "Read"]
            pub const READ: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End of SDR Message"]
    pub mod END {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Not the end. SDR message ends waiting for a new SDR message (issues a repeated START for a new message)."]
            pub const NOT_END: u32 = 0;
            #[doc = "End. SDR message ends at the STOP."]
            pub const END: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "I2C"]
    pub mod I2C {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "I3C message"]
            pub const I3CMESSAGE: u32 = 0;
            #[doc = "I2C message"]
            pub const I2CMESSAGE: u32 = 1;
        }
        pub mod RW {}
    }
    #[doc = "Length"]
    pub mod LEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Write Message Data in SDR mode"]
pub mod MWMSG_SDR_DATA {
    #[doc = "Data"]
    pub mod DATA16B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Read Message in SDR mode"]
pub mod MRMSG_SDR {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Write Message in DDR mode: First Control Word"]
pub mod MWMSG_DDR_CONTROL {
    #[doc = "Address Command"]
    pub mod ADDRCMD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Write Message in DDR mode Control 2"]
pub mod MWMSG_DDR_CONTROL2 {
    #[doc = "Length of Message"]
    pub mod LEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "End of message"]
    pub mod END {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {
            #[doc = "Not the end. DDR message ends waiting for a new DDR message (will issue a HDR Restart for the new message)."]
            pub const NOT_END: u32 = 0;
            #[doc = "End. DDR message ends on HDR Exit."]
            pub const END: u32 = 1;
        }
        pub mod RW {}
    }
}
#[doc = "Controller Write Message Data in DDR mode"]
pub mod MWMSG_DDR_DATA {
    #[doc = "Data"]
    pub mod DATA16B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Read Message in DDR mode"]
pub mod MRMSG_DDR {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Dynamic Address"]
pub mod MDYNADDR {
    #[doc = "Dynamic address valid"]
    pub mod DAVALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No valid DA assigned"]
            pub const NO_VALID: u32 = 0;
            #[doc = "Valid DA assigned"]
            pub const VALID: u32 = 1;
        }
    }
    #[doc = "Dynamic address"]
    pub mod DADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Map Feature Control 0"]
pub mod SMAPCTRL0 {
    #[doc = "Enable Primary Dynamic Address"]
    pub mod ENA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Dynamic Address"]
    pub mod DA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Cause"]
    pub mod CAUSE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {
            #[doc = "No information. This value occurs when not configured to write DA."]
            pub const NONE: u32 = 0;
            #[doc = "Set using ENTDAA"]
            pub const ENTDAA: u32 = 1;
            #[doc = "Set using SETDASA, SETAASA, or SETNEWDA"]
            pub const SETDASA: u32 = 2;
            #[doc = "Cleared using RSTDAA"]
            pub const RSTDAA: u32 = 3;
            #[doc = "Auto MAP change happened last. The change may have changed this DA as well (for example, ENTDAA, and SETAASA), but at least one MAP entry automatically changed after."]
            pub const AUTOMAP: u32 = 4;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Extended IBI Data 1"]
pub mod IBIEXT1 {
    #[doc = "Count"]
    pub mod CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum"]
    pub mod MAX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extra byte 1"]
    pub mod EXT1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extra byte 2"]
    pub mod EXT2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extra byte 3"]
    pub mod EXT3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Extended IBI Data 2"]
pub mod IBIEXT2 {
    #[doc = "Extra byte 4"]
    pub mod EXT4 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extra byte 5"]
    pub mod EXT5 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extra byte 6"]
    pub mod EXT6 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Extra byte 7"]
    pub mod EXT7 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Module ID"]
pub mod SID {
    #[doc = "ID"]
    pub mod ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
