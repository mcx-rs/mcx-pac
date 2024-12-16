#![no_std]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]

mod common;

#[cfg_attr(feature = "mcxa153", path = "mcxa153/lib.rs")]
#[cfg_attr(feature = "mcxa156", path = "mcxa156/lib.rs")]
#[cfg_attr(feature = "mcxa276", path = "mcxa276/lib.rs")]
#[cfg_attr(feature = "mcxn236", path = "mcxn236/lib.rs")]
#[cfg_attr(feature = "mcxn947", path = "mcxn947/lib.rs")]
mod device;
#[cfg(feature = "_device_selected")]
pub use device::*;
