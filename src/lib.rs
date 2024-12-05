#![no_std]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]

mod common;

#[cfg_attr(feature = "mcxn947", path = "mcxn947/chip.rs")]
mod device;

#[cfg(feature = "_device_selected")]
pub use device::*;
