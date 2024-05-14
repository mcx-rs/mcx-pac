#![no_std]
#![allow(non_camel_case_types)]

pub mod common;

#[cfg(not(feature = "device-selected"))]
compile_error!("No device selected");

#[cfg(any(feature = "mcxn947", feature = "mcxn946"))]
#[path = "devices/mcxn947/pac.rs"]
pub mod pac;

#[cfg(any(feature = "mcxn547", feature = "mcxn546"))]
#[path = "devices/mcxn947/pac.rs"]
pub mod pac;
