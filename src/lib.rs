#![no_std]

pub use ral_registers::{modify_reg, read_reg, write_reg, RORegister, RWRegister, WORegister};

mod instance;
pub use instance::*;

pub(crate) mod sealed {
    pub trait Sealed {}
}

#[cfg_attr(
    any(feature = "mcxn947", feature = "mcxn946"),
    path = "devices/mcxn947/mod.rs"
)]
#[cfg_attr(
    any(feature = "mcxn547", feature = "mcxn546"),
    path = "devices/mcxn547/mod.rs"
)]
pub mod device;
