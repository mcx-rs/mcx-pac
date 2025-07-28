#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SGI {
    ptr: *mut u8,
}
unsafe impl Send for SGI {}
unsafe impl Sync for SGI {}
impl SGI {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn SGI_DATIN0A(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN0B(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN0C(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN0D(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN1A(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN1B(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN1C(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN1D(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x021cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN2A(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN2B(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0224usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN2C(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN2D(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x022cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN3A(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN3B(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0234usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN3C(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0238usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN3D(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x023cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY0A(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY0B(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0244usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY0C(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0248usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY0D(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x024cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY1A(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0250usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY1B(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0254usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY1C(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0258usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY1D(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x025cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY2A(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY2B(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0264usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY2C(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0268usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY2D(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x026cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY3A(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0270usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY3B(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0274usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY3C(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0278usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY3D(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x027cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY4A(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY4B(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY4C(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0288usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY4D(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x028cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY5A(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0290usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY5B(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0294usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY5C(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0298usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY5D(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x029cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY6A(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a0usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY6B(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a4usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY6C(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a8usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY6D(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02acusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY7A(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b0usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY7B(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b4usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY7C(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b8usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY7D(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02bcusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATOUTA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c0usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATOUTB(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c4usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATOUTC(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c8usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATOUTD(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02ccusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_STATUS(self) -> crate::common::Reg<regs::SGI_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c00usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_COUNT(self) -> crate::common::Reg<regs::SGI_COUNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c04usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEYCHK(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c08usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_CTRL(self) -> crate::common::Reg<regs::SGI_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d00usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_CTRL2(self) -> crate::common::Reg<regs::SGI_CTRL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d04usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DUMMY_CTRL(
        self,
    ) -> crate::common::Reg<regs::SGI_DUMMY_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d08usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_SFR_SW_MASK(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d0cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_SFRSEED(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d10usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_SHA2_CTRL(self) -> crate::common::Reg<regs::SGI_SHA2_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d14usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_SHA_FIFO(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d18usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_CONFIG(self) -> crate::common::Reg<regs::SGI_CONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d1cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_CONFIG2(self) -> crate::common::Reg<regs::SGI_CONFIG2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d20usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_AUTO_MODE(self) -> crate::common::Reg<regs::SGI_AUTO_MODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d24usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_AUTO_DMA_CTRL(
        self,
    ) -> crate::common::Reg<regs::SGI_AUTO_DMA_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d28usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_PRNG_SW_SEED(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d30usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY_CTRL(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d40usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY_WRAP(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d50usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_VERSION(self) -> crate::common::Reg<regs::SGI_VERSION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0f08usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_ACCESS_ERR(
        self,
    ) -> crate::common::Reg<regs::SGI_ACCESS_ERR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fc0usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_ACCESS_ERR_CLR(
        self,
    ) -> crate::common::Reg<regs::SGI_ACCESS_ERR_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fc4usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_INT_STATUS(
        self,
    ) -> crate::common::Reg<regs::SGI_INT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe0usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_INT_ENABLE(
        self,
    ) -> crate::common::Reg<regs::SGI_INT_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe4usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_INT_STATUS_CLR(
        self,
    ) -> crate::common::Reg<regs::SGI_INT_STATUS_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe8usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_INT_STATUS_SET(
        self,
    ) -> crate::common::Reg<regs::SGI_INT_STATUS_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fecusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_MODULE_ID(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "Access Error"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_ACCESS_ERR(pub u32);
    impl SGI_ACCESS_ERR {
        #[must_use]
        #[inline(always)]
        pub const fn APB_NOTAV(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_APB_NOTAV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn APB_WRGMD(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_APB_WRGMD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn APB_MASTER(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_APB_MASTER(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for SGI_ACCESS_ERR {
        #[inline(always)]
        fn default() -> SGI_ACCESS_ERR {
            SGI_ACCESS_ERR(0)
        }
    }
    impl core::fmt::Debug for SGI_ACCESS_ERR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_ACCESS_ERR")
                .field("APB_NOTAV", &self.APB_NOTAV())
                .field("APB_WRGMD", &self.APB_WRGMD())
                .field("APB_MASTER", &self.APB_MASTER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_ACCESS_ERR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SGI_ACCESS_ERR {{ APB_NOTAV: {=bool:?}, APB_WRGMD: {=bool:?}, APB_MASTER: {=u8:?} }}" , self . APB_NOTAV () , self . APB_WRGMD () , self . APB_MASTER ())
        }
    }
    #[doc = "Clear Access Error"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_ACCESS_ERR_CLR(pub u32);
    impl SGI_ACCESS_ERR_CLR {
        #[must_use]
        #[inline(always)]
        pub const fn ERR_CLR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERR_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SGI_ACCESS_ERR_CLR {
        #[inline(always)]
        fn default() -> SGI_ACCESS_ERR_CLR {
            SGI_ACCESS_ERR_CLR(0)
        }
    }
    impl core::fmt::Debug for SGI_ACCESS_ERR_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_ACCESS_ERR_CLR")
                .field("ERR_CLR", &self.ERR_CLR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_ACCESS_ERR_CLR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SGI_ACCESS_ERR_CLR {{ ERR_CLR: {=bool:?} }}",
                self.ERR_CLR()
            )
        }
    }
    #[doc = "SGI Auto Mode Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_AUTO_DMA_CTRL(pub u32);
    impl SGI_AUTO_DMA_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn IFE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OFE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for SGI_AUTO_DMA_CTRL {
        #[inline(always)]
        fn default() -> SGI_AUTO_DMA_CTRL {
            SGI_AUTO_DMA_CTRL(0)
        }
    }
    impl core::fmt::Debug for SGI_AUTO_DMA_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_AUTO_DMA_CTRL")
                .field("IFE", &self.IFE())
                .field("OFE", &self.OFE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_AUTO_DMA_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SGI_AUTO_DMA_CTRL {{ IFE: {=bool:?}, OFE: {=bool:?} }}",
                self.IFE(),
                self.OFE()
            )
        }
    }
    #[doc = "SGI Auto Mode Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_AUTO_MODE(pub u32);
    impl SGI_AUTO_MODE {
        #[must_use]
        #[inline(always)]
        pub const fn AUTO_MODE_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AUTO_MODE_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AUTO_MODE_STOP(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AUTO_MODE_STOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INCR_MODE(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_INCR_MODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CMD(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CMD(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for SGI_AUTO_MODE {
        #[inline(always)]
        fn default() -> SGI_AUTO_MODE {
            SGI_AUTO_MODE(0)
        }
    }
    impl core::fmt::Debug for SGI_AUTO_MODE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_AUTO_MODE")
                .field("AUTO_MODE_EN", &self.AUTO_MODE_EN())
                .field("AUTO_MODE_STOP", &self.AUTO_MODE_STOP())
                .field("INCR_MODE", &self.INCR_MODE())
                .field("CMD", &self.CMD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_AUTO_MODE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SGI_AUTO_MODE {{ AUTO_MODE_EN: {=bool:?}, AUTO_MODE_STOP: {=bool:?}, INCR_MODE: {=u8:?}, CMD: {=u8:?} }}" , self . AUTO_MODE_EN () , self . AUTO_MODE_STOP () , self . INCR_MODE () , self . CMD ())
        }
    }
    #[doc = "SHA Configuration Reg"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_CONFIG(pub u32);
    impl SGI_CONFIG {
        #[must_use]
        #[inline(always)]
        pub const fn ROW(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ROW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CHINA(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CHINA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CC(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HAS_AES(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HAS_AES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HAS_DES(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HAS_DES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HAS_SHA(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HAS_SHA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HAS_MOVEM(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HAS_MOVEM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HAS_CMAC(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HAS_CMAC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HAS_GFMUL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HAS_GFMUL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INTERNAL_PRNG(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INTERNAL_PRNG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn KEY_DIGEST(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_KEY_DIGEST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COUNT_SIZE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COUNT_SIZE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FA(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BUS_WIDTH(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BUS_WIDTH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NUM_DATIN(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NUM_DATIN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NUM_KEY(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NUM_KEY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDC(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EDC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHA_256_ONLY(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SHA_256_ONLY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPB_SUPPORT(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPB_SUPPORT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPB_MASKING(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPB_MASKING(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SFR_SW_MASK(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SFR_SW_MASK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for SGI_CONFIG {
        #[inline(always)]
        fn default() -> SGI_CONFIG {
            SGI_CONFIG(0)
        }
    }
    impl core::fmt::Debug for SGI_CONFIG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_CONFIG")
                .field("ROW", &self.ROW())
                .field("CHINA", &self.CHINA())
                .field("CC", &self.CC())
                .field("HAS_AES", &self.HAS_AES())
                .field("HAS_DES", &self.HAS_DES())
                .field("HAS_SHA", &self.HAS_SHA())
                .field("HAS_MOVEM", &self.HAS_MOVEM())
                .field("HAS_CMAC", &self.HAS_CMAC())
                .field("HAS_GFMUL", &self.HAS_GFMUL())
                .field("INTERNAL_PRNG", &self.INTERNAL_PRNG())
                .field("KEY_DIGEST", &self.KEY_DIGEST())
                .field("COUNT_SIZE", &self.COUNT_SIZE())
                .field("FA", &self.FA())
                .field("BUS_WIDTH", &self.BUS_WIDTH())
                .field("NUM_DATIN", &self.NUM_DATIN())
                .field("NUM_KEY", &self.NUM_KEY())
                .field("EDC", &self.EDC())
                .field("SHA_256_ONLY", &self.SHA_256_ONLY())
                .field("SPB_SUPPORT", &self.SPB_SUPPORT())
                .field("SPB_MASKING", &self.SPB_MASKING())
                .field("SFR_SW_MASK", &self.SFR_SW_MASK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_CONFIG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SGI_CONFIG {{ ROW: {=bool:?}, CHINA: {=bool:?}, CC: {=bool:?}, HAS_AES: {=bool:?}, HAS_DES: {=bool:?}, HAS_SHA: {=bool:?}, HAS_MOVEM: {=bool:?}, HAS_CMAC: {=bool:?}, HAS_GFMUL: {=bool:?}, INTERNAL_PRNG: {=bool:?}, KEY_DIGEST: {=bool:?}, COUNT_SIZE: {=bool:?}, FA: {=bool:?}, BUS_WIDTH: {=bool:?}, NUM_DATIN: {=u8:?}, NUM_KEY: {=u8:?}, EDC: {=bool:?}, SHA_256_ONLY: {=bool:?}, SPB_SUPPORT: {=bool:?}, SPB_MASKING: {=bool:?}, SFR_SW_MASK: {=bool:?} }}" , self . ROW () , self . CHINA () , self . CC () , self . HAS_AES () , self . HAS_DES () , self . HAS_SHA () , self . HAS_MOVEM () , self . HAS_CMAC () , self . HAS_GFMUL () , self . INTERNAL_PRNG () , self . KEY_DIGEST () , self . COUNT_SIZE () , self . FA () , self . BUS_WIDTH () , self . NUM_DATIN () , self . NUM_KEY () , self . EDC () , self . SHA_256_ONLY () , self . SPB_SUPPORT () , self . SPB_MASKING () , self . SFR_SW_MASK ())
        }
    }
    #[doc = "SHA Configuration 2 Reg"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_CONFIG2(pub u32);
    impl SGI_CONFIG2 {
        #[must_use]
        #[inline(always)]
        pub const fn AES_USED(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_AES_USED(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AES_NUM_SBOXES(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_AES_NUM_SBOXES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AES_KEYSIZE(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_AES_KEYSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DES_USED(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DES_USED(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DES_NUM_SBOXES(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DES_NUM_SBOXES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
    }
    impl Default for SGI_CONFIG2 {
        #[inline(always)]
        fn default() -> SGI_CONFIG2 {
            SGI_CONFIG2(0)
        }
    }
    impl core::fmt::Debug for SGI_CONFIG2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_CONFIG2")
                .field("AES_USED", &self.AES_USED())
                .field("AES_NUM_SBOXES", &self.AES_NUM_SBOXES())
                .field("AES_KEYSIZE", &self.AES_KEYSIZE())
                .field("DES_USED", &self.DES_USED())
                .field("DES_NUM_SBOXES", &self.DES_NUM_SBOXES())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_CONFIG2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SGI_CONFIG2 {{ AES_USED: {=u8:?}, AES_NUM_SBOXES: {=u8:?}, AES_KEYSIZE: {=u8:?}, DES_USED: {=u8:?}, DES_NUM_SBOXES: {=u8:?} }}" , self . AES_USED () , self . AES_NUM_SBOXES () , self . AES_KEYSIZE () , self . DES_USED () , self . DES_NUM_SBOXES ())
        }
    }
    #[doc = "Calculation counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_COUNT(pub u32);
    impl SGI_COUNT {
        #[must_use]
        #[inline(always)]
        pub const fn COUNT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_COUNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for SGI_COUNT {
        #[inline(always)]
        fn default() -> SGI_COUNT {
            SGI_COUNT(0)
        }
    }
    impl core::fmt::Debug for SGI_COUNT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_COUNT")
                .field("COUNT", &self.COUNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_COUNT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SGI_COUNT {{ COUNT: {=u16:?} }}", self.COUNT())
        }
    }
    #[doc = "SGI Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_CTRL(pub u32);
    impl SGI_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn START(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_START(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DECRYPT(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DECRYPT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AESKEYSZ(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_AESKEYSZ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CRYPTO_OP(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CRYPTO_OP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INSEL(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_INSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OUTSEL(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_OUTSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DATOUT_RES(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DATOUT_RES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AES_EN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AES_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DES_EN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DES_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GCM_EN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GCM_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PRNG_EN(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PRNG_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INKEYSEL(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_INKEYSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TDESKEY(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TDESKEY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AES_NO_KL(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AES_NO_KL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AES_SEL(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AES_SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for SGI_CTRL {
        #[inline(always)]
        fn default() -> SGI_CTRL {
            SGI_CTRL(0)
        }
    }
    impl core::fmt::Debug for SGI_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_CTRL")
                .field("START", &self.START())
                .field("DECRYPT", &self.DECRYPT())
                .field("AESKEYSZ", &self.AESKEYSZ())
                .field("CRYPTO_OP", &self.CRYPTO_OP())
                .field("INSEL", &self.INSEL())
                .field("OUTSEL", &self.OUTSEL())
                .field("DATOUT_RES", &self.DATOUT_RES())
                .field("AES_EN", &self.AES_EN())
                .field("DES_EN", &self.DES_EN())
                .field("GCM_EN", &self.GCM_EN())
                .field("PRNG_EN", &self.PRNG_EN())
                .field("INKEYSEL", &self.INKEYSEL())
                .field("TDESKEY", &self.TDESKEY())
                .field("AES_NO_KL", &self.AES_NO_KL())
                .field("AES_SEL", &self.AES_SEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SGI_CTRL {{ START: {=bool:?}, DECRYPT: {=bool:?}, AESKEYSZ: {=u8:?}, CRYPTO_OP: {=u8:?}, INSEL: {=u8:?}, OUTSEL: {=u8:?}, DATOUT_RES: {=u8:?}, AES_EN: {=bool:?}, DES_EN: {=bool:?}, GCM_EN: {=bool:?}, PRNG_EN: {=bool:?}, INKEYSEL: {=u8:?}, TDESKEY: {=bool:?}, AES_NO_KL: {=bool:?}, AES_SEL: {=bool:?} }}" , self . START () , self . DECRYPT () , self . AESKEYSZ () , self . CRYPTO_OP () , self . INSEL () , self . OUTSEL () , self . DATOUT_RES () , self . AES_EN () , self . DES_EN () , self . GCM_EN () , self . PRNG_EN () , self . INKEYSEL () , self . TDESKEY () , self . AES_NO_KL () , self . AES_SEL ())
        }
    }
    #[doc = "SGI Control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_CTRL2(pub u32);
    impl SGI_CTRL2 {
        #[must_use]
        #[inline(always)]
        pub const fn FLUSH(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLUSH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn KEY_FLUSH(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_KEY_FLUSH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DATIN_FLUSH(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DATIN_FLUSH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INCR(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INCR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn XORWR(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_XORWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FLUSHWR(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FLUSHWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn INCR_CIN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INCR_CIN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMASKEN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SMASKEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMASKSTEP(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SMASKSTEP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMASKSW(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SMASKSW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MOVEM(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MOVEM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn KEYRES(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_KEYRES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RKEY(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RKEY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BYTES_ORDER(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BYTES_ORDER(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GCM_INXOR(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GCM_INXOR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for SGI_CTRL2 {
        #[inline(always)]
        fn default() -> SGI_CTRL2 {
            SGI_CTRL2(0)
        }
    }
    impl core::fmt::Debug for SGI_CTRL2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_CTRL2")
                .field("FLUSH", &self.FLUSH())
                .field("KEY_FLUSH", &self.KEY_FLUSH())
                .field("DATIN_FLUSH", &self.DATIN_FLUSH())
                .field("INCR", &self.INCR())
                .field("XORWR", &self.XORWR())
                .field("FLUSHWR", &self.FLUSHWR())
                .field("INCR_CIN", &self.INCR_CIN())
                .field("SMASKEN", &self.SMASKEN())
                .field("SMASKSTEP", &self.SMASKSTEP())
                .field("SMASKSW", &self.SMASKSW())
                .field("MOVEM", &self.MOVEM())
                .field("KEYRES", &self.KEYRES())
                .field("RKEY", &self.RKEY())
                .field("BYTES_ORDER", &self.BYTES_ORDER())
                .field("GCM_INXOR", &self.GCM_INXOR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_CTRL2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SGI_CTRL2 {{ FLUSH: {=bool:?}, KEY_FLUSH: {=bool:?}, DATIN_FLUSH: {=bool:?}, INCR: {=bool:?}, XORWR: {=bool:?}, FLUSHWR: {=bool:?}, INCR_CIN: {=bool:?}, SMASKEN: {=bool:?}, SMASKSTEP: {=bool:?}, SMASKSW: {=bool:?}, MOVEM: {=u8:?}, KEYRES: {=u8:?}, RKEY: {=bool:?}, BYTES_ORDER: {=bool:?}, GCM_INXOR: {=bool:?} }}" , self . FLUSH () , self . KEY_FLUSH () , self . DATIN_FLUSH () , self . INCR () , self . XORWR () , self . FLUSHWR () , self . INCR_CIN () , self . SMASKEN () , self . SMASKSTEP () , self . SMASKSW () , self . MOVEM () , self . KEYRES () , self . RKEY () , self . BYTES_ORDER () , self . GCM_INXOR ())
        }
    }
    #[doc = "Configuration of dummy controls"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DUMMY_CTRL(pub u32);
    impl SGI_DUMMY_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn DDCTRL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_DDCTRL(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADCTRL(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ADCTRL(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for SGI_DUMMY_CTRL {
        #[inline(always)]
        fn default() -> SGI_DUMMY_CTRL {
            SGI_DUMMY_CTRL(0)
        }
    }
    impl core::fmt::Debug for SGI_DUMMY_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_DUMMY_CTRL")
                .field("DDCTRL", &self.DDCTRL())
                .field("ADCTRL", &self.ADCTRL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_DUMMY_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SGI_DUMMY_CTRL {{ DDCTRL: {=u16:?}, ADCTRL: {=u16:?} }}",
                self.DDCTRL(),
                self.ADCTRL()
            )
        }
    }
    #[doc = "Interrupt enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_INT_ENABLE(pub u32);
    impl SGI_INT_ENABLE {
        #[must_use]
        #[inline(always)]
        pub const fn INT_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SGI_INT_ENABLE {
        #[inline(always)]
        fn default() -> SGI_INT_ENABLE {
            SGI_INT_ENABLE(0)
        }
    }
    impl core::fmt::Debug for SGI_INT_ENABLE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_INT_ENABLE")
                .field("INT_EN", &self.INT_EN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_INT_ENABLE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SGI_INT_ENABLE {{ INT_EN: {=bool:?} }}", self.INT_EN())
        }
    }
    #[doc = "Interrupt status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_INT_STATUS(pub u32);
    impl SGI_INT_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn INT_PDONE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT_PDONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SGI_INT_STATUS {
        #[inline(always)]
        fn default() -> SGI_INT_STATUS {
            SGI_INT_STATUS(0)
        }
    }
    impl core::fmt::Debug for SGI_INT_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_INT_STATUS")
                .field("INT_PDONE", &self.INT_PDONE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_INT_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SGI_INT_STATUS {{ INT_PDONE: {=bool:?} }}",
                self.INT_PDONE()
            )
        }
    }
    #[doc = "Interrupt status clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_INT_STATUS_CLR(pub u32);
    impl SGI_INT_STATUS_CLR {
        #[must_use]
        #[inline(always)]
        pub const fn INT_CLR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT_CLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SGI_INT_STATUS_CLR {
        #[inline(always)]
        fn default() -> SGI_INT_STATUS_CLR {
            SGI_INT_STATUS_CLR(0)
        }
    }
    impl core::fmt::Debug for SGI_INT_STATUS_CLR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_INT_STATUS_CLR")
                .field("INT_CLR", &self.INT_CLR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_INT_STATUS_CLR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SGI_INT_STATUS_CLR {{ INT_CLR: {=bool:?} }}",
                self.INT_CLR()
            )
        }
    }
    #[doc = "Interrupt status set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_INT_STATUS_SET(pub u32);
    impl SGI_INT_STATUS_SET {
        #[must_use]
        #[inline(always)]
        pub const fn INT_SET(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_INT_SET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SGI_INT_STATUS_SET {
        #[inline(always)]
        fn default() -> SGI_INT_STATUS_SET {
            SGI_INT_STATUS_SET(0)
        }
    }
    impl core::fmt::Debug for SGI_INT_STATUS_SET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_INT_STATUS_SET")
                .field("INT_SET", &self.INT_SET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_INT_STATUS_SET {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SGI_INT_STATUS_SET {{ INT_SET: {=bool:?} }}",
                self.INT_SET()
            )
        }
    }
    #[doc = "SHA Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_SHA2_CTRL(pub u32);
    impl SGI_SHA2_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn SHA2_EN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SHA2_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHA2_MODE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SHA2_MODE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHA2_SIZE(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SHA2_SIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHA2_LOW_LIM(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SHA2_LOW_LIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHA2_HIGH_LIM(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SHA2_HIGH_LIM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHA2_COUNT_EN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SHA2_COUNT_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HASH_RELOAD(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HASH_RELOAD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHA2_STOP(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SHA2_STOP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NO_AUTO_INIT(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NO_AUTO_INIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for SGI_SHA2_CTRL {
        #[inline(always)]
        fn default() -> SGI_SHA2_CTRL {
            SGI_SHA2_CTRL(0)
        }
    }
    impl core::fmt::Debug for SGI_SHA2_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_SHA2_CTRL")
                .field("SHA2_EN", &self.SHA2_EN())
                .field("SHA2_MODE", &self.SHA2_MODE())
                .field("SHA2_SIZE", &self.SHA2_SIZE())
                .field("SHA2_LOW_LIM", &self.SHA2_LOW_LIM())
                .field("SHA2_HIGH_LIM", &self.SHA2_HIGH_LIM())
                .field("SHA2_COUNT_EN", &self.SHA2_COUNT_EN())
                .field("HASH_RELOAD", &self.HASH_RELOAD())
                .field("SHA2_STOP", &self.SHA2_STOP())
                .field("NO_AUTO_INIT", &self.NO_AUTO_INIT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_SHA2_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SGI_SHA2_CTRL {{ SHA2_EN: {=bool:?}, SHA2_MODE: {=bool:?}, SHA2_SIZE: {=u8:?}, SHA2_LOW_LIM: {=u8:?}, SHA2_HIGH_LIM: {=u8:?}, SHA2_COUNT_EN: {=bool:?}, HASH_RELOAD: {=bool:?}, SHA2_STOP: {=bool:?}, NO_AUTO_INIT: {=bool:?} }}" , self . SHA2_EN () , self . SHA2_MODE () , self . SHA2_SIZE () , self . SHA2_LOW_LIM () , self . SHA2_HIGH_LIM () , self . SHA2_COUNT_EN () , self . HASH_RELOAD () , self . SHA2_STOP () , self . NO_AUTO_INIT ())
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_STATUS(pub u32);
    impl SGI_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn BUSY(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OFLOW(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OFLOW(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PRNG_RDY(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PRNG_RDY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERROR(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ERROR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHA2_BUSY(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SHA2_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IRQ(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IRQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHA_FIFO_FULL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SHA_FIFO_FULL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHA_FIFO_LEVEL(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SHA_FIFO_LEVEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SHA_ERROR(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SHA_ERROR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn KEY_READ_ERR(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_KEY_READ_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn KEY_UNWRAP_ERR(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_KEY_UNWRAP_ERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for SGI_STATUS {
        #[inline(always)]
        fn default() -> SGI_STATUS {
            SGI_STATUS(0)
        }
    }
    impl core::fmt::Debug for SGI_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_STATUS")
                .field("BUSY", &self.BUSY())
                .field("OFLOW", &self.OFLOW())
                .field("PRNG_RDY", &self.PRNG_RDY())
                .field("ERROR", &self.ERROR())
                .field("SHA2_BUSY", &self.SHA2_BUSY())
                .field("IRQ", &self.IRQ())
                .field("SHA_FIFO_FULL", &self.SHA_FIFO_FULL())
                .field("SHA_FIFO_LEVEL", &self.SHA_FIFO_LEVEL())
                .field("SHA_ERROR", &self.SHA_ERROR())
                .field("KEY_READ_ERR", &self.KEY_READ_ERR())
                .field("KEY_UNWRAP_ERR", &self.KEY_UNWRAP_ERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SGI_STATUS {{ BUSY: {=bool:?}, OFLOW: {=bool:?}, PRNG_RDY: {=bool:?}, ERROR: {=u8:?}, SHA2_BUSY: {=bool:?}, IRQ: {=bool:?}, SHA_FIFO_FULL: {=bool:?}, SHA_FIFO_LEVEL: {=u8:?}, SHA_ERROR: {=bool:?}, KEY_READ_ERR: {=bool:?}, KEY_UNWRAP_ERR: {=bool:?} }}" , self . BUSY () , self . OFLOW () , self . PRNG_RDY () , self . ERROR () , self . SHA2_BUSY () , self . IRQ () , self . SHA_FIFO_FULL () , self . SHA_FIFO_LEVEL () , self . SHA_ERROR () , self . KEY_READ_ERR () , self . KEY_UNWRAP_ERR ())
        }
    }
    #[doc = "SGI Version"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_VERSION(pub u32);
    impl SGI_VERSION {
        #[must_use]
        #[inline(always)]
        pub const fn Z(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Z(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Y2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Y2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Y1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_Y1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn X(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_X(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MILESTONE(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MILESTONE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for SGI_VERSION {
        #[inline(always)]
        fn default() -> SGI_VERSION {
            SGI_VERSION(0)
        }
    }
    impl core::fmt::Debug for SGI_VERSION {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SGI_VERSION")
                .field("Z", &self.Z())
                .field("Y2", &self.Y2())
                .field("Y1", &self.Y1())
                .field("X", &self.X())
                .field("MILESTONE", &self.MILESTONE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SGI_VERSION {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SGI_VERSION {{ Z: {=u8:?}, Y2: {=u8:?}, Y1: {=u8:?}, X: {=u8:?}, MILESTONE: {=u8:?} }}" , self . Z () , self . Y2 () , self . Y1 () , self . X () , self . MILESTONE ())
        }
    }
}
