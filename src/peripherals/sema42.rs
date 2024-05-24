#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "SEMA42"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Gate"]
    pub GATE3: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE2: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE1: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE0: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE7: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE6: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE5: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE4: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE11: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE10: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE9: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE8: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE15: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE14: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE13: crate::RWRegister<u8>,
    #[doc = "Gate"]
    pub GATE12: crate::RWRegister<u8>,
    _reserved0: [u8; 0x32],
    #[doc = "Reset Gate Read"]
    pub RSTGT_R: crate::RWRegister<u16>,
    #[doc = "Reset Gate Write"]
    pub RSTGT_W: crate::RWRegister<u16>,
}
#[doc = "Gate"]
pub mod GATE3 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE2 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE1 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE0 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE7 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE6 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE5 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE4 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE11 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE10 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE9 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE8 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE15 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE14 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE13 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Gate"]
pub mod GATE12 {
    #[doc = "Gate Finite State Machine"]
    pub mod GTFSM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The gate is unlocked (free)."]
            pub const UNLOCKED: u32 = 0;
            #[doc = "Domain 0 locked the gate."]
            pub const LOCKED_BY_D0: u32 = 1;
            #[doc = "Domain 1 locked the gate."]
            pub const LOCKED_BY_D1: u32 = 2;
            #[doc = "Domain 2 locked the gate."]
            pub const LOCKED_BY_D2: u32 = 3;
            #[doc = "Domain 3 locked the gate."]
            pub const LOCKED_BY_D3: u32 = 4;
            #[doc = "Domain 4 locked the gate."]
            pub const LOCKED_BY_D4: u32 = 5;
            #[doc = "Domain 5 locked the gate."]
            pub const LOCKED_BY_D5: u32 = 6;
            #[doc = "Domain 6 locked the gate."]
            pub const LOCKED_BY_D6: u32 = 7;
            #[doc = "Domain 7 locked the gate."]
            pub const LOCKED_BY_D7: u32 = 8;
            #[doc = "Domain 8 locked the gate."]
            pub const LOCKED_BY_D8: u32 = 9;
            #[doc = "Domain 9 locked the gate."]
            pub const LOCKED_BY_D9: u32 = 10;
            #[doc = "Domain 10 locked the gate."]
            pub const LOCKED_BY_D10: u32 = 11;
            #[doc = "Domain 11 locked the gate."]
            pub const LOCKED_BY_D11: u32 = 12;
            #[doc = "Domain 12 locked the gate."]
            pub const LOCKED_BY_D12: u32 = 13;
            #[doc = "Domain 13 locked the gate."]
            pub const LOCKED_BY_D13: u32 = 14;
            #[doc = "Domain 14 locked the gate."]
            pub const LOCKED_BY_D14: u32 = 15;
        }
    }
}
#[doc = "Reset Gate Read"]
pub mod RSTGT_R {
    #[doc = "Reset Gate Number"]
    pub mod RSTGTN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset Gate Domain"]
    pub mod RSTGMS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset Gate Finite State Machine"]
    pub mod RSTGSM {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {
            #[doc = "Idle, waiting for the first data pattern write."]
            pub const IDLE: u32 = 0;
            #[doc = "Waiting for the second data pattern write"]
            pub const WAITING: u32 = 1;
            #[doc = "The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state."]
            pub const TWO_WRITE_DONE: u32 = 2;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Reset Gate Write"]
pub mod RSTGT_W {
    #[doc = "Reset Gate Number"]
    pub mod RSTGTN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reset Gate Data Pattern"]
    pub mod RSTGDP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
