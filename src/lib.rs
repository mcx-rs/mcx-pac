#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

pub mod common;

mod device;
pub use device::*;

use core::marker::PhantomData;
pub struct Instance<T, const N: u8> {
    _t: PhantomData<T>,
}
impl<T, const N: u8> Instance<T, N> {
    pub const fn number() -> u8 {
        N
    }
}

#[cfg(feature = "metadata")]
pub mod metadata {
    include!("metadata.rs");
    include!(env!("MCXPAC_METADATA_PATH"));
}
