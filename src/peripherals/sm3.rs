#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]
#[doc = "SAFO_SM3_SGI"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x200],
    #[doc = "Input Data register 0 lower-bank low"]
    pub DATIN0A: crate::RWRegister<u32>,
    #[doc = "Input Data register 0 lower-bank high"]
    pub DATIN0B: crate::RWRegister<u32>,
    #[doc = "Input Data register 0 upper-bank low"]
    pub DATIN0C: crate::RWRegister<u32>,
    #[doc = "Input Data register 0 upper-bank high"]
    pub DATIN0D: crate::RWRegister<u32>,
    #[doc = "Input Data register 1 lower-bank low"]
    pub DATIN1A: crate::RWRegister<u32>,
    #[doc = "Input Data register 1 lower-bank high"]
    pub DATIN1B: crate::RWRegister<u32>,
    #[doc = "Input Data register 1 upper-bank low"]
    pub DATIN1C: crate::RWRegister<u32>,
    #[doc = "Input Data register 1 upper-bank high"]
    pub DATIN1D: crate::RWRegister<u32>,
    _reserved1: [u8; 0x20],
    #[doc = "Input Key register 0 lower-bank low"]
    pub KEY0A: crate::RWRegister<u32>,
    #[doc = "Input Key register 0 lower-bank high"]
    pub KEY0B: crate::RWRegister<u32>,
    #[doc = "Input Key register 0 upper-bank low"]
    pub KEY0C: crate::RWRegister<u32>,
    #[doc = "Input Key register 0 upper-bank high"]
    pub KEY0D: crate::RWRegister<u32>,
    #[doc = "Input Key register 1 lower-bank low"]
    pub KEY1A: crate::RWRegister<u32>,
    #[doc = "Input Key register 1 lower-bank high"]
    pub KEY1B: crate::RWRegister<u32>,
    #[doc = "Input Key register 1 upper-bank low"]
    pub KEY1C: crate::RWRegister<u32>,
    #[doc = "Input Key register 1 upper-bank high"]
    pub KEY1D: crate::RWRegister<u32>,
    _reserved2: [u8; 0x60],
    #[doc = "Output Data register lower-bank low"]
    pub DATOUTA: crate::RWRegister<u32>,
    #[doc = "Ouput Data register lower-bank high"]
    pub DATOUTB: crate::RWRegister<u32>,
    #[doc = "Ouput Data register upper-bank low"]
    pub DATOUTC: crate::RWRegister<u32>,
    #[doc = "Output Data register upper-bank high"]
    pub DATOUTD: crate::RWRegister<u32>,
    _reserved3: [u8; 0x930],
    #[doc = "Status Register"]
    pub STATUS: crate::RWRegister<u32>,
    #[doc = "Calculation Counter"]
    pub COUNT: crate::RWRegister<u32>,
    _reserved4: [u8; 0xf8],
    #[doc = "SGI Control Register"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "SGI Control Register 2"]
    pub CTRL2: crate::RWRegister<u32>,
    _reserved5: [u8; 0xc],
    #[doc = "SM3 Control Register"]
    pub SM3_CTRL: crate::RWRegister<u32>,
    #[doc = "SM3 FIFO Register"]
    pub SM3_FIFO: crate::RWRegister<u32>,
    #[doc = "SGI Configuration Register"]
    pub CONFIG: crate::RWRegister<u32>,
    _reserved6: [u8; 0x2c4],
    #[doc = "Interrupt Enable"]
    pub INT_ENABLE: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Clear"]
    pub INT_STATUS_CLR: crate::RWRegister<u32>,
    #[doc = "Interrupt Status Set"]
    pub INT_STATUS_SET: crate::RWRegister<u32>,
}
#[doc = "Input Data register 0 lower-bank low"]
pub mod DATIN0A {
    #[doc = "Input Data register"]
    pub mod DATIN0A {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Data register 0 lower-bank high"]
pub mod DATIN0B {
    #[doc = "Input Data register"]
    pub mod DATIN0B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Data register 0 upper-bank low"]
pub mod DATIN0C {
    #[doc = "Input Data register"]
    pub mod DATIN0C {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Data register 0 upper-bank high"]
pub mod DATIN0D {
    #[doc = "Input Data register"]
    pub mod DATIN0D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Data register 1 lower-bank low"]
pub mod DATIN1A {
    #[doc = "Input Data register"]
    pub mod DATIN1A {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Data register 1 lower-bank high"]
pub mod DATIN1B {
    #[doc = "Input Data register"]
    pub mod DATIN1B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Data register 1 upper-bank low"]
pub mod DATIN1C {
    #[doc = "Input Data register"]
    pub mod DATIN1C {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Data register 1 upper-bank high"]
pub mod DATIN1D {
    #[doc = "Input Data register"]
    pub mod DATIN1D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Key register 0 lower-bank low"]
pub mod KEY0A {
    #[doc = "Input Key register"]
    pub mod KEY0A {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Key register 0 lower-bank high"]
pub mod KEY0B {
    #[doc = "Input Key register"]
    pub mod KEY0B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Key register 0 upper-bank low"]
pub mod KEY0C {
    #[doc = "Input Key register"]
    pub mod KEY0C {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Key register 0 upper-bank high"]
pub mod KEY0D {
    #[doc = "Input Key register"]
    pub mod KEY0D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Key register 1 lower-bank low"]
pub mod KEY1A {
    #[doc = "Input Key register"]
    pub mod KEY1A {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Key register 1 lower-bank high"]
pub mod KEY1B {
    #[doc = "Input Key register"]
    pub mod KEY1B {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Key register 1 upper-bank low"]
pub mod KEY1C {
    #[doc = "Input Key register"]
    pub mod KEY1C {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Key register 1 upper-bank high"]
pub mod KEY1D {
    #[doc = "Input Key register"]
    pub mod KEY1D {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Data register lower-bank low"]
pub mod DATOUTA {
    #[doc = "Output Data register"]
    pub mod DATOUTA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ouput Data register lower-bank high"]
pub mod DATOUTB {
    #[doc = "Ouput Data register"]
    pub mod DATOUTB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ouput Data register upper-bank low"]
pub mod DATOUTC {
    #[doc = "Ouput Data register"]
    pub mod DATOUTC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Output Data register upper-bank high"]
pub mod DATOUTD {
    #[doc = "Output Data register"]
    pub mod DOUTD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register"]
pub mod STATUS {
    #[doc = "Busy Flag"]
    pub mod BUSY {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "PRNG Ready"]
    pub mod PRNG_RDY {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Error Indicator"]
    pub mod ERROR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SM3 Busy Status Flag"]
    pub mod SM3_BUSY {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Interrupt Status Flag"]
    pub mod IRQ {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SM3 FIFO Full Indicator"]
    pub mod SM3_FIFO_FULL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SM3 FIFO Level"]
    pub mod SM3_FIFO_LEVEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SM3 ERROR"]
    pub mod SM3_ERROR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Calculation Counter"]
pub mod COUNT {
    #[doc = "Calculation Counter"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SGI Control Register"]
pub mod CTRL {
    #[doc = "Start Crypto Operation"]
    pub mod START {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Crypto Operation Type"]
    pub mod CRYPTO_OP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Kernels Data Out Options"]
    pub mod DATOUT_RES {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SGI Control Register 2"]
pub mod CTRL2 {
    #[doc = "Start Full SGI Flush"]
    pub mod FLUSH {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start KEY register-bank Flush"]
    pub mod KEY_FLUSH {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start DATIN register-bank Flush"]
    pub mod DATIN_FLUSH {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Flush Write control"]
    pub mod FLUSHWR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Byte order of regbank read/write data"]
    pub mod BYTES_ORDER {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SM3 Control Register"]
pub mod SM3_CTRL {
    #[doc = "SM3 mode normal or automatic"]
    pub mod SM3_MODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SM3 FIFO low limit"]
    pub mod SM3_LOW_LIM {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SM3 FIFO high limit"]
    pub mod SM3_HIGH_LIM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SM3 Calculation Counter Enable"]
    pub mod SM3_COUNT_EN {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SM3 HASH reload"]
    pub mod HASH_RELOAD {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "STOP SM3 AUTO mode"]
    pub mod SM3_STOP {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SM3 no automatic HASH initialisation."]
    pub mod NO_AUTO_INIT {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SM3 enable"]
    pub mod SM3_EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SM3 FIFO Register"]
pub mod SM3_FIFO {
    #[doc = "SM3 FIFO Register"]
    pub mod FIFO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b11111111111111111111111111111111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SGI Configuration Register"]
pub mod CONFIG {
    #[doc = "ROW"]
    pub mod ROW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CHINA"]
    pub mod CHINA {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CC"]
    pub mod CC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Has AES"]
    pub mod HAS_AES {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Has DES"]
    pub mod HAS_DES {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Has SHA"]
    pub mod HAS_SHA {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Has MOVEM"]
    pub mod HAS_MOVEM {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Has CMAC"]
    pub mod HAS_CMAC {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Has GFMUL"]
    pub mod HAS_GFMUL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Has internal PRNG"]
    pub mod INTERNAL_PRNG {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Has key digest"]
    pub mod KEY_DIGEST {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Has MST"]
    pub mod MST {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus width"]
    pub mod BUS_WIDTH {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number DATIN"]
    pub mod NUM_DATIN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0b11 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number KEY"]
    pub mod NUM_KEY {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0b111 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "EDC enable"]
    pub mod EDC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Has dual SGI"]
    pub mod DUAL_SGI {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Has SHA 256 only"]
    pub mod SHA_256_ONLY {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID_CFG_SPB_SUPPORT is set"]
    pub mod SPB_SUPPORT {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID_CFG_SGI_SPB_MASKING is set"]
    pub mod SPB_MASKING {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ID_CFG_SGI_USE_SFR_SW_MASK is set"]
    pub mod SFR_SW_MASK {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Has SM3"]
    pub mod HAS_SM3 {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable"]
pub mod INT_ENABLE {
    #[doc = "Interrupt enable bit"]
    pub mod INT_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Clear"]
pub mod INT_STATUS_CLR {
    #[doc = "Interrupt Status Clear"]
    pub mod INT_CLR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Status Set"]
pub mod INT_STATUS_SET {
    #[doc = "Set Interrupt by SW"]
    pub mod INT_SET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0b1 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
