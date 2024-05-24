#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "CACHE64_CTRL"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x800],
    #[doc = "Cache Control"]
    pub CCR: crate::RWRegister<u32>,
    #[doc = "Cache Line Control"]
    pub CLCR: crate::RWRegister<u32>,
    #[doc = "Cache Search Address"]
    pub CSAR: crate::RWRegister<u32>,
    #[doc = "Cache Read/Write Value"]
    pub CCVR: crate::RWRegister<u32>,
}
#[doc = "Cache Control"]
pub mod CCR {
    #[doc = "Cache Enable"]
    pub mod ENCACHE {
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
    #[doc = "Enable Write Buffer"]
    pub mod ENWRBUF {
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
    #[doc = "Invalidate Way 0"]
    pub mod INVW0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No operation"]
            pub const NO_OPERATION: u32 = 0;
            #[doc = "Invalidates all lines in way 0"]
            pub const INVW0: u32 = 1;
        }
    }
    #[doc = "Push Way 0"]
    pub mod PUSHW0 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No operation"]
            pub const NO_OPERATION: u32 = 0;
            #[doc = "Push all modified lines in way 0"]
            pub const PUSHW0: u32 = 1;
        }
    }
    #[doc = "Invalidate Way 1"]
    pub mod INVW1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No operation"]
            pub const NO_OPERATION: u32 = 0;
            #[doc = "Invalidates all lines in way 1"]
            pub const INVW1: u32 = 1;
        }
    }
    #[doc = "Push Way 1"]
    pub mod PUSHW1 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No operation"]
            pub const NO_OPERATION: u32 = 0;
            #[doc = "Push all modified lines in way 1"]
            pub const PUSHW1: u32 = 1;
        }
    }
    #[doc = "Initiate Cache Command"]
    pub mod GO {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write: no effect; Read: no cache command active"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Write: initiates cache command; Read: cache command active"]
            pub const INIT_CMD: u32 = 1;
        }
    }
}
#[doc = "Cache Line Control"]
pub mod CLCR {
    #[doc = "Initiate Cache Line Command"]
    pub mod LGO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write: no effect; Read: no line command active"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Write: initiate line command; Read: line command active"]
            pub const INIT_CMD: u32 = 1;
        }
    }
    #[doc = "Cache Address"]
    pub mod CACHEADDR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b11111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Way Select"]
    pub mod WSEL {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Way 0"]
            pub const WAY0: u32 = 0;
            #[doc = "Way 1"]
            pub const WAY1: u32 = 1;
        }
    }
    #[doc = "Tag Or Data Select"]
    pub mod TDSEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Data"]
            pub const DATA: u32 = 0;
            #[doc = "Tag"]
            pub const TAG: u32 = 1;
        }
    }
    #[doc = "Line Command Initial Valid Bit"]
    pub mod LCIVB {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Initial state 0"]
            pub const LCIVB_0: u32 = 0;
            #[doc = "Initial state 1"]
            pub const LCIVB_1: u32 = 1;
        }
    }
    #[doc = "Line Command Initial Modified Bit"]
    pub mod LCIMB {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Initial state 0"]
            pub const LCIMB_0: u32 = 0;
            #[doc = "Initial state 1"]
            pub const LCIMB_1: u32 = 1;
        }
    }
    #[doc = "Line Command Way"]
    pub mod LCWAY {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Way 0"]
            pub const WAY0: u32 = 0;
            #[doc = "Way 1"]
            pub const WAY1: u32 = 1;
        }
    }
    #[doc = "Line Command"]
    pub mod LCMD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Search and read or write"]
            pub const SEARCH_RW: u32 = 0;
            #[doc = "Invalidate"]
            pub const INVALIDATE: u32 = 1;
            #[doc = "Push"]
            pub const PUSH: u32 = 2;
            #[doc = "Clear"]
            pub const CLEAR: u32 = 3;
        }
    }
    #[doc = "Line Address Select"]
    pub mod LADSEL {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Cache"]
            pub const CACHE_ADDR: u32 = 0;
            #[doc = "Physical"]
            pub const PHYS_ADDR: u32 = 1;
        }
    }
    #[doc = "Line Access Type"]
    pub mod LACC {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read"]
            pub const READ: u32 = 0;
            #[doc = "Write"]
            pub const WRITE: u32 = 1;
        }
    }
}
#[doc = "Cache Search Address"]
pub mod CSAR {
    #[doc = "Initiate Cache Line Command"]
    pub mod LGO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Write: no effect; Read: no line command active"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Write: initiate line command; Read: line command active"]
            pub const INIT_CMD: u32 = 1;
        }
    }
    #[doc = "Physical Address"]
    pub mod PHYADDR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Cache Read/Write Value"]
pub mod CCVR {
    #[doc = "Cache Read/Write Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
