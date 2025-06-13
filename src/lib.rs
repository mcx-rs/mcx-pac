#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

pub mod common;

mod device;
use core::marker::PhantomData;

pub use device::*;

#[cfg(feature = "metadata")]
pub(crate) mod _metadata;

#[cfg(feature = "metadata")]
pub mod metadata {
    #![allow(dead_code)]
    #![allow(unused_imports)]

    include!(concat!(env!("OUT_DIR"), "/_generated_metadata.rs"));
}

pub struct Instance<T, const N: u8> {
    _t: PhantomData<T>,
}
impl<T, const N: u8> Instance<T, N> {
    pub const fn number() -> u8 {
        N
    }
}
