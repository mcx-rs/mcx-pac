#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CDOG {
    ptr: *mut u8,
}
unsafe impl Send for CDOG {}
unsafe impl Sync for CDOG {}
impl CDOG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CONTROL(self) -> crate::common::Reg<regs::CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn RELOAD(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn INSTRUCTION_TIMER(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn STATUS2(self) -> crate::common::Reg<regs::STATUS2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn FLAGS(self) -> crate::common::Reg<regs::FLAGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn PERSISTENT(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn START(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn STOP(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn RESTART(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn ADD(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn ADD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn ADD16(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn ADD256(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn SUB(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn SUB1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn SUB16(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn SUB256(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn ASSERT16(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONTROL(pub u32);
    impl CONTROL {
        #[must_use]
        #[inline(always)]
        pub const fn LOCK_CTRL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_LOCK_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TIMEOUT_CTRL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TIMEOUT_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MISCOMPARE_CTRL(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MISCOMPARE_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEQUENCE_CTRL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SEQUENCE_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STATE_CTRL(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_STATE_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDRESS_CTRL(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADDRESS_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IRQ_PAUSE(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IRQ_PAUSE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DEBUG_HALT_CTRL(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DEBUG_HALT_CTRL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for CONTROL {
        #[inline(always)]
        fn default() -> CONTROL {
            CONTROL(0)
        }
    }
    impl core::fmt::Debug for CONTROL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CONTROL")
                .field("LOCK_CTRL", &self.LOCK_CTRL())
                .field("TIMEOUT_CTRL", &self.TIMEOUT_CTRL())
                .field("MISCOMPARE_CTRL", &self.MISCOMPARE_CTRL())
                .field("SEQUENCE_CTRL", &self.SEQUENCE_CTRL())
                .field("STATE_CTRL", &self.STATE_CTRL())
                .field("ADDRESS_CTRL", &self.ADDRESS_CTRL())
                .field("IRQ_PAUSE", &self.IRQ_PAUSE())
                .field("DEBUG_HALT_CTRL", &self.DEBUG_HALT_CTRL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CONTROL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "CONTROL {{ LOCK_CTRL: {=u8:?}, TIMEOUT_CTRL: {=u8:?}, MISCOMPARE_CTRL: {=u8:?}, SEQUENCE_CTRL: {=u8:?}, STATE_CTRL: {=u8:?}, ADDRESS_CTRL: {=u8:?}, IRQ_PAUSE: {=u8:?}, DEBUG_HALT_CTRL: {=u8:?} }}" , self . LOCK_CTRL () , self . TIMEOUT_CTRL () , self . MISCOMPARE_CTRL () , self . SEQUENCE_CTRL () , self . STATE_CTRL () , self . ADDRESS_CTRL () , self . IRQ_PAUSE () , self . DEBUG_HALT_CTRL ())
        }
    }
    #[doc = "Flags Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLAGS(pub u32);
    impl FLAGS {
        #[must_use]
        #[inline(always)]
        pub const fn TO_FLAG(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TO_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MISCOM_FLAG(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MISCOM_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SEQ_FLAG(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SEQ_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CNT_FLAG(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CNT_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STATE_FLAG(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_STATE_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDR_FLAG(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ADDR_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn POR_FLAG(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_POR_FLAG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for FLAGS {
        #[inline(always)]
        fn default() -> FLAGS {
            FLAGS(0)
        }
    }
    impl core::fmt::Debug for FLAGS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLAGS")
                .field("TO_FLAG", &self.TO_FLAG())
                .field("MISCOM_FLAG", &self.MISCOM_FLAG())
                .field("SEQ_FLAG", &self.SEQ_FLAG())
                .field("CNT_FLAG", &self.CNT_FLAG())
                .field("STATE_FLAG", &self.STATE_FLAG())
                .field("ADDR_FLAG", &self.ADDR_FLAG())
                .field("POR_FLAG", &self.POR_FLAG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLAGS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLAGS {{ TO_FLAG: {=bool:?}, MISCOM_FLAG: {=bool:?}, SEQ_FLAG: {=bool:?}, CNT_FLAG: {=bool:?}, STATE_FLAG: {=bool:?}, ADDR_FLAG: {=bool:?}, POR_FLAG: {=bool:?} }}" , self . TO_FLAG () , self . MISCOM_FLAG () , self . SEQ_FLAG () , self . CNT_FLAG () , self . STATE_FLAG () , self . ADDR_FLAG () , self . POR_FLAG ())
        }
    }
    #[doc = "Status 1 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATUS(pub u32);
    impl STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn NUMTOF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NUMTOF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NUMMISCOMPF(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NUMMISCOMPF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NUMILSEQF(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NUMILSEQF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CURST(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CURST(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for STATUS {
        #[inline(always)]
        fn default() -> STATUS {
            STATUS(0)
        }
    }
    impl core::fmt::Debug for STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STATUS")
                .field("NUMTOF", &self.NUMTOF())
                .field("NUMMISCOMPF", &self.NUMMISCOMPF())
                .field("NUMILSEQF", &self.NUMILSEQF())
                .field("CURST", &self.CURST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "STATUS {{ NUMTOF: {=u8:?}, NUMMISCOMPF: {=u8:?}, NUMILSEQF: {=u8:?}, CURST: {=u8:?} }}" , self . NUMTOF () , self . NUMMISCOMPF () , self . NUMILSEQF () , self . CURST ())
        }
    }
    #[doc = "Status 2 Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATUS2(pub u32);
    impl STATUS2 {
        #[must_use]
        #[inline(always)]
        pub const fn NUMCNTF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NUMCNTF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NUMILLSTF(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NUMILLSTF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NUMILLA(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NUMILLA(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for STATUS2 {
        #[inline(always)]
        fn default() -> STATUS2 {
            STATUS2(0)
        }
    }
    impl core::fmt::Debug for STATUS2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STATUS2")
                .field("NUMCNTF", &self.NUMCNTF())
                .field("NUMILLSTF", &self.NUMILLSTF())
                .field("NUMILLA", &self.NUMILLA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATUS2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "STATUS2 {{ NUMCNTF: {=u8:?}, NUMILLSTF: {=u8:?}, NUMILLA: {=u8:?} }}",
                self.NUMCNTF(),
                self.NUMILLSTF(),
                self.NUMILLA()
            )
        }
    }
}
