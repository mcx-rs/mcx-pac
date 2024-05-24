#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "PORT"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RWRegister<u32>,
    _reserved0: [u8; 0xc],
    #[doc = "Global Pin Control Low"]
    pub GPCLR: crate::RWRegister<u32>,
    #[doc = "Global Pin Control High"]
    pub GPCHR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x8],
    #[doc = "Configuration"]
    pub CONFIG: crate::RWRegister<u32>,
    _reserved2: [u8; 0x1c],
    #[doc = "EFT Detect Flag"]
    pub EDFR: crate::RWRegister<u32>,
    #[doc = "EFT Detect Interrupt Enable"]
    pub EDIER: crate::RWRegister<u32>,
    #[doc = "EFT Detect Clear"]
    pub EDCR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x14],
    #[doc = "Calibration 0"]
    pub CALIB0: crate::RWRegister<u32>,
    #[doc = "Calibration 1"]
    pub CALIB1: crate::RWRegister<u32>,
    _reserved4: [u8; 0x18],
    #[doc = "Pin Control 0"]
    pub PCR: [crate::RWRegister<u32>; 32usize],
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {
            #[doc = "Basic implementation"]
            pub const FEATURE0: u32 = 0;
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
#[doc = "Global Pin Control Low"]
pub mod GPCLR {
    #[doc = "Global Pin Write Data"]
    pub mod GPWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE6 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE7 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE8 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE9 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE10 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE11 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE12 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE13 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE14 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE15 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
}
#[doc = "Global Pin Control High"]
pub mod GPCHR {
    #[doc = "Global Pin Write Data"]
    pub mod GPWD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
    #[doc = "Global Pin Write Enable"]
    pub mod GPWE31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not updated"]
            pub const GPWE0: u32 = 0;
            #[doc = "Updated"]
            pub const GPWE1: u32 = 1;
        }
    }
}
#[doc = "Configuration"]
pub mod CONFIG {
    #[doc = "Port Voltage Range"]
    pub mod RANGE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1.71 V-3.6 V"]
            pub const RANGE0: u32 = 0;
            #[doc = "2.70 V-3.6 V"]
            pub const RANGE1: u32 = 1;
        }
    }
}
#[doc = "EFT Detect Flag"]
pub mod EDFR {
    #[doc = "EFT Detect Flag"]
    pub mod EDF0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EFT Detect Flag"]
    pub mod EDF31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {
            #[doc = "No EFT event detected"]
            pub const EDIE0: u32 = 0;
            #[doc = "High or/and low EFT event detected"]
            pub const EDIE1: u32 = 1;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "EFT Detect Interrupt Enable"]
pub mod EDIER {
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE10 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE11 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE12 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE13 {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE14 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE15 {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE16 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE17 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE18 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE19 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE20 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE21 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE22 {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE23 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE24 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE25 {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE26 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE27 {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE28 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE29 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE30 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Interrupt Enable"]
    pub mod EDIE31 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt not generated upon detection of the EFT event"]
            pub const EDIE0: u32 = 0;
            #[doc = "Interrupt generated upon detection of the EFT event"]
            pub const EDIE1: u32 = 1;
        }
    }
}
#[doc = "EFT Detect Clear"]
pub mod EDCR {
    #[doc = "EFT Detect High Clear"]
    pub mod EDHC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not clear"]
            pub const EDHC0: u32 = 0;
            #[doc = "Clears"]
            pub const EDHC1: u32 = 1;
        }
    }
    #[doc = "EFT Detect Low Clear"]
    pub mod EDLC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not clear"]
            pub const EDLC0: u32 = 0;
            #[doc = "Clears"]
            pub const EDLC1: u32 = 1;
        }
    }
}
#[doc = "Calibration 0"]
pub mod CALIB0 {
    #[doc = "Calibration of NMOS Output Driver"]
    pub mod NCAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Calibration of PMOS Output Driver"]
    pub mod PCAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Calibration 1"]
pub mod CALIB1 {
    #[doc = "Calibration of NMOS Output Driver"]
    pub mod NCAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Calibration of PMOS Output Driver"]
    pub mod PCAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pin Control 0"]
pub mod PCR {
    #[doc = "Pull Select"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enables internal pulldown resistor"]
            pub const PS0: u32 = 0;
            #[doc = "Enables internal pullup resistor"]
            pub const PS1: u32 = 1;
        }
    }
    #[doc = "Pull Enable"]
    pub mod PE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const PE0: u32 = 0;
            #[doc = "Enables"]
            pub const PE1: u32 = 1;
        }
    }
    #[doc = "Slew Rate Enable"]
    pub mod SRE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fast"]
            pub const SRE0: u32 = 0;
            #[doc = "Slow"]
            pub const SRE1: u32 = 1;
        }
    }
    #[doc = "Open Drain Enable"]
    pub mod ODE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const ODE0: u32 = 0;
            #[doc = "Enables"]
            pub const ODE1: u32 = 1;
        }
    }
    #[doc = "Drive Strength Enable"]
    pub mod DSE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low"]
            pub const DSE0: u32 = 0;
            #[doc = "High"]
            pub const DSE1: u32 = 1;
        }
    }
    #[doc = "Pin Multiplex Control"]
    pub mod MUX {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Alternative 0 (GPIO)"]
            pub const MUX00: u32 = 0;
            #[doc = "Alternative 1 (chip-specific)"]
            pub const MUX01: u32 = 1;
            #[doc = "Alternative 2 (chip-specific)"]
            pub const MUX10: u32 = 2;
            #[doc = "Alternative 3 (chip-specific)"]
            pub const MUX11: u32 = 3;
            #[doc = "Alternative 4 (chip-specific)"]
            pub const MUX100: u32 = 4;
            #[doc = "Alternative 5 (chip-specific)"]
            pub const MUX101: u32 = 5;
            #[doc = "Alternative 6 (chip-specific)"]
            pub const MUX110: u32 = 6;
            #[doc = "Alternative 7 (chip-specific)"]
            pub const MUX111: u32 = 7;
            #[doc = "Alternative 8 (chip-specific)"]
            pub const MUX1000: u32 = 8;
            #[doc = "Alternative 9 (chip-specific)"]
            pub const MUX1001: u32 = 9;
            #[doc = "Alternative 10 (chip-specific)"]
            pub const MUX1010: u32 = 10;
            #[doc = "Alternative 11 (chip-specific)"]
            pub const MUX1011: u32 = 11;
            #[doc = "Alternative 12 (chip-specific)"]
            pub const MUX1100: u32 = 12;
            #[doc = "Alternative 13 (chip-specific)"]
            pub const MUX1101: u32 = 13;
        }
    }
    #[doc = "Input Buffer Enable"]
    pub mod IBE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const IBE0: u32 = 0;
            #[doc = "Enables"]
            pub const IBE1: u32 = 1;
        }
    }
    #[doc = "Invert Input"]
    pub mod INV {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not invert"]
            pub const INV0: u32 = 0;
            #[doc = "Inverts"]
            pub const INV1: u32 = 1;
        }
    }
    #[doc = "Lock Register"]
    pub mod LK {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Does not lock"]
            pub const LK0: u32 = 0;
            #[doc = "Locks"]
            pub const LK1: u32 = 1;
        }
    }
    #[doc = "Pull Value"]
    pub mod PV {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low"]
            pub const PV0: u32 = 0;
            #[doc = "High"]
            pub const PV1: u32 = 1;
        }
    }
    #[doc = "Passive Filter Enable"]
    pub mod PFE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const PFE0: u32 = 0;
            #[doc = "Enable"]
            pub const PFE1: u32 = 1;
        }
    }
}
