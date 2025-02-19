#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ITRC {
    ptr: *mut u8,
}
unsafe impl Send for ITRC {}
unsafe impl Sync for ITRC {}
impl ITRC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn STATUS1(self) -> crate::common::Reg<regs::STATUS1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn OUT_SEL(self, n: usize) -> crate::common::Reg<regs::OUT_SEL, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 8usize) as _) }
    }
    #[inline(always)]
    pub const fn OUT_SEL_1(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::OUT_SEL_1, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize + n * 8usize) as _) }
    }
    #[inline(always)]
    pub const fn OUT_SEL_2(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::OUT_SEL_2, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize + n * 8usize) as _) }
    }
    #[inline(always)]
    pub const fn SW_EVENT0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn SW_EVENT1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Trigger Source IN0 to IN15 selector"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUT_SEL(pub u32);
    impl OUT_SEL {
        #[inline(always)]
        pub const fn IN0_SELn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN0_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn IN1_SELn(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN1_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn IN2_SELn(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN2_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn IN3_SELn(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN3_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn IN4_SELn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN4_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn IN5_SELn(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN5_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn IN6_SELn(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN6_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn IN7_SELn(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN7_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn IN8_SELn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN8_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn IN9_SELn(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN9_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[inline(always)]
        pub const fn IN10_SELn(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN10_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[inline(always)]
        pub const fn IN11_SELn(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN11_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[inline(always)]
        pub const fn IN12_SELn(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN12_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn IN13_SELn(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN13_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[inline(always)]
        pub const fn IN14_SELn(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN14_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[inline(always)]
        pub const fn IN15_SELn(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN15_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for OUT_SEL {
        #[inline(always)]
        fn default() -> OUT_SEL {
            OUT_SEL(0)
        }
    }
    impl core::fmt::Debug for OUT_SEL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OUT_SEL")
                .field("IN0_SELn", &self.IN0_SELn())
                .field("IN1_SELn", &self.IN1_SELn())
                .field("IN2_SELn", &self.IN2_SELn())
                .field("IN3_SELn", &self.IN3_SELn())
                .field("IN4_SELn", &self.IN4_SELn())
                .field("IN5_SELn", &self.IN5_SELn())
                .field("IN6_SELn", &self.IN6_SELn())
                .field("IN7_SELn", &self.IN7_SELn())
                .field("IN8_SELn", &self.IN8_SELn())
                .field("IN9_SELn", &self.IN9_SELn())
                .field("IN10_SELn", &self.IN10_SELn())
                .field("IN11_SELn", &self.IN11_SELn())
                .field("IN12_SELn", &self.IN12_SELn())
                .field("IN13_SELn", &self.IN13_SELn())
                .field("IN14_SELn", &self.IN14_SELn())
                .field("IN15_SELn", &self.IN15_SELn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OUT_SEL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "OUT_SEL {{ IN0_SELn: {=u8:?}, IN1_SELn: {=u8:?}, IN2_SELn: {=u8:?}, IN3_SELn: {=u8:?}, IN4_SELn: {=u8:?}, IN5_SELn: {=u8:?}, IN6_SELn: {=u8:?}, IN7_SELn: {=u8:?}, IN8_SELn: {=u8:?}, IN9_SELn: {=u8:?}, IN10_SELn: {=u8:?}, IN11_SELn: {=u8:?}, IN12_SELn: {=u8:?}, IN13_SELn: {=u8:?}, IN14_SELn: {=u8:?}, IN15_SELn: {=u8:?} }}" , self . IN0_SELn () , self . IN1_SELn () , self . IN2_SELn () , self . IN3_SELn () , self . IN4_SELn () , self . IN5_SELn () , self . IN6_SELn () , self . IN7_SELn () , self . IN8_SELn () , self . IN9_SELn () , self . IN10_SELn () , self . IN11_SELn () , self . IN12_SELn () , self . IN13_SELn () , self . IN14_SELn () , self . IN15_SELn ())
        }
    }
    #[doc = "Trigger Source IN16 to IN31 selector"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUT_SEL_1(pub u32);
    impl OUT_SEL_1 {
        #[inline(always)]
        pub const fn IN16_SELn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN16_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn IN17_SELn(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN17_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn IN18_SELn(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN18_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn IN19_SELn(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN19_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn IN20_SELn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN20_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn IN21_SELn(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN21_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn IN22_SELn(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN22_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn IN23_SELn(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN23_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn IN24_SELn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN24_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn IN25_SELn(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN25_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[inline(always)]
        pub const fn IN26_SELn(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN26_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[inline(always)]
        pub const fn IN27_SELn(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN27_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[inline(always)]
        pub const fn IN28_SELn(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN28_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn IN29_SELn(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN29_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[inline(always)]
        pub const fn IN30_SELn(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN30_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[inline(always)]
        pub const fn IN31_SELn(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN31_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for OUT_SEL_1 {
        #[inline(always)]
        fn default() -> OUT_SEL_1 {
            OUT_SEL_1(0)
        }
    }
    impl core::fmt::Debug for OUT_SEL_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OUT_SEL_1")
                .field("IN16_SELn", &self.IN16_SELn())
                .field("IN17_SELn", &self.IN17_SELn())
                .field("IN18_SELn", &self.IN18_SELn())
                .field("IN19_SELn", &self.IN19_SELn())
                .field("IN20_SELn", &self.IN20_SELn())
                .field("IN21_SELn", &self.IN21_SELn())
                .field("IN22_SELn", &self.IN22_SELn())
                .field("IN23_SELn", &self.IN23_SELn())
                .field("IN24_SELn", &self.IN24_SELn())
                .field("IN25_SELn", &self.IN25_SELn())
                .field("IN26_SELn", &self.IN26_SELn())
                .field("IN27_SELn", &self.IN27_SELn())
                .field("IN28_SELn", &self.IN28_SELn())
                .field("IN29_SELn", &self.IN29_SELn())
                .field("IN30_SELn", &self.IN30_SELn())
                .field("IN31_SELn", &self.IN31_SELn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OUT_SEL_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "OUT_SEL_1 {{ IN16_SELn: {=u8:?}, IN17_SELn: {=u8:?}, IN18_SELn: {=u8:?}, IN19_SELn: {=u8:?}, IN20_SELn: {=u8:?}, IN21_SELn: {=u8:?}, IN22_SELn: {=u8:?}, IN23_SELn: {=u8:?}, IN24_SELn: {=u8:?}, IN25_SELn: {=u8:?}, IN26_SELn: {=u8:?}, IN27_SELn: {=u8:?}, IN28_SELn: {=u8:?}, IN29_SELn: {=u8:?}, IN30_SELn: {=u8:?}, IN31_SELn: {=u8:?} }}" , self . IN16_SELn () , self . IN17_SELn () , self . IN18_SELn () , self . IN19_SELn () , self . IN20_SELn () , self . IN21_SELn () , self . IN22_SELn () , self . IN23_SELn () , self . IN24_SELn () , self . IN25_SELn () , self . IN26_SELn () , self . IN27_SELn () , self . IN28_SELn () , self . IN29_SELn () , self . IN30_SELn () , self . IN31_SELn ())
        }
    }
    #[doc = "Trigger source IN32 to IN47 selector"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OUT_SEL_2(pub u32);
    impl OUT_SEL_2 {
        #[inline(always)]
        pub const fn IN32_SELn(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN32_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn IN33_SELn(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN33_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn IN34_SELn(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN34_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn IN35_SELn(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN35_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn IN36_SELn(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN36_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn IN37_SELn(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN37_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn IN46_SELn(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN46_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[inline(always)]
        pub const fn IN47_SELn(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN47_SELn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for OUT_SEL_2 {
        #[inline(always)]
        fn default() -> OUT_SEL_2 {
            OUT_SEL_2(0)
        }
    }
    impl core::fmt::Debug for OUT_SEL_2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OUT_SEL_2")
                .field("IN32_SELn", &self.IN32_SELn())
                .field("IN33_SELn", &self.IN33_SELn())
                .field("IN34_SELn", &self.IN34_SELn())
                .field("IN35_SELn", &self.IN35_SELn())
                .field("IN36_SELn", &self.IN36_SELn())
                .field("IN37_SELn", &self.IN37_SELn())
                .field("IN46_SELn", &self.IN46_SELn())
                .field("IN47_SELn", &self.IN47_SELn())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OUT_SEL_2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "OUT_SEL_2 {{ IN32_SELn: {=u8:?}, IN33_SELn: {=u8:?}, IN34_SELn: {=u8:?}, IN35_SELn: {=u8:?}, IN36_SELn: {=u8:?}, IN37_SELn: {=u8:?}, IN46_SELn: {=u8:?}, IN47_SELn: {=u8:?} }}" , self . IN32_SELn () , self . IN33_SELn () , self . IN34_SELn () , self . IN35_SELn () , self . IN36_SELn () , self . IN37_SELn () , self . IN46_SELn () , self . IN47_SELn ())
        }
    }
    #[doc = "ITRC outputs and IN0 to IN15 Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATUS(pub u32);
    impl STATUS {
        #[inline(always)]
        pub const fn IN0_STATUS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN0_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn IN1_STATUS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN1_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn IN2_STATUS(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN2_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn IN3_STATUS(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN3_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn IN4_STATUS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN4_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn IN5_STATUS(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN5_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn IN6_STATUS(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN6_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn IN7_STATUS(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN7_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn IN8_STATUS(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN8_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn IN9_STATUS(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN9_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn IN10_STATUS(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN10_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn IN11_STATUS(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN11_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn IN112_STATUS(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN112_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn IN113_STATUS(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN113_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn IN14_STATUS(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN14_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn IN15_STATUS(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN15_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn OUT0_STATUS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT0_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn OUT1_STATUS(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT1_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn OUT2_STATUS(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT2_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn OUT3_STATUS(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT3_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn OUT4_STATUS(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT4_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn OUT5_STATUS(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT5_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn OUT6_STATUS(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OUT6_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
                .field("IN0_STATUS", &self.IN0_STATUS())
                .field("IN1_STATUS", &self.IN1_STATUS())
                .field("IN2_STATUS", &self.IN2_STATUS())
                .field("IN3_STATUS", &self.IN3_STATUS())
                .field("IN4_STATUS", &self.IN4_STATUS())
                .field("IN5_STATUS", &self.IN5_STATUS())
                .field("IN6_STATUS", &self.IN6_STATUS())
                .field("IN7_STATUS", &self.IN7_STATUS())
                .field("IN8_STATUS", &self.IN8_STATUS())
                .field("IN9_STATUS", &self.IN9_STATUS())
                .field("IN10_STATUS", &self.IN10_STATUS())
                .field("IN11_STATUS", &self.IN11_STATUS())
                .field("IN112_STATUS", &self.IN112_STATUS())
                .field("IN113_STATUS", &self.IN113_STATUS())
                .field("IN14_STATUS", &self.IN14_STATUS())
                .field("IN15_STATUS", &self.IN15_STATUS())
                .field("OUT0_STATUS", &self.OUT0_STATUS())
                .field("OUT1_STATUS", &self.OUT1_STATUS())
                .field("OUT2_STATUS", &self.OUT2_STATUS())
                .field("OUT3_STATUS", &self.OUT3_STATUS())
                .field("OUT4_STATUS", &self.OUT4_STATUS())
                .field("OUT5_STATUS", &self.OUT5_STATUS())
                .field("OUT6_STATUS", &self.OUT6_STATUS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "STATUS {{ IN0_STATUS: {=bool:?}, IN1_STATUS: {=bool:?}, IN2_STATUS: {=bool:?}, IN3_STATUS: {=bool:?}, IN4_STATUS: {=bool:?}, IN5_STATUS: {=bool:?}, IN6_STATUS: {=bool:?}, IN7_STATUS: {=bool:?}, IN8_STATUS: {=bool:?}, IN9_STATUS: {=bool:?}, IN10_STATUS: {=bool:?}, IN11_STATUS: {=bool:?}, IN112_STATUS: {=bool:?}, IN113_STATUS: {=bool:?}, IN14_STATUS: {=bool:?}, IN15_STATUS: {=bool:?}, OUT0_STATUS: {=bool:?}, OUT1_STATUS: {=bool:?}, OUT2_STATUS: {=bool:?}, OUT3_STATUS: {=bool:?}, OUT4_STATUS: {=bool:?}, OUT5_STATUS: {=bool:?}, OUT6_STATUS: {=bool:?} }}" , self . IN0_STATUS () , self . IN1_STATUS () , self . IN2_STATUS () , self . IN3_STATUS () , self . IN4_STATUS () , self . IN5_STATUS () , self . IN6_STATUS () , self . IN7_STATUS () , self . IN8_STATUS () , self . IN9_STATUS () , self . IN10_STATUS () , self . IN11_STATUS () , self . IN112_STATUS () , self . IN113_STATUS () , self . IN14_STATUS () , self . IN15_STATUS () , self . OUT0_STATUS () , self . OUT1_STATUS () , self . OUT2_STATUS () , self . OUT3_STATUS () , self . OUT4_STATUS () , self . OUT5_STATUS () , self . OUT6_STATUS ())
        }
    }
    #[doc = "ITRC IN16 to IN47 Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STATUS1(pub u32);
    impl STATUS1 {
        #[inline(always)]
        pub const fn IN16_STATUS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN16_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn IN17_STATUS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN17_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn IN18_STATUS(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN18_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn IN19_STATUS(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN19_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn IN20_STATUS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN20_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn IN24_21_STATUS(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN24_21_STATUS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[inline(always)]
        pub const fn IN32_25_STATUS(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_IN32_25_STATUS(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 9usize)) | (((val as u32) & 0xff) << 9usize);
        }
        #[inline(always)]
        pub const fn IN33_STATUS(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN33_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn IN34_STATUS(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN34_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn IN35_STATUS(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN35_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn IN36_STATUS(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN36_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[inline(always)]
        pub const fn IN37_STATUS(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN37_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn IN46_STATUS(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN46_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn IN47_STATUS(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IN47_STATUS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for STATUS1 {
        #[inline(always)]
        fn default() -> STATUS1 {
            STATUS1(0)
        }
    }
    impl core::fmt::Debug for STATUS1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STATUS1")
                .field("IN16_STATUS", &self.IN16_STATUS())
                .field("IN17_STATUS", &self.IN17_STATUS())
                .field("IN18_STATUS", &self.IN18_STATUS())
                .field("IN19_STATUS", &self.IN19_STATUS())
                .field("IN20_STATUS", &self.IN20_STATUS())
                .field("IN24_21_STATUS", &self.IN24_21_STATUS())
                .field("IN32_25_STATUS", &self.IN32_25_STATUS())
                .field("IN33_STATUS", &self.IN33_STATUS())
                .field("IN34_STATUS", &self.IN34_STATUS())
                .field("IN35_STATUS", &self.IN35_STATUS())
                .field("IN36_STATUS", &self.IN36_STATUS())
                .field("IN37_STATUS", &self.IN37_STATUS())
                .field("IN46_STATUS", &self.IN46_STATUS())
                .field("IN47_STATUS", &self.IN47_STATUS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STATUS1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "STATUS1 {{ IN16_STATUS: {=bool:?}, IN17_STATUS: {=bool:?}, IN18_STATUS: {=bool:?}, IN19_STATUS: {=bool:?}, IN20_STATUS: {=bool:?}, IN24_21_STATUS: {=u8:?}, IN32_25_STATUS: {=u8:?}, IN33_STATUS: {=bool:?}, IN34_STATUS: {=bool:?}, IN35_STATUS: {=bool:?}, IN36_STATUS: {=bool:?}, IN37_STATUS: {=bool:?}, IN46_STATUS: {=bool:?}, IN47_STATUS: {=bool:?} }}" , self . IN16_STATUS () , self . IN17_STATUS () , self . IN18_STATUS () , self . IN19_STATUS () , self . IN20_STATUS () , self . IN24_21_STATUS () , self . IN32_25_STATUS () , self . IN33_STATUS () , self . IN34_STATUS () , self . IN35_STATUS () , self . IN36_STATUS () , self . IN37_STATUS () , self . IN46_STATUS () , self . IN47_STATUS ())
        }
    }
}
