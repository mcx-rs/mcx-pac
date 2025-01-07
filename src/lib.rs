#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

mod common;

mod device;
use core::marker::PhantomData;

pub use device::*;

pub struct Instance<T, const N: u8> {
    _t: PhantomData<T>,
}
impl<T, const N: u8> Instance<T, N> {
    pub const fn number() -> u8 {
        N
    }
}
