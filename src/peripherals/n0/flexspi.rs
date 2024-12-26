#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
    pub const fn LUTKEY(self) -> crate::common::Reg<regs::LUTKEY, crate::common::RW> {
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
    pub const fn IPCR0(self) -> crate::common::Reg<regs::IPCR0, crate::common::RW> {
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
    pub const fn DLPR(self) -> crate::common::Reg<regs::DLPR, crate::common::RW> {
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
    pub const fn RFDR(self, n: usize) -> crate::common::Reg<regs::RFDR, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn TFDR(self, n: usize) -> crate::common::Reg<regs::TFDR, crate::common::RW> {
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
    pub const fn IPEDCTXCTRL(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX0IV0(self) -> crate::common::Reg<regs::IPEDCTX0IV0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX0IV1(self) -> crate::common::Reg<regs::IPEDCTX0IV1, crate::common::RW> {
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
    pub const fn IPEDCTX0AAD0(self) -> crate::common::Reg<regs::IPEDCTX0AAD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX0AAD1(self) -> crate::common::Reg<regs::IPEDCTX0AAD1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX1IV0(self) -> crate::common::Reg<regs::IPEDCTX1IV0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX1IV1(self) -> crate::common::Reg<regs::IPEDCTX1IV1, crate::common::RW> {
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
    pub const fn IPEDCTX1AAD0(self) -> crate::common::Reg<regs::IPEDCTX1AAD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX1AAD1(self) -> crate::common::Reg<regs::IPEDCTX1AAD1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX2IV0(self) -> crate::common::Reg<regs::IPEDCTX2IV0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX2IV1(self) -> crate::common::Reg<regs::IPEDCTX2IV1, crate::common::RW> {
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
    pub const fn IPEDCTX2AAD0(self) -> crate::common::Reg<regs::IPEDCTX2AAD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0570usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX2AAD1(self) -> crate::common::Reg<regs::IPEDCTX2AAD1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0574usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX3IV0(self) -> crate::common::Reg<regs::IPEDCTX3IV0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX3IV1(self) -> crate::common::Reg<regs::IPEDCTX3IV1, crate::common::RW> {
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
    pub const fn IPEDCTX3AAD0(self) -> crate::common::Reg<regs::IPEDCTX3AAD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0590usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX3AAD1(self) -> crate::common::Reg<regs::IPEDCTX3AAD1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0594usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX4IV0(self) -> crate::common::Reg<regs::IPEDCTX4IV0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX4IV1(self) -> crate::common::Reg<regs::IPEDCTX4IV1, crate::common::RW> {
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
    pub const fn IPEDCTX4AAD0(self) -> crate::common::Reg<regs::IPEDCTX4AAD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX4AAD1(self) -> crate::common::Reg<regs::IPEDCTX4AAD1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX5IV0(self) -> crate::common::Reg<regs::IPEDCTX5IV0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX5IV1(self) -> crate::common::Reg<regs::IPEDCTX5IV1, crate::common::RW> {
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
    pub const fn IPEDCTX5AAD0(self) -> crate::common::Reg<regs::IPEDCTX5AAD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX5AAD1(self) -> crate::common::Reg<regs::IPEDCTX5AAD1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d4usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX6IV0(self) -> crate::common::Reg<regs::IPEDCTX6IV0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX6IV1(self) -> crate::common::Reg<regs::IPEDCTX6IV1, crate::common::RW> {
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
    pub const fn IPEDCTX6AAD0(self) -> crate::common::Reg<regs::IPEDCTX6AAD0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f0usize) as _) }
    }
    #[inline(always)]
    pub const fn IPEDCTX6AAD1(self) -> crate::common::Reg<regs::IPEDCTX6AAD1, crate::common::RW> {
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
    #[doc = "Data Learning Pattern"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DLPR(pub u32);
    impl DLPR {
        #[inline(always)]
        pub const fn DLP(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_DLP(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DLPR {
        #[inline(always)]
        fn default() -> DLPR {
            DLPR(0)
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
    #[doc = "IP Control 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPCR0(pub u32);
    impl IPCR0 {
        #[inline(always)]
        pub const fn SFAR(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_SFAR(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPCR0 {
        #[inline(always)]
        fn default() -> IPCR0 {
            IPCR0(0)
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
    #[doc = "IPED Context0 Additional Authenticated Data0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX0AAD0(pub u32);
    impl IPEDCTX0AAD0 {
        #[inline(always)]
        pub const fn CTX0_AAD0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX0_AAD0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX0AAD0 {
        #[inline(always)]
        fn default() -> IPEDCTX0AAD0 {
            IPEDCTX0AAD0(0)
        }
    }
    #[doc = "IPED Context0 Additional Authenticated Data1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX0AAD1(pub u32);
    impl IPEDCTX0AAD1 {
        #[inline(always)]
        pub const fn CTX0_AAD1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX0_AAD1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX0AAD1 {
        #[inline(always)]
        fn default() -> IPEDCTX0AAD1 {
            IPEDCTX0AAD1(0)
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
    #[doc = "IPED Context0 IV0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX0IV0(pub u32);
    impl IPEDCTX0IV0 {
        #[inline(always)]
        pub const fn CTX0_IV0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX0_IV0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX0IV0 {
        #[inline(always)]
        fn default() -> IPEDCTX0IV0 {
            IPEDCTX0IV0(0)
        }
    }
    #[doc = "IPED Context0 IV1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX0IV1(pub u32);
    impl IPEDCTX0IV1 {
        #[inline(always)]
        pub const fn CTX0_IV1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX0_IV1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX0IV1 {
        #[inline(always)]
        fn default() -> IPEDCTX0IV1 {
            IPEDCTX0IV1(0)
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
    #[doc = "IPED Context1 Additional Authenticated Data0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX1AAD0(pub u32);
    impl IPEDCTX1AAD0 {
        #[inline(always)]
        pub const fn CTX1_AAD0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX1_AAD0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX1AAD0 {
        #[inline(always)]
        fn default() -> IPEDCTX1AAD0 {
            IPEDCTX1AAD0(0)
        }
    }
    #[doc = "IPED Context1 Additional Authenticated Data1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX1AAD1(pub u32);
    impl IPEDCTX1AAD1 {
        #[inline(always)]
        pub const fn CTX1_AAD1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX1_AAD1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX1AAD1 {
        #[inline(always)]
        fn default() -> IPEDCTX1AAD1 {
            IPEDCTX1AAD1(0)
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
    #[doc = "IPED Context1 IV0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX1IV0(pub u32);
    impl IPEDCTX1IV0 {
        #[inline(always)]
        pub const fn CTX1_IV0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX1_IV0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX1IV0 {
        #[inline(always)]
        fn default() -> IPEDCTX1IV0 {
            IPEDCTX1IV0(0)
        }
    }
    #[doc = "IPED Context1 IV1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX1IV1(pub u32);
    impl IPEDCTX1IV1 {
        #[inline(always)]
        pub const fn CTX1_IV1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX1_IV1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX1IV1 {
        #[inline(always)]
        fn default() -> IPEDCTX1IV1 {
            IPEDCTX1IV1(0)
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
    #[doc = "IPED Context2 Additional Authenticated Data0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX2AAD0(pub u32);
    impl IPEDCTX2AAD0 {
        #[inline(always)]
        pub const fn CTX2_AAD0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX2_AAD0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX2AAD0 {
        #[inline(always)]
        fn default() -> IPEDCTX2AAD0 {
            IPEDCTX2AAD0(0)
        }
    }
    #[doc = "IPED Context2 Additional Authenticated Data1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX2AAD1(pub u32);
    impl IPEDCTX2AAD1 {
        #[inline(always)]
        pub const fn CTX2_AAD1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX2_AAD1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX2AAD1 {
        #[inline(always)]
        fn default() -> IPEDCTX2AAD1 {
            IPEDCTX2AAD1(0)
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
    #[doc = "IPED Context2 IV0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX2IV0(pub u32);
    impl IPEDCTX2IV0 {
        #[inline(always)]
        pub const fn CTX2_IV0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX2_IV0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX2IV0 {
        #[inline(always)]
        fn default() -> IPEDCTX2IV0 {
            IPEDCTX2IV0(0)
        }
    }
    #[doc = "IPED Context2 IV1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX2IV1(pub u32);
    impl IPEDCTX2IV1 {
        #[inline(always)]
        pub const fn CTX2_IV1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX2_IV1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX2IV1 {
        #[inline(always)]
        fn default() -> IPEDCTX2IV1 {
            IPEDCTX2IV1(0)
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
    #[doc = "IPED Context3 Additional Authenticated Data0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX3AAD0(pub u32);
    impl IPEDCTX3AAD0 {
        #[inline(always)]
        pub const fn CTX3_AAD0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX3_AAD0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX3AAD0 {
        #[inline(always)]
        fn default() -> IPEDCTX3AAD0 {
            IPEDCTX3AAD0(0)
        }
    }
    #[doc = "IPED Context3 Additional Authenticated Data1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX3AAD1(pub u32);
    impl IPEDCTX3AAD1 {
        #[inline(always)]
        pub const fn CTX3_AAD1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX3_AAD1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX3AAD1 {
        #[inline(always)]
        fn default() -> IPEDCTX3AAD1 {
            IPEDCTX3AAD1(0)
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
    #[doc = "IPED Context3 IV0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX3IV0(pub u32);
    impl IPEDCTX3IV0 {
        #[inline(always)]
        pub const fn CTX3_IV0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX3_IV0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX3IV0 {
        #[inline(always)]
        fn default() -> IPEDCTX3IV0 {
            IPEDCTX3IV0(0)
        }
    }
    #[doc = "IPED Context3 IV1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX3IV1(pub u32);
    impl IPEDCTX3IV1 {
        #[inline(always)]
        pub const fn CTX3_IV1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX3_IV1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX3IV1 {
        #[inline(always)]
        fn default() -> IPEDCTX3IV1 {
            IPEDCTX3IV1(0)
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
    #[doc = "IPED Context4 Additional Authenticated Data0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX4AAD0(pub u32);
    impl IPEDCTX4AAD0 {
        #[inline(always)]
        pub const fn CTX4_AAD0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX4_AAD0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX4AAD0 {
        #[inline(always)]
        fn default() -> IPEDCTX4AAD0 {
            IPEDCTX4AAD0(0)
        }
    }
    #[doc = "IPED Context4 Additional Authenticated Data1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX4AAD1(pub u32);
    impl IPEDCTX4AAD1 {
        #[inline(always)]
        pub const fn CTX4_AAD1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX4_AAD1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX4AAD1 {
        #[inline(always)]
        fn default() -> IPEDCTX4AAD1 {
            IPEDCTX4AAD1(0)
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
    #[doc = "IPED Context4 IV0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX4IV0(pub u32);
    impl IPEDCTX4IV0 {
        #[inline(always)]
        pub const fn CTX4_IV0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX4_IV0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX4IV0 {
        #[inline(always)]
        fn default() -> IPEDCTX4IV0 {
            IPEDCTX4IV0(0)
        }
    }
    #[doc = "IPED Context4 IV1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX4IV1(pub u32);
    impl IPEDCTX4IV1 {
        #[inline(always)]
        pub const fn CTX4_IV1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX4_IV1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX4IV1 {
        #[inline(always)]
        fn default() -> IPEDCTX4IV1 {
            IPEDCTX4IV1(0)
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
    #[doc = "IPED Context5 Additional Authenticated Data0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX5AAD0(pub u32);
    impl IPEDCTX5AAD0 {
        #[inline(always)]
        pub const fn CTX5_AAD0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX5_AAD0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX5AAD0 {
        #[inline(always)]
        fn default() -> IPEDCTX5AAD0 {
            IPEDCTX5AAD0(0)
        }
    }
    #[doc = "IPED Context5 Additional Authenticated Data1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX5AAD1(pub u32);
    impl IPEDCTX5AAD1 {
        #[inline(always)]
        pub const fn CTX5_AAD1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX5_AAD1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX5AAD1 {
        #[inline(always)]
        fn default() -> IPEDCTX5AAD1 {
            IPEDCTX5AAD1(0)
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
    #[doc = "IPED Context5 IV0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX5IV0(pub u32);
    impl IPEDCTX5IV0 {
        #[inline(always)]
        pub const fn CTX5_IV0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX5_IV0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX5IV0 {
        #[inline(always)]
        fn default() -> IPEDCTX5IV0 {
            IPEDCTX5IV0(0)
        }
    }
    #[doc = "IPED Context5 IV1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX5IV1(pub u32);
    impl IPEDCTX5IV1 {
        #[inline(always)]
        pub const fn CTX5_IV1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX5_IV1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX5IV1 {
        #[inline(always)]
        fn default() -> IPEDCTX5IV1 {
            IPEDCTX5IV1(0)
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
    #[doc = "IPED Context6 Additional Authenticated Data0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX6AAD0(pub u32);
    impl IPEDCTX6AAD0 {
        #[inline(always)]
        pub const fn CTX6_AAD0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX6_AAD0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX6AAD0 {
        #[inline(always)]
        fn default() -> IPEDCTX6AAD0 {
            IPEDCTX6AAD0(0)
        }
    }
    #[doc = "IPED Context6 Additional Authenticated Data1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX6AAD1(pub u32);
    impl IPEDCTX6AAD1 {
        #[inline(always)]
        pub const fn CTX6_AAD1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX6_AAD1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX6AAD1 {
        #[inline(always)]
        fn default() -> IPEDCTX6AAD1 {
            IPEDCTX6AAD1(0)
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
    #[doc = "IPED Context6 IV0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX6IV0(pub u32);
    impl IPEDCTX6IV0 {
        #[inline(always)]
        pub const fn CTX6_IV0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX6_IV0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX6IV0 {
        #[inline(always)]
        fn default() -> IPEDCTX6IV0 {
            IPEDCTX6IV0(0)
        }
    }
    #[doc = "IPED Context6 IV1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IPEDCTX6IV1(pub u32);
    impl IPEDCTX6IV1 {
        #[inline(always)]
        pub const fn CTX6_IV1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_CTX6_IV1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IPEDCTX6IV1 {
        #[inline(always)]
        fn default() -> IPEDCTX6IV1 {
            IPEDCTX6IV1(0)
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
    #[doc = "LUT Key"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LUTKEY(pub u32);
    impl LUTKEY {
        #[inline(always)]
        pub const fn KEY(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_KEY(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for LUTKEY {
        #[inline(always)]
        fn default() -> LUTKEY {
            LUTKEY(0)
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
        pub const fn ARDFEN(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ARDFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[inline(always)]
        pub const fn ATDFEN(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_ATDFEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
    #[doc = "IP Receive FIFO Data 0..IP Receive FIFO Data 31"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RFDR(pub u32);
    impl RFDR {
        #[inline(always)]
        pub const fn RXDATA(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_RXDATA(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for RFDR {
        #[inline(always)]
        fn default() -> RFDR {
            RFDR(0)
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
    #[doc = "IP TX FIFO Data 0..IP TX FIFO Data 31"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TFDR(pub u32);
    impl TFDR {
        #[inline(always)]
        pub const fn TXDATA(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub fn set_TXDATA(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for TFDR {
        #[inline(always)]
        fn default() -> TFDR {
            TFDR(0)
        }
    }
}
