#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "PowerQuad"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Output Base"]
    pub OUTBASE: crate::RWRegister<u32>,
    #[doc = "Output Format"]
    pub OUTFORMAT: crate::RWRegister<u32>,
    #[doc = "Temporary Base"]
    pub TMPBASE: crate::RWRegister<u32>,
    #[doc = "Temporary Format"]
    pub TMPFORMAT: crate::RWRegister<u32>,
    #[doc = "Input A base"]
    pub INABASE: crate::RWRegister<u32>,
    #[doc = "Input A format"]
    pub INAFORMAT: crate::RWRegister<u32>,
    #[doc = "Input B base"]
    pub INBBASE: crate::RWRegister<u32>,
    #[doc = "Input B format"]
    pub INBFORMAT: crate::RWRegister<u32>,
    _reserved0: [u8; 0xe0],
    #[doc = "Control"]
    pub CONTROL: crate::RWRegister<u32>,
    #[doc = "Length"]
    pub LENGTH: crate::RWRegister<u32>,
    #[doc = "Coprocessor Pre-scale"]
    pub CPPRE: crate::RWRegister<u32>,
    #[doc = "Miscellaneous"]
    pub MISC: crate::RWRegister<u32>,
    #[doc = "Cursory"]
    pub CURSORY: crate::RWRegister<u32>,
    _reserved1: [u8; 0x6c],
    #[doc = "CORDIC input X"]
    pub CORDIC_X: crate::RWRegister<u32>,
    #[doc = "CORDIC input Y"]
    pub CORDIC_Y: crate::RWRegister<u32>,
    #[doc = "CORDIC input Z"]
    pub CORDIC_Z: crate::RWRegister<u32>,
    #[doc = "Error Status"]
    pub ERRSTAT: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable"]
    pub INTREN: crate::RWRegister<u32>,
    #[doc = "Event Enable"]
    pub EVENTEN: crate::RWRegister<u32>,
    #[doc = "Interrupt Status"]
    pub INTRSTAT: crate::RWRegister<u32>,
    _reserved2: [u8; 0x64],
    #[doc = "General Purpose Register Bank n"]
    pub GPREG: [crate::RWRegister<u32>; 16usize],
    #[doc = "Compute Register Bank n"]
    pub COMPREG: [crate::RWRegister<u32>; 8usize],
}
#[doc = "Output Base"]
pub mod OUTBASE {
    #[doc = "Base address for the output region"]
    pub mod OUTBASE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Format"]
pub mod OUTFORMAT {
    #[doc = "Output internal format"]
    pub mod OUT_FORMATINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "q15"]
            pub const Q15: u32 = 0;
            #[doc = "q31"]
            pub const Q31: u32 = 1;
            #[doc = "float"]
            pub const FLOAT: u32 = 2;
        }
    }
    #[doc = "Output external format"]
    pub mod OUT_FORMATEXT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "q15"]
            pub const Q15: u32 = 0;
            #[doc = "q31"]
            pub const Q31: u32 = 1;
            #[doc = "float"]
            pub const FLOAT: u32 = 2;
        }
    }
    #[doc = "Output scaler value"]
    pub mod OUT_SCALER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Temporary Base"]
pub mod TMPBASE {
    #[doc = "Base address for the temporary region"]
    pub mod TMPBASE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Temporary Format"]
pub mod TMPFORMAT {
    #[doc = "Temporary internal format"]
    pub mod TMP_FORMATINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "q15"]
            pub const Q15: u32 = 0;
            #[doc = "q31"]
            pub const Q31: u32 = 1;
            #[doc = "float"]
            pub const FLOAT: u32 = 2;
        }
    }
    #[doc = "Temporary external format"]
    pub mod TMP_FORMATEXT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "q15"]
            pub const Q15: u32 = 0;
            #[doc = "q31"]
            pub const Q31: u32 = 1;
            #[doc = "float"]
            pub const FLOAT: u32 = 2;
        }
    }
    #[doc = "Temporary scaler value"]
    pub mod TMP_SCALER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input A base"]
pub mod INABASE {
    #[doc = "Input A base"]
    pub mod INABASE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input A format"]
pub mod INAFORMAT {
    #[doc = "Input A internal format"]
    pub mod INA_FORMATINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "q15"]
            pub const Q15: u32 = 0;
            #[doc = "q31"]
            pub const Q31: u32 = 1;
            #[doc = "float"]
            pub const FLOAT: u32 = 2;
        }
    }
    #[doc = "Input A external format"]
    pub mod INA_FORMATEXT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "q15"]
            pub const Q15: u32 = 0;
            #[doc = "q31"]
            pub const Q31: u32 = 1;
            #[doc = "float"]
            pub const FLOAT: u32 = 2;
        }
    }
    #[doc = "Input A scaler value"]
    pub mod INA_SCALER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input B base"]
pub mod INBBASE {
    #[doc = "Input B base"]
    pub mod INBBASE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input B format"]
pub mod INBFORMAT {
    #[doc = "Input B internal format"]
    pub mod INB_FORMATINT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "q15"]
            pub const Q15: u32 = 0;
            #[doc = "q31"]
            pub const Q31: u32 = 1;
            #[doc = "float"]
            pub const FLOAT: u32 = 2;
        }
    }
    #[doc = "Input B external format"]
    pub mod INB_FORMATEXT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "q15"]
            pub const Q15: u32 = 0;
            #[doc = "q31"]
            pub const Q31: u32 = 1;
            #[doc = "float"]
            pub const FLOAT: u32 = 2;
        }
    }
    #[doc = "Input B scaler value"]
    pub mod INB_SCALER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control"]
pub mod CONTROL {
    #[doc = "Decode opcode"]
    pub mod DECODE_OPCODE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Decode machine"]
    pub mod DECODE_MACHINE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Instruction busy"]
    pub mod INST_BUSY {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "Not busy"]
            pub const NOT_BUSY: u32 = 0;
            #[doc = "busy"]
            pub const BUSY: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Length"]
pub mod LENGTH {
    #[doc = "Instruction length"]
    pub mod INST_LENGTH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Coprocessor Pre-scale"]
pub mod CPPRE {
    #[doc = "Input"]
    pub mod CPPRE_IN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output"]
    pub mod CPPRE_OUT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b11111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Saturation"]
    pub mod CPPRE_SAT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No saturation"]
            pub const DISABLE: u32 = 0;
            #[doc = "Forces sub-32 bit saturation"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Saturation 8"]
    pub mod CPPRE_SAT8 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8 bits"]
            pub const SAT_8_BITS: u32 = 0;
            #[doc = "16 bits"]
            pub const SAT_16_BITS: u32 = 1;
        }
    }
}
#[doc = "Miscellaneous"]
pub mod MISC {
    #[doc = "For matrix operations used for scaling factor"]
    pub mod INST_MISC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Cursory"]
pub mod CURSORY {
    #[doc = "Cursory Mode"]
    pub mod CURSORY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable cursory mode"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable cursory Mode"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "CORDIC input X"]
pub mod CORDIC_X {
    #[doc = "CORDIC input x"]
    pub mod CORDIC_X {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CORDIC input Y"]
pub mod CORDIC_Y {
    #[doc = "CORDIC input y"]
    pub mod CORDIC_Y {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CORDIC input Z"]
pub mod CORDIC_Z {
    #[doc = "CORDIC input z"]
    pub mod CORDIC_Z {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Error Status"]
pub mod ERRSTAT {
    #[doc = "Floating point overflow"]
    pub mod OVERFLOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error on floating point overflow"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Floating Point NaN"]
    pub mod NAN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error on Floating Point NaN"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Fixed point overflow"]
    pub mod FIXEDOVERFLOW {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error on fixed point overflow"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Underflow"]
    pub mod UNDERFLOW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error on underflow"]
            pub const ERROR: u32 = 1;
        }
    }
    #[doc = "Bus error"]
    pub mod BUSERROR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No error"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Error on bus"]
            pub const ERROR: u32 = 1;
        }
    }
}
#[doc = "Interrupt Enable"]
pub mod INTREN {
    #[doc = "Interrupt floating point overflow"]
    pub mod INTR_OFLOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Interrupt floating point NaN"]
    pub mod INTR_NAN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Interrupt on fixed-point overflow"]
    pub mod INTR_FIXED {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Interrupt on subnormal truncation"]
    pub mod INTR_UFLOW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Interrupt on AHBM bus error"]
    pub mod INTR_BERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Interrupt on instruction completion"]
    pub mod INTR_COMP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Event Enable"]
pub mod EVENTEN {
    #[doc = "Event trigger on floating point overflow"]
    pub mod EVENT_OFLOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable event trigger"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable event trigger"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event trigger on floating point NaN"]
    pub mod EVENT_NAN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable event trigger"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable event trigger"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event trigger on fixed-point overflow"]
    pub mod EVENT_FIXED {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable event trigger"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable event trigger"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event trigger on subnormal truncation"]
    pub mod EVENT_UFLOW {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable event trigger"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable event trigger"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event trigger on AHBM bus error"]
    pub mod EVENT_BERR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable event trigger"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable event trigger"]
            pub const ENABLE: u32 = 1;
        }
    }
    #[doc = "Event trigger on instruction completion"]
    pub mod EVENT_COMP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable event trigger"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable event trigger"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "Interrupt Status"]
pub mod INTRSTAT {
    #[doc = "Interrupt Status"]
    pub mod INTR_STAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No new interrupt"]
            pub const DISABLE: u32 = 0;
            #[doc = "Interrupt captured"]
            pub const ENABLE: u32 = 1;
        }
    }
}
#[doc = "General Purpose Register Bank n"]
pub mod GPREG {
    #[doc = "General Purpose Bank"]
    pub mod GPREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Compute Register Bank n"]
pub mod COMPREG {
    #[doc = "Compute bank"]
    pub mod COMPREG {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
