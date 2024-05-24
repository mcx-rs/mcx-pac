#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "RTC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Year and Month Counters"]
    pub YEARMON: crate::RWRegister<u16>,
    #[doc = "Days and Day-of-Week Counters"]
    pub DAYS: crate::RWRegister<u16>,
    #[doc = "Hours and Minutes Counters"]
    pub HOURMIN: crate::RWRegister<u16>,
    #[doc = "Seconds Counters"]
    pub SECONDS: crate::RWRegister<u16>,
    #[doc = "Year and Months Alarm"]
    pub ALM_YEARMON: crate::RWRegister<u16>,
    #[doc = "Days Alarm"]
    pub ALM_DAYS: crate::RWRegister<u16>,
    #[doc = "Hours and Minutes Alarm"]
    pub ALM_HOURMIN: crate::RWRegister<u16>,
    #[doc = "Seconds Alarm"]
    pub ALM_SECONDS: crate::RWRegister<u16>,
    #[doc = "Control"]
    pub CTRL: crate::RWRegister<u16>,
    #[doc = "Status"]
    pub STATUS: crate::RWRegister<u16>,
    #[doc = "Interrupt Status"]
    pub ISR: crate::RWRegister<u16>,
    #[doc = "Interrupt Enable"]
    pub IER: crate::RWRegister<u16>,
    _reserved0: [u8; 0x4],
    #[doc = "Sub Second Counter"]
    pub RTC_TEST2: crate::RWRegister<u16>,
    _reserved1: [u8; 0x4],
    #[doc = "Daylight Saving Hour"]
    pub DST_HOUR: crate::RWRegister<u16>,
    #[doc = "Daylight Saving Month"]
    pub DST_MONTH: crate::RWRegister<u16>,
    #[doc = "Daylight Saving Day"]
    pub DST_DAY: crate::RWRegister<u16>,
    #[doc = "Compensation"]
    pub COMPEN: crate::RWRegister<u16>,
}
#[doc = "Year and Month Counters"]
pub mod YEARMON {
    #[doc = "Month Counter"]
    pub mod MON_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Illegal Value"]
            pub const ILLEGAL_VALUE_0: u32 = 0;
            #[doc = "January"]
            pub const JANUARY: u32 = 1;
            #[doc = "February"]
            pub const FEBRUARY: u32 = 2;
            #[doc = "March"]
            pub const MARCH: u32 = 3;
            #[doc = "April"]
            pub const APRIL: u32 = 4;
            #[doc = "May"]
            pub const MAY: u32 = 5;
            #[doc = "June"]
            pub const JUNE: u32 = 6;
            #[doc = "July"]
            pub const JULY: u32 = 7;
            #[doc = "August"]
            pub const AUGUST: u32 = 8;
            #[doc = "September"]
            pub const SEPTEMBER: u32 = 9;
            #[doc = "October"]
            pub const OCTOBER: u32 = 10;
            #[doc = "November"]
            pub const NOVEMBER: u32 = 11;
            #[doc = "December"]
            pub const DECEMBER: u32 = 12;
            #[doc = "Illegal Value"]
            pub const ILLEGAL_VALUE_13: u32 = 13;
            #[doc = "Illegal Value"]
            pub const ILLEGAL_VALUE_14: u32 = 14;
            #[doc = "Illegal Value"]
            pub const ILLEGAL_VALUE_15: u32 = 15;
        }
    }
    #[doc = "Year Offset Count Value"]
    pub mod YROFST {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Days and Day-of-Week Counters"]
pub mod DAYS {
    #[doc = "Days Counter Value"]
    pub mod DAY_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Day of Week Counter Value"]
    pub mod DOW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sunday"]
            pub const SUNDAY: u32 = 0;
            #[doc = "Monday"]
            pub const MONDAY: u32 = 1;
            #[doc = "Tuesday"]
            pub const TUESDAY: u32 = 2;
            #[doc = "Wednesday"]
            pub const WEDNESDAY: u32 = 3;
            #[doc = "Thursday"]
            pub const THURSDAY: u32 = 4;
            #[doc = "Friday"]
            pub const FRIDAY: u32 = 5;
            #[doc = "Saturday"]
            pub const SATURDAY: u32 = 6;
        }
    }
}
#[doc = "Hours and Minutes Counters"]
pub mod HOURMIN {
    #[doc = "Minutes Counter Value"]
    pub mod MIN_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hours Counter Value"]
    pub mod HOUR_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Seconds Counters"]
pub mod SECONDS {
    #[doc = "Seconds Counter Value"]
    pub mod SEC_CNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Year and Months Alarm"]
pub mod ALM_YEARMON {
    #[doc = "Months Value for Alarm"]
    pub mod ALM_MON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Year Value for Alarm"]
    pub mod ALM_YEAR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Days Alarm"]
pub mod ALM_DAYS {
    #[doc = "Days Value for Alarm"]
    pub mod ALM_DAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hours and Minutes Alarm"]
pub mod ALM_HOURMIN {
    #[doc = "Minutes Value for Alarm"]
    pub mod ALM_MIN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Hours Value for Alarm"]
    pub mod ALM_HOUR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Seconds Alarm"]
pub mod ALM_SECONDS {
    #[doc = "Seconds Alarm Value"]
    pub mod ALM_SEC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decrement Seconds Counter by 1."]
    pub mod DEC_SEC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Increment Seconds Counter by 1."]
    pub mod INC_SEC {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control"]
pub mod CTRL {
    #[doc = "Fine Compensation Enable"]
    pub mod FINEEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fine compensation is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Fine compensation is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Compensation Enable"]
    pub mod COMP_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Coarse compensation is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Coarse compensation is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Alarm Match"]
    pub mod ALM_MATCH {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Only seconds, minutes, and hours matched."]
            pub const HOURS: u32 = 0;
            #[doc = "Only seconds, minutes, hours, and days matched."]
            pub const DAYS: u32 = 1;
            #[doc = "Only seconds, minutes, hours, days, and months matched."]
            pub const MONTHS: u32 = 2;
            #[doc = "Only seconds, minutes, hours, days, months, and year (offset) matched."]
            pub const YEAR: u32 = 3;
        }
    }
    #[doc = "Daylight Saving Enable"]
    pub mod DST_EN {
        pub const offset: u32 = 6;
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
    pub mod SWR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Software Reset cleared"]
            pub const CLEARED: u32 = 0;
            #[doc = "Software Reset asserted"]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "RTC Clock Select"]
    pub mod CLK_SEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16.384 kHz clock is selected"]
            pub const CLK_16_384KHZ: u32 = 0;
            #[doc = "32.768 kHz clock is selected"]
            pub const CLK_32_768KHZ: u32 = 1;
        }
    }
    #[doc = "Clock Output Disable"]
    pub mod CLKO_DIS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The selected clock is output to other peripherals."]
            pub const TO_OTHER_PERIPHERALS: u32 = 0;
            #[doc = "The selected clock is not output to other peripherals."]
            pub const NOT_TO_OTHER_PERIPHERALS: u32 = 1;
        }
    }
    #[doc = "RTC Clock Output Selection"]
    pub mod CLKOUT {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No output clock"]
            pub const NO_OUTPUT_CLK: u32 = 0;
            #[doc = "Fine 1 Hz clock with both precise edges"]
            pub const CLK_1HZ_FINE: u32 = 1;
            #[doc = "32.768 or 16.384 kHz clock"]
            pub const CLK_32_168KHZ: u32 = 2;
            #[doc = "Coarse 1 Hz clock with both precise edges"]
            pub const CLK_1HZ_COARSE: u32 = 3;
        }
    }
}
#[doc = "Status"]
pub mod STATUS {
    #[doc = "Invalidate CPU Read/Write Access"]
    pub mod INVAL_BIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Time and date counters can be read or written. Time and date is valid."]
            pub const VALID: u32 = 0;
            #[doc = "Time and date counter values are changing or time and date is invalid and cannot be read or written."]
            pub const INVALID: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Protect Enable Status"]
    pub mod WRITE_PROT_EN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Registers are unlocked and can be accessed."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Registers are locked and in read-only mode."]
            pub const LOCKED: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Compensation Interval"]
    pub mod CMP_INT {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Enable"]
    pub mod WE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable Write Protection - Registers are locked."]
            pub const LOCKED: u32 = 2;
        }
    }
    #[doc = "Bus Error"]
    pub mod BUS_ERR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read and write accesses are normal."]
            pub const NORMAL: u32 = 0;
            #[doc = "Read or write accesses occurred when STATUS[INVAL_BIT] was asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Compensation Done"]
    pub mod CMP_DONE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Compensation busy or not enabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Compensation completed"]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Interrupt Status"]
pub mod ISR {
    #[doc = "Alarm Interrupt Status"]
    pub mod ALM_IS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Days Interrupt Status"]
    pub mod DAY_IS {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Hours Interrupt Status"]
    pub mod HOUR_IS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "Minutes Interrupt Status"]
    pub mod MIN_IS {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "1 Hz Interval Interrupt Status"]
    pub mod IS_1HZ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "2 Hz Interval Interrupt Status"]
    pub mod IS_2HZ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "4 Hz Interval Interrupt Status"]
    pub mod IS_4HZ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "8 Hz Interval Interrupt Status"]
    pub mod IS_8HZ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "16 Hz Interval Interrupt Status"]
    pub mod IS_16HZ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "32 Hz Interval Interrupt Status"]
    pub mod IS_32HZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "64 Hz Interval Interrupt Status"]
    pub mod IS_64HZ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "128 Hz Interval Interrupt Status"]
    pub mod IS_128HZ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "256 Hz Interval Interrupt Status"]
    pub mod IS_256HZ {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
    #[doc = "512 Hz Interval Interrupt Status"]
    pub mod IS_512HZ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is de-asserted."]
            pub const DEASSERTED: u32 = 0;
            #[doc = "Interrupt is asserted."]
            pub const ASSERTED: u32 = 1;
        }
    }
}
#[doc = "Interrupt Enable"]
pub mod IER {
    #[doc = "Alarm Interrupt Enable"]
    pub mod ALM_IE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Days Interrupt Enable"]
    pub mod DAY_IE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Hours Interrupt Enable"]
    pub mod HOUR_IE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "Minutes Interrupt Enable"]
    pub mod MIN_IE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "1 Hz Interval Interrupt Enable"]
    pub mod IE_1HZ {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "2 Hz Interval Interrupt Enable"]
    pub mod IE_2HZ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "4 Hz Interval Interrupt Enable"]
    pub mod IE_4HZ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "8 Hz Interval Interrupt Enable"]
    pub mod IE_8HZ {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "16 Hz Interval Interrupt Enable"]
    pub mod IE_16HZ {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "32 Hz Interval Interrupt Enable"]
    pub mod IE_32HZ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "64 Hz Interval Interrupt Enable"]
    pub mod IE_64HZ {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "128 Hz Interval Interrupt Enable"]
    pub mod IE_128HZ {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "256 Hz Interval Interrupt Enable"]
    pub mod IE_256HZ {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
    #[doc = "512 Hz Interval Interrupt Enable"]
    pub mod IE_512HZ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Interrupt is enabled."]
            pub const ENABLED: u32 = 1;
        }
    }
}
#[doc = "Sub Second Counter"]
pub mod RTC_TEST2 {
    #[doc = "Sub Second Counter Value"]
    pub mod SUB_SECOND_COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Daylight Saving Hour"]
pub mod DST_HOUR {
    #[doc = "Daylight Saving Time (DST) Hours End Value"]
    pub mod DST_END_HOUR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Daylight Saving Time (DST) Hours Start Value"]
    pub mod DST_START_HOUR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Daylight Saving Month"]
pub mod DST_MONTH {
    #[doc = "Daylight Saving Time (DST) Month End Value"]
    pub mod DST_END_MONTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Daylight Saving Time (DST) Month Start Value"]
    pub mod DST_START_MONTH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Daylight Saving Day"]
pub mod DST_DAY {
    #[doc = "Daylight Saving Time (DST) Day End Value"]
    pub mod DST_END_DAY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Daylight Saving Time (DST) Day Start Value"]
    pub mod DST_START_DAY {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Compensation"]
pub mod COMPEN {
    #[doc = "Compensation Value"]
    pub mod COMPEN_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
