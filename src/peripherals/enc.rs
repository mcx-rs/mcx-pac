#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "ENC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control"]
    pub CTRL: crate::RWRegister<u16>,
    #[doc = "Input Filter"]
    pub FILT: crate::RWRegister<u16>,
    #[doc = "Watchdog Timeout"]
    pub WTR: crate::RWRegister<u16>,
    #[doc = "Position Difference Counter"]
    pub POSD: crate::RWRegister<u16>,
    #[doc = "Position Difference Hold"]
    pub POSDH: crate::RWRegister<u16>,
    #[doc = "Revolution Counter"]
    pub REV: crate::RWRegister<u16>,
    #[doc = "Revolution Hold"]
    pub REVH: crate::RWRegister<u16>,
    #[doc = "Upper Position Counter"]
    pub UPOS: crate::RWRegister<u16>,
    #[doc = "Lower Position Counter"]
    pub LPOS: crate::RWRegister<u16>,
    #[doc = "Upper Position Hold"]
    pub UPOSH: crate::RWRegister<u16>,
    #[doc = "Lower Position Hold"]
    pub LPOSH: crate::RWRegister<u16>,
    #[doc = "Upper Initialization"]
    pub UINIT: crate::RWRegister<u16>,
    #[doc = "Lower Initialization"]
    pub LINIT: crate::RWRegister<u16>,
    #[doc = "Input Monitor"]
    pub IMR: crate::RWRegister<u16>,
    #[doc = "Test"]
    pub TST: crate::RWRegister<u16>,
    #[doc = "Control 2"]
    pub CTRL2: crate::RWRegister<u16>,
    #[doc = "Upper Modulus"]
    pub UMOD: crate::RWRegister<u16>,
    #[doc = "Lower Modulus"]
    pub LMOD: crate::RWRegister<u16>,
    #[doc = "Upper Position Compare"]
    pub UCOMP: crate::RWRegister<u16>,
    #[doc = "Lower Position Compare"]
    pub LCOMP: crate::RWRegister<u16>,
    #[doc = "Last Edge Time"]
    pub LASTEDGE: crate::RWRegister<u16>,
    #[doc = "Last Edge Time Hold"]
    pub LASTEDGEH: crate::RWRegister<u16>,
    #[doc = "Position Difference Period Counter"]
    pub POSDPER: crate::RWRegister<u16>,
    #[doc = "Position Difference Period Buffer"]
    pub POSDPERBFR: crate::RWRegister<u16>,
    #[doc = "Position Difference Period Hold"]
    pub POSDPERH: crate::RWRegister<u16>,
    #[doc = "Control 3"]
    pub CTRL3: crate::RWRegister<u16>,
}
#[doc = "Control"]
pub mod CTRL {
    #[doc = "Compare Interrupt Enable"]
    pub mod CMPIE {
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
    #[doc = "Compare Interrupt Request"]
    pub mod CMPIRQ {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No match has occurred"]
            pub const NOMAT: u32 = 0;
            #[doc = "COMP match has occurred"]
            pub const MAT: u32 = 1;
        }
    }
    #[doc = "Watchdog Enable"]
    pub mod WDE {
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
    #[doc = "Watchdog Timeout Interrupt Enable"]
    pub mod DIE {
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
    #[doc = "Watchdog Timeout Interrupt Request"]
    pub mod DIRQ {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not occurred"]
            pub const NOINT: u32 = 0;
            #[doc = "Occurred"]
            pub const INT: u32 = 1;
        }
    }
    #[doc = "Select Positive and Negative Edge of INDEX and PRESET Pulse"]
    pub mod XNE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use positive edge"]
            pub const XNE_0: u32 = 0;
            #[doc = "Use negative edge"]
            pub const XNE_1: u32 = 1;
        }
    }
    #[doc = "INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    pub mod XIP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not initialize"]
            pub const NOINT: u32 = 0;
            #[doc = "Initializes"]
            pub const INT: u32 = 1;
        }
    }
    #[doc = "INDEX Pulse Interrupt Enable"]
    pub mod XIE {
        pub const offset: u32 = 7;
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
    #[doc = "INDEX Pulse Interrupt Request"]
    pub mod XIRQ {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not occurred"]
            pub const NOIND: u32 = 0;
            #[doc = "Occurred"]
            pub const INDEX: u32 = 1;
        }
    }
    #[doc = "Enable Signal Phase Count Mode"]
    pub mod PH1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Uses the standard quadrature decoder, where PHASEA and PHASEB represent a two-phase quadrature signal."]
            pub const USE: u32 = 0;
            #[doc = "Bypasses the quadrature decoder. A positive transition of the PHASEA input generates a count signal. PHASEB input and CTRL[REV] controls the counter direction. If the value of CTRL[REV] and PHASEB are identical; then count is up. If the value of CTRL[REV] and PHASEB is different, then count is down."]
            pub const BYPASS: u32 = 1;
        }
    }
    #[doc = "Enable Reverse Direction Counting"]
    pub mod REV {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counts normally"]
            pub const NORMAL: u32 = 0;
            #[doc = "Counts in the reverse direction"]
            pub const REVERSE: u32 = 1;
        }
    }
    #[doc = "Software-Triggered Initialization of Position Counters UPOS and LPOS"]
    pub mod SWIP {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const NOACTION: u32 = 0;
            #[doc = "Initialize position counter"]
            pub const INIT: u32 = 1;
        }
    }
    #[doc = "Use Negative Edge of HOME Input"]
    pub mod HNE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use positive-going edge-to-trigger initialization of position counters UPOS and LPOS"]
            pub const POS: u32 = 0;
            #[doc = "Use negative-going edge-to-trigger initialization of position counters UPOS and LPOS"]
            pub const NEG: u32 = 1;
        }
    }
    #[doc = "Enable HOME to Initialize Position Counters UPOS and LPOS"]
    pub mod HIP {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const NOACTION: u32 = 0;
            #[doc = "HOME signal initializes the position counter"]
            pub const HOME: u32 = 1;
        }
    }
    #[doc = "HOME Interrupt Enable"]
    pub mod HIE {
        pub const offset: u32 = 14;
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
    #[doc = "HOME Signal Transition Interrupt Request"]
    pub mod HIRQ {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not occurred"]
            pub const NOTRANS: u32 = 0;
            #[doc = "Occurred"]
            pub const TRANSI: u32 = 1;
        }
    }
}
#[doc = "Input Filter"]
pub mod FILT {
    #[doc = "Input Filter Sample Period"]
    pub mod FILT_PER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Filter Sample Count"]
    pub mod FILT_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Prescaler Divide IPBus Clock to FILT Clock"]
    pub mod FILT_PRSC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Watchdog Timeout"]
pub mod WTR {
    #[doc = "WDOG"]
    pub mod WDOG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Position Difference Counter"]
pub mod POSD {
    #[doc = "POSD"]
    pub mod POSD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Position Difference Hold"]
pub mod POSDH {
    #[doc = "POSDH"]
    pub mod POSDH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Revolution Counter"]
pub mod REV {
    #[doc = "REV"]
    pub mod REV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Revolution Hold"]
pub mod REVH {
    #[doc = "REVH"]
    pub mod REVH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Position Counter"]
pub mod UPOS {
    #[doc = "POS"]
    pub mod POS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Position Counter"]
pub mod LPOS {
    #[doc = "POS"]
    pub mod POS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Position Hold"]
pub mod UPOSH {
    #[doc = "POSH"]
    pub mod POSH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Position Hold"]
pub mod LPOSH {
    #[doc = "POSH"]
    pub mod POSH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Initialization"]
pub mod UINIT {
    #[doc = "INIT"]
    pub mod INIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Initialization"]
pub mod LINIT {
    #[doc = "INIT"]
    pub mod INIT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Monitor"]
pub mod IMR {
    #[doc = "HOME"]
    pub mod HOME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "INDEX"]
    pub mod INDEX {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHB"]
    pub mod PHB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PHA"]
    pub mod PHA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FHOM"]
    pub mod FHOM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIND"]
    pub mod FIND {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FPHB"]
    pub mod FPHB {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FPHA"]
    pub mod FPHA {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Test"]
pub mod TST {
    #[doc = "TEST_COUNT"]
    pub mod TEST_COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TEST_PERIOD"]
    pub mod TEST_PERIOD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Quadrature Decoder Negative Signal"]
    pub mod QDN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Positive quadrature decoder signal"]
            pub const POSITIVE: u32 = 0;
            #[doc = "Negative quadrature decoder signal"]
            pub const NEGATIVE: u32 = 1;
        }
    }
    #[doc = "Test Counter Enable"]
    pub mod TCE {
        pub const offset: u32 = 14;
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
    #[doc = "Test Mode Enable"]
    pub mod TEN {
        pub const offset: u32 = 15;
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
#[doc = "Control 2"]
pub mod CTRL2 {
    #[doc = "Update Hold Registers"]
    pub mod UPDHLD {
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
    #[doc = "Update Position Registers"]
    pub mod UPDPOS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No action"]
            pub const NOACTION: u32 = 0;
            #[doc = "Clear"]
            pub const CLEAR: u32 = 1;
        }
    }
    #[doc = "Enable Modulo Counting"]
    pub mod MOD {
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
    #[doc = "Count Direction Flag"]
    pub mod DIR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Down direction"]
            pub const DOWN: u32 = 0;
            #[doc = "Up direction"]
            pub const UP: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Roll-under Interrupt Enable"]
    pub mod RUIE {
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
    #[doc = "Roll-under Interrupt Request"]
    pub mod RUIRQ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No roll-under has occurred"]
            pub const NOROLL: u32 = 0;
            #[doc = "Roll-under has occurred"]
            pub const ROLL: u32 = 1;
        }
    }
    #[doc = "Roll-over Interrupt Enable"]
    pub mod ROIE {
        pub const offset: u32 = 6;
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
    #[doc = "Roll-over Interrupt Request"]
    pub mod ROIRQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Did not occur"]
            pub const NOROLL: u32 = 0;
            #[doc = "Occurred"]
            pub const ROLL: u32 = 1;
        }
    }
    #[doc = "Revolution Counter Modulus Enable"]
    pub mod REVMOD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use INDEX pulse"]
            pub const INDEX: u32 = 0;
            #[doc = "Use modulus counting roll-over or roll-under"]
            pub const COUNT: u32 = 1;
        }
    }
    #[doc = "Output Control"]
    pub mod OUTCTL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "POSMATCH pulses when a match occurs between the position counters (POS) and the corresponding compare value (COMP )"]
            pub const COMPARE: u32 = 0;
            #[doc = "POSMATCH pulses when the UPOS, LPOS, REV, or POSD registers are read"]
            pub const READ: u32 = 1;
        }
    }
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Enable"]
    pub mod SABIE {
        pub const offset: u32 = 10;
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
    #[doc = "Simultaneous PHASEA and PHASEB Change Interrupt Request"]
    pub mod SABIRQ {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No simultaneous change has occurred"]
            pub const NOSIMUL: u32 = 0;
            #[doc = "A simultaneous change has occurred"]
            pub const SIMUL: u32 = 1;
        }
    }
    #[doc = "Initialize Position Registers"]
    pub mod INITPOS {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Don\'t initialize position counter"]
            pub const NOINIT: u32 = 0;
            #[doc = "Initialize position counter"]
            pub const INIT: u32 = 1;
        }
    }
}
#[doc = "Upper Modulus"]
pub mod UMOD {
    #[doc = "MOD"]
    pub mod MOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Modulus"]
pub mod LMOD {
    #[doc = "MOD"]
    pub mod MOD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Upper Position Compare"]
pub mod UCOMP {
    #[doc = "COMP"]
    pub mod COMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Lower Position Compare"]
pub mod LCOMP {
    #[doc = "COMP"]
    pub mod COMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Last Edge Time"]
pub mod LASTEDGE {
    #[doc = "Last Edge Time Counter"]
    pub mod LASTEDGE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Last Edge Time Hold"]
pub mod LASTEDGEH {
    #[doc = "Last Edge Time Hold"]
    pub mod LASTEDGEH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Position Difference Period Counter"]
pub mod POSDPER {
    #[doc = "Position difference period"]
    pub mod POSDPER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Position Difference Period Buffer"]
pub mod POSDPERBFR {
    #[doc = "Position difference period buffer"]
    pub mod POSDPERBFR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Position Difference Period Hold"]
pub mod POSDPERH {
    #[doc = "Position difference period hold"]
    pub mod POSDPERH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control 3"]
pub mod CTRL3 {
    #[doc = "Period Measurement Function Enable"]
    pub mod PMEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not used"]
            pub const NOTUSED: u32 = 0;
            #[doc = "Used"]
            pub const USED: u32 = 1;
        }
    }
    #[doc = "Prescaler"]
    pub mod PRSC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
