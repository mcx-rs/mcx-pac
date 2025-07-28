#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MBC_INDEX {
    ptr: *mut u8,
}
unsafe impl Send for MBC_INDEX {}
unsafe impl Sync for MBC_INDEX {}
impl MBC_INDEX {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MBC_MEM_GLBCFG(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MBC_MEMN_GLBAC(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MBC_DOM0_MEM0_BLK_CFG_W(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MBC_DOM0_MEM1_BLK_CFG_W(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MBC_DOM0_MEM2_BLK_CFG_W(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize + n * 4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRDC {
    ptr: *mut u8,
}
unsafe impl Send for TRDC {}
unsafe impl Sync for TRDC {}
impl TRDC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MBC_INDEX(self, n: usize) -> MBC_INDEX {
        assert!(n < 1usize);
        unsafe { MBC_INDEX::from_ptr(self.ptr.add(0x0usize + n * 428usize) as _) }
    }
}
pub mod regs {
    #[doc = "MBC Memory Block Configuration Word"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBC_INDEX_MBC_DOM0_MEM0_BLK_CFG_W(pub u32);
    impl MBC_INDEX_MBC_DOM0_MEM0_BLK_CFG_W {
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE2(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE5(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE6(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE7(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MBC_INDEX_MBC_DOM0_MEM0_BLK_CFG_W {
        #[inline(always)]
        fn default() -> MBC_INDEX_MBC_DOM0_MEM0_BLK_CFG_W {
            MBC_INDEX_MBC_DOM0_MEM0_BLK_CFG_W(0)
        }
    }
    impl core::fmt::Debug for MBC_INDEX_MBC_DOM0_MEM0_BLK_CFG_W {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MBC_INDEX_MBC_DOM0_MEM0_BLK_CFG_W")
                .field("MBACSEL0", &self.MBACSEL0())
                .field("NSE0", &self.NSE0())
                .field("MBACSEL1", &self.MBACSEL1())
                .field("NSE1", &self.NSE1())
                .field("MBACSEL2", &self.MBACSEL2())
                .field("NSE2", &self.NSE2())
                .field("MBACSEL3", &self.MBACSEL3())
                .field("NSE3", &self.NSE3())
                .field("MBACSEL4", &self.MBACSEL4())
                .field("NSE4", &self.NSE4())
                .field("MBACSEL5", &self.MBACSEL5())
                .field("NSE5", &self.NSE5())
                .field("MBACSEL6", &self.MBACSEL6())
                .field("NSE6", &self.NSE6())
                .field("MBACSEL7", &self.MBACSEL7())
                .field("NSE7", &self.NSE7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MBC_INDEX_MBC_DOM0_MEM0_BLK_CFG_W {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MBC_INDEX_MBC_DOM0_MEM0_BLK_CFG_W {{ MBACSEL0: {=u8:?}, NSE0: {=bool:?}, MBACSEL1: {=u8:?}, NSE1: {=bool:?}, MBACSEL2: {=u8:?}, NSE2: {=bool:?}, MBACSEL3: {=u8:?}, NSE3: {=bool:?}, MBACSEL4: {=u8:?}, NSE4: {=bool:?}, MBACSEL5: {=u8:?}, NSE5: {=bool:?}, MBACSEL6: {=u8:?}, NSE6: {=bool:?}, MBACSEL7: {=u8:?}, NSE7: {=bool:?} }}" , self . MBACSEL0 () , self . NSE0 () , self . MBACSEL1 () , self . NSE1 () , self . MBACSEL2 () , self . NSE2 () , self . MBACSEL3 () , self . NSE3 () , self . MBACSEL4 () , self . NSE4 () , self . MBACSEL5 () , self . NSE5 () , self . MBACSEL6 () , self . NSE6 () , self . MBACSEL7 () , self . NSE7 ())
        }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBC_INDEX_MBC_DOM0_MEM1_BLK_CFG_W(pub u32);
    impl MBC_INDEX_MBC_DOM0_MEM1_BLK_CFG_W {
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE2(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE5(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE6(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE7(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MBC_INDEX_MBC_DOM0_MEM1_BLK_CFG_W {
        #[inline(always)]
        fn default() -> MBC_INDEX_MBC_DOM0_MEM1_BLK_CFG_W {
            MBC_INDEX_MBC_DOM0_MEM1_BLK_CFG_W(0)
        }
    }
    impl core::fmt::Debug for MBC_INDEX_MBC_DOM0_MEM1_BLK_CFG_W {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MBC_INDEX_MBC_DOM0_MEM1_BLK_CFG_W")
                .field("MBACSEL0", &self.MBACSEL0())
                .field("NSE0", &self.NSE0())
                .field("MBACSEL1", &self.MBACSEL1())
                .field("NSE1", &self.NSE1())
                .field("MBACSEL2", &self.MBACSEL2())
                .field("NSE2", &self.NSE2())
                .field("MBACSEL3", &self.MBACSEL3())
                .field("NSE3", &self.NSE3())
                .field("MBACSEL4", &self.MBACSEL4())
                .field("NSE4", &self.NSE4())
                .field("MBACSEL5", &self.MBACSEL5())
                .field("NSE5", &self.NSE5())
                .field("MBACSEL6", &self.MBACSEL6())
                .field("NSE6", &self.NSE6())
                .field("MBACSEL7", &self.MBACSEL7())
                .field("NSE7", &self.NSE7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MBC_INDEX_MBC_DOM0_MEM1_BLK_CFG_W {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MBC_INDEX_MBC_DOM0_MEM1_BLK_CFG_W {{ MBACSEL0: {=u8:?}, NSE0: {=bool:?}, MBACSEL1: {=u8:?}, NSE1: {=bool:?}, MBACSEL2: {=u8:?}, NSE2: {=bool:?}, MBACSEL3: {=u8:?}, NSE3: {=bool:?}, MBACSEL4: {=u8:?}, NSE4: {=bool:?}, MBACSEL5: {=u8:?}, NSE5: {=bool:?}, MBACSEL6: {=u8:?}, NSE6: {=bool:?}, MBACSEL7: {=u8:?}, NSE7: {=bool:?} }}" , self . MBACSEL0 () , self . NSE0 () , self . MBACSEL1 () , self . NSE1 () , self . MBACSEL2 () , self . NSE2 () , self . MBACSEL3 () , self . NSE3 () , self . MBACSEL4 () , self . NSE4 () , self . MBACSEL5 () , self . NSE5 () , self . MBACSEL6 () , self . NSE6 () , self . MBACSEL7 () , self . NSE7 ())
        }
    }
    #[doc = "MBC Memory Block Configuration Word"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBC_INDEX_MBC_DOM0_MEM2_BLK_CFG_W(pub u32);
    impl MBC_INDEX_MBC_DOM0_MEM2_BLK_CFG_W {
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE2(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE3(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE5(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE6(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MBACSEL7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MBACSEL7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NSE7(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NSE7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MBC_INDEX_MBC_DOM0_MEM2_BLK_CFG_W {
        #[inline(always)]
        fn default() -> MBC_INDEX_MBC_DOM0_MEM2_BLK_CFG_W {
            MBC_INDEX_MBC_DOM0_MEM2_BLK_CFG_W(0)
        }
    }
    impl core::fmt::Debug for MBC_INDEX_MBC_DOM0_MEM2_BLK_CFG_W {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MBC_INDEX_MBC_DOM0_MEM2_BLK_CFG_W")
                .field("MBACSEL0", &self.MBACSEL0())
                .field("NSE0", &self.NSE0())
                .field("MBACSEL1", &self.MBACSEL1())
                .field("NSE1", &self.NSE1())
                .field("MBACSEL2", &self.MBACSEL2())
                .field("NSE2", &self.NSE2())
                .field("MBACSEL3", &self.MBACSEL3())
                .field("NSE3", &self.NSE3())
                .field("MBACSEL4", &self.MBACSEL4())
                .field("NSE4", &self.NSE4())
                .field("MBACSEL5", &self.MBACSEL5())
                .field("NSE5", &self.NSE5())
                .field("MBACSEL6", &self.MBACSEL6())
                .field("NSE6", &self.NSE6())
                .field("MBACSEL7", &self.MBACSEL7())
                .field("NSE7", &self.NSE7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MBC_INDEX_MBC_DOM0_MEM2_BLK_CFG_W {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MBC_INDEX_MBC_DOM0_MEM2_BLK_CFG_W {{ MBACSEL0: {=u8:?}, NSE0: {=bool:?}, MBACSEL1: {=u8:?}, NSE1: {=bool:?}, MBACSEL2: {=u8:?}, NSE2: {=bool:?}, MBACSEL3: {=u8:?}, NSE3: {=bool:?}, MBACSEL4: {=u8:?}, NSE4: {=bool:?}, MBACSEL5: {=u8:?}, NSE5: {=bool:?}, MBACSEL6: {=u8:?}, NSE6: {=bool:?}, MBACSEL7: {=u8:?}, NSE7: {=bool:?} }}" , self . MBACSEL0 () , self . NSE0 () , self . MBACSEL1 () , self . NSE1 () , self . MBACSEL2 () , self . NSE2 () , self . MBACSEL3 () , self . NSE3 () , self . MBACSEL4 () , self . NSE4 () , self . MBACSEL5 () , self . NSE5 () , self . MBACSEL6 () , self . NSE6 () , self . MBACSEL7 () , self . NSE7 ())
        }
    }
    #[doc = "MBC Global Access Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBC_INDEX_MBC_MEMN_GLBAC(pub u32);
    impl MBC_INDEX_MBC_MEMN_GLBAC {
        #[must_use]
        #[inline(always)]
        pub const fn NUX(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NUX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NUW(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NUW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NUR(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NUR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NPX(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NPX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NPW(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NPW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NPR(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NPR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SUX(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SUX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SUW(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SUW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SUR(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SUR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPX(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPW(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPR(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MBC_INDEX_MBC_MEMN_GLBAC {
        #[inline(always)]
        fn default() -> MBC_INDEX_MBC_MEMN_GLBAC {
            MBC_INDEX_MBC_MEMN_GLBAC(0)
        }
    }
    impl core::fmt::Debug for MBC_INDEX_MBC_MEMN_GLBAC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MBC_INDEX_MBC_MEMN_GLBAC")
                .field("NUX", &self.NUX())
                .field("NUW", &self.NUW())
                .field("NUR", &self.NUR())
                .field("NPX", &self.NPX())
                .field("NPW", &self.NPW())
                .field("NPR", &self.NPR())
                .field("SUX", &self.SUX())
                .field("SUW", &self.SUW())
                .field("SUR", &self.SUR())
                .field("SPX", &self.SPX())
                .field("SPW", &self.SPW())
                .field("SPR", &self.SPR())
                .field("LK", &self.LK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MBC_INDEX_MBC_MEMN_GLBAC {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MBC_INDEX_MBC_MEMN_GLBAC {{ NUX: {=bool:?}, NUW: {=bool:?}, NUR: {=bool:?}, NPX: {=bool:?}, NPW: {=bool:?}, NPR: {=bool:?}, SUX: {=bool:?}, SUW: {=bool:?}, SUR: {=bool:?}, SPX: {=bool:?}, SPW: {=bool:?}, SPR: {=bool:?}, LK: {=bool:?} }}" , self . NUX () , self . NUW () , self . NUR () , self . NPX () , self . NPW () , self . NPR () , self . SUX () , self . SUW () , self . SUR () , self . SPX () , self . SPW () , self . SPR () , self . LK ())
        }
    }
    #[doc = "MBC Global Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBC_INDEX_MBC_MEM_GLBCFG(pub u32);
    impl MBC_INDEX_MBC_MEM_GLBCFG {
        #[must_use]
        #[inline(always)]
        pub const fn NBLKS(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_NBLKS(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SIZE_LOG2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SIZE_LOG2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CLRE(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CLRE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for MBC_INDEX_MBC_MEM_GLBCFG {
        #[inline(always)]
        fn default() -> MBC_INDEX_MBC_MEM_GLBCFG {
            MBC_INDEX_MBC_MEM_GLBCFG(0)
        }
    }
    impl core::fmt::Debug for MBC_INDEX_MBC_MEM_GLBCFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MBC_INDEX_MBC_MEM_GLBCFG")
                .field("NBLKS", &self.NBLKS())
                .field("SIZE_LOG2", &self.SIZE_LOG2())
                .field("CLRE", &self.CLRE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MBC_INDEX_MBC_MEM_GLBCFG {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MBC_INDEX_MBC_MEM_GLBCFG {{ NBLKS: {=u16:?}, SIZE_LOG2: {=u8:?}, CLRE: {=u8:?} }}",
                self.NBLKS(),
                self.SIZE_LOG2(),
                self.CLRE()
            )
        }
    }
}
