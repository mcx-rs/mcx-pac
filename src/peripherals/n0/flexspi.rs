#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (0303941 2025-02-18))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXSPI {
    ptr: *mut u8,
}
unsafe impl Send for FLEXSPI {}
unsafe impl Sync for FLEXSPI {}
impl FLEXSPI {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MCR0(self) -> crate::common::Reg<regs::MCR0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn MCR1(self) -> crate::common::Reg<regs::MCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MCR2(self) -> crate::common::Reg<regs::MCR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBCR(self) -> crate::common::Reg<regs::AHBCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn INTEN(self) -> crate::common::Reg<regs::INTEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn INTR(self) -> crate::common::Reg<regs::INTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn LUTKEY(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn LUTCR(self) -> crate::common::Reg<regs::LUTCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn AHBRXBUFCR0(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::AHBRXBUFCR0, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLSHCR0(self, n: usize) -> crate::common::Reg<regs::FLSHCR0, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLSHCR1(self, n: usize) -> crate::common::Reg<regs::FLSHCR1, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLSHCR2(self, n: usize) -> crate::common::Reg<regs::FLSHCR2, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn FLSHCR4(self) -> crate::common::Reg<regs::FLSHCR4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn IPCR0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPCR1(self) -> crate::common::Reg<regs::IPCR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPCR2(self) -> crate::common::Reg<regs::IPCR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn IPCMD(self) -> crate::common::Reg<regs::IPCMD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn DLPR(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPRXFCR(self) -> crate::common::Reg<regs::IPRXFCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn IPTXFCR(self) -> crate::common::Reg<regs::IPTXFCR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[inline(always)]
    pub const fn DLLCR(self, n: usize) -> crate::common::Reg<regs::DLLCR, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn STS0(self) -> crate::common::Reg<regs::STS0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn STS1(self) -> crate::common::Reg<regs::STS1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn STS2(self) -> crate::common::Reg<regs::STS2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBSPNDSTS(self) -> crate::common::Reg<regs::AHBSPNDSTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[inline(always)]
    pub const fn IPRXFSTS(self) -> crate::common::Reg<regs::IPRXFSTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPTXFSTS(self) -> crate::common::Reg<regs::IPTXFSTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn RFDR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TFDR(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn LUT(self, n: usize) -> crate::common::Reg<regs::LUT, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn HADDRSTART(self) -> crate::common::Reg<regs::HADDRSTART, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[inline(always)]
    pub const fn HADDREND(self) -> crate::common::Reg<regs::HADDREND, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0424usize) as _) }
    }
    #[inline(always)]
    pub const fn HADDROFFSET(self) -> crate::common::Reg<regs::HADDROFFSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0428usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTRL(self) -> crate::common::Reg<regs::IPEDCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x042cusize) as _) }
    }
    #[inline(always)]
    pub const fn IPSNSZSTART0(self) -> crate::common::Reg<regs::IPSNSZSTART0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0430usize) as _) }
    }
    #[inline(always)]
    pub const fn IPSNSZEND0(self) -> crate::common::Reg<regs::IPSNSZEND0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0434usize) as _) }
    }
    #[inline(always)]
    pub const fn IPSNSZSTART1(self) -> crate::common::Reg<regs::IPSNSZSTART1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0438usize) as _) }
    }
    #[inline(always)]
    pub const fn IPSNSZEND1(self) -> crate::common::Reg<regs::IPSNSZEND1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x043cusize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONSTART0(
        self,
    ) -> crate::common::Reg<regs::AHBBUFREGIONSTART0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONEND0(
        self,
    ) -> crate::common::Reg<regs::AHBBUFREGIONEND0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0444usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONSTART1(
        self,
    ) -> crate::common::Reg<regs::AHBBUFREGIONSTART1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0448usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONEND1(
        self,
    ) -> crate::common::Reg<regs::AHBBUFREGIONEND1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x044cusize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONSTART2(
        self,
    ) -> crate::common::Reg<regs::AHBBUFREGIONSTART2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0450usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONEND2(
        self,
    ) -> crate::common::Reg<regs::AHBBUFREGIONEND2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0454usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONSTART3(
        self,
    ) -> crate::common::Reg<regs::AHBBUFREGIONSTART3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0458usize) as _) }
    }
    #[inline(always)]
    pub const fn AHBBUFREGIONEND3(
        self,
    ) -> crate::common::Reg<regs::AHBBUFREGIONEND3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x045cusize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTXCTRL(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::IPEDCTXCTRL, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX0IV0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX0IV1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX0START(self) -> crate::common::Reg<regs::IPEDCTX0START, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX0END(self) -> crate::common::Reg<regs::IPEDCTX0END, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX0AAD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX0AAD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX1IV0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX1IV1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX1START(self) -> crate::common::Reg<regs::IPEDCTX1START, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX1END(self) -> crate::common::Reg<regs::IPEDCTX1END, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x054cusize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX1AAD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX1AAD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX2IV0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX2IV1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0564usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX2START(self) -> crate::common::Reg<regs::IPEDCTX2START, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0568usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX2END(self) -> crate::common::Reg<regs::IPEDCTX2END, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x056cusize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX2AAD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0570usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX2AAD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0574usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX3IV0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX3IV1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0584usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX3START(self) -> crate::common::Reg<regs::IPEDCTX3START, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX3END(self) -> crate::common::Reg<regs::IPEDCTX3END, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x058cusize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX3AAD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0590usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX3AAD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0594usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX4IV0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX4IV1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX4START(self) -> crate::common::Reg<regs::IPEDCTX4START, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a8usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX4END(self) -> crate::common::Reg<regs::IPEDCTX4END, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05acusize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX4AAD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX4AAD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX5IV0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX5IV1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX5START(self) -> crate::common::Reg<regs::IPEDCTX5START, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c8usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX5END(self) -> crate::common::Reg<regs::IPEDCTX5END, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05ccusize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX5AAD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX5AAD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX6IV0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX6IV1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX6START(self) -> crate::common::Reg<regs::IPEDCTX6START, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e8usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX6END(self) -> crate::common::Reg<regs::IPEDCTX6END, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05ecusize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX6AAD0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX6AAD1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Receive Buffer Region 0 End Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBBUFREGIONEND0(pub u32);
    impl AHBBUFREGIONEND0 {
        #[inline(always)]
        pub const fn END_ADDRESS(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_END_ADDRESS(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for AHBBUFREGIONEND0 {
        #[inline(always)]
        fn default() -> AHBBUFREGIONEND0 {
            AHBBUFREGIONEND0(0)
        }
    }
    impl core::fmt::Debug for AHBBUFREGIONEND0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBBUFREGIONEND0")
                .field("END_ADDRESS", &self.END_ADDRESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBBUFREGIONEND0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHBBUFREGIONEND0 {{ END_ADDRESS: {=u32:?} }}",
                self.END_ADDRESS()
            )
        }
    }
    #[doc = "Receive Buffer Region 1 End Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBBUFREGIONEND1(pub u32);
    impl AHBBUFREGIONEND1 {
        #[inline(always)]
        pub const fn END_ADDRESS(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_END_ADDRESS(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for AHBBUFREGIONEND1 {
        #[inline(always)]
        fn default() -> AHBBUFREGIONEND1 {
            AHBBUFREGIONEND1(0)
        }
    }
    impl core::fmt::Debug for AHBBUFREGIONEND1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBBUFREGIONEND1")
                .field("END_ADDRESS", &self.END_ADDRESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBBUFREGIONEND1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHBBUFREGIONEND1 {{ END_ADDRESS: {=u32:?} }}",
                self.END_ADDRESS()
            )
        }
    }
    #[doc = "Receive Buffer Region 2 End Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBBUFREGIONEND2(pub u32);
    impl AHBBUFREGIONEND2 {
        #[inline(always)]
        pub const fn END_ADDRESS(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_END_ADDRESS(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for AHBBUFREGIONEND2 {
        #[inline(always)]
        fn default() -> AHBBUFREGIONEND2 {
            AHBBUFREGIONEND2(0)
        }
    }
    impl core::fmt::Debug for AHBBUFREGIONEND2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBBUFREGIONEND2")
                .field("END_ADDRESS", &self.END_ADDRESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBBUFREGIONEND2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHBBUFREGIONEND2 {{ END_ADDRESS: {=u32:?} }}",
                self.END_ADDRESS()
            )
        }
    }
    #[doc = "Receive Buffer Region 3 End Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBBUFREGIONEND3(pub u32);
    impl AHBBUFREGIONEND3 {
        #[inline(always)]
        pub const fn END_ADDRESS(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_END_ADDRESS(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for AHBBUFREGIONEND3 {
        #[inline(always)]
        fn default() -> AHBBUFREGIONEND3 {
            AHBBUFREGIONEND3(0)
        }
    }
    impl core::fmt::Debug for AHBBUFREGIONEND3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBBUFREGIONEND3")
                .field("END_ADDRESS", &self.END_ADDRESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBBUFREGIONEND3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHBBUFREGIONEND3 {{ END_ADDRESS: {=u32:?} }}",
                self.END_ADDRESS()
            )
        }
    }
    #[doc = "Receive Buffer Start Address of Region 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBBUFREGIONSTART0(pub u32);
    impl AHBBUFREGIONSTART0 {
        #[inline(always)]
        pub const fn START_ADDRESS(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_START_ADDRESS(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for AHBBUFREGIONSTART0 {
        #[inline(always)]
        fn default() -> AHBBUFREGIONSTART0 {
            AHBBUFREGIONSTART0(0)
        }
    }
    impl core::fmt::Debug for AHBBUFREGIONSTART0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBBUFREGIONSTART0")
                .field("START_ADDRESS", &self.START_ADDRESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBBUFREGIONSTART0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHBBUFREGIONSTART0 {{ START_ADDRESS: {=u32:?} }}",
                self.START_ADDRESS()
            )
        }
    }
    #[doc = "Receive Buffer Start Address of Region 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBBUFREGIONSTART1(pub u32);
    impl AHBBUFREGIONSTART1 {
        #[inline(always)]
        pub const fn START_ADDRESS(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_START_ADDRESS(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for AHBBUFREGIONSTART1 {
        #[inline(always)]
        fn default() -> AHBBUFREGIONSTART1 {
            AHBBUFREGIONSTART1(0)
        }
    }
    impl core::fmt::Debug for AHBBUFREGIONSTART1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBBUFREGIONSTART1")
                .field("START_ADDRESS", &self.START_ADDRESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBBUFREGIONSTART1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHBBUFREGIONSTART1 {{ START_ADDRESS: {=u32:?} }}",
                self.START_ADDRESS()
            )
        }
    }
    #[doc = "Receive Buffer Start Address of Region 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBBUFREGIONSTART2(pub u32);
    impl AHBBUFREGIONSTART2 {
        #[inline(always)]
        pub const fn START_ADDRESS(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_START_ADDRESS(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for AHBBUFREGIONSTART2 {
        #[inline(always)]
        fn default() -> AHBBUFREGIONSTART2 {
            AHBBUFREGIONSTART2(0)
        }
    }
    impl core::fmt::Debug for AHBBUFREGIONSTART2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBBUFREGIONSTART2")
                .field("START_ADDRESS", &self.START_ADDRESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBBUFREGIONSTART2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHBBUFREGIONSTART2 {{ START_ADDRESS: {=u32:?} }}",
                self.START_ADDRESS()
            )
        }
    }
    #[doc = "Receive Buffer Start Address of Region 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBBUFREGIONSTART3(pub u32);
    impl AHBBUFREGIONSTART3 {
        #[inline(always)]
        pub const fn START_ADDRESS(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_START_ADDRESS(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for AHBBUFREGIONSTART3 {
        #[inline(always)]
        fn default() -> AHBBUFREGIONSTART3 {
            AHBBUFREGIONSTART3(0)
        }
    }
    impl core::fmt::Debug for AHBBUFREGIONSTART3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBBUFREGIONSTART3")
                .field("START_ADDRESS", &self.START_ADDRESS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBBUFREGIONSTART3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHBBUFREGIONSTART3 {{ START_ADDRESS: {=u32:?} }}",
                self.START_ADDRESS()
            )
        }
    }
    #[doc = "AHB Bus Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBCR(pub u32);
    impl AHBCR {
        #[inline(always)]
        pub const fn APAREN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_APAREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn CLRAHBRXBUF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLRAHBRXBUF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn CLRAHBTXBUF(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLRAHBTXBUF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn CACHABLEEN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CACHABLEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn BUFFERABLEEN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BUFFERABLEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn PREFETCHEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PREFETCHEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn READADDROPT(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_READADDROPT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn RESUMEDISABLE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RESUMEDISABLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn READSZALIGN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_READSZALIGN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn ALIGNMENT(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ALIGNMENT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[inline(always)]
        pub const fn AFLASHBASE(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_AFLASHBASE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for AHBCR {
        #[inline(always)]
        fn default() -> AHBCR {
            AHBCR(0)
        }
    }
    impl core::fmt::Debug for AHBCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBCR")
                .field("APAREN", &self.APAREN())
                .field("CLRAHBRXBUF", &self.CLRAHBRXBUF())
                .field("CLRAHBTXBUF", &self.CLRAHBTXBUF())
                .field("CACHABLEEN", &self.CACHABLEEN())
                .field("BUFFERABLEEN", &self.BUFFERABLEEN())
                .field("PREFETCHEN", &self.PREFETCHEN())
                .field("READADDROPT", &self.READADDROPT())
                .field("RESUMEDISABLE", &self.RESUMEDISABLE())
                .field("READSZALIGN", &self.READSZALIGN())
                .field("ALIGNMENT", &self.ALIGNMENT())
                .field("AFLASHBASE", &self.AFLASHBASE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AHBCR {{ APAREN: {=bool:?}, CLRAHBRXBUF: {=bool:?}, CLRAHBTXBUF: {=bool:?}, CACHABLEEN: {=bool:?}, BUFFERABLEEN: {=bool:?}, PREFETCHEN: {=bool:?}, READADDROPT: {=bool:?}, RESUMEDISABLE: {=bool:?}, READSZALIGN: {=bool:?}, ALIGNMENT: {=u8:?}, AFLASHBASE: {=u8:?} }}" , self . APAREN () , self . CLRAHBRXBUF () , self . CLRAHBTXBUF () , self . CACHABLEEN () , self . BUFFERABLEEN () , self . PREFETCHEN () , self . READADDROPT () , self . RESUMEDISABLE () , self . READSZALIGN () , self . ALIGNMENT () , self . AFLASHBASE ())
        }
    }
    #[doc = "AHB Receive Buffer 0 Control 0..AHB Receive Buffer 7 Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBRXBUFCR0(pub u32);
    impl AHBRXBUFCR0 {
        #[inline(always)]
        pub const fn BUFSZ(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_BUFSZ(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn MSTRID(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_MSTRID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[inline(always)]
        pub const fn PRIORITY(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_PRIORITY(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[inline(always)]
        pub const fn REGIONEN(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REGIONEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn PREFETCHEN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PREFETCHEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for AHBRXBUFCR0 {
        #[inline(always)]
        fn default() -> AHBRXBUFCR0 {
            AHBRXBUFCR0(0)
        }
    }
    impl core::fmt::Debug for AHBRXBUFCR0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBRXBUFCR0")
                .field("BUFSZ", &self.BUFSZ())
                .field("MSTRID", &self.MSTRID())
                .field("PRIORITY", &self.PRIORITY())
                .field("REGIONEN", &self.REGIONEN())
                .field("PREFETCHEN", &self.PREFETCHEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBRXBUFCR0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "AHBRXBUFCR0 {{ BUFSZ: {=u8:?}, MSTRID: {=u8:?}, PRIORITY: {=u8:?}, REGIONEN: {=bool:?}, PREFETCHEN: {=bool:?} }}" , self . BUFSZ () , self . MSTRID () , self . PRIORITY () , self . REGIONEN () , self . PREFETCHEN ())
        }
    }
    #[doc = "AHB Suspend Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AHBSPNDSTS(pub u32);
    impl AHBSPNDSTS {
        #[inline(always)]
        pub const fn ACTIVE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ACTIVE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn BUFID(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_BUFID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[inline(always)]
        pub const fn DATLFT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_DATLFT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for AHBSPNDSTS {
        #[inline(always)]
        fn default() -> AHBSPNDSTS {
            AHBSPNDSTS(0)
        }
    }
    impl core::fmt::Debug for AHBSPNDSTS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AHBSPNDSTS")
                .field("ACTIVE", &self.ACTIVE())
                .field("BUFID", &self.BUFID())
                .field("DATLFT", &self.DATLFT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AHBSPNDSTS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AHBSPNDSTS {{ ACTIVE: {=bool:?}, BUFID: {=u8:?}, DATLFT: {=u16:?} }}",
                self.ACTIVE(),
                self.BUFID(),
                self.DATLFT()
            )
        }
    }
    #[doc = "DLL Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DLLCR(pub u32);
    impl DLLCR {
        #[inline(always)]
        pub const fn DLLEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DLLEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn DLLRESET(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DLLRESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn SLVDLYTARGET(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_SLVDLYTARGET(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
        }
        #[inline(always)]
        pub const fn OVRDEN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_OVRDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn OVRDVAL(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_OVRDVAL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
        }
        #[inline(always)]
        pub const fn REFPHASEGAP(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_REFPHASEGAP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
        }
    }
    impl Default for DLLCR {
        #[inline(always)]
        fn default() -> DLLCR {
            DLLCR(0)
        }
    }
    impl core::fmt::Debug for DLLCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DLLCR")
                .field("DLLEN", &self.DLLEN())
                .field("DLLRESET", &self.DLLRESET())
                .field("SLVDLYTARGET", &self.SLVDLYTARGET())
                .field("OVRDEN", &self.OVRDEN())
                .field("OVRDVAL", &self.OVRDVAL())
                .field("REFPHASEGAP", &self.REFPHASEGAP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DLLCR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DLLCR {{ DLLEN: {=bool:?}, DLLRESET: {=bool:?}, SLVDLYTARGET: {=u8:?}, OVRDEN: {=bool:?}, OVRDVAL: {=u8:?}, REFPHASEGAP: {=u8:?} }}" , self . DLLEN () , self . DLLRESET () , self . SLVDLYTARGET () , self . OVRDEN () , self . OVRDVAL () , self . REFPHASEGAP ())
        }
    }
    #[doc = "Flash Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLSHCR0(pub u32);
    impl FLSHCR0 {
        #[inline(always)]
        pub const fn FLSHSZ(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x007f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_FLSHSZ(&mut self, val: u32) {
            self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
        }
        #[inline(always)]
        pub const fn ADDRSHIFT(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ADDRSHIFT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[inline(always)]
        pub const fn SPLITWREN(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLITWREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[inline(always)]
        pub const fn SPLITRDEN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SPLITRDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FLSHCR0 {
        #[inline(always)]
        fn default() -> FLSHCR0 {
            FLSHCR0(0)
        }
    }
    impl core::fmt::Debug for FLSHCR0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLSHCR0")
                .field("FLSHSZ", &self.FLSHSZ())
                .field("ADDRSHIFT", &self.ADDRSHIFT())
                .field("SPLITWREN", &self.SPLITWREN())
                .field("SPLITRDEN", &self.SPLITRDEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLSHCR0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLSHCR0 {{ FLSHSZ: {=u32:?}, ADDRSHIFT: {=bool:?}, SPLITWREN: {=bool:?}, SPLITRDEN: {=bool:?} }}" , self . FLSHSZ () , self . ADDRSHIFT () , self . SPLITWREN () , self . SPLITRDEN ())
        }
    }
    #[doc = "Flash Control 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLSHCR1(pub u32);
    impl FLSHCR1 {
        #[inline(always)]
        pub const fn TCSS(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TCSS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[inline(always)]
        pub const fn TCSH(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TCSH(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[inline(always)]
        pub const fn WA(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn CAS(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_CAS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[inline(always)]
        pub const fn CSINTERVALUNIT(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CSINTERVALUNIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn CSINTERVAL(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_CSINTERVAL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for FLSHCR1 {
        #[inline(always)]
        fn default() -> FLSHCR1 {
            FLSHCR1(0)
        }
    }
    impl core::fmt::Debug for FLSHCR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLSHCR1")
                .field("TCSS", &self.TCSS())
                .field("TCSH", &self.TCSH())
                .field("WA", &self.WA())
                .field("CAS", &self.CAS())
                .field("CSINTERVALUNIT", &self.CSINTERVALUNIT())
                .field("CSINTERVAL", &self.CSINTERVAL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLSHCR1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLSHCR1 {{ TCSS: {=u8:?}, TCSH: {=u8:?}, WA: {=bool:?}, CAS: {=u8:?}, CSINTERVALUNIT: {=bool:?}, CSINTERVAL: {=u16:?} }}" , self . TCSS () , self . TCSH () , self . WA () , self . CAS () , self . CSINTERVALUNIT () , self . CSINTERVAL ())
        }
    }
    #[doc = "Flash Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLSHCR2(pub u32);
    impl FLSHCR2 {
        #[inline(always)]
        pub const fn ARDSEQID(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ARDSEQID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn ARDSEQNUM(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_ARDSEQNUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[inline(always)]
        pub const fn AWRSEQID(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_AWRSEQID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn AWRSEQNUM(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_AWRSEQNUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[inline(always)]
        pub const fn AWRWAIT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub fn set_AWRWAIT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
        #[inline(always)]
        pub const fn AWRWAITUNIT(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_AWRWAITUNIT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[inline(always)]
        pub const fn CLRINSTRPTR(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLRINSTRPTR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for FLSHCR2 {
        #[inline(always)]
        fn default() -> FLSHCR2 {
            FLSHCR2(0)
        }
    }
    impl core::fmt::Debug for FLSHCR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLSHCR2")
                .field("ARDSEQID", &self.ARDSEQID())
                .field("ARDSEQNUM", &self.ARDSEQNUM())
                .field("AWRSEQID", &self.AWRSEQID())
                .field("AWRSEQNUM", &self.AWRSEQNUM())
                .field("AWRWAIT", &self.AWRWAIT())
                .field("AWRWAITUNIT", &self.AWRWAITUNIT())
                .field("CLRINSTRPTR", &self.CLRINSTRPTR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLSHCR2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "FLSHCR2 {{ ARDSEQID: {=u8:?}, ARDSEQNUM: {=u8:?}, AWRSEQID: {=u8:?}, AWRSEQNUM: {=u8:?}, AWRWAIT: {=u16:?}, AWRWAITUNIT: {=u8:?}, CLRINSTRPTR: {=bool:?} }}" , self . ARDSEQID () , self . ARDSEQNUM () , self . AWRSEQID () , self . AWRSEQNUM () , self . AWRWAIT () , self . AWRWAITUNIT () , self . CLRINSTRPTR ())
        }
    }
    #[doc = "Flash Control 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FLSHCR4(pub u32);
    impl FLSHCR4 {
        #[inline(always)]
        pub const fn WMOPT1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WMOPT1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn WMENA(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WMENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn WMENB(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_WMENB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for FLSHCR4 {
        #[inline(always)]
        fn default() -> FLSHCR4 {
            FLSHCR4(0)
        }
    }
    impl core::fmt::Debug for FLSHCR4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FLSHCR4")
                .field("WMOPT1", &self.WMOPT1())
                .field("WMENA", &self.WMENA())
                .field("WMENB", &self.WMENB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FLSHCR4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "FLSHCR4 {{ WMOPT1: {=bool:?}, WMENA: {=bool:?}, WMENB: {=bool:?} }}",
                self.WMOPT1(),
                self.WMENA(),
                self.WMENB()
            )
        }
    }
    #[doc = "HADDR REMAP END ADDR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HADDREND(pub u32);
    impl HADDREND {
        #[inline(always)]
        pub const fn ENDSTART(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_ENDSTART(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for HADDREND {
        #[inline(always)]
        fn default() -> HADDREND {
            HADDREND(0)
        }
    }
    impl core::fmt::Debug for HADDREND {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HADDREND")
                .field("ENDSTART", &self.ENDSTART())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HADDREND {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HADDREND {{ ENDSTART: {=u32:?} }}", self.ENDSTART())
        }
    }
    #[doc = "HADDR Remap Offset"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HADDROFFSET(pub u32);
    impl HADDROFFSET {
        #[inline(always)]
        pub const fn ADDROFFSET(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_ADDROFFSET(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for HADDROFFSET {
        #[inline(always)]
        fn default() -> HADDROFFSET {
            HADDROFFSET(0)
        }
    }
    impl core::fmt::Debug for HADDROFFSET {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HADDROFFSET")
                .field("ADDROFFSET", &self.ADDROFFSET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HADDROFFSET {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HADDROFFSET {{ ADDROFFSET: {=u32:?} }}",
                self.ADDROFFSET()
            )
        }
    }
    #[doc = "HADDR REMAP Start Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HADDRSTART(pub u32);
    impl HADDRSTART {
        #[inline(always)]
        pub const fn REMAPEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_REMAPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ADDRSTART(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_ADDRSTART(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for HADDRSTART {
        #[inline(always)]
        fn default() -> HADDRSTART {
            HADDRSTART(0)
        }
    }
    impl core::fmt::Debug for HADDRSTART {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HADDRSTART")
                .field("REMAPEN", &self.REMAPEN())
                .field("ADDRSTART", &self.ADDRSTART())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HADDRSTART {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HADDRSTART {{ REMAPEN: {=bool:?}, ADDRSTART: {=u32:?} }}",
                self.REMAPEN(),
                self.ADDRSTART()
            )
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTEN(pub u32);
    impl INTEN {
        #[inline(always)]
        pub const fn IPCMDDONEEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPCMDDONEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn IPCMDGEEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPCMDGEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn AHBCMDGEEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHBCMDGEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn IPCMDERREN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPCMDERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn AHBCMDERREN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHBCMDERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn IPRXWAEN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPRXWAEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn IPTXWEEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPTXWEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn DATALEARNFAILEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DATALEARNFAILEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn SCKSTOPBYRDEN(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCKSTOPBYRDEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn SCKSTOPBYWREN(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCKSTOPBYWREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn AHBBUSTIMEOUTEN(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHBBUSTIMEOUTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SEQTIMEOUTEN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEQTIMEOUTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn IPCMDSECUREVIOEN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPCMDSECUREVIOEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn AHBGCMERREN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHBGCMERREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for INTEN {
        #[inline(always)]
        fn default() -> INTEN {
            INTEN(0)
        }
    }
    impl core::fmt::Debug for INTEN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INTEN")
                .field("IPCMDDONEEN", &self.IPCMDDONEEN())
                .field("IPCMDGEEN", &self.IPCMDGEEN())
                .field("AHBCMDGEEN", &self.AHBCMDGEEN())
                .field("IPCMDERREN", &self.IPCMDERREN())
                .field("AHBCMDERREN", &self.AHBCMDERREN())
                .field("IPRXWAEN", &self.IPRXWAEN())
                .field("IPTXWEEN", &self.IPTXWEEN())
                .field("DATALEARNFAILEN", &self.DATALEARNFAILEN())
                .field("SCKSTOPBYRDEN", &self.SCKSTOPBYRDEN())
                .field("SCKSTOPBYWREN", &self.SCKSTOPBYWREN())
                .field("AHBBUSTIMEOUTEN", &self.AHBBUSTIMEOUTEN())
                .field("SEQTIMEOUTEN", &self.SEQTIMEOUTEN())
                .field("IPCMDSECUREVIOEN", &self.IPCMDSECUREVIOEN())
                .field("AHBGCMERREN", &self.AHBGCMERREN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INTEN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "INTEN {{ IPCMDDONEEN: {=bool:?}, IPCMDGEEN: {=bool:?}, AHBCMDGEEN: {=bool:?}, IPCMDERREN: {=bool:?}, AHBCMDERREN: {=bool:?}, IPRXWAEN: {=bool:?}, IPTXWEEN: {=bool:?}, DATALEARNFAILEN: {=bool:?}, SCKSTOPBYRDEN: {=bool:?}, SCKSTOPBYWREN: {=bool:?}, AHBBUSTIMEOUTEN: {=bool:?}, SEQTIMEOUTEN: {=bool:?}, IPCMDSECUREVIOEN: {=bool:?}, AHBGCMERREN: {=bool:?} }}" , self . IPCMDDONEEN () , self . IPCMDGEEN () , self . AHBCMDGEEN () , self . IPCMDERREN () , self . AHBCMDERREN () , self . IPRXWAEN () , self . IPTXWEEN () , self . DATALEARNFAILEN () , self . SCKSTOPBYRDEN () , self . SCKSTOPBYWREN () , self . AHBBUSTIMEOUTEN () , self . SEQTIMEOUTEN () , self . IPCMDSECUREVIOEN () , self . AHBGCMERREN ())
        }
    }
    #[doc = "Interrupt"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INTR(pub u32);
    impl INTR {
        #[inline(always)]
        pub const fn IPCMDDONE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPCMDDONE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn IPCMDGE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPCMDGE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn AHBCMDGE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHBCMDGE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn IPCMDERR(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPCMDERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn AHBCMDERR(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHBCMDERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn IPRXWA(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPRXWA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[inline(always)]
        pub const fn IPTXWE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPTXWE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn DATALEARNFAIL(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DATALEARNFAIL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn SCKSTOPBYRD(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCKSTOPBYRD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn SCKSTOPBYWR(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCKSTOPBYWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn AHBBUSTIMEOUT(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHBBUSTIMEOUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[inline(always)]
        pub const fn SEQTIMEOUT(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEQTIMEOUT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn IPCMDSECUREVIO(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPCMDSECUREVIO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn AHBGCMERR(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHBGCMERR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for INTR {
        #[inline(always)]
        fn default() -> INTR {
            INTR(0)
        }
    }
    impl core::fmt::Debug for INTR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INTR")
                .field("IPCMDDONE", &self.IPCMDDONE())
                .field("IPCMDGE", &self.IPCMDGE())
                .field("AHBCMDGE", &self.AHBCMDGE())
                .field("IPCMDERR", &self.IPCMDERR())
                .field("AHBCMDERR", &self.AHBCMDERR())
                .field("IPRXWA", &self.IPRXWA())
                .field("IPTXWE", &self.IPTXWE())
                .field("DATALEARNFAIL", &self.DATALEARNFAIL())
                .field("SCKSTOPBYRD", &self.SCKSTOPBYRD())
                .field("SCKSTOPBYWR", &self.SCKSTOPBYWR())
                .field("AHBBUSTIMEOUT", &self.AHBBUSTIMEOUT())
                .field("SEQTIMEOUT", &self.SEQTIMEOUT())
                .field("IPCMDSECUREVIO", &self.IPCMDSECUREVIO())
                .field("AHBGCMERR", &self.AHBGCMERR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INTR {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "INTR {{ IPCMDDONE: {=bool:?}, IPCMDGE: {=bool:?}, AHBCMDGE: {=bool:?}, IPCMDERR: {=bool:?}, AHBCMDERR: {=bool:?}, IPRXWA: {=bool:?}, IPTXWE: {=bool:?}, DATALEARNFAIL: {=bool:?}, SCKSTOPBYRD: {=bool:?}, SCKSTOPBYWR: {=bool:?}, AHBBUSTIMEOUT: {=bool:?}, SEQTIMEOUT: {=bool:?}, IPCMDSECUREVIO: {=bool:?}, AHBGCMERR: {=bool:?} }}" , self . IPCMDDONE () , self . IPCMDGE () , self . AHBCMDGE () , self . IPCMDERR () , self . AHBCMDERR () , self . IPRXWA () , self . IPTXWE () , self . DATALEARNFAIL () , self . SCKSTOPBYRD () , self . SCKSTOPBYWR () , self . AHBBUSTIMEOUT () , self . SEQTIMEOUT () , self . IPCMDSECUREVIO () , self . AHBGCMERR ())
        }
    }
    #[doc = "IP Command"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPCMD(pub u32);
    impl IPCMD {
        #[inline(always)]
        pub const fn TRG(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TRG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for IPCMD {
        #[inline(always)]
        fn default() -> IPCMD {
            IPCMD(0)
        }
    }
    impl core::fmt::Debug for IPCMD {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPCMD").field("TRG", &self.TRG()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPCMD {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IPCMD {{ TRG: {=bool:?} }}", self.TRG())
        }
    }
    #[doc = "IP Control 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPCR1(pub u32);
    impl IPCR1 {
        #[inline(always)]
        pub const fn IDATSZ(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_IDATSZ(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn ISEQID(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ISEQID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn ISEQNUM(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_ISEQNUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[inline(always)]
        pub const fn IPAREN(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPAREN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for IPCR1 {
        #[inline(always)]
        fn default() -> IPCR1 {
            IPCR1(0)
        }
    }
    impl core::fmt::Debug for IPCR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPCR1")
                .field("IDATSZ", &self.IDATSZ())
                .field("ISEQID", &self.ISEQID())
                .field("ISEQNUM", &self.ISEQNUM())
                .field("IPAREN", &self.IPAREN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPCR1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IPCR1 {{ IDATSZ: {=u16:?}, ISEQID: {=u8:?}, ISEQNUM: {=u8:?}, IPAREN: {=bool:?} }}" , self . IDATSZ () , self . ISEQID () , self . ISEQNUM () , self . IPAREN ())
        }
    }
    #[doc = "IP Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPCR2(pub u32);
    impl IPCR2 {
        #[inline(always)]
        pub const fn IPBLKAHBREQ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPBLKAHBREQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn IPBLKAHBACK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPBLKAHBACK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn IPBLKALLAHB(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPBLKALLAHB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for IPCR2 {
        #[inline(always)]
        fn default() -> IPCR2 {
            IPCR2(0)
        }
    }
    impl core::fmt::Debug for IPCR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPCR2")
                .field("IPBLKAHBREQ", &self.IPBLKAHBREQ())
                .field("IPBLKAHBACK", &self.IPBLKAHBACK())
                .field("IPBLKALLAHB", &self.IPBLKALLAHB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPCR2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IPCR2 {{ IPBLKAHBREQ: {=bool:?}, IPBLKAHBACK: {=bool:?}, IPBLKALLAHB: {=bool:?} }}" , self . IPBLKAHBREQ () , self . IPBLKAHBACK () , self . IPBLKALLAHB ())
        }
    }
    #[doc = "IPED Function Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTRL(pub u32);
    impl IPEDCTRL {
        #[inline(always)]
        pub const fn CONFIG(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CONFIG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn IPED_EN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPED_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn IPWR_EN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPWR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[inline(always)]
        pub const fn AHBWR_EN(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHBWR_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[inline(always)]
        pub const fn AHBRD_EN(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHBRD_EN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[inline(always)]
        pub const fn IPGCMWR(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPGCMWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn AHGCMWR(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHGCMWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[inline(always)]
        pub const fn AHBGCMRD(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AHBGCMRD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[inline(always)]
        pub const fn IPED_PROTECT(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPED_PROTECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[inline(always)]
        pub const fn IPED_SWRESET(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_IPED_SWRESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for IPEDCTRL {
        #[inline(always)]
        fn default() -> IPEDCTRL {
            IPEDCTRL(0)
        }
    }
    impl core::fmt::Debug for IPEDCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTRL")
                .field("CONFIG", &self.CONFIG())
                .field("IPED_EN", &self.IPED_EN())
                .field("IPWR_EN", &self.IPWR_EN())
                .field("AHBWR_EN", &self.AHBWR_EN())
                .field("AHBRD_EN", &self.AHBRD_EN())
                .field("IPGCMWR", &self.IPGCMWR())
                .field("AHGCMWR", &self.AHGCMWR())
                .field("AHBGCMRD", &self.AHBGCMRD())
                .field("IPED_PROTECT", &self.IPED_PROTECT())
                .field("IPED_SWRESET", &self.IPED_SWRESET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IPEDCTRL {{ CONFIG: {=bool:?}, IPED_EN: {=bool:?}, IPWR_EN: {=bool:?}, AHBWR_EN: {=bool:?}, AHBRD_EN: {=bool:?}, IPGCMWR: {=bool:?}, AHGCMWR: {=bool:?}, AHBGCMRD: {=bool:?}, IPED_PROTECT: {=bool:?}, IPED_SWRESET: {=bool:?} }}" , self . CONFIG () , self . IPED_EN () , self . IPWR_EN () , self . AHBWR_EN () , self . AHBRD_EN () , self . IPGCMWR () , self . AHGCMWR () , self . AHBGCMRD () , self . IPED_PROTECT () , self . IPED_SWRESET ())
        }
    }
    #[doc = "End Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX0END(pub u32);
    impl IPEDCTX0END {
        #[inline(always)]
        pub const fn end_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_end_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX0END {
        #[inline(always)]
        fn default() -> IPEDCTX0END {
            IPEDCTX0END(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX0END {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX0END")
                .field("end_address", &self.end_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX0END {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPEDCTX0END {{ end_address: {=u32:?} }}",
                self.end_address()
            )
        }
    }
    #[doc = "Start Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX0START(pub u32);
    impl IPEDCTX0START {
        #[inline(always)]
        pub const fn GCM(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ahbbuserror_dis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ahbbuserror_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn start_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_start_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX0START {
        #[inline(always)]
        fn default() -> IPEDCTX0START {
            IPEDCTX0START(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX0START {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX0START")
                .field("GCM", &self.GCM())
                .field("ahbbuserror_dis", &self.ahbbuserror_dis())
                .field("start_address", &self.start_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX0START {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IPEDCTX0START {{ GCM: {=bool:?}, ahbbuserror_dis: {=bool:?}, start_address: {=u32:?} }}" , self . GCM () , self . ahbbuserror_dis () , self . start_address ())
        }
    }
    #[doc = "End Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX1END(pub u32);
    impl IPEDCTX1END {
        #[inline(always)]
        pub const fn end_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_end_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX1END {
        #[inline(always)]
        fn default() -> IPEDCTX1END {
            IPEDCTX1END(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX1END {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX1END")
                .field("end_address", &self.end_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX1END {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPEDCTX1END {{ end_address: {=u32:?} }}",
                self.end_address()
            )
        }
    }
    #[doc = "Start Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX1START(pub u32);
    impl IPEDCTX1START {
        #[inline(always)]
        pub const fn GCM(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ahbbuserror_dis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ahbbuserror_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn start_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_start_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX1START {
        #[inline(always)]
        fn default() -> IPEDCTX1START {
            IPEDCTX1START(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX1START {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX1START")
                .field("GCM", &self.GCM())
                .field("ahbbuserror_dis", &self.ahbbuserror_dis())
                .field("start_address", &self.start_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX1START {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IPEDCTX1START {{ GCM: {=bool:?}, ahbbuserror_dis: {=bool:?}, start_address: {=u32:?} }}" , self . GCM () , self . ahbbuserror_dis () , self . start_address ())
        }
    }
    #[doc = "End Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX2END(pub u32);
    impl IPEDCTX2END {
        #[inline(always)]
        pub const fn end_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_end_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX2END {
        #[inline(always)]
        fn default() -> IPEDCTX2END {
            IPEDCTX2END(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX2END {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX2END")
                .field("end_address", &self.end_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX2END {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPEDCTX2END {{ end_address: {=u32:?} }}",
                self.end_address()
            )
        }
    }
    #[doc = "Start Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX2START(pub u32);
    impl IPEDCTX2START {
        #[inline(always)]
        pub const fn GCM(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ahbbuserror_dis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ahbbuserror_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn start_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_start_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX2START {
        #[inline(always)]
        fn default() -> IPEDCTX2START {
            IPEDCTX2START(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX2START {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX2START")
                .field("GCM", &self.GCM())
                .field("ahbbuserror_dis", &self.ahbbuserror_dis())
                .field("start_address", &self.start_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX2START {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IPEDCTX2START {{ GCM: {=bool:?}, ahbbuserror_dis: {=bool:?}, start_address: {=u32:?} }}" , self . GCM () , self . ahbbuserror_dis () , self . start_address ())
        }
    }
    #[doc = "End Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX3END(pub u32);
    impl IPEDCTX3END {
        #[inline(always)]
        pub const fn end_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_end_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX3END {
        #[inline(always)]
        fn default() -> IPEDCTX3END {
            IPEDCTX3END(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX3END {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX3END")
                .field("end_address", &self.end_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX3END {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPEDCTX3END {{ end_address: {=u32:?} }}",
                self.end_address()
            )
        }
    }
    #[doc = "Start Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX3START(pub u32);
    impl IPEDCTX3START {
        #[inline(always)]
        pub const fn GCM(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ahbbuserror_dis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ahbbuserror_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn start_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_start_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX3START {
        #[inline(always)]
        fn default() -> IPEDCTX3START {
            IPEDCTX3START(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX3START {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX3START")
                .field("GCM", &self.GCM())
                .field("ahbbuserror_dis", &self.ahbbuserror_dis())
                .field("start_address", &self.start_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX3START {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IPEDCTX3START {{ GCM: {=bool:?}, ahbbuserror_dis: {=bool:?}, start_address: {=u32:?} }}" , self . GCM () , self . ahbbuserror_dis () , self . start_address ())
        }
    }
    #[doc = "End Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX4END(pub u32);
    impl IPEDCTX4END {
        #[inline(always)]
        pub const fn end_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_end_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX4END {
        #[inline(always)]
        fn default() -> IPEDCTX4END {
            IPEDCTX4END(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX4END {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX4END")
                .field("end_address", &self.end_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX4END {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPEDCTX4END {{ end_address: {=u32:?} }}",
                self.end_address()
            )
        }
    }
    #[doc = "Start Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX4START(pub u32);
    impl IPEDCTX4START {
        #[inline(always)]
        pub const fn GCM(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ahbbuserror_dis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ahbbuserror_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn start_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_start_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX4START {
        #[inline(always)]
        fn default() -> IPEDCTX4START {
            IPEDCTX4START(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX4START {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX4START")
                .field("GCM", &self.GCM())
                .field("ahbbuserror_dis", &self.ahbbuserror_dis())
                .field("start_address", &self.start_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX4START {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IPEDCTX4START {{ GCM: {=bool:?}, ahbbuserror_dis: {=bool:?}, start_address: {=u32:?} }}" , self . GCM () , self . ahbbuserror_dis () , self . start_address ())
        }
    }
    #[doc = "End Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX5END(pub u32);
    impl IPEDCTX5END {
        #[inline(always)]
        pub const fn end_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_end_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX5END {
        #[inline(always)]
        fn default() -> IPEDCTX5END {
            IPEDCTX5END(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX5END {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX5END")
                .field("end_address", &self.end_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX5END {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPEDCTX5END {{ end_address: {=u32:?} }}",
                self.end_address()
            )
        }
    }
    #[doc = "Start Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX5START(pub u32);
    impl IPEDCTX5START {
        #[inline(always)]
        pub const fn GCM(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ahbbuserror_dis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ahbbuserror_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn start_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_start_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX5START {
        #[inline(always)]
        fn default() -> IPEDCTX5START {
            IPEDCTX5START(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX5START {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX5START")
                .field("GCM", &self.GCM())
                .field("ahbbuserror_dis", &self.ahbbuserror_dis())
                .field("start_address", &self.start_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX5START {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IPEDCTX5START {{ GCM: {=bool:?}, ahbbuserror_dis: {=bool:?}, start_address: {=u32:?} }}" , self . GCM () , self . ahbbuserror_dis () , self . start_address ())
        }
    }
    #[doc = "End Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX6END(pub u32);
    impl IPEDCTX6END {
        #[inline(always)]
        pub const fn end_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_end_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX6END {
        #[inline(always)]
        fn default() -> IPEDCTX6END {
            IPEDCTX6END(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX6END {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX6END")
                .field("end_address", &self.end_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX6END {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPEDCTX6END {{ end_address: {=u32:?} }}",
                self.end_address()
            )
        }
    }
    #[doc = "Start Address of Region"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX6START(pub u32);
    impl IPEDCTX6START {
        #[inline(always)]
        pub const fn GCM(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_GCM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ahbbuserror_dis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ahbbuserror_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn start_address(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_start_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for IPEDCTX6START {
        #[inline(always)]
        fn default() -> IPEDCTX6START {
            IPEDCTX6START(0)
        }
    }
    impl core::fmt::Debug for IPEDCTX6START {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTX6START")
                .field("GCM", &self.GCM())
                .field("ahbbuserror_dis", &self.ahbbuserror_dis())
                .field("start_address", &self.start_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTX6START {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IPEDCTX6START {{ GCM: {=bool:?}, ahbbuserror_dis: {=bool:?}, start_address: {=u32:?} }}" , self . GCM () , self . ahbbuserror_dis () , self . start_address ())
        }
    }
    #[doc = "IPED context control 0..IPED context control 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTXCTRL(pub u32);
    impl IPEDCTXCTRL {
        #[inline(always)]
        pub const fn CTX0_FREEZE0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX0_FREEZE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn CTX0_FREEZE1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX0_FREEZE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[inline(always)]
        pub const fn CTX1_FREEZE0(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX1_FREEZE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn CTX1_FREEZE1(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX1_FREEZE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn CTX2_FREEZE0(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX2_FREEZE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn CTX2_FREEZE1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX2_FREEZE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn CTX3_FREEZE0(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX3_FREEZE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn CTX3_FREEZE1(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX3_FREEZE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[inline(always)]
        pub const fn CTX4_FREEZE0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX4_FREEZE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn CTX4_FREEZE1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX4_FREEZE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn CTX5_FREEZE0(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX5_FREEZE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn CTX5_FREEZE1(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX5_FREEZE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[inline(always)]
        pub const fn CTX6_FREEZE0(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX6_FREEZE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[inline(always)]
        pub const fn CTX6_FREEZE1(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_CTX6_FREEZE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
    }
    impl Default for IPEDCTXCTRL {
        #[inline(always)]
        fn default() -> IPEDCTXCTRL {
            IPEDCTXCTRL(0)
        }
    }
    impl core::fmt::Debug for IPEDCTXCTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPEDCTXCTRL")
                .field("CTX0_FREEZE0", &self.CTX0_FREEZE0())
                .field("CTX0_FREEZE1", &self.CTX0_FREEZE1())
                .field("CTX1_FREEZE0", &self.CTX1_FREEZE0())
                .field("CTX1_FREEZE1", &self.CTX1_FREEZE1())
                .field("CTX2_FREEZE0", &self.CTX2_FREEZE0())
                .field("CTX2_FREEZE1", &self.CTX2_FREEZE1())
                .field("CTX3_FREEZE0", &self.CTX3_FREEZE0())
                .field("CTX3_FREEZE1", &self.CTX3_FREEZE1())
                .field("CTX4_FREEZE0", &self.CTX4_FREEZE0())
                .field("CTX4_FREEZE1", &self.CTX4_FREEZE1())
                .field("CTX5_FREEZE0", &self.CTX5_FREEZE0())
                .field("CTX5_FREEZE1", &self.CTX5_FREEZE1())
                .field("CTX6_FREEZE0", &self.CTX6_FREEZE0())
                .field("CTX6_FREEZE1", &self.CTX6_FREEZE1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPEDCTXCTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "IPEDCTXCTRL {{ CTX0_FREEZE0: {=u8:?}, CTX0_FREEZE1: {=u8:?}, CTX1_FREEZE0: {=u8:?}, CTX1_FREEZE1: {=u8:?}, CTX2_FREEZE0: {=u8:?}, CTX2_FREEZE1: {=u8:?}, CTX3_FREEZE0: {=u8:?}, CTX3_FREEZE1: {=u8:?}, CTX4_FREEZE0: {=u8:?}, CTX4_FREEZE1: {=u8:?}, CTX5_FREEZE0: {=u8:?}, CTX5_FREEZE1: {=u8:?}, CTX6_FREEZE0: {=u8:?}, CTX6_FREEZE1: {=u8:?} }}" , self . CTX0_FREEZE0 () , self . CTX0_FREEZE1 () , self . CTX1_FREEZE0 () , self . CTX1_FREEZE1 () , self . CTX2_FREEZE0 () , self . CTX2_FREEZE1 () , self . CTX3_FREEZE0 () , self . CTX3_FREEZE1 () , self . CTX4_FREEZE0 () , self . CTX4_FREEZE1 () , self . CTX5_FREEZE0 () , self . CTX5_FREEZE1 () , self . CTX6_FREEZE0 () , self . CTX6_FREEZE1 ())
        }
    }
    #[doc = "IP Receive FIFO Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPRXFCR(pub u32);
    impl IPRXFCR {
        #[inline(always)]
        pub const fn CLRIPRXF(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLRIPRXF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn RXDMAEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RXDMAEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RXWMRK(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXWMRK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
        }
    }
    impl Default for IPRXFCR {
        #[inline(always)]
        fn default() -> IPRXFCR {
            IPRXFCR(0)
        }
    }
    impl core::fmt::Debug for IPRXFCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPRXFCR")
                .field("CLRIPRXF", &self.CLRIPRXF())
                .field("RXDMAEN", &self.RXDMAEN())
                .field("RXWMRK", &self.RXWMRK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPRXFCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPRXFCR {{ CLRIPRXF: {=bool:?}, RXDMAEN: {=bool:?}, RXWMRK: {=u8:?} }}",
                self.CLRIPRXF(),
                self.RXDMAEN(),
                self.RXWMRK()
            )
        }
    }
    #[doc = "IP Receive FIFO Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPRXFSTS(pub u32);
    impl IPRXFSTS {
        #[inline(always)]
        pub const fn FILL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn RDCNTR(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_RDCNTR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for IPRXFSTS {
        #[inline(always)]
        fn default() -> IPRXFSTS {
            IPRXFSTS(0)
        }
    }
    impl core::fmt::Debug for IPRXFSTS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPRXFSTS")
                .field("FILL", &self.FILL())
                .field("RDCNTR", &self.RDCNTR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPRXFSTS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPRXFSTS {{ FILL: {=u8:?}, RDCNTR: {=u16:?} }}",
                self.FILL(),
                self.RDCNTR()
            )
        }
    }
    #[doc = "IPS Nonsecure Region 0 End Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPSNSZEND0(pub u32);
    impl IPSNSZEND0 {
        #[inline(always)]
        pub const fn end_address(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_end_address(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for IPSNSZEND0 {
        #[inline(always)]
        fn default() -> IPSNSZEND0 {
            IPSNSZEND0(0)
        }
    }
    impl core::fmt::Debug for IPSNSZEND0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPSNSZEND0")
                .field("end_address", &self.end_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPSNSZEND0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPSNSZEND0 {{ end_address: {=u32:?} }}",
                self.end_address()
            )
        }
    }
    #[doc = "IPS Nonsecure Region 1 End Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPSNSZEND1(pub u32);
    impl IPSNSZEND1 {
        #[inline(always)]
        pub const fn end_address(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_end_address(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for IPSNSZEND1 {
        #[inline(always)]
        fn default() -> IPSNSZEND1 {
            IPSNSZEND1(0)
        }
    }
    impl core::fmt::Debug for IPSNSZEND1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPSNSZEND1")
                .field("end_address", &self.end_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPSNSZEND1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPSNSZEND1 {{ end_address: {=u32:?} }}",
                self.end_address()
            )
        }
    }
    #[doc = "IPS Nonsecure Region 0 Start Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPSNSZSTART0(pub u32);
    impl IPSNSZSTART0 {
        #[inline(always)]
        pub const fn start_address(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_start_address(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for IPSNSZSTART0 {
        #[inline(always)]
        fn default() -> IPSNSZSTART0 {
            IPSNSZSTART0(0)
        }
    }
    impl core::fmt::Debug for IPSNSZSTART0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPSNSZSTART0")
                .field("start_address", &self.start_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPSNSZSTART0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPSNSZSTART0 {{ start_address: {=u32:?} }}",
                self.start_address()
            )
        }
    }
    #[doc = "IPS Nonsecure Region 1 Start Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPSNSZSTART1(pub u32);
    impl IPSNSZSTART1 {
        #[inline(always)]
        pub const fn start_address(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_start_address(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for IPSNSZSTART1 {
        #[inline(always)]
        fn default() -> IPSNSZSTART1 {
            IPSNSZSTART1(0)
        }
    }
    impl core::fmt::Debug for IPSNSZSTART1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPSNSZSTART1")
                .field("start_address", &self.start_address())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPSNSZSTART1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPSNSZSTART1 {{ start_address: {=u32:?} }}",
                self.start_address()
            )
        }
    }
    #[doc = "IP Transmit FIFO Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPTXFCR(pub u32);
    impl IPTXFCR {
        #[inline(always)]
        pub const fn CLRIPTXF(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLRIPTXF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn TXDMAEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_TXDMAEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn TXWMRK(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x7f;
            val as u8
        }
        #[inline(always)]
        pub fn set_TXWMRK(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
        }
    }
    impl Default for IPTXFCR {
        #[inline(always)]
        fn default() -> IPTXFCR {
            IPTXFCR(0)
        }
    }
    impl core::fmt::Debug for IPTXFCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPTXFCR")
                .field("CLRIPTXF", &self.CLRIPTXF())
                .field("TXDMAEN", &self.TXDMAEN())
                .field("TXWMRK", &self.TXWMRK())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPTXFCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPTXFCR {{ CLRIPTXF: {=bool:?}, TXDMAEN: {=bool:?}, TXWMRK: {=u8:?} }}",
                self.CLRIPTXF(),
                self.TXDMAEN(),
                self.TXWMRK()
            )
        }
    }
    #[doc = "IP Transmit FIFO Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPTXFSTS(pub u32);
    impl IPTXFSTS {
        #[inline(always)]
        pub const fn FILL(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_FILL(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn WRCNTR(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_WRCNTR(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for IPTXFSTS {
        #[inline(always)]
        fn default() -> IPTXFSTS {
            IPTXFSTS(0)
        }
    }
    impl core::fmt::Debug for IPTXFSTS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IPTXFSTS")
                .field("FILL", &self.FILL())
                .field("WRCNTR", &self.WRCNTR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IPTXFSTS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IPTXFSTS {{ FILL: {=u8:?}, WRCNTR: {=u16:?} }}",
                self.FILL(),
                self.WRCNTR()
            )
        }
    }
    #[doc = "Lookup Table 0..Lookup Table 63"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LUT(pub u32);
    impl LUT {
        #[inline(always)]
        pub const fn OPERAND0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_OPERAND0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[inline(always)]
        pub const fn NUM_PADS0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_NUM_PADS0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[inline(always)]
        pub const fn OPCODE0(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_OPCODE0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
        }
        #[inline(always)]
        pub const fn OPERAND1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_OPERAND1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn NUM_PADS1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_NUM_PADS1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[inline(always)]
        pub const fn OPCODE1(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_OPCODE1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
        }
    }
    impl Default for LUT {
        #[inline(always)]
        fn default() -> LUT {
            LUT(0)
        }
    }
    impl core::fmt::Debug for LUT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LUT")
                .field("OPERAND0", &self.OPERAND0())
                .field("NUM_PADS0", &self.NUM_PADS0())
                .field("OPCODE0", &self.OPCODE0())
                .field("OPERAND1", &self.OPERAND1())
                .field("NUM_PADS1", &self.NUM_PADS1())
                .field("OPCODE1", &self.OPCODE1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LUT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "LUT {{ OPERAND0: {=u8:?}, NUM_PADS0: {=u8:?}, OPCODE0: {=u8:?}, OPERAND1: {=u8:?}, NUM_PADS1: {=u8:?}, OPCODE1: {=u8:?} }}" , self . OPERAND0 () , self . NUM_PADS0 () , self . OPCODE0 () , self . OPERAND1 () , self . NUM_PADS1 () , self . OPCODE1 ())
        }
    }
    #[doc = "LUT Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LUTCR(pub u32);
    impl LUTCR {
        #[inline(always)]
        pub const fn LOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn UNLOCK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_UNLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn PROTECT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_PROTECT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for LUTCR {
        #[inline(always)]
        fn default() -> LUTCR {
            LUTCR(0)
        }
    }
    impl core::fmt::Debug for LUTCR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LUTCR")
                .field("LOCK", &self.LOCK())
                .field("UNLOCK", &self.UNLOCK())
                .field("PROTECT", &self.PROTECT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for LUTCR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "LUTCR {{ LOCK: {=bool:?}, UNLOCK: {=bool:?}, PROTECT: {=bool:?} }}",
                self.LOCK(),
                self.UNLOCK(),
                self.PROTECT()
            )
        }
    }
    #[doc = "Module Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCR0(pub u32);
    impl MCR0 {
        #[inline(always)]
        pub const fn SWRESET(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SWRESET(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn MDIS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_MDIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn RXCLKSRC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXCLKSRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[inline(always)]
        pub const fn SERCLKDIV(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub fn set_SERCLKDIV(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[inline(always)]
        pub const fn HSEN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_HSEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn DOZEEN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_DOZEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[inline(always)]
        pub const fn COMBINATIONEN(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_COMBINATIONEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[inline(always)]
        pub const fn SCKFREERUNEN(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCKFREERUNEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn LEARNEN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_LEARNEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn IPGRANTWAIT(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_IPGRANTWAIT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[inline(always)]
        pub const fn AHBGRANTWAIT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_AHBGRANTWAIT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for MCR0 {
        #[inline(always)]
        fn default() -> MCR0 {
            MCR0(0)
        }
    }
    impl core::fmt::Debug for MCR0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCR0")
                .field("SWRESET", &self.SWRESET())
                .field("MDIS", &self.MDIS())
                .field("RXCLKSRC", &self.RXCLKSRC())
                .field("SERCLKDIV", &self.SERCLKDIV())
                .field("HSEN", &self.HSEN())
                .field("DOZEEN", &self.DOZEEN())
                .field("COMBINATIONEN", &self.COMBINATIONEN())
                .field("SCKFREERUNEN", &self.SCKFREERUNEN())
                .field("LEARNEN", &self.LEARNEN())
                .field("IPGRANTWAIT", &self.IPGRANTWAIT())
                .field("AHBGRANTWAIT", &self.AHBGRANTWAIT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCR0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MCR0 {{ SWRESET: {=bool:?}, MDIS: {=bool:?}, RXCLKSRC: {=u8:?}, SERCLKDIV: {=u8:?}, HSEN: {=bool:?}, DOZEEN: {=bool:?}, COMBINATIONEN: {=bool:?}, SCKFREERUNEN: {=bool:?}, LEARNEN: {=bool:?}, IPGRANTWAIT: {=u8:?}, AHBGRANTWAIT: {=u8:?} }}" , self . SWRESET () , self . MDIS () , self . RXCLKSRC () , self . SERCLKDIV () , self . HSEN () , self . DOZEEN () , self . COMBINATIONEN () , self . SCKFREERUNEN () , self . LEARNEN () , self . IPGRANTWAIT () , self . AHBGRANTWAIT ())
        }
    }
    #[doc = "Module Control 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCR1(pub u32);
    impl MCR1 {
        #[inline(always)]
        pub const fn AHBBUSWAIT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_AHBBUSWAIT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[inline(always)]
        pub const fn SEQWAIT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub fn set_SEQWAIT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MCR1 {
        #[inline(always)]
        fn default() -> MCR1 {
            MCR1(0)
        }
    }
    impl core::fmt::Debug for MCR1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCR1")
                .field("AHBBUSWAIT", &self.AHBBUSWAIT())
                .field("SEQWAIT", &self.SEQWAIT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCR1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MCR1 {{ AHBBUSWAIT: {=u16:?}, SEQWAIT: {=u16:?} }}",
                self.AHBBUSWAIT(),
                self.SEQWAIT()
            )
        }
    }
    #[doc = "Module Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MCR2(pub u32);
    impl MCR2 {
        #[inline(always)]
        pub const fn CLRAHBBUFOPT(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLRAHBBUFOPT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[inline(always)]
        pub const fn CLRLEARNPHASE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_CLRLEARNPHASE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[inline(always)]
        pub const fn SAMEDEVICEEN(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SAMEDEVICEEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[inline(always)]
        pub const fn SCKBDIFFOPT(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SCKBDIFFOPT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[inline(always)]
        pub const fn RXCLKSRC_B(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_RXCLKSRC_B(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[inline(always)]
        pub const fn RX_CLK_SRC_DIFF(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_RX_CLK_SRC_DIFF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[inline(always)]
        pub const fn RESUMEWAIT(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub fn set_RESUMEWAIT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for MCR2 {
        #[inline(always)]
        fn default() -> MCR2 {
            MCR2(0)
        }
    }
    impl core::fmt::Debug for MCR2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MCR2")
                .field("CLRAHBBUFOPT", &self.CLRAHBBUFOPT())
                .field("CLRLEARNPHASE", &self.CLRLEARNPHASE())
                .field("SAMEDEVICEEN", &self.SAMEDEVICEEN())
                .field("SCKBDIFFOPT", &self.SCKBDIFFOPT())
                .field("RXCLKSRC_B", &self.RXCLKSRC_B())
                .field("RX_CLK_SRC_DIFF", &self.RX_CLK_SRC_DIFF())
                .field("RESUMEWAIT", &self.RESUMEWAIT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MCR2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MCR2 {{ CLRAHBBUFOPT: {=bool:?}, CLRLEARNPHASE: {=bool:?}, SAMEDEVICEEN: {=bool:?}, SCKBDIFFOPT: {=bool:?}, RXCLKSRC_B: {=u8:?}, RX_CLK_SRC_DIFF: {=bool:?}, RESUMEWAIT: {=u8:?} }}" , self . CLRAHBBUFOPT () , self . CLRLEARNPHASE () , self . SAMEDEVICEEN () , self . SCKBDIFFOPT () , self . RXCLKSRC_B () , self . RX_CLK_SRC_DIFF () , self . RESUMEWAIT ())
        }
    }
    #[doc = "Status 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STS0(pub u32);
    impl STS0 {
        #[inline(always)]
        pub const fn SEQIDLE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_SEQIDLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn ARBIDLE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ARBIDLE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ARBCMDSRC(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub fn set_ARBCMDSRC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[inline(always)]
        pub const fn DATALEARNPHASEA(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATALEARNPHASEA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[inline(always)]
        pub const fn DATALEARNPHASEB(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_DATALEARNPHASEB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for STS0 {
        #[inline(always)]
        fn default() -> STS0 {
            STS0(0)
        }
    }
    impl core::fmt::Debug for STS0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STS0")
                .field("SEQIDLE", &self.SEQIDLE())
                .field("ARBIDLE", &self.ARBIDLE())
                .field("ARBCMDSRC", &self.ARBCMDSRC())
                .field("DATALEARNPHASEA", &self.DATALEARNPHASEA())
                .field("DATALEARNPHASEB", &self.DATALEARNPHASEB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STS0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "STS0 {{ SEQIDLE: {=bool:?}, ARBIDLE: {=bool:?}, ARBCMDSRC: {=u8:?}, DATALEARNPHASEA: {=u8:?}, DATALEARNPHASEB: {=u8:?} }}" , self . SEQIDLE () , self . ARBIDLE () , self . ARBCMDSRC () , self . DATALEARNPHASEA () , self . DATALEARNPHASEB ())
        }
    }
    #[doc = "Status 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STS1(pub u32);
    impl STS1 {
        #[inline(always)]
        pub const fn AHBCMDERRID(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_AHBCMDERRID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[inline(always)]
        pub const fn AHBCMDERRCODE(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_AHBCMDERRCODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[inline(always)]
        pub const fn IPCMDERRID(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_IPCMDERRID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[inline(always)]
        pub const fn IPCMDERRCODE(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub fn set_IPCMDERRCODE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for STS1 {
        #[inline(always)]
        fn default() -> STS1 {
            STS1(0)
        }
    }
    impl core::fmt::Debug for STS1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STS1")
                .field("AHBCMDERRID", &self.AHBCMDERRID())
                .field("AHBCMDERRCODE", &self.AHBCMDERRCODE())
                .field("IPCMDERRID", &self.IPCMDERRID())
                .field("IPCMDERRCODE", &self.IPCMDERRCODE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STS1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "STS1 {{ AHBCMDERRID: {=u8:?}, AHBCMDERRCODE: {=u8:?}, IPCMDERRID: {=u8:?}, IPCMDERRCODE: {=u8:?} }}" , self . AHBCMDERRID () , self . AHBCMDERRCODE () , self . IPCMDERRID () , self . IPCMDERRCODE ())
        }
    }
    #[doc = "Status 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct STS2(pub u32);
    impl STS2 {
        #[inline(always)]
        pub const fn ASLVLOCK(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ASLVLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[inline(always)]
        pub const fn AREFLOCK(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_AREFLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[inline(always)]
        pub const fn ASLVSEL(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_ASLVSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
        }
        #[inline(always)]
        pub const fn AREFSEL(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_AREFSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[inline(always)]
        pub const fn BSLVLOCK(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BSLVLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[inline(always)]
        pub const fn BREFLOCK(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_BREFLOCK(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[inline(always)]
        pub const fn BSLVSEL(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_BSLVSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
        }
        #[inline(always)]
        pub const fn BREFSEL(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub fn set_BREFSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for STS2 {
        #[inline(always)]
        fn default() -> STS2 {
            STS2(0)
        }
    }
    impl core::fmt::Debug for STS2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("STS2")
                .field("ASLVLOCK", &self.ASLVLOCK())
                .field("AREFLOCK", &self.AREFLOCK())
                .field("ASLVSEL", &self.ASLVSEL())
                .field("AREFSEL", &self.AREFSEL())
                .field("BSLVLOCK", &self.BSLVLOCK())
                .field("BREFLOCK", &self.BREFLOCK())
                .field("BSLVSEL", &self.BSLVSEL())
                .field("BREFSEL", &self.BREFSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for STS2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "STS2 {{ ASLVLOCK: {=bool:?}, AREFLOCK: {=bool:?}, ASLVSEL: {=u8:?}, AREFSEL: {=u8:?}, BSLVLOCK: {=bool:?}, BREFLOCK: {=bool:?}, BSLVSEL: {=u8:?}, BREFSEL: {=u8:?} }}" , self . ASLVLOCK () , self . AREFLOCK () , self . ASLVSEL () , self . AREFSEL () , self . BSLVLOCK () , self . BREFLOCK () , self . BSLVSEL () , self . BREFSEL ())
        }
    }
}
