#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "EVTG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "no description available"]
    pub EVTG_INST: [evtg_inst::RegisterBlock; 4usize],
}
pub mod evtg_inst {
    #![allow(
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals,
        dead_code
    )]
    #[doc = "no description available"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "AOI0 Boolean Function Term 0 and 1 Configuration"]
        pub EVTG_AOI0_BFT01: crate::RWRegister<u16>,
        #[doc = "AOI0 Boolean Function Term 2 and 3 Configuration"]
        pub EVTG_AOI0_BFT23: crate::RWRegister<u16>,
        #[doc = "AOI1 Boolean Function Term 0 and 1 Configuration"]
        pub EVTG_AOI1_BFT01: crate::RWRegister<u16>,
        #[doc = "AOI1 Boolean Function Term 2 and 3 Configuration"]
        pub EVTG_AOI1_BFT23: crate::RWRegister<u16>,
        _reserved0: [u8; 0x2],
        #[doc = "Control and Status"]
        pub EVTG_CTRL: crate::RWRegister<u16>,
        #[doc = "AOI0 Output Filter"]
        pub EVTG_AOI0_FILT: crate::RWRegister<u16>,
        #[doc = "AOI1 Output Filter"]
        pub EVTG_AOI1_FILT: crate::RWRegister<u16>,
    }
    #[doc = "AOI0 Boolean Function Term 0 and 1 Configuration"]
    pub mod EVTG_AOI0_BFT01 {
        #[doc = "Product Term 1, D Input Configuration"]
        pub mod PT1_DC {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the D input in this product term to a logical zero"]
                pub const PT1_DC0: u32 = 0;
                #[doc = "Pass the D input in this product term"]
                pub const PT1_DC1: u32 = 1;
                #[doc = "Complement the D input in this product term"]
                pub const PT1_DC2: u32 = 2;
                #[doc = "Force the D input in this product term to a logical one"]
                pub const PT1_DC3: u32 = 3;
            }
        }
        #[doc = "Product Term 1, C Input Configuration"]
        pub mod PT1_CC {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the C input in this product term to a logical zero"]
                pub const CC_0: u32 = 0;
                #[doc = "Pass the C input in this product term"]
                pub const CC_1: u32 = 1;
                #[doc = "Complement the C input in this product term"]
                pub const CC_2: u32 = 2;
                #[doc = "Force the C input in this product term to a logical one"]
                pub const CC_3: u32 = 3;
            }
        }
        #[doc = "Product Term 1, B Input Configuration"]
        pub mod PT1_BC {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the B input in this product term to a logical zero"]
                pub const BC_0: u32 = 0;
                #[doc = "Pass the B input in this product term"]
                pub const BC_1: u32 = 1;
                #[doc = "Complement the B input in this product term"]
                pub const BC_2: u32 = 2;
                #[doc = "Force the B input in this product term to a logical one"]
                pub const BC_3: u32 = 3;
            }
        }
        #[doc = "Product Term 1, A Input Configuration"]
        pub mod PT1_AC {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the A input in this product term to a logical zero"]
                pub const AC_0: u32 = 0;
                #[doc = "Pass the A input in this product term"]
                pub const AC_1: u32 = 1;
                #[doc = "Complement the A input in this product term"]
                pub const AC_2: u32 = 2;
                #[doc = "Force the A input in this product term to a logical one"]
                pub const AC_3: u32 = 3;
            }
        }
        #[doc = "Product Term 0, D Input Configuration"]
        pub mod PT0_DC {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the D input in this product term to a logical zero"]
                pub const DC_0: u32 = 0;
                #[doc = "Pass the D input in this product term"]
                pub const DC_1: u32 = 1;
                #[doc = "Complement the D input in this product term"]
                pub const DC_2: u32 = 2;
                #[doc = "Force the D input in this product term to a logical one"]
                pub const DC_3: u32 = 3;
            }
        }
        #[doc = "Product Term 0, C Input Configuration"]
        pub mod PT0_CC {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the C input in this product term to a logical zero"]
                pub const CC0_0: u32 = 0;
                #[doc = "Pass the C input in this product term"]
                pub const CC0_1: u32 = 1;
                #[doc = "Complement the C input in this product term"]
                pub const CC0_2: u32 = 2;
                #[doc = "Force the C input in this product term to a logical one"]
                pub const CC0_3: u32 = 3;
            }
        }
        #[doc = "Product Term 0, B Input Configuration"]
        pub mod PT0_BC {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the B input in this product term to a logical zero"]
                pub const PT0_BC0: u32 = 0;
                #[doc = "Pass the B input in this product term"]
                pub const PT0_BC1: u32 = 1;
                #[doc = "Complement the B input in this product term"]
                pub const PT0_BC2: u32 = 2;
                #[doc = "Force the B input in this product term to a logical one"]
                pub const PT0_BC3: u32 = 3;
            }
        }
        #[doc = "Product Term 0, A Input Configuration"]
        pub mod PT0_AC {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the A input in this product term to a logical zero"]
                pub const PT0_AC0: u32 = 0;
                #[doc = "Pass the A input in this product term"]
                pub const PT0_AC1: u32 = 1;
                #[doc = "Complement the A input in this product term"]
                pub const PT0_AC2: u32 = 2;
                #[doc = "Force the A input in this product term to a logical one"]
                pub const PT0_AC3: u32 = 3;
            }
        }
    }
    #[doc = "AOI0 Boolean Function Term 2 and 3 Configuration"]
    pub mod EVTG_AOI0_BFT23 {
        #[doc = "Product Term 3, D Input Configuration"]
        pub mod PT3_DC {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the D input in this product term to a logical zero"]
                pub const PT3_DC0: u32 = 0;
                #[doc = "Pass the D input in this product term"]
                pub const PT3_DC1: u32 = 1;
                #[doc = "Complement the D input in this product term"]
                pub const PT3_DC2: u32 = 2;
                #[doc = "Force the D input in this product term to a logical one"]
                pub const PT3_DC3: u32 = 3;
            }
        }
        #[doc = "Product Term 3, C Input Configuration"]
        pub mod PT3_CC {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the C input in this product term to a logical zero"]
                pub const PT3_CC0: u32 = 0;
                #[doc = "Pass the C input in this product term"]
                pub const PT3_CC1: u32 = 1;
                #[doc = "Complement the C input in this product term"]
                pub const PT3_CC2: u32 = 2;
                #[doc = "Force the C input in this product term to a logical one"]
                pub const PT3_CC3: u32 = 3;
            }
        }
        #[doc = "Product Term 3, B Input Configuration"]
        pub mod PT3_BC {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the B input in this product term to a logical zero"]
                pub const PT3_BC0: u32 = 0;
                #[doc = "Pass the B input in this product term"]
                pub const PT3_BC1: u32 = 1;
                #[doc = "Complement the B input in this product term"]
                pub const PT3_BC2: u32 = 2;
                #[doc = "Force the B input in this product term to a logical one"]
                pub const PT3_BC3: u32 = 3;
            }
        }
        #[doc = "Product Term 3, A Input Configuration"]
        pub mod PT3_AC {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the A input in this product term to a logical zero"]
                pub const PT3_AC0: u32 = 0;
                #[doc = "Pass the A input in this product term"]
                pub const PT3_AC1: u32 = 1;
                #[doc = "Complement the A input in this product term"]
                pub const PT3_AC2: u32 = 2;
                #[doc = "Force the A input in this product term to a logical one"]
                pub const PT3_AC3: u32 = 3;
            }
        }
        #[doc = "Product Term 2, D Input Configuration"]
        pub mod PT2_DC {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the D input in this product term to a logical zero"]
                pub const PT2_DC0: u32 = 0;
                #[doc = "Pass the D input in this product term"]
                pub const PT2_DC1: u32 = 1;
                #[doc = "Complement the D input in this product term"]
                pub const PT2_DC2: u32 = 2;
                #[doc = "Force the D input in this product term to a logical one"]
                pub const PT2_DC3: u32 = 3;
            }
        }
        #[doc = "Product Term 2, C Input Configuration"]
        pub mod PT2_CC {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the C input in this product term to a logical zero"]
                pub const PT2_CC0: u32 = 0;
                #[doc = "Pass the C input in this product term"]
                pub const PT2_CC1: u32 = 1;
                #[doc = "Complement the C input in this product term"]
                pub const PT2_CC2: u32 = 2;
                #[doc = "Force the C input in this product term to a logical one"]
                pub const PT2_CC3: u32 = 3;
            }
        }
        #[doc = "Product Term 2, B Input Configuration"]
        pub mod PT2_BC {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the B input in this product term to a logical zero"]
                pub const PT2_BC0: u32 = 0;
                #[doc = "Pass the B input in this product term"]
                pub const PT2_BC1: u32 = 1;
                #[doc = "Complement the B input in this product term"]
                pub const PT2_BC2: u32 = 2;
                #[doc = "Force the B input in this product term to a logical one"]
                pub const PT2_BC3: u32 = 3;
            }
        }
        #[doc = "Product Term 2, A Input Configuration"]
        pub mod PT2_AC {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the A input in this product term to a logical zero"]
                pub const PT2_AC0: u32 = 0;
                #[doc = "Pass the A input in this product term"]
                pub const PT2_AC1: u32 = 1;
                #[doc = "Complement the A input in this product term"]
                pub const PT2_AC2: u32 = 2;
                #[doc = "Force the A input in this product term to a logical one"]
                pub const PT2_AC3: u32 = 3;
            }
        }
    }
    #[doc = "AOI1 Boolean Function Term 0 and 1 Configuration"]
    pub mod EVTG_AOI1_BFT01 {
        #[doc = "Product Term 1, D Input Configuration"]
        pub mod PT1_DC {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the D input in this product term to a logical zero"]
                pub const PT1_DC0: u32 = 0;
                #[doc = "Pass the D input in this product term"]
                pub const PT1_DC1: u32 = 1;
                #[doc = "Complement the D input in this product term"]
                pub const PT1_DC2: u32 = 2;
                #[doc = "Force the D input in this product term to a logical one"]
                pub const PT1_DC3: u32 = 3;
            }
        }
        #[doc = "Product Term 1, C Input Configuration"]
        pub mod PT1_CC {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the C input in this product term to a logical zero"]
                pub const PT1_CC0: u32 = 0;
                #[doc = "Pass the C input in this product term"]
                pub const PT1_CC1: u32 = 1;
                #[doc = "Complement the C input in this product term"]
                pub const PT1_CC2: u32 = 2;
                #[doc = "Force the C input in this product term to a logical one"]
                pub const PT1_CC3: u32 = 3;
            }
        }
        #[doc = "Product Term 1, B Input Configuration"]
        pub mod PT1_BC {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the B input in this product term to a logical zero"]
                pub const PT1_BC0: u32 = 0;
                #[doc = "Pass the B input in this product term"]
                pub const PT1_BC1: u32 = 1;
                #[doc = "Complement the B input in this product term"]
                pub const PT1_BC2: u32 = 2;
                #[doc = "Force the B input in this product term to a logical one"]
                pub const PT1_BC3: u32 = 3;
            }
        }
        #[doc = "Product Term 1, A Input Configuration"]
        pub mod PT1_AC {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the A input in this product term to a logical zero"]
                pub const PT1_AC0: u32 = 0;
                #[doc = "Pass the A input in this product term"]
                pub const PT1_AC1: u32 = 1;
                #[doc = "Complement the A input in this product term"]
                pub const PT1_AC2: u32 = 2;
                #[doc = "Force the A input in this product term to a logical one"]
                pub const PT1_AC3: u32 = 3;
            }
        }
        #[doc = "Product Term 0, D Input Configuration"]
        pub mod PT0_DC {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the D input in this product term to a logical zero"]
                pub const PT0_DC0: u32 = 0;
                #[doc = "Pass the D input in this product term"]
                pub const PT0_DC1: u32 = 1;
                #[doc = "Complement the D input in this product term"]
                pub const PT0_DC2: u32 = 2;
                #[doc = "Force the D input in this product term to a logical one"]
                pub const PT0_DC3: u32 = 3;
            }
        }
        #[doc = "Product Term 0, C Input Configuration"]
        pub mod PT0_CC {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the C input in this product term to a logical zero"]
                pub const PT0_CC0: u32 = 0;
                #[doc = "Pass the C input in this product term"]
                pub const PT0_CC1: u32 = 1;
                #[doc = "Complement the C input in this product term"]
                pub const PT0_CC2: u32 = 2;
                #[doc = "Force the C input in this product term to a logical one"]
                pub const PT0_CC3: u32 = 3;
            }
        }
        #[doc = "Product Term 0, B Input Configuration"]
        pub mod PT0_BC {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the B input in this product term to a logical zero"]
                pub const PT0_BC0: u32 = 0;
                #[doc = "Pass the B input in this product term"]
                pub const PT0_BC1: u32 = 1;
                #[doc = "Complement the B input in this product term"]
                pub const PT0_BC2: u32 = 2;
                #[doc = "Force the B input in this product term to a logical one"]
                pub const PT0_BC3: u32 = 3;
            }
        }
        #[doc = "Product Term 0, A Input Configuration"]
        pub mod PT0_AC {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the A input in this product term to a logical zero"]
                pub const PT0_AC0: u32 = 0;
                #[doc = "Pass the A input in this product term"]
                pub const PT0_AC1: u32 = 1;
                #[doc = "Complement the A input in this product term"]
                pub const PT0_AC2: u32 = 2;
                #[doc = "Force the A input in this product term to a logical one"]
                pub const PT0_AC3: u32 = 3;
            }
        }
    }
    #[doc = "AOI1 Boolean Function Term 2 and 3 Configuration"]
    pub mod EVTG_AOI1_BFT23 {
        #[doc = "Product Term 3, D Input Configuration"]
        pub mod PT3_DC {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the D input in this product term to a logical zero"]
                pub const PT3_DC0: u32 = 0;
                #[doc = "Pass the D input in this product term"]
                pub const PT3_DC1: u32 = 1;
                #[doc = "Complement the D input in this product term"]
                pub const PT3_DC2: u32 = 2;
                #[doc = "Force the D input in this product term to a logical one"]
                pub const PT3_DC3: u32 = 3;
            }
        }
        #[doc = "Product Term 3, C Input Configuration"]
        pub mod PT3_CC {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the C input in this product term to a logical zero"]
                pub const PT3_CC0: u32 = 0;
                #[doc = "Pass the C input in this product term"]
                pub const PT3_CC1: u32 = 1;
                #[doc = "Complement the C input in this product term"]
                pub const PT3_CC2: u32 = 2;
                #[doc = "Force the C input in this product term to a logical one"]
                pub const PT3_CC3: u32 = 3;
            }
        }
        #[doc = "Product Term 3, B Input Configuration"]
        pub mod PT3_BC {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the B input in this product term to a logical zero"]
                pub const PT3_BC0: u32 = 0;
                #[doc = "Pass the B input in this product term"]
                pub const PT3_BC1: u32 = 1;
                #[doc = "Complement the B input in this product term"]
                pub const PT3_BC2: u32 = 2;
                #[doc = "Force the B input in this product term to a logical one"]
                pub const PT3_BC3: u32 = 3;
            }
        }
        #[doc = "Product Term 3, A Input Configuration"]
        pub mod PT3_AC {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the A input in this product term to a logical zero"]
                pub const PT3_AC0: u32 = 0;
                #[doc = "Pass the A input in this product term"]
                pub const PT3_AC1: u32 = 1;
                #[doc = "Complement the A input in this product term"]
                pub const PT3_AC2: u32 = 2;
                #[doc = "Force the A input in this product term to a logical one"]
                pub const PT3_AC3: u32 = 3;
            }
        }
        #[doc = "Product Term 2, D Input Configuration"]
        pub mod PT2_DC {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the D input in this product term to a logical zero"]
                pub const PT2_DC0: u32 = 0;
                #[doc = "Pass the D input in this product term"]
                pub const PT2_DC1: u32 = 1;
                #[doc = "Complement the D input in this product term"]
                pub const PT2_DC2: u32 = 2;
                #[doc = "Force the D input in this product term to a logical one"]
                pub const PT2_DC3: u32 = 3;
            }
        }
        #[doc = "Product Term 2, C Input Configuration"]
        pub mod PT2_CC {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the C input in this product term to a logical zero"]
                pub const PT2_CC0: u32 = 0;
                #[doc = "Pass the C input in this product term"]
                pub const PT2_CC1: u32 = 1;
                #[doc = "Complement the C input in this product term"]
                pub const PT2_CC2: u32 = 2;
                #[doc = "Force the C input in this product term to a logical one"]
                pub const PT2_CC3: u32 = 3;
            }
        }
        #[doc = "Product Term 2, B Input Configuration"]
        pub mod PT2_BC {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the B input in this product term to a logical zero"]
                pub const PT2_BC0: u32 = 0;
                #[doc = "Pass the B input in this product term"]
                pub const PT2_BC1: u32 = 1;
                #[doc = "Complement the B input in this product term"]
                pub const PT2_BC2: u32 = 2;
                #[doc = "Force the B input in this product term to a logical one"]
                pub const PT2_BC3: u32 = 3;
            }
        }
        #[doc = "Product Term 2, A Input Configuration"]
        pub mod PT2_AC {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Force the A input in this product term to a logical zero"]
                pub const PT2_AC0: u32 = 0;
                #[doc = "Pass the A input in this product term"]
                pub const PT2_AC1: u32 = 1;
                #[doc = "Complement the A input in this product term"]
                pub const PT2_AC2: u32 = 2;
                #[doc = "Force the A input in this product term to a logical one"]
                pub const PT2_AC3: u32 = 3;
            }
        }
    }
    #[doc = "Control and Status"]
    pub mod EVTG_CTRL {
        #[doc = "Flip flop Initial Value Configuration"]
        pub mod FF_INIT {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "0"]
                pub const FF0: u32 = 0;
                #[doc = "1"]
                pub const FF1: u32 = 1;
            }
        }
        #[doc = "Flip-Flop Initial Output Enable Control"]
        pub mod INIT_EN {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0b1 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Write 0 does not generate enable pulse"]
                pub const PULSE: u32 = 0;
                #[doc = "Write 1 generates enable pulse"]
                pub const NO_PULSE: u32 = 1;
            }
        }
        #[doc = "Flip-Flop Mode Selection"]
        pub mod MODE_SEL {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Bypass mode"]
                pub const BYPASS: u32 = 0;
                #[doc = "RS Trigger mode"]
                pub const RS: u32 = 1;
                #[doc = "T-FF mode"]
                pub const TFF: u32 = 2;
                #[doc = "D-FF mode"]
                pub const DFF: u32 = 3;
                #[doc = "JK-FF mode"]
                pub const JKFF: u32 = 4;
                #[doc = "Latch mode"]
                pub const LATCH: u32 = 5;
            }
        }
        #[doc = "EVTG Output Feedback Override Control"]
        pub mod FB_OVRD {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Replace An"]
                pub const AN: u32 = 0;
                #[doc = "Replace Bn"]
                pub const BN: u32 = 1;
                #[doc = "Replace Cn"]
                pub const CN: u32 = 2;
                #[doc = "Replace Dn"]
                pub const DN: u32 = 3;
            }
        }
        #[doc = "Synchronize Control"]
        pub mod SYNC_CTRL {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b1111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "EVTG input \"An\" will not be synced"]
                pub const A_NOTSYNC: u32 = 0;
                #[doc = "EVTG input \"An\" will be synced by two bus clk cycles"]
                pub const A_SYNC: u32 = 1;
            }
        }
        #[doc = "Force Bypass Control"]
        pub mod FORCE_BYPASS {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0b11 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Will not force the bypass"]
                pub const NFB0: u32 = 0;
                #[doc = "Whatever MODE_SEL is, will force bypass flip-flop and route the AOI_0(Filter_0) value directly to EVTG_OUTA"]
                pub const F_FF_A: u32 = 1;
            }
        }
    }
    #[doc = "AOI0 Output Filter"]
    pub mod EVTG_AOI0_FILT {
        #[doc = "Output Filter Sample Period"]
        pub mod FILT_PER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Output Filter Sample Count"]
        pub mod FILT_CNT {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "AOI1 Output Filter"]
    pub mod EVTG_AOI1_FILT {
        #[doc = "Output Filter Sample Period"]
        pub mod FILT_PER {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0b11111111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "Output Filter Sample Count"]
        pub mod FILT_CNT {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0b111 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
