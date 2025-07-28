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
    pub const fn MBC_NSE_BLK_INDEX(
        self,
    ) -> crate::common::Reg<regs::MBC_INDEX_MBC_NSE_BLK_INDEX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn MBC_NSE_BLK_SET(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn MBC_NSE_BLK_CLR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn MBC_NSE_BLK_CLR_ALL(
        self,
    ) -> crate::common::Reg<regs::MBC_INDEX_MBC_NSE_BLK_CLR_ALL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
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
    pub const fn MBC_DOM0_MEM0_BLK_NSE_W(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize + n * 4usize) as _) }
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
    pub const fn MBC_DOM0_MEM1_BLK_NSE_W(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MBC_DOM0_MEM2_BLK_CFG_W(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MBC_DOM0_MEM2_BLK_NSE_W(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize + n * 4usize) as _) }
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
        unsafe { MBC_INDEX::from_ptr(self.ptr.add(0x0usize + n * 460usize) as _) }
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
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBC_INDEX_MBC_DOM0_MEM0_BLK_NSE_W(pub u32);
    impl MBC_INDEX_MBC_DOM0_MEM0_BLK_NSE_W {
        #[must_use]
        #[inline(always)]
        pub const fn BIT0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MBC_INDEX_MBC_DOM0_MEM0_BLK_NSE_W {
        #[inline(always)]
        fn default() -> MBC_INDEX_MBC_DOM0_MEM0_BLK_NSE_W {
            MBC_INDEX_MBC_DOM0_MEM0_BLK_NSE_W(0)
        }
    }
    impl core::fmt::Debug for MBC_INDEX_MBC_DOM0_MEM0_BLK_NSE_W {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MBC_INDEX_MBC_DOM0_MEM0_BLK_NSE_W")
                .field("BIT0", &self.BIT0())
                .field("BIT1", &self.BIT1())
                .field("BIT2", &self.BIT2())
                .field("BIT3", &self.BIT3())
                .field("BIT4", &self.BIT4())
                .field("BIT5", &self.BIT5())
                .field("BIT6", &self.BIT6())
                .field("BIT7", &self.BIT7())
                .field("BIT8", &self.BIT8())
                .field("BIT9", &self.BIT9())
                .field("BIT10", &self.BIT10())
                .field("BIT11", &self.BIT11())
                .field("BIT12", &self.BIT12())
                .field("BIT13", &self.BIT13())
                .field("BIT14", &self.BIT14())
                .field("BIT15", &self.BIT15())
                .field("BIT16", &self.BIT16())
                .field("BIT17", &self.BIT17())
                .field("BIT18", &self.BIT18())
                .field("BIT19", &self.BIT19())
                .field("BIT20", &self.BIT20())
                .field("BIT21", &self.BIT21())
                .field("BIT22", &self.BIT22())
                .field("BIT23", &self.BIT23())
                .field("BIT24", &self.BIT24())
                .field("BIT25", &self.BIT25())
                .field("BIT26", &self.BIT26())
                .field("BIT27", &self.BIT27())
                .field("BIT28", &self.BIT28())
                .field("BIT29", &self.BIT29())
                .field("BIT30", &self.BIT30())
                .field("BIT31", &self.BIT31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MBC_INDEX_MBC_DOM0_MEM0_BLK_NSE_W {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MBC_INDEX_MBC_DOM0_MEM0_BLK_NSE_W {{ BIT0: {=bool:?}, BIT1: {=bool:?}, BIT2: {=bool:?}, BIT3: {=bool:?}, BIT4: {=bool:?}, BIT5: {=bool:?}, BIT6: {=bool:?}, BIT7: {=bool:?}, BIT8: {=bool:?}, BIT9: {=bool:?}, BIT10: {=bool:?}, BIT11: {=bool:?}, BIT12: {=bool:?}, BIT13: {=bool:?}, BIT14: {=bool:?}, BIT15: {=bool:?}, BIT16: {=bool:?}, BIT17: {=bool:?}, BIT18: {=bool:?}, BIT19: {=bool:?}, BIT20: {=bool:?}, BIT21: {=bool:?}, BIT22: {=bool:?}, BIT23: {=bool:?}, BIT24: {=bool:?}, BIT25: {=bool:?}, BIT26: {=bool:?}, BIT27: {=bool:?}, BIT28: {=bool:?}, BIT29: {=bool:?}, BIT30: {=bool:?}, BIT31: {=bool:?} }}" , self . BIT0 () , self . BIT1 () , self . BIT2 () , self . BIT3 () , self . BIT4 () , self . BIT5 () , self . BIT6 () , self . BIT7 () , self . BIT8 () , self . BIT9 () , self . BIT10 () , self . BIT11 () , self . BIT12 () , self . BIT13 () , self . BIT14 () , self . BIT15 () , self . BIT16 () , self . BIT17 () , self . BIT18 () , self . BIT19 () , self . BIT20 () , self . BIT21 () , self . BIT22 () , self . BIT23 () , self . BIT24 () , self . BIT25 () , self . BIT26 () , self . BIT27 () , self . BIT28 () , self . BIT29 () , self . BIT30 () , self . BIT31 ())
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
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBC_INDEX_MBC_DOM0_MEM1_BLK_NSE_W(pub u32);
    impl MBC_INDEX_MBC_DOM0_MEM1_BLK_NSE_W {
        #[must_use]
        #[inline(always)]
        pub const fn BIT0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MBC_INDEX_MBC_DOM0_MEM1_BLK_NSE_W {
        #[inline(always)]
        fn default() -> MBC_INDEX_MBC_DOM0_MEM1_BLK_NSE_W {
            MBC_INDEX_MBC_DOM0_MEM1_BLK_NSE_W(0)
        }
    }
    impl core::fmt::Debug for MBC_INDEX_MBC_DOM0_MEM1_BLK_NSE_W {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MBC_INDEX_MBC_DOM0_MEM1_BLK_NSE_W")
                .field("BIT0", &self.BIT0())
                .field("BIT1", &self.BIT1())
                .field("BIT2", &self.BIT2())
                .field("BIT3", &self.BIT3())
                .field("BIT4", &self.BIT4())
                .field("BIT5", &self.BIT5())
                .field("BIT6", &self.BIT6())
                .field("BIT7", &self.BIT7())
                .field("BIT8", &self.BIT8())
                .field("BIT9", &self.BIT9())
                .field("BIT10", &self.BIT10())
                .field("BIT11", &self.BIT11())
                .field("BIT12", &self.BIT12())
                .field("BIT13", &self.BIT13())
                .field("BIT14", &self.BIT14())
                .field("BIT15", &self.BIT15())
                .field("BIT16", &self.BIT16())
                .field("BIT17", &self.BIT17())
                .field("BIT18", &self.BIT18())
                .field("BIT19", &self.BIT19())
                .field("BIT20", &self.BIT20())
                .field("BIT21", &self.BIT21())
                .field("BIT22", &self.BIT22())
                .field("BIT23", &self.BIT23())
                .field("BIT24", &self.BIT24())
                .field("BIT25", &self.BIT25())
                .field("BIT26", &self.BIT26())
                .field("BIT27", &self.BIT27())
                .field("BIT28", &self.BIT28())
                .field("BIT29", &self.BIT29())
                .field("BIT30", &self.BIT30())
                .field("BIT31", &self.BIT31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MBC_INDEX_MBC_DOM0_MEM1_BLK_NSE_W {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MBC_INDEX_MBC_DOM0_MEM1_BLK_NSE_W {{ BIT0: {=bool:?}, BIT1: {=bool:?}, BIT2: {=bool:?}, BIT3: {=bool:?}, BIT4: {=bool:?}, BIT5: {=bool:?}, BIT6: {=bool:?}, BIT7: {=bool:?}, BIT8: {=bool:?}, BIT9: {=bool:?}, BIT10: {=bool:?}, BIT11: {=bool:?}, BIT12: {=bool:?}, BIT13: {=bool:?}, BIT14: {=bool:?}, BIT15: {=bool:?}, BIT16: {=bool:?}, BIT17: {=bool:?}, BIT18: {=bool:?}, BIT19: {=bool:?}, BIT20: {=bool:?}, BIT21: {=bool:?}, BIT22: {=bool:?}, BIT23: {=bool:?}, BIT24: {=bool:?}, BIT25: {=bool:?}, BIT26: {=bool:?}, BIT27: {=bool:?}, BIT28: {=bool:?}, BIT29: {=bool:?}, BIT30: {=bool:?}, BIT31: {=bool:?} }}" , self . BIT0 () , self . BIT1 () , self . BIT2 () , self . BIT3 () , self . BIT4 () , self . BIT5 () , self . BIT6 () , self . BIT7 () , self . BIT8 () , self . BIT9 () , self . BIT10 () , self . BIT11 () , self . BIT12 () , self . BIT13 () , self . BIT14 () , self . BIT15 () , self . BIT16 () , self . BIT17 () , self . BIT18 () , self . BIT19 () , self . BIT20 () , self . BIT21 () , self . BIT22 () , self . BIT23 () , self . BIT24 () , self . BIT25 () , self . BIT26 () , self . BIT27 () , self . BIT28 () , self . BIT29 () , self . BIT30 () , self . BIT31 ())
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
    #[doc = "MBC Memory Block NonSecure Enable Word"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBC_INDEX_MBC_DOM0_MEM2_BLK_NSE_W(pub u32);
    impl MBC_INDEX_MBC_DOM0_MEM2_BLK_NSE_W {
        #[must_use]
        #[inline(always)]
        pub const fn BIT0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BIT31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BIT31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MBC_INDEX_MBC_DOM0_MEM2_BLK_NSE_W {
        #[inline(always)]
        fn default() -> MBC_INDEX_MBC_DOM0_MEM2_BLK_NSE_W {
            MBC_INDEX_MBC_DOM0_MEM2_BLK_NSE_W(0)
        }
    }
    impl core::fmt::Debug for MBC_INDEX_MBC_DOM0_MEM2_BLK_NSE_W {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MBC_INDEX_MBC_DOM0_MEM2_BLK_NSE_W")
                .field("BIT0", &self.BIT0())
                .field("BIT1", &self.BIT1())
                .field("BIT2", &self.BIT2())
                .field("BIT3", &self.BIT3())
                .field("BIT4", &self.BIT4())
                .field("BIT5", &self.BIT5())
                .field("BIT6", &self.BIT6())
                .field("BIT7", &self.BIT7())
                .field("BIT8", &self.BIT8())
                .field("BIT9", &self.BIT9())
                .field("BIT10", &self.BIT10())
                .field("BIT11", &self.BIT11())
                .field("BIT12", &self.BIT12())
                .field("BIT13", &self.BIT13())
                .field("BIT14", &self.BIT14())
                .field("BIT15", &self.BIT15())
                .field("BIT16", &self.BIT16())
                .field("BIT17", &self.BIT17())
                .field("BIT18", &self.BIT18())
                .field("BIT19", &self.BIT19())
                .field("BIT20", &self.BIT20())
                .field("BIT21", &self.BIT21())
                .field("BIT22", &self.BIT22())
                .field("BIT23", &self.BIT23())
                .field("BIT24", &self.BIT24())
                .field("BIT25", &self.BIT25())
                .field("BIT26", &self.BIT26())
                .field("BIT27", &self.BIT27())
                .field("BIT28", &self.BIT28())
                .field("BIT29", &self.BIT29())
                .field("BIT30", &self.BIT30())
                .field("BIT31", &self.BIT31())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MBC_INDEX_MBC_DOM0_MEM2_BLK_NSE_W {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MBC_INDEX_MBC_DOM0_MEM2_BLK_NSE_W {{ BIT0: {=bool:?}, BIT1: {=bool:?}, BIT2: {=bool:?}, BIT3: {=bool:?}, BIT4: {=bool:?}, BIT5: {=bool:?}, BIT6: {=bool:?}, BIT7: {=bool:?}, BIT8: {=bool:?}, BIT9: {=bool:?}, BIT10: {=bool:?}, BIT11: {=bool:?}, BIT12: {=bool:?}, BIT13: {=bool:?}, BIT14: {=bool:?}, BIT15: {=bool:?}, BIT16: {=bool:?}, BIT17: {=bool:?}, BIT18: {=bool:?}, BIT19: {=bool:?}, BIT20: {=bool:?}, BIT21: {=bool:?}, BIT22: {=bool:?}, BIT23: {=bool:?}, BIT24: {=bool:?}, BIT25: {=bool:?}, BIT26: {=bool:?}, BIT27: {=bool:?}, BIT28: {=bool:?}, BIT29: {=bool:?}, BIT30: {=bool:?}, BIT31: {=bool:?} }}" , self . BIT0 () , self . BIT1 () , self . BIT2 () , self . BIT3 () , self . BIT4 () , self . BIT5 () , self . BIT6 () , self . BIT7 () , self . BIT8 () , self . BIT9 () , self . BIT10 () , self . BIT11 () , self . BIT12 () , self . BIT13 () , self . BIT14 () , self . BIT15 () , self . BIT16 () , self . BIT17 () , self . BIT18 () , self . BIT19 () , self . BIT20 () , self . BIT21 () , self . BIT22 () , self . BIT23 () , self . BIT24 () , self . BIT25 () , self . BIT26 () , self . BIT27 () , self . BIT28 () , self . BIT29 () , self . BIT30 () , self . BIT31 ())
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
    #[doc = "MBC NonSecure Enable Block Clear All"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBC_INDEX_MBC_NSE_BLK_CLR_ALL(pub u32);
    impl MBC_INDEX_MBC_NSE_BLK_CLR_ALL {
        #[must_use]
        #[inline(always)]
        pub const fn MEMSEL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MEMSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DID_SEL0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DID_SEL0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for MBC_INDEX_MBC_NSE_BLK_CLR_ALL {
        #[inline(always)]
        fn default() -> MBC_INDEX_MBC_NSE_BLK_CLR_ALL {
            MBC_INDEX_MBC_NSE_BLK_CLR_ALL(0)
        }
    }
    impl core::fmt::Debug for MBC_INDEX_MBC_NSE_BLK_CLR_ALL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MBC_INDEX_MBC_NSE_BLK_CLR_ALL")
                .field("MEMSEL", &self.MEMSEL())
                .field("DID_SEL0", &self.DID_SEL0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MBC_INDEX_MBC_NSE_BLK_CLR_ALL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MBC_INDEX_MBC_NSE_BLK_CLR_ALL {{ MEMSEL: {=u8:?}, DID_SEL0: {=bool:?} }}",
                self.MEMSEL(),
                self.DID_SEL0()
            )
        }
    }
    #[doc = "MBC NonSecure Enable Block Index"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MBC_INDEX_MBC_NSE_BLK_INDEX(pub u32);
    impl MBC_INDEX_MBC_NSE_BLK_INDEX {
        #[must_use]
        #[inline(always)]
        pub const fn WNDX(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WNDX(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MEM_SEL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MEM_SEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DID_SEL0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DID_SEL0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AI(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MBC_INDEX_MBC_NSE_BLK_INDEX {
        #[inline(always)]
        fn default() -> MBC_INDEX_MBC_NSE_BLK_INDEX {
            MBC_INDEX_MBC_NSE_BLK_INDEX(0)
        }
    }
    impl core::fmt::Debug for MBC_INDEX_MBC_NSE_BLK_INDEX {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MBC_INDEX_MBC_NSE_BLK_INDEX")
                .field("WNDX", &self.WNDX())
                .field("MEM_SEL", &self.MEM_SEL())
                .field("DID_SEL0", &self.DID_SEL0())
                .field("AI", &self.AI())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MBC_INDEX_MBC_NSE_BLK_INDEX {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MBC_INDEX_MBC_NSE_BLK_INDEX {{ WNDX: {=u8:?}, MEM_SEL: {=u8:?}, DID_SEL0: {=bool:?}, AI: {=bool:?} }}" , self . WNDX () , self . MEM_SEL () , self . DID_SEL0 () , self . AI ())
        }
    }
}
