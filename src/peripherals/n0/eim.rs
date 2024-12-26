#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EIM {
    ptr: *mut u8,
}
unsafe impl Send for EIM {}
unsafe impl Sync for EIM {}
impl EIM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn EIMCR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHEN(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD0_WORD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD0_WORD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD1_WORD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD1_WORD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD2_WORD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD2_WORD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD3_WORD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD3_WORD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD4_WORD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD4_WORD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD5_WORD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD5_WORD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0244usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD6_WORD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD6_WORD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD7_WORD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c0usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD7_WORD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c4usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD8_WORD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn EICHD8_WORD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
}
pub mod regs {
    #[doc = "Error Injection Channel Descriptor 0, Word0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD0_WORD0(pub u32);
    impl EICHD0_WORD0 {
        #[inline(always)]
        pub const fn CHKBIT_MASK(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CHKBIT_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for EICHD0_WORD0 {
        #[inline(always)]
        fn default() -> EICHD0_WORD0 {
            EICHD0_WORD0(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 0, Word1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD0_WORD1(pub u32);
    impl EICHD0_WORD1 {
        #[inline(always)]
        pub const fn B0_3DATA_MASK(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_B0_3DATA_MASK(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EICHD0_WORD1 {
        #[inline(always)]
        fn default() -> EICHD0_WORD1 {
            EICHD0_WORD1(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 1, Word0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD1_WORD0(pub u32);
    impl EICHD1_WORD0 {
        #[inline(always)]
        pub const fn CHKBIT_MASK(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CHKBIT_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for EICHD1_WORD0 {
        #[inline(always)]
        fn default() -> EICHD1_WORD0 {
            EICHD1_WORD0(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 1, Word1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD1_WORD1(pub u32);
    impl EICHD1_WORD1 {
        #[inline(always)]
        pub const fn B0_3DATA_MASK(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_B0_3DATA_MASK(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EICHD1_WORD1 {
        #[inline(always)]
        fn default() -> EICHD1_WORD1 {
            EICHD1_WORD1(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 2, Word0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD2_WORD0(pub u32);
    impl EICHD2_WORD0 {
        #[inline(always)]
        pub const fn CHKBIT_MASK(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CHKBIT_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for EICHD2_WORD0 {
        #[inline(always)]
        fn default() -> EICHD2_WORD0 {
            EICHD2_WORD0(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 2, Word1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD2_WORD1(pub u32);
    impl EICHD2_WORD1 {
        #[inline(always)]
        pub const fn B0_3DATA_MASK(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_B0_3DATA_MASK(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EICHD2_WORD1 {
        #[inline(always)]
        fn default() -> EICHD2_WORD1 {
            EICHD2_WORD1(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 3, Word0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD3_WORD0(pub u32);
    impl EICHD3_WORD0 {
        #[inline(always)]
        pub const fn CHKBIT_MASK(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CHKBIT_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for EICHD3_WORD0 {
        #[inline(always)]
        fn default() -> EICHD3_WORD0 {
            EICHD3_WORD0(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 3, Word1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD3_WORD1(pub u32);
    impl EICHD3_WORD1 {
        #[inline(always)]
        pub const fn B0_3DATA_MASK(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_B0_3DATA_MASK(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EICHD3_WORD1 {
        #[inline(always)]
        fn default() -> EICHD3_WORD1 {
            EICHD3_WORD1(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 4, Word0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD4_WORD0(pub u32);
    impl EICHD4_WORD0 {
        #[inline(always)]
        pub const fn CHKBIT_MASK(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CHKBIT_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for EICHD4_WORD0 {
        #[inline(always)]
        fn default() -> EICHD4_WORD0 {
            EICHD4_WORD0(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 4, Word1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD4_WORD1(pub u32);
    impl EICHD4_WORD1 {
        #[inline(always)]
        pub const fn B0_3DATA_MASK(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_B0_3DATA_MASK(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EICHD4_WORD1 {
        #[inline(always)]
        fn default() -> EICHD4_WORD1 {
            EICHD4_WORD1(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 5, Word0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD5_WORD0(pub u32);
    impl EICHD5_WORD0 {
        #[inline(always)]
        pub const fn CHKBIT_MASK(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CHKBIT_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for EICHD5_WORD0 {
        #[inline(always)]
        fn default() -> EICHD5_WORD0 {
            EICHD5_WORD0(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 5, Word1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD5_WORD1(pub u32);
    impl EICHD5_WORD1 {
        #[inline(always)]
        pub const fn B0_3DATA_MASK(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_B0_3DATA_MASK(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EICHD5_WORD1 {
        #[inline(always)]
        fn default() -> EICHD5_WORD1 {
            EICHD5_WORD1(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 6, Word0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD6_WORD0(pub u32);
    impl EICHD6_WORD0 {
        #[inline(always)]
        pub const fn CHKBIT_MASK(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CHKBIT_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for EICHD6_WORD0 {
        #[inline(always)]
        fn default() -> EICHD6_WORD0 {
            EICHD6_WORD0(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 6, Word1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD6_WORD1(pub u32);
    impl EICHD6_WORD1 {
        #[inline(always)]
        pub const fn B0_3DATA_MASK(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_B0_3DATA_MASK(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EICHD6_WORD1 {
        #[inline(always)]
        fn default() -> EICHD6_WORD1 {
            EICHD6_WORD1(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 7, Word0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD7_WORD0(pub u32);
    impl EICHD7_WORD0 {
        #[inline(always)]
        pub const fn CHKBIT_MASK(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CHKBIT_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EICHD7_WORD0 {
        #[inline(always)]
        fn default() -> EICHD7_WORD0 {
            EICHD7_WORD0(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 7, Word1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD7_WORD1(pub u32);
    impl EICHD7_WORD1 {
        #[inline(always)]
        pub const fn B0_3DATA_MASK(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_B0_3DATA_MASK(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EICHD7_WORD1 {
        #[inline(always)]
        fn default() -> EICHD7_WORD1 {
            EICHD7_WORD1(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 8, Word0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD8_WORD0(pub u32);
    impl EICHD8_WORD0 {
        #[inline(always)]
        pub const fn CHKBIT_MASK(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CHKBIT_MASK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for EICHD8_WORD0 {
        #[inline(always)]
        fn default() -> EICHD8_WORD0 {
            EICHD8_WORD0(0)
        }
    }
    #[doc = "Error Injection Channel Descriptor 8, Word1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHD8_WORD1(pub u32);
    impl EICHD8_WORD1 {
        #[inline(always)]
        pub const fn B0_3DATA_MASK(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_B0_3DATA_MASK(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EICHD8_WORD1 {
        #[inline(always)]
        fn default() -> EICHD8_WORD1 {
            EICHD8_WORD1(0)
        }
    }
    #[doc = "Error Injection Channel Enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EICHEN(pub u32);
    impl EICHEN {
        #[inline(always)]
        pub const fn EICH8EN(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EICH8EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn EICH7EN(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EICH7EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn EICH6EN(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EICH6EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn EICH5EN(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EICH5EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn EICH4EN(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EICH4EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn EICH3EN(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EICH3EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[inline(always)]
        pub const fn EICH2EN(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EICH2EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn EICH1EN(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EICH1EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn EICH0EN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_EICH0EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EICHEN {
        #[inline(always)]
        fn default() -> EICHEN {
            EICHEN(0)
        }
    }
    #[doc = "Error Injection Module Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EIMCR(pub u32);
    impl EIMCR {
        #[inline(always)]
        pub const fn GEIEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GEIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for EIMCR {
        #[inline(always)]
        fn default() -> EIMCR {
            EIMCR(0)
        }
    }
}
