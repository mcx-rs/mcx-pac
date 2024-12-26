#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
    pub const fn SGI_DATIN0A(self) -> crate::common::Reg<regs::SGI_DATIN0A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN0B(self) -> crate::common::Reg<regs::SGI_DATIN0B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN0C(self) -> crate::common::Reg<regs::SGI_DATIN0C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN0D(self) -> crate::common::Reg<regs::SGI_DATIN0D, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN1A(self) -> crate::common::Reg<regs::SGI_DATIN1A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN1B(self) -> crate::common::Reg<regs::SGI_DATIN1B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN1C(self) -> crate::common::Reg<regs::SGI_DATIN1C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN1D(self) -> crate::common::Reg<regs::SGI_DATIN1D, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x021cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN2A(self) -> crate::common::Reg<regs::SGI_DATIN2A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN2B(self) -> crate::common::Reg<regs::SGI_DATIN2B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0224usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN2C(self) -> crate::common::Reg<regs::SGI_DATIN2C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN2D(self) -> crate::common::Reg<regs::SGI_DATIN2D, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x022cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN3A(self) -> crate::common::Reg<regs::SGI_DATIN3A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN3B(self) -> crate::common::Reg<regs::SGI_DATIN3B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0234usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN3C(self) -> crate::common::Reg<regs::SGI_DATIN3C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0238usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATIN3D(self) -> crate::common::Reg<regs::SGI_DATIN3D, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x023cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY0A(self) -> crate::common::Reg<regs::SGI_KEY0A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY0B(self) -> crate::common::Reg<regs::SGI_KEY0B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0244usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY0C(self) -> crate::common::Reg<regs::SGI_KEY0C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0248usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY0D(self) -> crate::common::Reg<regs::SGI_KEY0D, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x024cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY1A(self) -> crate::common::Reg<regs::SGI_KEY1A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0250usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY1B(self) -> crate::common::Reg<regs::SGI_KEY1B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0254usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY1C(self) -> crate::common::Reg<regs::SGI_KEY1C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0258usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY1D(self) -> crate::common::Reg<regs::SGI_KEY1D, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x025cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY2A(self) -> crate::common::Reg<regs::SGI_KEY2A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY2B(self) -> crate::common::Reg<regs::SGI_KEY2B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0264usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY2C(self) -> crate::common::Reg<regs::SGI_KEY2C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0268usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY2D(self) -> crate::common::Reg<regs::SGI_KEY2D, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x026cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY3A(self) -> crate::common::Reg<regs::SGI_KEY3A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0270usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY3B(self) -> crate::common::Reg<regs::SGI_KEY3B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0274usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY3C(self) -> crate::common::Reg<regs::SGI_KEY3C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0278usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY3D(self) -> crate::common::Reg<regs::SGI_KEY3D, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x027cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY4A(self) -> crate::common::Reg<regs::SGI_KEY4A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY4B(self) -> crate::common::Reg<regs::SGI_KEY4B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY4C(self) -> crate::common::Reg<regs::SGI_KEY4C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0288usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY4D(self) -> crate::common::Reg<regs::SGI_KEY4D, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x028cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY5A(self) -> crate::common::Reg<regs::SGI_KEY5A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0290usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY5B(self) -> crate::common::Reg<regs::SGI_KEY5B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0294usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY5C(self) -> crate::common::Reg<regs::SGI_KEY5C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0298usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY5D(self) -> crate::common::Reg<regs::SGI_KEY5D, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x029cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY6A(self) -> crate::common::Reg<regs::SGI_KEY6A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a0usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY6B(self) -> crate::common::Reg<regs::SGI_KEY6B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a4usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY6C(self) -> crate::common::Reg<regs::SGI_KEY6C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a8usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY6D(self) -> crate::common::Reg<regs::SGI_KEY6D, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02acusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY7A(self) -> crate::common::Reg<regs::SGI_KEY7A, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b0usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY7B(self) -> crate::common::Reg<regs::SGI_KEY7B, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b4usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY7C(self) -> crate::common::Reg<regs::SGI_KEY7C, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b8usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY7D(self) -> crate::common::Reg<regs::SGI_KEY7D, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02bcusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATOUTA(self) -> crate::common::Reg<regs::SGI_DATOUTA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c0usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATOUTB(self) -> crate::common::Reg<regs::SGI_DATOUTB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c4usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATOUTC(self) -> crate::common::Reg<regs::SGI_DATOUTC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c8usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_DATOUTD(self) -> crate::common::Reg<regs::SGI_DATOUTD, crate::common::RW> {
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
    pub const fn SGI_KEYCHK(self) -> crate::common::Reg<regs::SGI_KEYCHK, crate::common::RW> {
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
    pub const fn SGI_SFR_SW_MASK(
        self,
    ) -> crate::common::Reg<regs::SGI_SFR_SW_MASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d0cusize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_SFRSEED(self) -> crate::common::Reg<regs::SGI_SFRSEED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d10usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_SHA2_CTRL(self) -> crate::common::Reg<regs::SGI_SHA2_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d14usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_SHA_FIFO(self) -> crate::common::Reg<regs::SGI_SHA_FIFO, crate::common::RW> {
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
    pub const fn SGI_PRNG_SW_SEED(
        self,
    ) -> crate::common::Reg<regs::SGI_PRNG_SW_SEED, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d30usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY_CTRL(self) -> crate::common::Reg<regs::SGI_KEY_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d40usize) as _) }
    }
    #[inline(always)]
    pub const fn SGI_KEY_WRAP(self) -> crate::common::Reg<regs::SGI_KEY_WRAP, crate::common::RW> {
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
    pub const fn SGI_MODULE_ID(self) -> crate::common::Reg<regs::SGI_MODULE_ID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "Access Error"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_ACCESS_ERR(pub u32);
    impl SGI_ACCESS_ERR {
        #[inline(always)]
        pub const fn apb_notav(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_apb_notav(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn apb_wrgmd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_apb_wrgmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn accerr_rsvd1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_accerr_rsvd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn apb_master(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_apb_master(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn accerr_rsvd2(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_accerr_rsvd2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for SGI_ACCESS_ERR {
        #[inline(always)]
        fn default() -> SGI_ACCESS_ERR {
            SGI_ACCESS_ERR(0)
        }
    }
    #[doc = "Clear Access Error"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_ACCESS_ERR_CLR(pub u32);
    impl SGI_ACCESS_ERR_CLR {
        #[inline(always)]
        pub const fn err_clr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_err_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn accerrc_rsvd(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_accerrc_rsvd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for SGI_ACCESS_ERR_CLR {
        #[inline(always)]
        fn default() -> SGI_ACCESS_ERR_CLR {
            SGI_ACCESS_ERR_CLR(0)
        }
    }
    #[doc = "SGI Auto Mode Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_AUTO_DMA_CTRL(pub u32);
    impl SGI_AUTO_DMA_CTRL {
        #[inline(always)]
        pub const fn ife(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ife(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn auto_dma_rsvd1(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_auto_dma_rsvd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[inline(always)]
        pub const fn ofe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ofe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn auto_dma_rsvd2(&self) -> u32 {
            let val = (self.0 >> 9usize) & 0x007f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_auto_dma_rsvd2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
        }
    }
    impl Default for SGI_AUTO_DMA_CTRL {
        #[inline(always)]
        fn default() -> SGI_AUTO_DMA_CTRL {
            SGI_AUTO_DMA_CTRL(0)
        }
    }
    #[doc = "SGI Auto Mode Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_AUTO_MODE(pub u32);
    impl SGI_AUTO_MODE {
        #[inline(always)]
        pub const fn auto_mode_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_auto_mode_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn auto_mode_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_auto_mode_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn auto_mode_rsvd1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_auto_mode_rsvd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn incr_mode(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_incr_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn auto_mode_rsvd2(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_auto_mode_rsvd2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn cmd(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[inline(always)]
        pub const fn auto_mode_rsvd3(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_auto_mode_rsvd3(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SGI_AUTO_MODE {
        #[inline(always)]
        fn default() -> SGI_AUTO_MODE {
            SGI_AUTO_MODE(0)
        }
    }
    #[doc = "SHA Configuration Reg"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_CONFIG(pub u32);
    impl SGI_CONFIG {
        #[inline(always)]
        pub const fn row(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_row(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn china(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_china(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn cc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_cc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn has_aes(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_has_aes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn has_des(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_has_des(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn has_sha(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_has_sha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn has_movem(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_has_movem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn has_cmac(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_has_cmac(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn has_gfmul(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_has_gfmul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn internal_prng(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_internal_prng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn key_digest(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_key_digest(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn count_size(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_count_size(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn configc_rsvd(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_configc_rsvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn fa(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_fa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn configb2_rsvd(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_configb2_rsvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn bus_width(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_bus_width(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn num_datin(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_num_datin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn num_key(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_num_key(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[inline(always)]
        pub const fn edc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_edc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn configb_rsvd(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_configb_rsvd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[inline(always)]
        pub const fn sha_256_only(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sha_256_only(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[inline(always)]
        pub const fn spb_support(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_spb_support(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn spb_masking(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_spb_masking(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn sfr_sw_mask(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sfr_sw_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn configa_rsvd(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_configa_rsvd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for SGI_CONFIG {
        #[inline(always)]
        fn default() -> SGI_CONFIG {
            SGI_CONFIG(0)
        }
    }
    #[doc = "SHA Configuration 2 Reg"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_CONFIG2(pub u32);
    impl SGI_CONFIG2 {
        #[inline(always)]
        pub const fn aes_used(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_aes_used(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn aes_num_sboxes(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_aes_num_sboxes(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
        }
        #[inline(always)]
        pub const fn aes_keysize(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_aes_keysize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[inline(always)]
        pub const fn config2b_rsvd(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_config2b_rsvd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
        #[inline(always)]
        pub const fn des_used(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_des_used(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn des_num_sboxes(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_des_num_sboxes(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
        #[inline(always)]
        pub const fn config2a_rsvd(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_config2a_rsvd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
        }
    }
    impl Default for SGI_CONFIG2 {
        #[inline(always)]
        fn default() -> SGI_CONFIG2 {
            SGI_CONFIG2(0)
        }
    }
    #[doc = "Calculation counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_COUNT(pub u32);
    impl SGI_COUNT {
        #[inline(always)]
        pub const fn count(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_count(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn count_rsvd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_count_rsvd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SGI_COUNT {
        #[inline(always)]
        fn default() -> SGI_COUNT {
            SGI_COUNT(0)
        }
    }
    #[doc = "SGI Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_CTRL(pub u32);
    impl SGI_CTRL {
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn decrypt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_decrypt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn aeskeysz(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_aeskeysz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn crypto_op(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_crypto_op(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[inline(always)]
        pub const fn insel(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_insel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
        }
        #[inline(always)]
        pub const fn outsel(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_outsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
        }
        #[inline(always)]
        pub const fn datout_res(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_datout_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[inline(always)]
        pub const fn aes_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_aes_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn des_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_des_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn gcm_en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_gcm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn prng_en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_prng_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn inkeysel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_inkeysel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
        }
        #[inline(always)]
        pub const fn tdeskey(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_tdeskey(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[inline(always)]
        pub const fn aes_no_kl(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_aes_no_kl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[inline(always)]
        pub const fn aes_sel(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_aes_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[inline(always)]
        pub const fn ctrl_rsvd(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ctrl_rsvd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for SGI_CTRL {
        #[inline(always)]
        fn default() -> SGI_CTRL {
            SGI_CTRL(0)
        }
    }
    #[doc = "SGI Control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_CTRL2(pub u32);
    impl SGI_CTRL2 {
        #[inline(always)]
        pub const fn flush(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_flush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn key_flush(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_key_flush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn datin_flush(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_datin_flush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn incr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_incr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn xorwr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_xorwr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn flushwr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_flushwr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn incr_cin(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_incr_cin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ctrl2_rsvd3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ctrl2_rsvd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn smasken(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_smasken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn smaskstep(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_smaskstep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn smasksw(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_smasksw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ctrl2_rsvd2(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ctrl2_rsvd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn movem(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_movem(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[inline(always)]
        pub const fn keyres(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_keyres(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[inline(always)]
        pub const fn rkey(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_rkey(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[inline(always)]
        pub const fn bytes_order(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_bytes_order(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[inline(always)]
        pub const fn gcm_inxor(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_gcm_inxor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn ctrl2_rsvd1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_ctrl2_rsvd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for SGI_CTRL2 {
        #[inline(always)]
        fn default() -> SGI_CTRL2 {
            SGI_CTRL2(0)
        }
    }
    #[doc = "Input Data register 0 - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN0A(pub u32);
    impl SGI_DATIN0A {
        #[inline(always)]
        pub const fn datin0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN0A {
        #[inline(always)]
        fn default() -> SGI_DATIN0A {
            SGI_DATIN0A(0)
        }
    }
    #[doc = "Input Data register 0 - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN0B(pub u32);
    impl SGI_DATIN0B {
        #[inline(always)]
        pub const fn datin0b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin0b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN0B {
        #[inline(always)]
        fn default() -> SGI_DATIN0B {
            SGI_DATIN0B(0)
        }
    }
    #[doc = "Input Data register 0 - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN0C(pub u32);
    impl SGI_DATIN0C {
        #[inline(always)]
        pub const fn datin0c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin0c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN0C {
        #[inline(always)]
        fn default() -> SGI_DATIN0C {
            SGI_DATIN0C(0)
        }
    }
    #[doc = "Input Data register 0 - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN0D(pub u32);
    impl SGI_DATIN0D {
        #[inline(always)]
        pub const fn datin0d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin0d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN0D {
        #[inline(always)]
        fn default() -> SGI_DATIN0D {
            SGI_DATIN0D(0)
        }
    }
    #[doc = "Input Data register 1 - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN1A(pub u32);
    impl SGI_DATIN1A {
        #[inline(always)]
        pub const fn datin1a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin1a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN1A {
        #[inline(always)]
        fn default() -> SGI_DATIN1A {
            SGI_DATIN1A(0)
        }
    }
    #[doc = "Input Data register 1 - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN1B(pub u32);
    impl SGI_DATIN1B {
        #[inline(always)]
        pub const fn datin1b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin1b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN1B {
        #[inline(always)]
        fn default() -> SGI_DATIN1B {
            SGI_DATIN1B(0)
        }
    }
    #[doc = "Input Data register 1 - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN1C(pub u32);
    impl SGI_DATIN1C {
        #[inline(always)]
        pub const fn datin1c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin1c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN1C {
        #[inline(always)]
        fn default() -> SGI_DATIN1C {
            SGI_DATIN1C(0)
        }
    }
    #[doc = "Input Data register 1 - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN1D(pub u32);
    impl SGI_DATIN1D {
        #[inline(always)]
        pub const fn datin1d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin1d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN1D {
        #[inline(always)]
        fn default() -> SGI_DATIN1D {
            SGI_DATIN1D(0)
        }
    }
    #[doc = "Input Data register 2 - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN2A(pub u32);
    impl SGI_DATIN2A {
        #[inline(always)]
        pub const fn datin2a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin2a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN2A {
        #[inline(always)]
        fn default() -> SGI_DATIN2A {
            SGI_DATIN2A(0)
        }
    }
    #[doc = "Input Data register 2 - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN2B(pub u32);
    impl SGI_DATIN2B {
        #[inline(always)]
        pub const fn datin2b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin2b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN2B {
        #[inline(always)]
        fn default() -> SGI_DATIN2B {
            SGI_DATIN2B(0)
        }
    }
    #[doc = "Input Data register 2 - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN2C(pub u32);
    impl SGI_DATIN2C {
        #[inline(always)]
        pub const fn datin2c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin2c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN2C {
        #[inline(always)]
        fn default() -> SGI_DATIN2C {
            SGI_DATIN2C(0)
        }
    }
    #[doc = "Input Data register 2 - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN2D(pub u32);
    impl SGI_DATIN2D {
        #[inline(always)]
        pub const fn datin2d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin2d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN2D {
        #[inline(always)]
        fn default() -> SGI_DATIN2D {
            SGI_DATIN2D(0)
        }
    }
    #[doc = "Input Data register 3 - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN3A(pub u32);
    impl SGI_DATIN3A {
        #[inline(always)]
        pub const fn datin3a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin3a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN3A {
        #[inline(always)]
        fn default() -> SGI_DATIN3A {
            SGI_DATIN3A(0)
        }
    }
    #[doc = "Input Data register 3 - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN3B(pub u32);
    impl SGI_DATIN3B {
        #[inline(always)]
        pub const fn datin3b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin3b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN3B {
        #[inline(always)]
        fn default() -> SGI_DATIN3B {
            SGI_DATIN3B(0)
        }
    }
    #[doc = "Input Data register 3 - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN3C(pub u32);
    impl SGI_DATIN3C {
        #[inline(always)]
        pub const fn datin3c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin3c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN3C {
        #[inline(always)]
        fn default() -> SGI_DATIN3C {
            SGI_DATIN3C(0)
        }
    }
    #[doc = "Input Data register 3 - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATIN3D(pub u32);
    impl SGI_DATIN3D {
        #[inline(always)]
        pub const fn datin3d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datin3d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATIN3D {
        #[inline(always)]
        fn default() -> SGI_DATIN3D {
            SGI_DATIN3D(0)
        }
    }
    #[doc = "Output Data register - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATOUTA(pub u32);
    impl SGI_DATOUTA {
        #[inline(always)]
        pub const fn datouta(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datouta(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATOUTA {
        #[inline(always)]
        fn default() -> SGI_DATOUTA {
            SGI_DATOUTA(0)
        }
    }
    #[doc = "Output Data register - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATOUTB(pub u32);
    impl SGI_DATOUTB {
        #[inline(always)]
        pub const fn datoutb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datoutb(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATOUTB {
        #[inline(always)]
        fn default() -> SGI_DATOUTB {
            SGI_DATOUTB(0)
        }
    }
    #[doc = "Output Data register - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATOUTC(pub u32);
    impl SGI_DATOUTC {
        #[inline(always)]
        pub const fn datoutc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datoutc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATOUTC {
        #[inline(always)]
        fn default() -> SGI_DATOUTC {
            SGI_DATOUTC(0)
        }
    }
    #[doc = "Output Data register - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DATOUTD(pub u32);
    impl SGI_DATOUTD {
        #[inline(always)]
        pub const fn datoutd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_datoutd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_DATOUTD {
        #[inline(always)]
        fn default() -> SGI_DATOUTD {
            SGI_DATOUTD(0)
        }
    }
    #[doc = "Configuration of dummy controls"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_DUMMY_CTRL(pub u32);
    impl SGI_DUMMY_CTRL {
        #[inline(always)]
        pub const fn ddctrl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_ddctrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[inline(always)]
        pub const fn dmyctl_rsvd2(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_dmyctl_rsvd2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
        }
        #[inline(always)]
        pub const fn adctrl(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub fn set_adctrl(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[inline(always)]
        pub const fn dmyctl_rsvd1(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_dmyctl_rsvd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
        }
    }
    impl Default for SGI_DUMMY_CTRL {
        #[inline(always)]
        fn default() -> SGI_DUMMY_CTRL {
            SGI_DUMMY_CTRL(0)
        }
    }
    #[doc = "Interrupt enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_INT_ENABLE(pub u32);
    impl SGI_INT_ENABLE {
        #[inline(always)]
        pub const fn int_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn int_ena_rsvd(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_int_ena_rsvd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for SGI_INT_ENABLE {
        #[inline(always)]
        fn default() -> SGI_INT_ENABLE {
            SGI_INT_ENABLE(0)
        }
    }
    #[doc = "Interrupt status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_INT_STATUS(pub u32);
    impl SGI_INT_STATUS {
        #[inline(always)]
        pub const fn int_pdone(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_int_pdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn intst_rsvd(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_intst_rsvd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for SGI_INT_STATUS {
        #[inline(always)]
        fn default() -> SGI_INT_STATUS {
            SGI_INT_STATUS(0)
        }
    }
    #[doc = "Interrupt status clear"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_INT_STATUS_CLR(pub u32);
    impl SGI_INT_STATUS_CLR {
        #[inline(always)]
        pub const fn int_clr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_int_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn int_stsc_rsvd(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_int_stsc_rsvd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for SGI_INT_STATUS_CLR {
        #[inline(always)]
        fn default() -> SGI_INT_STATUS_CLR {
            SGI_INT_STATUS_CLR(0)
        }
    }
    #[doc = "Interrupt status set"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_INT_STATUS_SET(pub u32);
    impl SGI_INT_STATUS_SET {
        #[inline(always)]
        pub const fn int_set(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_int_set(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn int_stss_rsvd(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_int_stss_rsvd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for SGI_INT_STATUS_SET {
        #[inline(always)]
        fn default() -> SGI_INT_STATUS_SET {
            SGI_INT_STATUS_SET(0)
        }
    }
    #[doc = "Input Key register 0 - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY0A(pub u32);
    impl SGI_KEY0A {
        #[inline(always)]
        pub const fn key0a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key0a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY0A {
        #[inline(always)]
        fn default() -> SGI_KEY0A {
            SGI_KEY0A(0)
        }
    }
    #[doc = "Input Key register 0 - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY0B(pub u32);
    impl SGI_KEY0B {
        #[inline(always)]
        pub const fn key0b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key0b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY0B {
        #[inline(always)]
        fn default() -> SGI_KEY0B {
            SGI_KEY0B(0)
        }
    }
    #[doc = "Input Key register 0 - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY0C(pub u32);
    impl SGI_KEY0C {
        #[inline(always)]
        pub const fn key0c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key0c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY0C {
        #[inline(always)]
        fn default() -> SGI_KEY0C {
            SGI_KEY0C(0)
        }
    }
    #[doc = "Input Key register 0 - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY0D(pub u32);
    impl SGI_KEY0D {
        #[inline(always)]
        pub const fn key0d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key0d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY0D {
        #[inline(always)]
        fn default() -> SGI_KEY0D {
            SGI_KEY0D(0)
        }
    }
    #[doc = "Input Key register 1 - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY1A(pub u32);
    impl SGI_KEY1A {
        #[inline(always)]
        pub const fn key1a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key1a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY1A {
        #[inline(always)]
        fn default() -> SGI_KEY1A {
            SGI_KEY1A(0)
        }
    }
    #[doc = "Input Key register 1 - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY1B(pub u32);
    impl SGI_KEY1B {
        #[inline(always)]
        pub const fn key1b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key1b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY1B {
        #[inline(always)]
        fn default() -> SGI_KEY1B {
            SGI_KEY1B(0)
        }
    }
    #[doc = "Input Key register 1 - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY1C(pub u32);
    impl SGI_KEY1C {
        #[inline(always)]
        pub const fn key1c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key1c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY1C {
        #[inline(always)]
        fn default() -> SGI_KEY1C {
            SGI_KEY1C(0)
        }
    }
    #[doc = "Input Key register 1 - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY1D(pub u32);
    impl SGI_KEY1D {
        #[inline(always)]
        pub const fn key1d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key1d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY1D {
        #[inline(always)]
        fn default() -> SGI_KEY1D {
            SGI_KEY1D(0)
        }
    }
    #[doc = "Input Key register 2 - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY2A(pub u32);
    impl SGI_KEY2A {
        #[inline(always)]
        pub const fn key2a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key2a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY2A {
        #[inline(always)]
        fn default() -> SGI_KEY2A {
            SGI_KEY2A(0)
        }
    }
    #[doc = "Input Key register 2 - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY2B(pub u32);
    impl SGI_KEY2B {
        #[inline(always)]
        pub const fn key2b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key2b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY2B {
        #[inline(always)]
        fn default() -> SGI_KEY2B {
            SGI_KEY2B(0)
        }
    }
    #[doc = "Input Key register 2 - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY2C(pub u32);
    impl SGI_KEY2C {
        #[inline(always)]
        pub const fn key2c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key2c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY2C {
        #[inline(always)]
        fn default() -> SGI_KEY2C {
            SGI_KEY2C(0)
        }
    }
    #[doc = "Input Key register 2 - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY2D(pub u32);
    impl SGI_KEY2D {
        #[inline(always)]
        pub const fn key2d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key2d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY2D {
        #[inline(always)]
        fn default() -> SGI_KEY2D {
            SGI_KEY2D(0)
        }
    }
    #[doc = "Input Key register 3 - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY3A(pub u32);
    impl SGI_KEY3A {
        #[inline(always)]
        pub const fn key3a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key3a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY3A {
        #[inline(always)]
        fn default() -> SGI_KEY3A {
            SGI_KEY3A(0)
        }
    }
    #[doc = "Input Key register 3 - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY3B(pub u32);
    impl SGI_KEY3B {
        #[inline(always)]
        pub const fn key3b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key3b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY3B {
        #[inline(always)]
        fn default() -> SGI_KEY3B {
            SGI_KEY3B(0)
        }
    }
    #[doc = "Input Key register 3 - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY3C(pub u32);
    impl SGI_KEY3C {
        #[inline(always)]
        pub const fn key3c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key3c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY3C {
        #[inline(always)]
        fn default() -> SGI_KEY3C {
            SGI_KEY3C(0)
        }
    }
    #[doc = "Input Key register 3 - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY3D(pub u32);
    impl SGI_KEY3D {
        #[inline(always)]
        pub const fn key3d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key3d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY3D {
        #[inline(always)]
        fn default() -> SGI_KEY3D {
            SGI_KEY3D(0)
        }
    }
    #[doc = "Input Key register 4 - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY4A(pub u32);
    impl SGI_KEY4A {
        #[inline(always)]
        pub const fn key4a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key4a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY4A {
        #[inline(always)]
        fn default() -> SGI_KEY4A {
            SGI_KEY4A(0)
        }
    }
    #[doc = "Input Key register 4 - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY4B(pub u32);
    impl SGI_KEY4B {
        #[inline(always)]
        pub const fn key4b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key4b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY4B {
        #[inline(always)]
        fn default() -> SGI_KEY4B {
            SGI_KEY4B(0)
        }
    }
    #[doc = "Input Key register 4 - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY4C(pub u32);
    impl SGI_KEY4C {
        #[inline(always)]
        pub const fn key4c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key4c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY4C {
        #[inline(always)]
        fn default() -> SGI_KEY4C {
            SGI_KEY4C(0)
        }
    }
    #[doc = "Input Key register 4 - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY4D(pub u32);
    impl SGI_KEY4D {
        #[inline(always)]
        pub const fn key4d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key4d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY4D {
        #[inline(always)]
        fn default() -> SGI_KEY4D {
            SGI_KEY4D(0)
        }
    }
    #[doc = "Input Key register 5 - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY5A(pub u32);
    impl SGI_KEY5A {
        #[inline(always)]
        pub const fn key5a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key5a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY5A {
        #[inline(always)]
        fn default() -> SGI_KEY5A {
            SGI_KEY5A(0)
        }
    }
    #[doc = "Input Key register 5 - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY5B(pub u32);
    impl SGI_KEY5B {
        #[inline(always)]
        pub const fn key5b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key5b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY5B {
        #[inline(always)]
        fn default() -> SGI_KEY5B {
            SGI_KEY5B(0)
        }
    }
    #[doc = "Input Key register 5 - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY5C(pub u32);
    impl SGI_KEY5C {
        #[inline(always)]
        pub const fn key5c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key5c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY5C {
        #[inline(always)]
        fn default() -> SGI_KEY5C {
            SGI_KEY5C(0)
        }
    }
    #[doc = "Input Key register 5 - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY5D(pub u32);
    impl SGI_KEY5D {
        #[inline(always)]
        pub const fn key5d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key5d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY5D {
        #[inline(always)]
        fn default() -> SGI_KEY5D {
            SGI_KEY5D(0)
        }
    }
    #[doc = "Input Key register 6 - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY6A(pub u32);
    impl SGI_KEY6A {
        #[inline(always)]
        pub const fn key6a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key6a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY6A {
        #[inline(always)]
        fn default() -> SGI_KEY6A {
            SGI_KEY6A(0)
        }
    }
    #[doc = "Input Key register 6 - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY6B(pub u32);
    impl SGI_KEY6B {
        #[inline(always)]
        pub const fn key6b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key6b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY6B {
        #[inline(always)]
        fn default() -> SGI_KEY6B {
            SGI_KEY6B(0)
        }
    }
    #[doc = "Input Key register 6 - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY6C(pub u32);
    impl SGI_KEY6C {
        #[inline(always)]
        pub const fn key6c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key6c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY6C {
        #[inline(always)]
        fn default() -> SGI_KEY6C {
            SGI_KEY6C(0)
        }
    }
    #[doc = "Input Key register 6 - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY6D(pub u32);
    impl SGI_KEY6D {
        #[inline(always)]
        pub const fn key6d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key6d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY6D {
        #[inline(always)]
        fn default() -> SGI_KEY6D {
            SGI_KEY6D(0)
        }
    }
    #[doc = "Input Key register 7 - Word-3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY7A(pub u32);
    impl SGI_KEY7A {
        #[inline(always)]
        pub const fn key7a(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key7a(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY7A {
        #[inline(always)]
        fn default() -> SGI_KEY7A {
            SGI_KEY7A(0)
        }
    }
    #[doc = "Input Key register 7 - Word-2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY7B(pub u32);
    impl SGI_KEY7B {
        #[inline(always)]
        pub const fn key7b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key7b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY7B {
        #[inline(always)]
        fn default() -> SGI_KEY7B {
            SGI_KEY7B(0)
        }
    }
    #[doc = "Input Key register 7 - Word-1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY7C(pub u32);
    impl SGI_KEY7C {
        #[inline(always)]
        pub const fn key7c(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key7c(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY7C {
        #[inline(always)]
        fn default() -> SGI_KEY7C {
            SGI_KEY7C(0)
        }
    }
    #[doc = "Input Key register 7 - Word-0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY7D(pub u32);
    impl SGI_KEY7D {
        #[inline(always)]
        pub const fn key7d(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key7d(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY7D {
        #[inline(always)]
        fn default() -> SGI_KEY7D {
            SGI_KEY7D(0)
        }
    }
    #[doc = "Key checksum register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEYCHK(pub u32);
    impl SGI_KEYCHK {
        #[inline(always)]
        pub const fn keychksum(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_keychksum(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEYCHK {
        #[inline(always)]
        fn default() -> SGI_KEYCHK {
            SGI_KEYCHK(0)
        }
    }
    #[doc = "SGI Key Control SFR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY_CTRL(pub u32);
    impl SGI_KEY_CTRL {
        #[inline(always)]
        pub const fn key_wo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_key_wo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY_CTRL {
        #[inline(always)]
        fn default() -> SGI_KEY_CTRL {
            SGI_KEY_CTRL(0)
        }
    }
    #[doc = "Wrapped key read SFR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_KEY_WRAP(pub u32);
    impl SGI_KEY_WRAP {
        #[inline(always)]
        pub const fn kw_data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_kw_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_KEY_WRAP {
        #[inline(always)]
        fn default() -> SGI_KEY_WRAP {
            SGI_KEY_WRAP(0)
        }
    }
    #[doc = "Module ID"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_MODULE_ID(pub u32);
    impl SGI_MODULE_ID {
        #[inline(always)]
        pub const fn placeholder(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_placeholder(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_MODULE_ID {
        #[inline(always)]
        fn default() -> SGI_MODULE_ID {
            SGI_MODULE_ID(0)
        }
    }
    #[doc = "SGI internal PRNG SW seeding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_PRNG_SW_SEED(pub u32);
    impl SGI_PRNG_SW_SEED {
        #[inline(always)]
        pub const fn seed(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_seed(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_PRNG_SW_SEED {
        #[inline(always)]
        fn default() -> SGI_PRNG_SW_SEED {
            SGI_PRNG_SW_SEED(0)
        }
    }
    #[doc = "SFRSEED register for SFRMASK feature."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_SFRSEED(pub u32);
    impl SGI_SFRSEED {
        #[inline(always)]
        pub const fn sfrseed(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_sfrseed(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_SFRSEED {
        #[inline(always)]
        fn default() -> SGI_SFRSEED {
            SGI_SFRSEED(0)
        }
    }
    #[doc = "Sofware Assisted Masking register ."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_SFR_SW_MASK(pub u32);
    impl SGI_SFR_SW_MASK {
        #[inline(always)]
        pub const fn sfr_mask_val(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_sfr_mask_val(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_SFR_SW_MASK {
        #[inline(always)]
        fn default() -> SGI_SFR_SW_MASK {
            SGI_SFR_SW_MASK(0)
        }
    }
    #[doc = "SHA Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_SHA2_CTRL(pub u32);
    impl SGI_SHA2_CTRL {
        #[inline(always)]
        pub const fn sha2_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sha2_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn sha2_mode(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sha2_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn sha2_size(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_sha2_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn sha2_low_lim(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_sha2_low_lim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn sha2_high_lim(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_sha2_high_lim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn sha2_count_en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sha2_count_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn hash_reload(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_hash_reload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn sha2_stop(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sha2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn no_auto_init(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_no_auto_init(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn sha2ctl_rsvd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_sha2ctl_rsvd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for SGI_SHA2_CTRL {
        #[inline(always)]
        fn default() -> SGI_SHA2_CTRL {
            SGI_SHA2_CTRL(0)
        }
    }
    #[doc = "SHA FIFO lower-bank low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_SHA_FIFO(pub u32);
    impl SGI_SHA_FIFO {
        #[inline(always)]
        pub const fn fifo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_fifo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for SGI_SHA_FIFO {
        #[inline(always)]
        fn default() -> SGI_SHA_FIFO {
            SGI_SHA_FIFO(0)
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_STATUS(pub u32);
    impl SGI_STATUS {
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn oflow(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_oflow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn prng_rdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_prng_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn error(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_error(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[inline(always)]
        pub const fn sha2_busy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sha2_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn irq(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_irq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn sha_fifo_full(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sha_fifo_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn sha_fifo_level(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_sha_fifo_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
        }
        #[inline(always)]
        pub const fn sha_error(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_sha_error(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn key_read_err(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_key_read_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn key_unwrap_err(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_key_unwrap_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn status_rsvd3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_status_rsvd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[inline(always)]
        pub const fn status_rsvd(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x1fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_status_rsvd(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 19usize)) | (((val as u32) & 0x1fff) << 19usize);
        }
    }
    impl Default for SGI_STATUS {
        #[inline(always)]
        fn default() -> SGI_STATUS {
            SGI_STATUS(0)
        }
    }
    #[doc = "SGI Version"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SGI_VERSION(pub u32);
    impl SGI_VERSION {
        #[inline(always)]
        pub const fn z(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_z(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn y2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_y2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn y1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_y1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn x(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_x(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[inline(always)]
        pub const fn milestone(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_milestone(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[inline(always)]
        pub const fn version_rsvd_1(&self) -> u16 {
            let val = (self.0 >> 18usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_version_rsvd_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 18usize)) | (((val as u32) & 0x3fff) << 18usize);
        }
    }
    impl Default for SGI_VERSION {
        #[inline(always)]
        fn default() -> SGI_VERSION {
            SGI_VERSION(0)
        }
    }
}
