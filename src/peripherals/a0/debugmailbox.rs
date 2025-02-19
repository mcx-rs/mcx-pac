#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEBUGMAILBOX {
    ptr: *mut u8,
}
unsafe impl Send for DEBUGMAILBOX {}
unsafe impl Sync for DEBUGMAILBOX {}
impl DEBUGMAILBOX {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CSW(self) -> crate::common::Reg<regs::CSW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn REQUEST(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn RETURN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn ID(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
}
pub mod regs {
    #[doc = "Command and Status Word"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CSW(pub u32);
    impl CSW {
        #[inline(always)]
        pub const fn RESYNCH_REQ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESYNCH_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn REQ_PENDING(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REQ_PENDING(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DBG_OR_ERR(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DBG_OR_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn AHB_OR_ERR(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHB_OR_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn SOFT_RESET(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SOFT_RESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn CHIP_RESET_REQ(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHIP_RESET_REQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for CSW {
        #[inline(always)]
        fn default() -> CSW {
            CSW(0)
        }
    }
    impl core::fmt::Debug for CSW {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CSW")
                .field("RESYNCH_REQ", &self.RESYNCH_REQ())
                .field("REQ_PENDING", &self.REQ_PENDING())
                .field("DBG_OR_ERR", &self.DBG_OR_ERR())
                .field("AHB_OR_ERR", &self.AHB_OR_ERR())
                .field("SOFT_RESET", &self.SOFT_RESET())
                .field("CHIP_RESET_REQ", &self.CHIP_RESET_REQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CSW {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CSW {{ RESYNCH_REQ: {=bool:?}, REQ_PENDING: {=bool:?}, DBG_OR_ERR: {=bool:?}, AHB_OR_ERR: {=bool:?}, SOFT_RESET: {=bool:?}, CHIP_RESET_REQ: {=bool:?} }}" , self . RESYNCH_REQ () , self . REQ_PENDING () , self . DBG_OR_ERR () , self . AHB_OR_ERR () , self . SOFT_RESET () , self . CHIP_RESET_REQ ())
        }
    }
}
