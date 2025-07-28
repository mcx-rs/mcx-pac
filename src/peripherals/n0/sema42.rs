#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SEMA42 {
    ptr: *mut u8,
}
unsafe impl Send for SEMA42 {}
unsafe impl Sync for SEMA42 {}
impl SEMA42 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn GATE3(self) -> crate::common::Reg<regs::GATE3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn GATE2(self) -> crate::common::Reg<regs::GATE2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[inline(always)]
    pub const fn GATE1(self) -> crate::common::Reg<regs::GATE1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[inline(always)]
    pub const fn GATE0(self) -> crate::common::Reg<regs::GATE0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[inline(always)]
    pub const fn GATE7(self) -> crate::common::Reg<regs::GATE7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn GATE6(self) -> crate::common::Reg<regs::GATE6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[inline(always)]
    pub const fn GATE5(self) -> crate::common::Reg<regs::GATE5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[inline(always)]
    pub const fn GATE4(self) -> crate::common::Reg<regs::GATE4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[inline(always)]
    pub const fn GATE11(self) -> crate::common::Reg<regs::GATE11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn GATE10(self) -> crate::common::Reg<regs::GATE10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
    }
    #[inline(always)]
    pub const fn GATE9(self) -> crate::common::Reg<regs::GATE9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[inline(always)]
    pub const fn GATE8(self) -> crate::common::Reg<regs::GATE8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0busize) as _) }
    }
    #[inline(always)]
    pub const fn GATE15(self) -> crate::common::Reg<regs::GATE15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn GATE14(self) -> crate::common::Reg<regs::GATE14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0dusize) as _) }
    }
    #[inline(always)]
    pub const fn GATE13(self) -> crate::common::Reg<regs::GATE13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[inline(always)]
    pub const fn GATE12(self) -> crate::common::Reg<regs::GATE12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fusize) as _) }
    }
    #[inline(always)]
    pub const fn RSTGT_R(self) -> crate::common::Reg<regs::RSTGT_R, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x42usize) as _) }
    }
    #[inline(always)]
    pub const fn RSTGT_W(self) -> crate::common::Reg<regs::RSTGT_W, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x42usize) as _) }
    }
}
pub mod regs {
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE0(pub u8);
    impl GATE0 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE0 {
        #[inline(always)]
        fn default() -> GATE0 {
            GATE0(0)
        }
    }
    impl core::fmt::Debug for GATE0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE0")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE0 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE1(pub u8);
    impl GATE1 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE1 {
        #[inline(always)]
        fn default() -> GATE1 {
            GATE1(0)
        }
    }
    impl core::fmt::Debug for GATE1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE1")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE1 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE10(pub u8);
    impl GATE10 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE10 {
        #[inline(always)]
        fn default() -> GATE10 {
            GATE10(0)
        }
    }
    impl core::fmt::Debug for GATE10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE10")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE10 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE11(pub u8);
    impl GATE11 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE11 {
        #[inline(always)]
        fn default() -> GATE11 {
            GATE11(0)
        }
    }
    impl core::fmt::Debug for GATE11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE11")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE11 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE12(pub u8);
    impl GATE12 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE12 {
        #[inline(always)]
        fn default() -> GATE12 {
            GATE12(0)
        }
    }
    impl core::fmt::Debug for GATE12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE12")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE12 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE13(pub u8);
    impl GATE13 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE13 {
        #[inline(always)]
        fn default() -> GATE13 {
            GATE13(0)
        }
    }
    impl core::fmt::Debug for GATE13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE13")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE13 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE14(pub u8);
    impl GATE14 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE14 {
        #[inline(always)]
        fn default() -> GATE14 {
            GATE14(0)
        }
    }
    impl core::fmt::Debug for GATE14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE14")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE14 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE15(pub u8);
    impl GATE15 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE15 {
        #[inline(always)]
        fn default() -> GATE15 {
            GATE15(0)
        }
    }
    impl core::fmt::Debug for GATE15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE15")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE15 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE2(pub u8);
    impl GATE2 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE2 {
        #[inline(always)]
        fn default() -> GATE2 {
            GATE2(0)
        }
    }
    impl core::fmt::Debug for GATE2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE2")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE2 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE3(pub u8);
    impl GATE3 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE3 {
        #[inline(always)]
        fn default() -> GATE3 {
            GATE3(0)
        }
    }
    impl core::fmt::Debug for GATE3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE3")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE3 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE4(pub u8);
    impl GATE4 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE4 {
        #[inline(always)]
        fn default() -> GATE4 {
            GATE4(0)
        }
    }
    impl core::fmt::Debug for GATE4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE4")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE4 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE5(pub u8);
    impl GATE5 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE5 {
        #[inline(always)]
        fn default() -> GATE5 {
            GATE5(0)
        }
    }
    impl core::fmt::Debug for GATE5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE5")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE5 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE6(pub u8);
    impl GATE6 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE6 {
        #[inline(always)]
        fn default() -> GATE6 {
            GATE6(0)
        }
    }
    impl core::fmt::Debug for GATE6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE6")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE6 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE7(pub u8);
    impl GATE7 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE7 {
        #[inline(always)]
        fn default() -> GATE7 {
            GATE7(0)
        }
    }
    impl core::fmt::Debug for GATE7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE7")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE7 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE8(pub u8);
    impl GATE8 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE8 {
        #[inline(always)]
        fn default() -> GATE8 {
            GATE8(0)
        }
    }
    impl core::fmt::Debug for GATE8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE8")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE8 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Gate"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GATE9(pub u8);
    impl GATE9 {
        #[must_use]
        #[inline(always)]
        pub const fn GTFSM(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_GTFSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for GATE9 {
        #[inline(always)]
        fn default() -> GATE9 {
            GATE9(0)
        }
    }
    impl core::fmt::Debug for GATE9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GATE9")
                .field("GTFSM", &self.GTFSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GATE9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "GATE9 {{ GTFSM: {=u8:?} }}", self.GTFSM())
        }
    }
    #[doc = "Reset Gate Read"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RSTGT_R(pub u16);
    impl RSTGT_R {
        #[must_use]
        #[inline(always)]
        pub const fn RSTGTN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RSTGTN(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSTGMS(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RSTGMS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSTGSM(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RSTGSM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u16) & 0x03) << 12usize);
        }
    }
    impl Default for RSTGT_R {
        #[inline(always)]
        fn default() -> RSTGT_R {
            RSTGT_R(0)
        }
    }
    impl core::fmt::Debug for RSTGT_R {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RSTGT_R")
                .field("RSTGTN", &self.RSTGTN())
                .field("RSTGMS", &self.RSTGMS())
                .field("RSTGSM", &self.RSTGSM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RSTGT_R {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RSTGT_R {{ RSTGTN: {=u8:?}, RSTGMS: {=u8:?}, RSTGSM: {=u8:?} }}",
                self.RSTGTN(),
                self.RSTGMS(),
                self.RSTGSM()
            )
        }
    }
    #[doc = "Reset Gate Write"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RSTGT_W(pub u16);
    impl RSTGT_W {
        #[must_use]
        #[inline(always)]
        pub const fn RSTGTN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RSTGTN(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSTGDP(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RSTGDP(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for RSTGT_W {
        #[inline(always)]
        fn default() -> RSTGT_W {
            RSTGT_W(0)
        }
    }
    impl core::fmt::Debug for RSTGT_W {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RSTGT_W")
                .field("RSTGTN", &self.RSTGTN())
                .field("RSTGDP", &self.RSTGDP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RSTGT_W {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "RSTGT_W {{ RSTGTN: {=u8:?}, RSTGDP: {=u8:?} }}",
                self.RSTGTN(),
                self.RSTGDP()
            )
        }
    }
}
