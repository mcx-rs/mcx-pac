#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LP_FLEXCOMM {
    ptr: *mut u8,
}
unsafe impl Send for LP_FLEXCOMM {}
unsafe impl Sync for LP_FLEXCOMM {}
impl LP_FLEXCOMM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ISTAT(self) -> crate::common::Reg<regs::ISTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff4usize) as _) }
    }
    #[inline(always)]
    pub const fn PSELID(self) -> crate::common::Reg<regs::PSELID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff8usize) as _) }
    }
}
pub mod regs {
    #[doc = "Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ISTAT(pub u32);
    impl ISTAT {
        #[must_use]
        #[inline(always)]
        pub const fn UARTTX(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UARTTX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UARTRX(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UARTRX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPI(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn I2CM(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_I2CM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn I2CS(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_I2CS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for ISTAT {
        #[inline(always)]
        fn default() -> ISTAT {
            ISTAT(0)
        }
    }
    impl core::fmt::Debug for ISTAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ISTAT")
                .field("UARTTX", &self.UARTTX())
                .field("UARTRX", &self.UARTRX())
                .field("SPI", &self.SPI())
                .field("I2CM", &self.I2CM())
                .field("I2CS", &self.I2CS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ISTAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "ISTAT {{ UARTTX: {=bool:?}, UARTRX: {=bool:?}, SPI: {=bool:?}, I2CM: {=bool:?}, I2CS: {=bool:?} }}" , self . UARTTX () , self . UARTRX () , self . SPI () , self . I2CM () , self . I2CS ())
        }
    }
    #[doc = "Peripheral Select and ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PSELID(pub u32);
    impl PSELID {
        #[must_use]
        #[inline(always)]
        pub const fn PERSEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PERSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UARTPRESENT(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UARTPRESENT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPIPRESENT(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPIPRESENT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn I2CPRESENT(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_I2CPRESENT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ID(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ID(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for PSELID {
        #[inline(always)]
        fn default() -> PSELID {
            PSELID(0)
        }
    }
    impl core::fmt::Debug for PSELID {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PSELID")
                .field("PERSEL", &self.PERSEL())
                .field("LOCK", &self.LOCK())
                .field("UARTPRESENT", &self.UARTPRESENT())
                .field("SPIPRESENT", &self.SPIPRESENT())
                .field("I2CPRESENT", &self.I2CPRESENT())
                .field("ID", &self.ID())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PSELID {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "PSELID {{ PERSEL: {=u8:?}, LOCK: {=bool:?}, UARTPRESENT: {=bool:?}, SPIPRESENT: {=bool:?}, I2CPRESENT: {=bool:?}, ID: {=u32:?} }}" , self . PERSEL () , self . LOCK () , self . UARTPRESENT () , self . SPIPRESENT () , self . I2CPRESENT () , self . ID ())
        }
    }
}
