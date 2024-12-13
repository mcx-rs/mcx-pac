#![no_std]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]

mod common;

macro_rules! mod_device {
    ($(($feat:literal, $path:literal))*) => {
        $(
            #[cfg_attr(feature = $feat, path = $path)]
        )*
        mod device;
    };
}

mod_device! {
    ("mcxn947", "mcxn947/chip.rs")
    ("mcxn236", "mcxn236/chip.rs")
}

#[cfg(feature = "_device_selected")]
pub use device::*;
