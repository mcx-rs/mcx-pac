// #![no_std]

// use cfg_if::cfg_if;

// mod common;

// cfg_if! {
//     if #[cfg(not(feature = "_device_selected"))] {
//         compile_error!("no device feature selected");
//     }
// }

// cfg_if! {
//     if #[cfg(feature = "mcxn947")] {
//         #[path = "mcxn947/chip.rs"]
//         mod device;
//     }
// }

// pub use device::*;

#![no_std]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]

mod common;

#[cfg_attr(feature = "mcxn947", path = "mcxn947/chip.rs")]
mod device;

#[cfg(feature = "_device_selected")]
pub use device::*;
