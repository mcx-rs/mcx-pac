#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PUF {
    ptr: *mut u8,
}
unsafe impl Send for PUF {}
unsafe impl Sync for PUF {}
impl PUF {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn CR(self) -> crate::common::Reg<regs::CR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ORR(self) -> crate::common::Reg<regs::ORR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn SR(self) -> crate::common::Reg<regs::SR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn AR(self) -> crate::common::Reg<regs::AR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn IER(self) -> crate::common::Reg<regs::IER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn IMR(self) -> crate::common::Reg<regs::IMR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn ISR(self) -> crate::common::Reg<regs::ISR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn DATA_DEST(self) -> crate::common::Reg<regs::DATA_DEST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn DATA_SRC(self) -> crate::common::Reg<regs::DATA_SRC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn DIR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn DOR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn MISC(self) -> crate::common::Reg<regs::MISC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn IF_SR(self) -> crate::common::Reg<regs::IF_SR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn PSR(self) -> crate::common::Reg<regs::PSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn HW_RUC0(self) -> crate::common::Reg<regs::HW_RUC0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn HW_RUC1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn HW_INFO(self) -> crate::common::Reg<regs::HW_INFO, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn HW_ID(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[inline(always)]
    pub const fn HW_VER(self) -> crate::common::Reg<regs::HW_VER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[inline(always)]
    pub const fn CONFIG(self) -> crate::common::Reg<regs::CONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn SEC_LOCK(self) -> crate::common::Reg<regs::SEC_LOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn APP_CTX_MASK(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAM_CFG(self) -> crate::common::Reg<regs::SRAM_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAM_STATUS(self) -> crate::common::Reg<regs::SRAM_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAM_INT_CLR_ENABLE(
        self,
    ) -> crate::common::Reg<regs::SRAM_INT_CLR_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d8usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAM_INT_SET_ENABLE(
        self,
    ) -> crate::common::Reg<regs::SRAM_INT_SET_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03dcusize) as _) }
    }
    #[inline(always)]
    pub const fn SRAM_INT_STATUS(
        self,
    ) -> crate::common::Reg<regs::SRAM_INT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e0usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAM_INT_ENABLE(
        self,
    ) -> crate::common::Reg<regs::SRAM_INT_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e4usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAM_INT_CLR_STATUS(
        self,
    ) -> crate::common::Reg<regs::SRAM_INT_CLR_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e8usize) as _) }
    }
    #[inline(always)]
    pub const fn SRAM_INT_SET_STATUS(
        self,
    ) -> crate::common::Reg<regs::SRAM_INT_SET_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03ecusize) as _) }
    }
}
pub mod regs {
    #[doc = "Allow"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AR(pub u32);
    impl AR {
        #[inline(always)]
        pub const fn ALLOW_ENROLL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALLOW_ENROLL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ALLOW_START(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALLOW_START(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn ALLOW_RECONSTRUCT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALLOW_RECONSTRUCT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn ALLOW_STOP(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALLOW_STOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn ALLOW_GET_KEY(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALLOW_GET_KEY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ALLOW_UNWRAP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALLOW_UNWRAP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn ALLOW_WRAP_GENERATED_RANDOM(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALLOW_WRAP_GENERATED_RANDOM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn ALLOW_WRAP(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALLOW_WRAP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn ALLOW_GENERATE_RANDOM(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALLOW_GENERATE_RANDOM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn ALLOW_TEST_MEMORY(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALLOW_TEST_MEMORY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn ALLOW_TEST_PUF(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ALLOW_TEST_PUF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for AR {
        #[inline(always)]
        fn default() -> AR {
            AR(0)
        }
    }
    impl core::fmt::Debug for AR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AR")
                .field("ALLOW_ENROLL", &self.ALLOW_ENROLL())
                .field("ALLOW_START", &self.ALLOW_START())
                .field("ALLOW_RECONSTRUCT", &self.ALLOW_RECONSTRUCT())
                .field("ALLOW_STOP", &self.ALLOW_STOP())
                .field("ALLOW_GET_KEY", &self.ALLOW_GET_KEY())
                .field("ALLOW_UNWRAP", &self.ALLOW_UNWRAP())
                .field(
                    "ALLOW_WRAP_GENERATED_RANDOM",
                    &self.ALLOW_WRAP_GENERATED_RANDOM(),
                )
                .field("ALLOW_WRAP", &self.ALLOW_WRAP())
                .field("ALLOW_GENERATE_RANDOM", &self.ALLOW_GENERATE_RANDOM())
                .field("ALLOW_TEST_MEMORY", &self.ALLOW_TEST_MEMORY())
                .field("ALLOW_TEST_PUF", &self.ALLOW_TEST_PUF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct AR {
                ALLOW_ENROLL: bool,
                ALLOW_START: bool,
                ALLOW_RECONSTRUCT: bool,
                ALLOW_STOP: bool,
                ALLOW_GET_KEY: bool,
                ALLOW_UNWRAP: bool,
                ALLOW_WRAP_GENERATED_RANDOM: bool,
                ALLOW_WRAP: bool,
                ALLOW_GENERATE_RANDOM: bool,
                ALLOW_TEST_MEMORY: bool,
                ALLOW_TEST_PUF: bool,
            }
            let proxy = AR {
                ALLOW_ENROLL: self.ALLOW_ENROLL(),
                ALLOW_START: self.ALLOW_START(),
                ALLOW_RECONSTRUCT: self.ALLOW_RECONSTRUCT(),
                ALLOW_STOP: self.ALLOW_STOP(),
                ALLOW_GET_KEY: self.ALLOW_GET_KEY(),
                ALLOW_UNWRAP: self.ALLOW_UNWRAP(),
                ALLOW_WRAP_GENERATED_RANDOM: self.ALLOW_WRAP_GENERATED_RANDOM(),
                ALLOW_WRAP: self.ALLOW_WRAP(),
                ALLOW_GENERATE_RANDOM: self.ALLOW_GENERATE_RANDOM(),
                ALLOW_TEST_MEMORY: self.ALLOW_TEST_MEMORY(),
                ALLOW_TEST_PUF: self.ALLOW_TEST_PUF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PUF command blocking configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CONFIG(pub u32);
    impl CONFIG {
        #[inline(always)]
        pub const fn DIS_PUF_ENROLL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_PUF_ENROLL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn DIS_PUF_START(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_PUF_START(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn DIS_PUF_STOP(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_PUF_STOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn DIS_PUF_GET_KEY(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_PUF_GET_KEY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn DIS_PUF_UNWRAP_KEY(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_PUF_UNWRAP_KEY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn DIS_PUF_GEN_WRAP_KEY(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_PUF_GEN_WRAP_KEY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn DIS_PUF_WRAP_KEY(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_PUF_WRAP_KEY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn DIS_PUF_GEN_RANDOM_NUMBER(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_PUF_GEN_RANDOM_NUMBER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn DIS_PUF_TEST(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DIS_PUF_TEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CONFIG {
        #[inline(always)]
        fn default() -> CONFIG {
            CONFIG(0)
        }
    }
    impl core::fmt::Debug for CONFIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CONFIG")
                .field("DIS_PUF_ENROLL", &self.DIS_PUF_ENROLL())
                .field("DIS_PUF_START", &self.DIS_PUF_START())
                .field("DIS_PUF_STOP", &self.DIS_PUF_STOP())
                .field("DIS_PUF_GET_KEY", &self.DIS_PUF_GET_KEY())
                .field("DIS_PUF_UNWRAP_KEY", &self.DIS_PUF_UNWRAP_KEY())
                .field("DIS_PUF_GEN_WRAP_KEY", &self.DIS_PUF_GEN_WRAP_KEY())
                .field("DIS_PUF_WRAP_KEY", &self.DIS_PUF_WRAP_KEY())
                .field(
                    "DIS_PUF_GEN_RANDOM_NUMBER",
                    &self.DIS_PUF_GEN_RANDOM_NUMBER(),
                )
                .field("DIS_PUF_TEST", &self.DIS_PUF_TEST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CONFIG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CONFIG {
                DIS_PUF_ENROLL: bool,
                DIS_PUF_START: bool,
                DIS_PUF_STOP: bool,
                DIS_PUF_GET_KEY: bool,
                DIS_PUF_UNWRAP_KEY: bool,
                DIS_PUF_GEN_WRAP_KEY: bool,
                DIS_PUF_WRAP_KEY: bool,
                DIS_PUF_GEN_RANDOM_NUMBER: bool,
                DIS_PUF_TEST: bool,
            }
            let proxy = CONFIG {
                DIS_PUF_ENROLL: self.DIS_PUF_ENROLL(),
                DIS_PUF_START: self.DIS_PUF_START(),
                DIS_PUF_STOP: self.DIS_PUF_STOP(),
                DIS_PUF_GET_KEY: self.DIS_PUF_GET_KEY(),
                DIS_PUF_UNWRAP_KEY: self.DIS_PUF_UNWRAP_KEY(),
                DIS_PUF_GEN_WRAP_KEY: self.DIS_PUF_GEN_WRAP_KEY(),
                DIS_PUF_WRAP_KEY: self.DIS_PUF_WRAP_KEY(),
                DIS_PUF_GEN_RANDOM_NUMBER: self.DIS_PUF_GEN_RANDOM_NUMBER(),
                DIS_PUF_TEST: self.DIS_PUF_TEST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CR(pub u32);
    impl CR {
        #[inline(always)]
        pub const fn ZEROIZE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZEROIZE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ENROLL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENROLL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn START(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_START(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn RECONSTRUCT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RECONSTRUCT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn STOP(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_STOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn GET_KEY(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GET_KEY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn UNWRAP(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNWRAP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn WRAP_GENERATED_RANDOM(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WRAP_GENERATED_RANDOM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn WRAP(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WRAP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn GENERATE_RANDOM(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GENERATE_RANDOM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn TEST_MEMORY(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TEST_MEMORY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn TEST_PUF(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TEST_PUF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CR {
        #[inline(always)]
        fn default() -> CR {
            CR(0)
        }
    }
    impl core::fmt::Debug for CR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CR")
                .field("ZEROIZE", &self.ZEROIZE())
                .field("ENROLL", &self.ENROLL())
                .field("START", &self.START())
                .field("RECONSTRUCT", &self.RECONSTRUCT())
                .field("STOP", &self.STOP())
                .field("GET_KEY", &self.GET_KEY())
                .field("UNWRAP", &self.UNWRAP())
                .field("WRAP_GENERATED_RANDOM", &self.WRAP_GENERATED_RANDOM())
                .field("WRAP", &self.WRAP())
                .field("GENERATE_RANDOM", &self.GENERATE_RANDOM())
                .field("TEST_MEMORY", &self.TEST_MEMORY())
                .field("TEST_PUF", &self.TEST_PUF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct CR {
                ZEROIZE: bool,
                ENROLL: bool,
                START: bool,
                RECONSTRUCT: bool,
                STOP: bool,
                GET_KEY: bool,
                UNWRAP: bool,
                WRAP_GENERATED_RANDOM: bool,
                WRAP: bool,
                GENERATE_RANDOM: bool,
                TEST_MEMORY: bool,
                TEST_PUF: bool,
            }
            let proxy = CR {
                ZEROIZE: self.ZEROIZE(),
                ENROLL: self.ENROLL(),
                START: self.START(),
                RECONSTRUCT: self.RECONSTRUCT(),
                STOP: self.STOP(),
                GET_KEY: self.GET_KEY(),
                UNWRAP: self.UNWRAP(),
                WRAP_GENERATED_RANDOM: self.WRAP_GENERATED_RANDOM(),
                WRAP: self.WRAP(),
                GENERATE_RANDOM: self.GENERATE_RANDOM(),
                TEST_MEMORY: self.TEST_MEMORY(),
                TEST_PUF: self.TEST_PUF(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Data Destination"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DATA_DEST(pub u32);
    impl DATA_DEST {
        #[inline(always)]
        pub const fn DEST_DOR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEST_DOR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DEST_SO(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DEST_SO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for DATA_DEST {
        #[inline(always)]
        fn default() -> DATA_DEST {
            DATA_DEST(0)
        }
    }
    impl core::fmt::Debug for DATA_DEST {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DATA_DEST")
                .field("DEST_DOR", &self.DEST_DOR())
                .field("DEST_SO", &self.DEST_SO())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DATA_DEST {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DATA_DEST {
                DEST_DOR: bool,
                DEST_SO: bool,
            }
            let proxy = DATA_DEST {
                DEST_DOR: self.DEST_DOR(),
                DEST_SO: self.DEST_SO(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Data Source"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DATA_SRC(pub u32);
    impl DATA_SRC {
        #[inline(always)]
        pub const fn SRC_DIR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRC_DIR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SRC_SI(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRC_SI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for DATA_SRC {
        #[inline(always)]
        fn default() -> DATA_SRC {
            DATA_SRC(0)
        }
    }
    impl core::fmt::Debug for DATA_SRC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DATA_SRC")
                .field("SRC_DIR", &self.SRC_DIR())
                .field("SRC_SI", &self.SRC_SI())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DATA_SRC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct DATA_SRC {
                SRC_DIR: bool,
                SRC_SI: bool,
            }
            let proxy = DATA_SRC {
                SRC_DIR: self.SRC_DIR(),
                SRC_SI: self.SRC_SI(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Hardware Information"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HW_INFO(pub u32);
    impl HW_INFO {
        #[inline(always)]
        pub const fn CONFIG_WRAP(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CONFIG_WRAP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn CONFIG_TYPE(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CONFIG_TYPE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for HW_INFO {
        #[inline(always)]
        fn default() -> HW_INFO {
            HW_INFO(0)
        }
    }
    impl core::fmt::Debug for HW_INFO {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HW_INFO")
                .field("CONFIG_WRAP", &self.CONFIG_WRAP())
                .field("CONFIG_TYPE", &self.CONFIG_TYPE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HW_INFO {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HW_INFO {
                CONFIG_WRAP: bool,
                CONFIG_TYPE: u8,
            }
            let proxy = HW_INFO {
                CONFIG_WRAP: self.CONFIG_WRAP(),
                CONFIG_TYPE: self.CONFIG_TYPE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Hardware Restrict User Context 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HW_RUC0(pub u32);
    impl HW_RUC0 {
        #[inline(always)]
        pub const fn LC_STATE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_LC_STATE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn BOOT_STATE(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_BOOT_STATE(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
        }
        #[inline(always)]
        pub const fn CPU0_DEBUG(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CPU0_DEBUG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn COOLFLUX_DEBUG(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COOLFLUX_DEBUG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn dsp_debug(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_dsp_debug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn ACCESS_LEVEL(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ACCESS_LEVEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for HW_RUC0 {
        #[inline(always)]
        fn default() -> HW_RUC0 {
            HW_RUC0(0)
        }
    }
    impl core::fmt::Debug for HW_RUC0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HW_RUC0")
                .field("LC_STATE", &self.LC_STATE())
                .field("BOOT_STATE", &self.BOOT_STATE())
                .field("CPU0_DEBUG", &self.CPU0_DEBUG())
                .field("COOLFLUX_DEBUG", &self.COOLFLUX_DEBUG())
                .field("dsp_debug", &self.dsp_debug())
                .field("ACCESS_LEVEL", &self.ACCESS_LEVEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HW_RUC0 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HW_RUC0 {
                LC_STATE: u8,
                BOOT_STATE: u16,
                CPU0_DEBUG: bool,
                COOLFLUX_DEBUG: bool,
                dsp_debug: bool,
                ACCESS_LEVEL: u8,
            }
            let proxy = HW_RUC0 {
                LC_STATE: self.LC_STATE(),
                BOOT_STATE: self.BOOT_STATE(),
                CPU0_DEBUG: self.CPU0_DEBUG(),
                COOLFLUX_DEBUG: self.COOLFLUX_DEBUG(),
                dsp_debug: self.dsp_debug(),
                ACCESS_LEVEL: self.ACCESS_LEVEL(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Hardware Version"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HW_VER(pub u32);
    impl HW_VER {
        #[inline(always)]
        pub const fn HW_REV(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_HW_REV(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn HW_VERSION_MINOR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_HW_VERSION_MINOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn HW_VERSION_MAJOR(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_HW_VERSION_MAJOR(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for HW_VER {
        #[inline(always)]
        fn default() -> HW_VER {
            HW_VER(0)
        }
    }
    impl core::fmt::Debug for HW_VER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HW_VER")
                .field("HW_REV", &self.HW_REV())
                .field("HW_VERSION_MINOR", &self.HW_VERSION_MINOR())
                .field("HW_VERSION_MAJOR", &self.HW_VERSION_MAJOR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HW_VER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct HW_VER {
                HW_REV: u8,
                HW_VERSION_MINOR: u8,
                HW_VERSION_MAJOR: u8,
            }
            let proxy = HW_VER {
                HW_REV: self.HW_REV(),
                HW_VERSION_MINOR: self.HW_VERSION_MINOR(),
                HW_VERSION_MAJOR: self.HW_VERSION_MAJOR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IER(pub u32);
    impl IER {
        #[inline(always)]
        pub const fn INT_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IER {
        #[inline(always)]
        fn default() -> IER {
            IER(0)
        }
    }
    impl core::fmt::Debug for IER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IER")
                .field("INT_EN", &self.INT_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IER {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IER {
                INT_EN: bool,
            }
            let proxy = IER {
                INT_EN: self.INT_EN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interface Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IF_SR(pub u32);
    impl IF_SR {
        #[inline(always)]
        pub const fn APB_ERROR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APB_ERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IF_SR {
        #[inline(always)]
        fn default() -> IF_SR {
            IF_SR(0)
        }
    }
    impl core::fmt::Debug for IF_SR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IF_SR")
                .field("APB_ERROR", &self.APB_ERROR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IF_SR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IF_SR {
                APB_ERROR: bool,
            }
            let proxy = IF_SR {
                APB_ERROR: self.APB_ERROR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Mask"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IMR(pub u32);
    impl IMR {
        #[inline(always)]
        pub const fn INT_EN_BUSY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_EN_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn INT_EN_OK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_EN_OK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn INT_EN_ERROR(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_EN_ERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn INT_EN_ZEROIZED(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_EN_ZEROIZED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn INT_EN_REJECTED(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_EN_REJECTED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn INT_EN_DI_REQUEST(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_EN_DI_REQUEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn INT_EN_DO_REQUEST(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_EN_DO_REQUEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for IMR {
        #[inline(always)]
        fn default() -> IMR {
            IMR(0)
        }
    }
    impl core::fmt::Debug for IMR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IMR")
                .field("INT_EN_BUSY", &self.INT_EN_BUSY())
                .field("INT_EN_OK", &self.INT_EN_OK())
                .field("INT_EN_ERROR", &self.INT_EN_ERROR())
                .field("INT_EN_ZEROIZED", &self.INT_EN_ZEROIZED())
                .field("INT_EN_REJECTED", &self.INT_EN_REJECTED())
                .field("INT_EN_DI_REQUEST", &self.INT_EN_DI_REQUEST())
                .field("INT_EN_DO_REQUEST", &self.INT_EN_DO_REQUEST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IMR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct IMR {
                INT_EN_BUSY: bool,
                INT_EN_OK: bool,
                INT_EN_ERROR: bool,
                INT_EN_ZEROIZED: bool,
                INT_EN_REJECTED: bool,
                INT_EN_DI_REQUEST: bool,
                INT_EN_DO_REQUEST: bool,
            }
            let proxy = IMR {
                INT_EN_BUSY: self.INT_EN_BUSY(),
                INT_EN_OK: self.INT_EN_OK(),
                INT_EN_ERROR: self.INT_EN_ERROR(),
                INT_EN_ZEROIZED: self.INT_EN_ZEROIZED(),
                INT_EN_REJECTED: self.INT_EN_REJECTED(),
                INT_EN_DI_REQUEST: self.INT_EN_DI_REQUEST(),
                INT_EN_DO_REQUEST: self.INT_EN_DO_REQUEST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ISR(pub u32);
    impl ISR {
        #[inline(always)]
        pub const fn INT_BUSY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn INT_OK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_OK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn INT_ERROR(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_ERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn INT_ZEROIZED(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_ZEROIZED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn INT_REJECTED(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_REJECTED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn INT_DI_REQUEST(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_DI_REQUEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn INT_DO_REQUEST(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_INT_DO_REQUEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for ISR {
        #[inline(always)]
        fn default() -> ISR {
            ISR(0)
        }
    }
    impl core::fmt::Debug for ISR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ISR")
                .field("INT_BUSY", &self.INT_BUSY())
                .field("INT_OK", &self.INT_OK())
                .field("INT_ERROR", &self.INT_ERROR())
                .field("INT_ZEROIZED", &self.INT_ZEROIZED())
                .field("INT_REJECTED", &self.INT_REJECTED())
                .field("INT_DI_REQUEST", &self.INT_DI_REQUEST())
                .field("INT_DO_REQUEST", &self.INT_DO_REQUEST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ISR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ISR {
                INT_BUSY: bool,
                INT_OK: bool,
                INT_ERROR: bool,
                INT_ZEROIZED: bool,
                INT_REJECTED: bool,
                INT_DI_REQUEST: bool,
                INT_DO_REQUEST: bool,
            }
            let proxy = ISR {
                INT_BUSY: self.INT_BUSY(),
                INT_OK: self.INT_OK(),
                INT_ERROR: self.INT_ERROR(),
                INT_ZEROIZED: self.INT_ZEROIZED(),
                INT_REJECTED: self.INT_REJECTED(),
                INT_DI_REQUEST: self.INT_DI_REQUEST(),
                INT_DO_REQUEST: self.INT_DO_REQUEST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Miscellaneous"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MISC(pub u32);
    impl MISC {
        #[inline(always)]
        pub const fn DATA_ENDIANNESS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DATA_ENDIANNESS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for MISC {
        #[inline(always)]
        fn default() -> MISC {
            MISC(0)
        }
    }
    impl core::fmt::Debug for MISC {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MISC")
                .field("DATA_ENDIANNESS", &self.DATA_ENDIANNESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MISC {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct MISC {
                DATA_ENDIANNESS: bool,
            }
            let proxy = MISC {
                DATA_ENDIANNESS: self.DATA_ENDIANNESS(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Operation Result"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ORR(pub u32);
    impl ORR {
        #[inline(always)]
        pub const fn RESULT_CODE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_RESULT_CODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn LAST_OPERATION(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_LAST_OPERATION(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for ORR {
        #[inline(always)]
        fn default() -> ORR {
            ORR(0)
        }
    }
    impl core::fmt::Debug for ORR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ORR")
                .field("RESULT_CODE", &self.RESULT_CODE())
                .field("LAST_OPERATION", &self.LAST_OPERATION())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ORR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct ORR {
                RESULT_CODE: u8,
                LAST_OPERATION: u8,
            }
            let proxy = ORR {
                RESULT_CODE: self.RESULT_CODE(),
                LAST_OPERATION: self.LAST_OPERATION(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PUF Score"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PSR(pub u32);
    impl PSR {
        #[inline(always)]
        pub const fn PUF_SCORE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_PUF_SCORE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for PSR {
        #[inline(always)]
        fn default() -> PSR {
            PSR(0)
        }
    }
    impl core::fmt::Debug for PSR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PSR")
                .field("PUF_SCORE", &self.PUF_SCORE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PSR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct PSR {
                PUF_SCORE: u8,
            }
            let proxy = PSR {
                PUF_SCORE: self.PUF_SCORE(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Security level lock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SEC_LOCK(pub u32);
    impl SEC_LOCK {
        #[inline(always)]
        pub const fn SEC_LEVEL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_SEC_LEVEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn ANTI_POLE_SEC_LEVEL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ANTI_POLE_SEC_LEVEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn PATTERN(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_PATTERN(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
    }
    impl Default for SEC_LOCK {
        #[inline(always)]
        fn default() -> SEC_LOCK {
            SEC_LOCK(0)
        }
    }
    impl core::fmt::Debug for SEC_LOCK {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SEC_LOCK")
                .field("SEC_LEVEL", &self.SEC_LEVEL())
                .field("ANTI_POLE_SEC_LEVEL", &self.ANTI_POLE_SEC_LEVEL())
                .field("PATTERN", &self.PATTERN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SEC_LOCK {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SEC_LOCK {
                SEC_LEVEL: u8,
                ANTI_POLE_SEC_LEVEL: u8,
                PATTERN: u16,
            }
            let proxy = SEC_LOCK {
                SEC_LEVEL: self.SEC_LEVEL(),
                ANTI_POLE_SEC_LEVEL: self.ANTI_POLE_SEC_LEVEL(),
                PATTERN: self.PATTERN(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SR(pub u32);
    impl SR {
        #[inline(always)]
        pub const fn BUSY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn OK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ERROR(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn ZEROIZED(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ZEROIZED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn REJECTED(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REJECTED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn DI_REQUEST(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DI_REQUEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn DO_REQUEST(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DO_REQUEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for SR {
        #[inline(always)]
        fn default() -> SR {
            SR(0)
        }
    }
    impl core::fmt::Debug for SR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SR")
                .field("BUSY", &self.BUSY())
                .field("OK", &self.OK())
                .field("ERROR", &self.ERROR())
                .field("ZEROIZED", &self.ZEROIZED())
                .field("REJECTED", &self.REJECTED())
                .field("DI_REQUEST", &self.DI_REQUEST())
                .field("DO_REQUEST", &self.DO_REQUEST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SR {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SR {
                BUSY: bool,
                OK: bool,
                ERROR: bool,
                ZEROIZED: bool,
                REJECTED: bool,
                DI_REQUEST: bool,
                DO_REQUEST: bool,
            }
            let proxy = SR {
                BUSY: self.BUSY(),
                OK: self.OK(),
                ERROR: self.ERROR(),
                ZEROIZED: self.ZEROIZED(),
                REJECTED: self.REJECTED(),
                DI_REQUEST: self.DI_REQUEST(),
                DO_REQUEST: self.DO_REQUEST(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "SRAM Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAM_CFG(pub u32);
    impl SRAM_CFG {
        #[inline(always)]
        pub const fn ENABLE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ENABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CKGATING(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CKGATING(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for SRAM_CFG {
        #[inline(always)]
        fn default() -> SRAM_CFG {
            SRAM_CFG(0)
        }
    }
    impl core::fmt::Debug for SRAM_CFG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAM_CFG")
                .field("ENABLE", &self.ENABLE())
                .field("CKGATING", &self.CKGATING())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAM_CFG {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAM_CFG {
                ENABLE: bool,
                CKGATING: bool,
            }
            let proxy = SRAM_CFG {
                ENABLE: self.ENABLE(),
                CKGATING: self.CKGATING(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Enable Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAM_INT_CLR_ENABLE(pub u32);
    impl SRAM_INT_CLR_ENABLE {
        #[inline(always)]
        pub const fn READY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_READY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn APB_ERR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APB_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SRAM_INT_CLR_ENABLE {
        #[inline(always)]
        fn default() -> SRAM_INT_CLR_ENABLE {
            SRAM_INT_CLR_ENABLE(0)
        }
    }
    impl core::fmt::Debug for SRAM_INT_CLR_ENABLE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAM_INT_CLR_ENABLE")
                .field("READY", &self.READY())
                .field("APB_ERR", &self.APB_ERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAM_INT_CLR_ENABLE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAM_INT_CLR_ENABLE {
                READY: bool,
                APB_ERR: bool,
            }
            let proxy = SRAM_INT_CLR_ENABLE {
                READY: self.READY(),
                APB_ERR: self.APB_ERR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Status Clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAM_INT_CLR_STATUS(pub u32);
    impl SRAM_INT_CLR_STATUS {
        #[inline(always)]
        pub const fn READY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_READY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn APB_ERR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APB_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SRAM_INT_CLR_STATUS {
        #[inline(always)]
        fn default() -> SRAM_INT_CLR_STATUS {
            SRAM_INT_CLR_STATUS(0)
        }
    }
    impl core::fmt::Debug for SRAM_INT_CLR_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAM_INT_CLR_STATUS")
                .field("READY", &self.READY())
                .field("APB_ERR", &self.APB_ERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAM_INT_CLR_STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAM_INT_CLR_STATUS {
                READY: bool,
                APB_ERR: bool,
            }
            let proxy = SRAM_INT_CLR_STATUS {
                READY: self.READY(),
                APB_ERR: self.APB_ERR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAM_INT_ENABLE(pub u32);
    impl SRAM_INT_ENABLE {
        #[inline(always)]
        pub const fn READY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_READY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn SRAM_APB_ERR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SRAM_APB_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SRAM_INT_ENABLE {
        #[inline(always)]
        fn default() -> SRAM_INT_ENABLE {
            SRAM_INT_ENABLE(0)
        }
    }
    impl core::fmt::Debug for SRAM_INT_ENABLE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAM_INT_ENABLE")
                .field("READY", &self.READY())
                .field("SRAM_APB_ERR", &self.SRAM_APB_ERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAM_INT_ENABLE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAM_INT_ENABLE {
                READY: bool,
                SRAM_APB_ERR: bool,
            }
            let proxy = SRAM_INT_ENABLE {
                READY: self.READY(),
                SRAM_APB_ERR: self.SRAM_APB_ERR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Enable Set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAM_INT_SET_ENABLE(pub u32);
    impl SRAM_INT_SET_ENABLE {
        #[inline(always)]
        pub const fn READY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_READY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn APB_ERR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APB_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SRAM_INT_SET_ENABLE {
        #[inline(always)]
        fn default() -> SRAM_INT_SET_ENABLE {
            SRAM_INT_SET_ENABLE(0)
        }
    }
    impl core::fmt::Debug for SRAM_INT_SET_ENABLE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAM_INT_SET_ENABLE")
                .field("READY", &self.READY())
                .field("APB_ERR", &self.APB_ERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAM_INT_SET_ENABLE {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAM_INT_SET_ENABLE {
                READY: bool,
                APB_ERR: bool,
            }
            let proxy = SRAM_INT_SET_ENABLE {
                READY: self.READY(),
                APB_ERR: self.APB_ERR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Status set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAM_INT_SET_STATUS(pub u32);
    impl SRAM_INT_SET_STATUS {
        #[inline(always)]
        pub const fn READY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_READY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn APB_ERR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APB_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SRAM_INT_SET_STATUS {
        #[inline(always)]
        fn default() -> SRAM_INT_SET_STATUS {
            SRAM_INT_SET_STATUS(0)
        }
    }
    impl core::fmt::Debug for SRAM_INT_SET_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAM_INT_SET_STATUS")
                .field("READY", &self.READY())
                .field("APB_ERR", &self.APB_ERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAM_INT_SET_STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAM_INT_SET_STATUS {
                READY: bool,
                APB_ERR: bool,
            }
            let proxy = SRAM_INT_SET_STATUS {
                READY: self.READY(),
                APB_ERR: self.APB_ERR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAM_INT_STATUS(pub u32);
    impl SRAM_INT_STATUS {
        #[inline(always)]
        pub const fn READY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_READY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn APB_ERR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APB_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SRAM_INT_STATUS {
        #[inline(always)]
        fn default() -> SRAM_INT_STATUS {
            SRAM_INT_STATUS(0)
        }
    }
    impl core::fmt::Debug for SRAM_INT_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAM_INT_STATUS")
                .field("READY", &self.READY())
                .field("APB_ERR", &self.APB_ERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAM_INT_STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAM_INT_STATUS {
                READY: bool,
                APB_ERR: bool,
            }
            let proxy = SRAM_INT_STATUS {
                READY: self.READY(),
                APB_ERR: self.APB_ERR(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SRAM_STATUS(pub u32);
    impl SRAM_STATUS {
        #[inline(always)]
        pub const fn READY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_READY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SRAM_STATUS {
        #[inline(always)]
        fn default() -> SRAM_STATUS {
            SRAM_STATUS(0)
        }
    }
    impl core::fmt::Debug for SRAM_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SRAM_STATUS")
                .field("READY", &self.READY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SRAM_STATUS {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct SRAM_STATUS {
                READY: bool,
            }
            let proxy = SRAM_STATUS {
                READY: self.READY(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
