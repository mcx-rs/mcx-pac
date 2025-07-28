#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (6651cd0 2025-05-06))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA_CH {
    ptr: *mut u8,
}
unsafe impl Send for DMA_CH {}
unsafe impl Sync for DMA_CH {}
impl DMA_CH {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn DMA_CHX_CTRL(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_TX_CTRL(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_TX_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_RX_CTRL(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_RX_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_TXDESC_LIST_ADDR(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_TXDESC_LIST_ADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_RXDESC_LIST_ADDR(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_RXDESC_LIST_ADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_TXDESC_TAIL_PTR(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_TXDESC_TAIL_PTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_RXDESC_TAIL_PTR(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_RXDESC_TAIL_PTR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_TXDESC_RING_LENGTH(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_TXDESC_RING_LENGTH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_RX_CONTROL2(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_RX_CONTROL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_INT_EN(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_INT_EN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_RX_INT_WDTIMER(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_RX_INT_WDTIMER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_SLOT_FUNC_CTRL_STAT(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_SLOT_FUNC_CTRL_STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_CUR_HST_TXDESC(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_CUR_HST_RXDESC(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_CUR_HST_TXBUF(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_CUR_HST_RXBUF(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_STAT(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_MISS_FRAME_CNT(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_MISS_FRAME_CNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CHX_RX_ERI_CNT(
        self,
    ) -> crate::common::Reg<regs::DMA_CH_DMA_CHX_RX_ERI_CNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENET {
    ptr: *mut u8,
}
unsafe impl Send for ENET {}
unsafe impl Sync for ENET {}
impl ENET {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MAC_CONFIGURATION(
        self,
    ) -> crate::common::Reg<regs::MAC_CONFIGURATION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_EXT_CONFIGURATION(
        self,
    ) -> crate::common::Reg<regs::MAC_EXT_CONFIGURATION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_PACKET_FILTER(
        self,
    ) -> crate::common::Reg<regs::MAC_PACKET_FILTER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_WATCHDOG_TIMEOUT(
        self,
    ) -> crate::common::Reg<regs::MAC_WATCHDOG_TIMEOUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_VLAN_TAG_CTRL(
        self,
    ) -> crate::common::Reg<regs::MAC_VLAN_TAG_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_VLAN_INCL(self) -> crate::common::Reg<regs::MAC_VLAN_INCL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_INNER_VLAN_INCL(
        self,
    ) -> crate::common::Reg<regs::MAC_INNER_VLAN_INCL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_TX_FLOW_CTRL_Q(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::MAC_TX_FLOW_CTRL_Q, crate::common::RW> {
        assert!(n < 1usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_RX_FLOW_CTRL(
        self,
    ) -> crate::common::Reg<regs::MAC_RX_FLOW_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_RXQ_CTRL4(self) -> crate::common::Reg<regs::MAC_RXQ_CTRL4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_RXQ_CTRL(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::MAC_RXQ_CTRL, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_INTERRUPT_STATUS(
        self,
    ) -> crate::common::Reg<regs::MAC_INTERRUPT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_INTERRUPT_ENABLE(
        self,
    ) -> crate::common::Reg<regs::MAC_INTERRUPT_ENABLE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_RX_TX_STATUS(
        self,
    ) -> crate::common::Reg<regs::MAC_RX_TX_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_PMT_CONTROL_STATUS(
        self,
    ) -> crate::common::Reg<regs::MAC_PMT_CONTROL_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_RWK_PACKET_FILTER(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_LPI_CONTROL_STATUS(
        self,
    ) -> crate::common::Reg<regs::MAC_LPI_CONTROL_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_LPI_TIMERS_CONTROL(
        self,
    ) -> crate::common::Reg<regs::MAC_LPI_TIMERS_CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_LPI_ENTRY_TIMER(
        self,
    ) -> crate::common::Reg<regs::MAC_LPI_ENTRY_TIMER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_ONEUS_TIC_COUNTER(
        self,
    ) -> crate::common::Reg<regs::MAC_ONEUS_TIC_COUNTER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_VERSION(self) -> crate::common::Reg<regs::MAC_VERSION, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_DEBUG(self) -> crate::common::Reg<regs::MAC_DEBUG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_HW_FEAT(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::MAC_HW_FEAT, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_MDIO_ADDRESS(
        self,
    ) -> crate::common::Reg<regs::MAC_MDIO_ADDRESS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_MDIO_DATA(self) -> crate::common::Reg<regs::MAC_MDIO_DATA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_CSR_SW_CTRL(
        self,
    ) -> crate::common::Reg<regs::MAC_CSR_SW_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_ADDRESS0_HIGH(
        self,
    ) -> crate::common::Reg<regs::MAC_ADDRESS0_HIGH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_ADDRESS0_LOW(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[inline(always)]
    pub const fn INDIR_ACCESS_CTRL(
        self,
    ) -> crate::common::Reg<regs::INDIR_ACCESS_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a70usize) as _) }
    }
    #[inline(always)]
    pub const fn INDIR_ACCESS_DATA(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a74usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_TIMESTAMP_CONTROL(
        self,
    ) -> crate::common::Reg<regs::MAC_TIMESTAMP_CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b00usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_SUB_SECOND_INCREMENT(
        self,
    ) -> crate::common::Reg<regs::MAC_SUB_SECOND_INCREMENT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b04usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_SYSTEM_TIME_SECONDS(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b08usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_SYSTEM_TIME_NANOSECONDS(
        self,
    ) -> crate::common::Reg<regs::MAC_SYSTEM_TIME_NANOSECONDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b0cusize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_SYSTEM_TIME_SECONDS_UPDATE(
        self,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b10usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_SYSTEM_TIME_NANOSECONDS_UPDATE(
        self,
    ) -> crate::common::Reg<regs::MAC_SYSTEM_TIME_NANOSECONDS_UPDATE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b14usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_TIMESTAMP_ADDEND(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b18usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_TIMESTAMP_STATUS(
        self,
    ) -> crate::common::Reg<regs::MAC_TIMESTAMP_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b20usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_TX_TIMESTAMP_STATUS_NANOSECONDS(
        self,
    ) -> crate::common::Reg<regs::MAC_TX_TIMESTAMP_STATUS_NANOSECONDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b30usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_TX_TIMESTAMP_STATUS_SECONDS(
        self,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b34usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_TIMESTAMP_INGRESS_CORR_NANOSECOND(
        self,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b58usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND(
        self,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b5cusize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_TIMESTAMP_INGRESS_LATENCY(
        self,
    ) -> crate::common::Reg<regs::MAC_TIMESTAMP_INGRESS_LATENCY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b68usize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_TIMESTAMP_EGRESS_LATENCY(
        self,
    ) -> crate::common::Reg<regs::MAC_TIMESTAMP_EGRESS_LATENCY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b6cusize) as _) }
    }
    #[inline(always)]
    pub const fn MAC_PPS_CONTROL(
        self,
    ) -> crate::common::Reg<regs::MAC_PPS_CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b70usize) as _) }
    }
    #[inline(always)]
    pub const fn PPS0_TARGET_TIME_SECONDS(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b80usize) as _) }
    }
    #[inline(always)]
    pub const fn PPS0_TARGET_TIME_NANOSECONDS(
        self,
    ) -> crate::common::Reg<regs::PPS0_TARGET_TIME_NANOSECONDS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b84usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_OPERATION_MODE(
        self,
    ) -> crate::common::Reg<regs::MTL_OPERATION_MODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c00usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_INTERRUPT_STATUS(
        self,
    ) -> crate::common::Reg<regs::MTL_INTERRUPT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c20usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_RXQ_DMA_MAP0(
        self,
    ) -> crate::common::Reg<regs::MTL_RXQ_DMA_MAP0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c30usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_QUEUE(self, n: usize) -> MTL_QUEUE {
        assert!(n < 2usize);
        unsafe { MTL_QUEUE::from_ptr(self.ptr.add(0x0d00usize + n * 64usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_MODE(self) -> crate::common::Reg<regs::DMA_MODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1000usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_SYSBUS_MODE(
        self,
    ) -> crate::common::Reg<regs::DMA_SYSBUS_MODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1004usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_INTERRUPT_STATUS(
        self,
    ) -> crate::common::Reg<regs::DMA_INTERRUPT_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1008usize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_DEBUG_STATUS0(
        self,
    ) -> crate::common::Reg<regs::DMA_DEBUG_STATUS0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x100cusize) as _) }
    }
    #[inline(always)]
    pub const fn DMA_CH(self, n: usize) -> DMA_CH {
        assert!(n < 2usize);
        unsafe { DMA_CH::from_ptr(self.ptr.add(0x1100usize + n * 128usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MTL_QUEUE {
    ptr: *mut u8,
}
unsafe impl Send for MTL_QUEUE {}
unsafe impl Sync for MTL_QUEUE {}
impl MTL_QUEUE {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn MTL_TXQX_OP_MODE(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_TXQX_OP_MODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_TXQX_UNDRFLW(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_TXQX_UNDRFLW, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_TXQX_DBG(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_TXQX_DBG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_TXQX_ETS_CTRL(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_TXQX_ETS_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_TXQX_ETS_STAT(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_TXQX_ETS_STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_TXQX_QNTM_WGHT(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_TXQX_QNTM_WGHT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_TXQX_SNDSLP_CRDT(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_TXQX_SNDSLP_CRDT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_TXQX_HI_CRDT(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_TXQX_HI_CRDT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_TXQX_LO_CRDT(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_TXQX_LO_CRDT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_QX_INTCTRL_STAT(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_QX_INTCTRL_STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_RXQX_OP_MODE(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_RXQX_OP_MODE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_RXQX_MISSPKT_OVRFLW_CNT(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_RXQX_MISSPKT_OVRFLW_CNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_RXQX_DBG(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_RXQX_DBG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn MTL_RXQX_CTRL(
        self,
    ) -> crate::common::Reg<regs::MTL_QUEUE_MTL_RXQX_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "DMA Channel 0 Control..DMA Channel 1 Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_CTRL(pub u32);
    impl DMA_CH_DMA_CHX_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn PBLx8(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PBLx8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DSL(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DSL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_CTRL {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_CTRL {
            DMA_CH_DMA_CHX_CTRL(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_CTRL")
                .field("PBLx8", &self.PBLx8())
                .field("DSL", &self.DSL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DMA_CH_DMA_CHX_CTRL {{ PBLx8: {=bool:?}, DSL: {=u8:?} }}",
                self.PBLx8(),
                self.DSL()
            )
        }
    }
    #[doc = "Channeli Interrupt Enable..Channel 1 Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_INT_EN(pub u32);
    impl DMA_CH_DMA_CHX_INT_EN {
        #[must_use]
        #[inline(always)]
        pub const fn TIE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXSE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TBUE(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TBUE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RIE(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RBUE(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RBUE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RWTE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RWTE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ETIE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ETIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERIE(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FBEE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FBEE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDEE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDEE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AIE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NIE(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_INT_EN {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_INT_EN {
            DMA_CH_DMA_CHX_INT_EN(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_INT_EN {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_INT_EN")
                .field("TIE", &self.TIE())
                .field("TXSE", &self.TXSE())
                .field("TBUE", &self.TBUE())
                .field("RIE", &self.RIE())
                .field("RBUE", &self.RBUE())
                .field("RSE", &self.RSE())
                .field("RWTE", &self.RWTE())
                .field("ETIE", &self.ETIE())
                .field("ERIE", &self.ERIE())
                .field("FBEE", &self.FBEE())
                .field("CDEE", &self.CDEE())
                .field("AIE", &self.AIE())
                .field("NIE", &self.NIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_INT_EN {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DMA_CH_DMA_CHX_INT_EN {{ TIE: {=bool:?}, TXSE: {=bool:?}, TBUE: {=bool:?}, RIE: {=bool:?}, RBUE: {=bool:?}, RSE: {=bool:?}, RWTE: {=bool:?}, ETIE: {=bool:?}, ERIE: {=bool:?}, FBEE: {=bool:?}, CDEE: {=bool:?}, AIE: {=bool:?}, NIE: {=bool:?} }}" , self . TIE () , self . TXSE () , self . TBUE () , self . RIE () , self . RBUE () , self . RSE () , self . RWTE () , self . ETIE () , self . ERIE () , self . FBEE () , self . CDEE () , self . AIE () , self . NIE ())
        }
    }
    #[doc = "Channel 0 Missed Frame Counter..Channel 1 Missed Frame Counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_MISS_FRAME_CNT(pub u32);
    impl DMA_CH_DMA_CHX_MISS_FRAME_CNT {
        #[must_use]
        #[inline(always)]
        pub const fn MFC(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_MFC(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MFCO(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MFCO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_MISS_FRAME_CNT {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_MISS_FRAME_CNT {
            DMA_CH_DMA_CHX_MISS_FRAME_CNT(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_MISS_FRAME_CNT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_MISS_FRAME_CNT")
                .field("MFC", &self.MFC())
                .field("MFCO", &self.MFCO())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_MISS_FRAME_CNT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DMA_CH_DMA_CHX_MISS_FRAME_CNT {{ MFC: {=u16:?}, MFCO: {=bool:?} }}",
                self.MFC(),
                self.MFCO()
            )
        }
    }
    #[doc = "Channel 0 Rx Descriptor List Address register..Channel 1 Rx Descriptor List Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_RXDESC_LIST_ADDR(pub u32);
    impl DMA_CH_DMA_CHX_RXDESC_LIST_ADDR {
        #[must_use]
        #[inline(always)]
        pub const fn RDESLA(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RDESLA(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_RXDESC_LIST_ADDR {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_RXDESC_LIST_ADDR {
            DMA_CH_DMA_CHX_RXDESC_LIST_ADDR(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_RXDESC_LIST_ADDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_RXDESC_LIST_ADDR")
                .field("RDESLA", &self.RDESLA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_RXDESC_LIST_ADDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DMA_CH_DMA_CHX_RXDESC_LIST_ADDR {{ RDESLA: {=u32:?} }}",
                self.RDESLA()
            )
        }
    }
    #[doc = "Channel 0 Rx Descriptor Tail Pointer..Channel 1 Rx Descriptor Tail Pointer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_RXDESC_TAIL_PTR(pub u32);
    impl DMA_CH_DMA_CHX_RXDESC_TAIL_PTR {
        #[must_use]
        #[inline(always)]
        pub const fn RDTP(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_RDTP(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_RXDESC_TAIL_PTR {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_RXDESC_TAIL_PTR {
            DMA_CH_DMA_CHX_RXDESC_TAIL_PTR(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_RXDESC_TAIL_PTR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_RXDESC_TAIL_PTR")
                .field("RDTP", &self.RDTP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_RXDESC_TAIL_PTR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DMA_CH_DMA_CHX_RXDESC_TAIL_PTR {{ RDTP: {=u32:?} }}",
                self.RDTP()
            )
        }
    }
    #[doc = "Channeli Receive Control..DMA Channel 1 Receive Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_RX_CONTROL2(pub u32);
    impl DMA_CH_DMA_CHX_RX_CONTROL2 {
        #[must_use]
        #[inline(always)]
        pub const fn RDRL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_RDRL(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ARBS(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ARBS(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_RX_CONTROL2 {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_RX_CONTROL2 {
            DMA_CH_DMA_CHX_RX_CONTROL2(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_RX_CONTROL2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_RX_CONTROL2")
                .field("RDRL", &self.RDRL())
                .field("ARBS", &self.ARBS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_RX_CONTROL2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DMA_CH_DMA_CHX_RX_CONTROL2 {{ RDRL: {=u16:?}, ARBS: {=u8:?} }}",
                self.RDRL(),
                self.ARBS()
            )
        }
    }
    #[doc = "DMA Channel 0 Receive Control..DMA Channel 1 Receive Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_RX_CTRL(pub u32);
    impl DMA_CH_DMA_CHX_RX_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn SR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RBSZ_X_0(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RBSZ_X_0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RBSZ_13_Y(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_RBSZ_13_Y(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 3usize)) | (((val as u32) & 0x0fff) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RxPBL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RxPBL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERIC(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERIC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RPF(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RPF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_RX_CTRL {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_RX_CTRL {
            DMA_CH_DMA_CHX_RX_CTRL(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_RX_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_RX_CTRL")
                .field("SR", &self.SR())
                .field("RBSZ_X_0", &self.RBSZ_X_0())
                .field("RBSZ_13_Y", &self.RBSZ_13_Y())
                .field("RxPBL", &self.RxPBL())
                .field("ERIC", &self.ERIC())
                .field("RPF", &self.RPF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_RX_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DMA_CH_DMA_CHX_RX_CTRL {{ SR: {=bool:?}, RBSZ_X_0: {=u8:?}, RBSZ_13_Y: {=u16:?}, RxPBL: {=u8:?}, ERIC: {=bool:?}, RPF: {=bool:?} }}" , self . SR () , self . RBSZ_X_0 () , self . RBSZ_13_Y () , self . RxPBL () , self . ERIC () , self . RPF ())
        }
    }
    #[doc = "Channel 0 Receive ERI Counter..Channel 1 Receive ERI Counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_RX_ERI_CNT(pub u32);
    impl DMA_CH_DMA_CHX_RX_ERI_CNT {
        #[must_use]
        #[inline(always)]
        pub const fn ECNT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ECNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_RX_ERI_CNT {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_RX_ERI_CNT {
            DMA_CH_DMA_CHX_RX_ERI_CNT(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_RX_ERI_CNT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_RX_ERI_CNT")
                .field("ECNT", &self.ECNT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_RX_ERI_CNT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DMA_CH_DMA_CHX_RX_ERI_CNT {{ ECNT: {=u16:?} }}",
                self.ECNT()
            )
        }
    }
    #[doc = "Channel 0 Receive Interrupt Watchdog Timer..Channel 1 Receive Interrupt Watchdog Timer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_RX_INT_WDTIMER(pub u32);
    impl DMA_CH_DMA_CHX_RX_INT_WDTIMER {
        #[must_use]
        #[inline(always)]
        pub const fn RWT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RWT(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RWTU(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RWTU(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_RX_INT_WDTIMER {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_RX_INT_WDTIMER {
            DMA_CH_DMA_CHX_RX_INT_WDTIMER(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_RX_INT_WDTIMER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_RX_INT_WDTIMER")
                .field("RWT", &self.RWT())
                .field("RWTU", &self.RWTU())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_RX_INT_WDTIMER {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DMA_CH_DMA_CHX_RX_INT_WDTIMER {{ RWT: {=u8:?}, RWTU: {=u8:?} }}",
                self.RWT(),
                self.RWTU()
            )
        }
    }
    #[doc = "Channel 0 Slot Function Control and Status..Channel 1 Slot Function Control and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_SLOT_FUNC_CTRL_STAT(pub u32);
    impl DMA_CH_DMA_CHX_SLOT_FUNC_CTRL_STAT {
        #[must_use]
        #[inline(always)]
        pub const fn ESC(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ESC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ASC(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ASC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SIV(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_SIV(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSN(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RSN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_SLOT_FUNC_CTRL_STAT {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_SLOT_FUNC_CTRL_STAT {
            DMA_CH_DMA_CHX_SLOT_FUNC_CTRL_STAT(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_SLOT_FUNC_CTRL_STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_SLOT_FUNC_CTRL_STAT")
                .field("ESC", &self.ESC())
                .field("ASC", &self.ASC())
                .field("SIV", &self.SIV())
                .field("RSN", &self.RSN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_SLOT_FUNC_CTRL_STAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DMA_CH_DMA_CHX_SLOT_FUNC_CTRL_STAT {{ ESC: {=bool:?}, ASC: {=bool:?}, SIV: {=u16:?}, RSN: {=u8:?} }}" , self . ESC () , self . ASC () , self . SIV () , self . RSN ())
        }
    }
    #[doc = "DMA Channel 0 Status..DMA Channel 1 Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_STAT(pub u32);
    impl DMA_CH_DMA_CHX_STAT {
        #[must_use]
        #[inline(always)]
        pub const fn TI(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TPS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TBU(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TBU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RI(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RBU(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RBU(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RPS(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RPS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RWT(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RWT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ETI(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ETI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERI(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FBE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FBE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CDE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CDE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AIS(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NIS(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TEB(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TEB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn REB(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_REB(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_STAT {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_STAT {
            DMA_CH_DMA_CHX_STAT(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_STAT")
                .field("TI", &self.TI())
                .field("TPS", &self.TPS())
                .field("TBU", &self.TBU())
                .field("RI", &self.RI())
                .field("RBU", &self.RBU())
                .field("RPS", &self.RPS())
                .field("RWT", &self.RWT())
                .field("ETI", &self.ETI())
                .field("ERI", &self.ERI())
                .field("FBE", &self.FBE())
                .field("CDE", &self.CDE())
                .field("AIS", &self.AIS())
                .field("NIS", &self.NIS())
                .field("TEB", &self.TEB())
                .field("REB", &self.REB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_STAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DMA_CH_DMA_CHX_STAT {{ TI: {=bool:?}, TPS: {=bool:?}, TBU: {=bool:?}, RI: {=bool:?}, RBU: {=bool:?}, RPS: {=bool:?}, RWT: {=bool:?}, ETI: {=bool:?}, ERI: {=bool:?}, FBE: {=bool:?}, CDE: {=bool:?}, AIS: {=bool:?}, NIS: {=bool:?}, TEB: {=u8:?}, REB: {=u8:?} }}" , self . TI () , self . TPS () , self . TBU () , self . RI () , self . RBU () , self . RPS () , self . RWT () , self . ETI () , self . ERI () , self . FBE () , self . CDE () , self . AIS () , self . NIS () , self . TEB () , self . REB ())
        }
    }
    #[doc = "Channel 0 Tx Descriptor List Address register..Channel 1 Tx Descriptor List Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_TXDESC_LIST_ADDR(pub u32);
    impl DMA_CH_DMA_CHX_TXDESC_LIST_ADDR {
        #[must_use]
        #[inline(always)]
        pub const fn TDESLA(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_TDESLA(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_TXDESC_LIST_ADDR {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_TXDESC_LIST_ADDR {
            DMA_CH_DMA_CHX_TXDESC_LIST_ADDR(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_TXDESC_LIST_ADDR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_TXDESC_LIST_ADDR")
                .field("TDESLA", &self.TDESLA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_TXDESC_LIST_ADDR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DMA_CH_DMA_CHX_TXDESC_LIST_ADDR {{ TDESLA: {=u32:?} }}",
                self.TDESLA()
            )
        }
    }
    #[doc = "Channel 0 Tx Descriptor Ring Length..Channel 1 Tx Descriptor Ring Length"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_TXDESC_RING_LENGTH(pub u32);
    impl DMA_CH_DMA_CHX_TXDESC_RING_LENGTH {
        #[must_use]
        #[inline(always)]
        pub const fn TDRL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TDRL(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_TXDESC_RING_LENGTH {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_TXDESC_RING_LENGTH {
            DMA_CH_DMA_CHX_TXDESC_RING_LENGTH(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_TXDESC_RING_LENGTH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_TXDESC_RING_LENGTH")
                .field("TDRL", &self.TDRL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_TXDESC_RING_LENGTH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DMA_CH_DMA_CHX_TXDESC_RING_LENGTH {{ TDRL: {=u16:?} }}",
                self.TDRL()
            )
        }
    }
    #[doc = "Channel 0 Tx Descriptor Tail Pointer..Channel 1 Tx Descriptor Tail Pointer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_TXDESC_TAIL_PTR(pub u32);
    impl DMA_CH_DMA_CHX_TXDESC_TAIL_PTR {
        #[must_use]
        #[inline(always)]
        pub const fn TDTP(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_TDTP(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_TXDESC_TAIL_PTR {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_TXDESC_TAIL_PTR {
            DMA_CH_DMA_CHX_TXDESC_TAIL_PTR(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_TXDESC_TAIL_PTR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_TXDESC_TAIL_PTR")
                .field("TDTP", &self.TDTP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_TXDESC_TAIL_PTR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DMA_CH_DMA_CHX_TXDESC_TAIL_PTR {{ TDTP: {=u32:?} }}",
                self.TDTP()
            )
        }
    }
    #[doc = "DMA Channel 0 Transmit Control..DMA Channel 1 Transmit Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_CH_DMA_CHX_TX_CTRL(pub u32);
    impl DMA_CH_DMA_CHX_TX_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn ST(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TCW(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TCW(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OSF(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OSF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TxPBL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TxPBL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ETIC(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ETIC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for DMA_CH_DMA_CHX_TX_CTRL {
        #[inline(always)]
        fn default() -> DMA_CH_DMA_CHX_TX_CTRL {
            DMA_CH_DMA_CHX_TX_CTRL(0)
        }
    }
    impl core::fmt::Debug for DMA_CH_DMA_CHX_TX_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_CH_DMA_CHX_TX_CTRL")
                .field("ST", &self.ST())
                .field("TCW", &self.TCW())
                .field("OSF", &self.OSF())
                .field("TxPBL", &self.TxPBL())
                .field("ETIC", &self.ETIC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_CH_DMA_CHX_TX_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DMA_CH_DMA_CHX_TX_CTRL {{ ST: {=bool:?}, TCW: {=u8:?}, OSF: {=bool:?}, TxPBL: {=u8:?}, ETIC: {=bool:?} }}" , self . ST () , self . TCW () , self . OSF () , self . TxPBL () , self . ETIC ())
        }
    }
    #[doc = "DMA Debug Status 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_DEBUG_STATUS0(pub u32);
    impl DMA_DEBUG_STATUS0 {
        #[must_use]
        #[inline(always)]
        pub const fn AXWHSTS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AXWHSTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RPS0(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RPS0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPS0(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPS0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RPS1(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RPS1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPS1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPS1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for DMA_DEBUG_STATUS0 {
        #[inline(always)]
        fn default() -> DMA_DEBUG_STATUS0 {
            DMA_DEBUG_STATUS0(0)
        }
    }
    impl core::fmt::Debug for DMA_DEBUG_STATUS0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_DEBUG_STATUS0")
                .field("AXWHSTS", &self.AXWHSTS())
                .field("RPS0", &self.RPS0())
                .field("TPS0", &self.TPS0())
                .field("RPS1", &self.RPS1())
                .field("TPS1", &self.TPS1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_DEBUG_STATUS0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DMA_DEBUG_STATUS0 {{ AXWHSTS: {=bool:?}, RPS0: {=u8:?}, TPS0: {=u8:?}, RPS1: {=u8:?}, TPS1: {=u8:?} }}" , self . AXWHSTS () , self . RPS0 () , self . TPS0 () , self . RPS1 () , self . TPS1 ())
        }
    }
    #[doc = "DMA Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_INTERRUPT_STATUS(pub u32);
    impl DMA_INTERRUPT_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn DC0IS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DC0IS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DC1IS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DC1IS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MTLIS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MTLIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MACIS(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MACIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for DMA_INTERRUPT_STATUS {
        #[inline(always)]
        fn default() -> DMA_INTERRUPT_STATUS {
            DMA_INTERRUPT_STATUS(0)
        }
    }
    impl core::fmt::Debug for DMA_INTERRUPT_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_INTERRUPT_STATUS")
                .field("DC0IS", &self.DC0IS())
                .field("DC1IS", &self.DC1IS())
                .field("MTLIS", &self.MTLIS())
                .field("MACIS", &self.MACIS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_INTERRUPT_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DMA_INTERRUPT_STATUS {{ DC0IS: {=bool:?}, DC1IS: {=bool:?}, MTLIS: {=bool:?}, MACIS: {=bool:?} }}" , self . DC0IS () , self . DC1IS () , self . MTLIS () , self . MACIS ())
        }
    }
    #[doc = "DMA Bus Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_MODE(pub u32);
    impl DMA_MODE {
        #[must_use]
        #[inline(always)]
        pub const fn SWR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DA(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TAA(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TAA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXPR(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXPR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PR(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
    }
    impl Default for DMA_MODE {
        #[inline(always)]
        fn default() -> DMA_MODE {
            DMA_MODE(0)
        }
    }
    impl core::fmt::Debug for DMA_MODE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_MODE")
                .field("SWR", &self.SWR())
                .field("DA", &self.DA())
                .field("TAA", &self.TAA())
                .field("TXPR", &self.TXPR())
                .field("PR", &self.PR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_MODE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DMA_MODE {{ SWR: {=bool:?}, DA: {=bool:?}, TAA: {=u8:?}, TXPR: {=bool:?}, PR: {=u8:?} }}" , self . SWR () , self . DA () , self . TAA () , self . TXPR () , self . PR ())
        }
    }
    #[doc = "DMA System Bus Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DMA_SYSBUS_MODE(pub u32);
    impl DMA_SYSBUS_MODE {
        #[must_use]
        #[inline(always)]
        pub const fn FB(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AAL(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AAL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MB(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RB(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for DMA_SYSBUS_MODE {
        #[inline(always)]
        fn default() -> DMA_SYSBUS_MODE {
            DMA_SYSBUS_MODE(0)
        }
    }
    impl core::fmt::Debug for DMA_SYSBUS_MODE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DMA_SYSBUS_MODE")
                .field("FB", &self.FB())
                .field("AAL", &self.AAL())
                .field("MB", &self.MB())
                .field("RB", &self.RB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DMA_SYSBUS_MODE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DMA_SYSBUS_MODE {{ FB: {=bool:?}, AAL: {=bool:?}, MB: {=bool:?}, RB: {=bool:?} }}",
                self.FB(),
                self.AAL(),
                self.MB(),
                self.RB()
            )
        }
    }
    #[doc = "Indirect Access Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct INDIR_ACCESS_CTRL(pub u32);
    impl INDIR_ACCESS_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn OB(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn COM(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_COM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AUTO(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AUTO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AOFF(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_AOFF(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MSEL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for INDIR_ACCESS_CTRL {
        #[inline(always)]
        fn default() -> INDIR_ACCESS_CTRL {
            INDIR_ACCESS_CTRL(0)
        }
    }
    impl core::fmt::Debug for INDIR_ACCESS_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("INDIR_ACCESS_CTRL")
                .field("OB", &self.OB())
                .field("COM", &self.COM())
                .field("AUTO", &self.AUTO())
                .field("AOFF", &self.AOFF())
                .field("MSEL", &self.MSEL())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for INDIR_ACCESS_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "INDIR_ACCESS_CTRL {{ OB: {=bool:?}, COM: {=bool:?}, AUTO: {=bool:?}, AOFF: {=u8:?}, MSEL: {=u8:?} }}" , self . OB () , self . COM () , self . AUTO () , self . AOFF () , self . MSEL ())
        }
    }
    #[doc = "MAC Address0 High"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_ADDRESS0_HIGH(pub u32);
    impl MAC_ADDRESS0_HIGH {
        #[must_use]
        #[inline(always)]
        pub const fn ADDRHI(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ADDRHI(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DCS(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_DCS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AE(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MAC_ADDRESS0_HIGH {
        #[inline(always)]
        fn default() -> MAC_ADDRESS0_HIGH {
            MAC_ADDRESS0_HIGH(0)
        }
    }
    impl core::fmt::Debug for MAC_ADDRESS0_HIGH {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_ADDRESS0_HIGH")
                .field("ADDRHI", &self.ADDRHI())
                .field("DCS", &self.DCS())
                .field("AE", &self.AE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_ADDRESS0_HIGH {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_ADDRESS0_HIGH {{ ADDRHI: {=u16:?}, DCS: {=u8:?}, AE: {=bool:?} }}",
                self.ADDRHI(),
                self.DCS(),
                self.AE()
            )
        }
    }
    #[doc = "MAC Configuration"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_CONFIGURATION(pub u32);
    impl MAC_CONFIGURATION {
        #[must_use]
        #[inline(always)]
        pub const fn RE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PRELEN(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PRELEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DC(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BL(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_BL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DR(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DCRS(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DCRS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DO(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DO(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ECRSFD(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ECRSFD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LM(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DM(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FES(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FES(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PS(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn JE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_JE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn JD(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_JD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn WD(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_WD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ACS(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ACS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CST(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn S2KP(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_S2KP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GPSLCE(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GPSLCE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IPG(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_IPG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn IPC(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_IPC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SARC(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SARC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for MAC_CONFIGURATION {
        #[inline(always)]
        fn default() -> MAC_CONFIGURATION {
            MAC_CONFIGURATION(0)
        }
    }
    impl core::fmt::Debug for MAC_CONFIGURATION {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_CONFIGURATION")
                .field("RE", &self.RE())
                .field("TE", &self.TE())
                .field("PRELEN", &self.PRELEN())
                .field("DC", &self.DC())
                .field("BL", &self.BL())
                .field("DR", &self.DR())
                .field("DCRS", &self.DCRS())
                .field("DO", &self.DO())
                .field("ECRSFD", &self.ECRSFD())
                .field("LM", &self.LM())
                .field("DM", &self.DM())
                .field("FES", &self.FES())
                .field("PS", &self.PS())
                .field("JE", &self.JE())
                .field("JD", &self.JD())
                .field("WD", &self.WD())
                .field("ACS", &self.ACS())
                .field("CST", &self.CST())
                .field("S2KP", &self.S2KP())
                .field("GPSLCE", &self.GPSLCE())
                .field("IPG", &self.IPG())
                .field("IPC", &self.IPC())
                .field("SARC", &self.SARC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_CONFIGURATION {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_CONFIGURATION {{ RE: {=bool:?}, TE: {=bool:?}, PRELEN: {=u8:?}, DC: {=bool:?}, BL: {=u8:?}, DR: {=bool:?}, DCRS: {=bool:?}, DO: {=bool:?}, ECRSFD: {=bool:?}, LM: {=bool:?}, DM: {=bool:?}, FES: {=bool:?}, PS: {=bool:?}, JE: {=bool:?}, JD: {=bool:?}, WD: {=bool:?}, ACS: {=bool:?}, CST: {=bool:?}, S2KP: {=bool:?}, GPSLCE: {=bool:?}, IPG: {=u8:?}, IPC: {=bool:?}, SARC: {=u8:?} }}" , self . RE () , self . TE () , self . PRELEN () , self . DC () , self . BL () , self . DR () , self . DCRS () , self . DO () , self . ECRSFD () , self . LM () , self . DM () , self . FES () , self . PS () , self . JE () , self . JD () , self . WD () , self . ACS () , self . CST () , self . S2KP () , self . GPSLCE () , self . IPG () , self . IPC () , self . SARC ())
        }
    }
    #[doc = "CSR Software Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_CSR_SW_CTRL(pub u32);
    impl MAC_CSR_SW_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn RCWE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RCWE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for MAC_CSR_SW_CTRL {
        #[inline(always)]
        fn default() -> MAC_CSR_SW_CTRL {
            MAC_CSR_SW_CTRL(0)
        }
    }
    impl core::fmt::Debug for MAC_CSR_SW_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_CSR_SW_CTRL")
                .field("RCWE", &self.RCWE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_CSR_SW_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MAC_CSR_SW_CTRL {{ RCWE: {=bool:?} }}", self.RCWE())
        }
    }
    #[doc = "MAC Debug"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_DEBUG(pub u32);
    impl MAC_DEBUG {
        #[must_use]
        #[inline(always)]
        pub const fn RPESTS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RPESTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RFCFCSTS(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RFCFCSTS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPESTS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TPESTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TFCSTS(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TFCSTS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
    }
    impl Default for MAC_DEBUG {
        #[inline(always)]
        fn default() -> MAC_DEBUG {
            MAC_DEBUG(0)
        }
    }
    impl core::fmt::Debug for MAC_DEBUG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_DEBUG")
                .field("RPESTS", &self.RPESTS())
                .field("RFCFCSTS", &self.RFCFCSTS())
                .field("TPESTS", &self.TPESTS())
                .field("TFCSTS", &self.TFCSTS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_DEBUG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_DEBUG {{ RPESTS: {=bool:?}, RFCFCSTS: {=u8:?}, TPESTS: {=bool:?}, TFCSTS: {=u8:?} }}" , self . RPESTS () , self . RFCFCSTS () , self . TPESTS () , self . TFCSTS ())
        }
    }
    #[doc = "MAC Extended Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_EXT_CONFIGURATION(pub u32);
    impl MAC_EXT_CONFIGURATION {
        #[must_use]
        #[inline(always)]
        pub const fn GPSL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_GPSL(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DCRCC(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DCRCC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPEN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USP(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_USP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PDC(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PDC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EIPGEN(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EIPGEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EIPG(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EIPG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
    }
    impl Default for MAC_EXT_CONFIGURATION {
        #[inline(always)]
        fn default() -> MAC_EXT_CONFIGURATION {
            MAC_EXT_CONFIGURATION(0)
        }
    }
    impl core::fmt::Debug for MAC_EXT_CONFIGURATION {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_EXT_CONFIGURATION")
                .field("GPSL", &self.GPSL())
                .field("DCRCC", &self.DCRCC())
                .field("SPEN", &self.SPEN())
                .field("USP", &self.USP())
                .field("PDC", &self.PDC())
                .field("EIPGEN", &self.EIPGEN())
                .field("EIPG", &self.EIPG())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_EXT_CONFIGURATION {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_EXT_CONFIGURATION {{ GPSL: {=u16:?}, DCRCC: {=bool:?}, SPEN: {=bool:?}, USP: {=bool:?}, PDC: {=bool:?}, EIPGEN: {=bool:?}, EIPG: {=u8:?} }}" , self . GPSL () , self . DCRCC () , self . SPEN () , self . USP () , self . PDC () , self . EIPGEN () , self . EIPG ())
        }
    }
    #[doc = "Hardware Features 0..Hardware Features 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_HW_FEAT(pub u32);
    impl MAC_HW_FEAT {
        #[must_use]
        #[inline(always)]
        pub const fn MIISEL(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MIISEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NRVF(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NRVF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXFIFOSIZE(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RXFIFOSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXQCNT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RXQCNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GMIISEL(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GMIISEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HDSEL(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_HDSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PCSSEL(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PCSSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CBTISEL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CBTISEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VLHASH(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VLHASH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DVLAN(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DVLAN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SMASEL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SMASEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPRAM(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPRAM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RWKSEL(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RWKSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXFIFOSIZE(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TXFIFOSIZE(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXQCNT(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TXQCNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MGKSEL(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MGKSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MMCSEL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MMCSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ARPOFFSEL(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ARPOFFSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PDUPSEL(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PDUPSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FRPSEL(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FRPSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FRPBS(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FRPBS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OSTEN(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OSTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PTOEN(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PTOEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXCHCNT(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RXCHCNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSSEL(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADVTHWORD(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ADVTHWORD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EEESEL(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EEESEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FRPES(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_FRPES(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDR64(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADDR64(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXCOESEL(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXCOESEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DCBEN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DCBEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ESTSEL(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ESTSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RDCSZ(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RDCSZ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXCOESEL(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXCOESEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ESTDEP(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ESTDEP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SPHEN(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SPHEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDMACADRSEL(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ADDMACADRSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSOEN(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSOEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXCHCNT(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TXCHCNT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DBGMEMA(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DBGMEMA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AVSEL(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AVSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ESTWID(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ESTWID(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RAVSEL(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RAVSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TDCSZ(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TDCSZ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MACADR32SEL(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MACADR32SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn POUOST(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_POUOST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn HASHTBLSZ(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_HASHTBLSZ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MACADR64SEL(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MACADR64SEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PPSOUTNUM(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PPSOUTNUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSSTSSEL(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TSSTSSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FPESEL(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FPESEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn L3L4FNUM(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_L3L4FNUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SAVLANINS(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SAVLANINS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TBSSEL(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TBSSEL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ACTPHYSEL(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ACTPHYSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ASP(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ASP(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AUXSNAPNUM(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_AUXSNAPNUM(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
    }
    impl Default for MAC_HW_FEAT {
        #[inline(always)]
        fn default() -> MAC_HW_FEAT {
            MAC_HW_FEAT(0)
        }
    }
    impl core::fmt::Debug for MAC_HW_FEAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_HW_FEAT")
                .field("MIISEL", &self.MIISEL())
                .field("NRVF", &self.NRVF())
                .field("RXFIFOSIZE", &self.RXFIFOSIZE())
                .field("RXQCNT", &self.RXQCNT())
                .field("GMIISEL", &self.GMIISEL())
                .field("HDSEL", &self.HDSEL())
                .field("PCSSEL", &self.PCSSEL())
                .field("CBTISEL", &self.CBTISEL())
                .field("VLHASH", &self.VLHASH())
                .field("DVLAN", &self.DVLAN())
                .field("SMASEL", &self.SMASEL())
                .field("SPRAM", &self.SPRAM())
                .field("RWKSEL", &self.RWKSEL())
                .field("TXFIFOSIZE", &self.TXFIFOSIZE())
                .field("TXQCNT", &self.TXQCNT())
                .field("MGKSEL", &self.MGKSEL())
                .field("MMCSEL", &self.MMCSEL())
                .field("ARPOFFSEL", &self.ARPOFFSEL())
                .field("PDUPSEL", &self.PDUPSEL())
                .field("FRPSEL", &self.FRPSEL())
                .field("FRPBS", &self.FRPBS())
                .field("OSTEN", &self.OSTEN())
                .field("PTOEN", &self.PTOEN())
                .field("RXCHCNT", &self.RXCHCNT())
                .field("TSSEL", &self.TSSEL())
                .field("ADVTHWORD", &self.ADVTHWORD())
                .field("EEESEL", &self.EEESEL())
                .field("FRPES", &self.FRPES())
                .field("ADDR64", &self.ADDR64())
                .field("TXCOESEL", &self.TXCOESEL())
                .field("DCBEN", &self.DCBEN())
                .field("ESTSEL", &self.ESTSEL())
                .field("RDCSZ", &self.RDCSZ())
                .field("RXCOESEL", &self.RXCOESEL())
                .field("ESTDEP", &self.ESTDEP())
                .field("SPHEN", &self.SPHEN())
                .field("ADDMACADRSEL", &self.ADDMACADRSEL())
                .field("TSOEN", &self.TSOEN())
                .field("TXCHCNT", &self.TXCHCNT())
                .field("DBGMEMA", &self.DBGMEMA())
                .field("AVSEL", &self.AVSEL())
                .field("ESTWID", &self.ESTWID())
                .field("RAVSEL", &self.RAVSEL())
                .field("TDCSZ", &self.TDCSZ())
                .field("MACADR32SEL", &self.MACADR32SEL())
                .field("POUOST", &self.POUOST())
                .field("HASHTBLSZ", &self.HASHTBLSZ())
                .field("MACADR64SEL", &self.MACADR64SEL())
                .field("PPSOUTNUM", &self.PPSOUTNUM())
                .field("TSSTSSEL", &self.TSSTSSEL())
                .field("FPESEL", &self.FPESEL())
                .field("L3L4FNUM", &self.L3L4FNUM())
                .field("SAVLANINS", &self.SAVLANINS())
                .field("TBSSEL", &self.TBSSEL())
                .field("ACTPHYSEL", &self.ACTPHYSEL())
                .field("ASP", &self.ASP())
                .field("AUXSNAPNUM", &self.AUXSNAPNUM())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_HW_FEAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_HW_FEAT {{ MIISEL: {=bool:?}, NRVF: {=u8:?}, RXFIFOSIZE: {=u8:?}, RXQCNT: {=u8:?}, GMIISEL: {=bool:?}, HDSEL: {=bool:?}, PCSSEL: {=bool:?}, CBTISEL: {=bool:?}, VLHASH: {=bool:?}, DVLAN: {=bool:?}, SMASEL: {=bool:?}, SPRAM: {=bool:?}, RWKSEL: {=bool:?}, TXFIFOSIZE: {=u8:?}, TXQCNT: {=u8:?}, MGKSEL: {=bool:?}, MMCSEL: {=bool:?}, ARPOFFSEL: {=bool:?}, PDUPSEL: {=bool:?}, FRPSEL: {=bool:?}, FRPBS: {=u8:?}, OSTEN: {=bool:?}, PTOEN: {=bool:?}, RXCHCNT: {=u8:?}, TSSEL: {=bool:?}, ADVTHWORD: {=bool:?}, EEESEL: {=bool:?}, FRPES: {=u8:?}, ADDR64: {=u8:?}, TXCOESEL: {=bool:?}, DCBEN: {=bool:?}, ESTSEL: {=bool:?}, RDCSZ: {=u8:?}, RXCOESEL: {=bool:?}, ESTDEP: {=u8:?}, SPHEN: {=bool:?}, ADDMACADRSEL: {=u8:?}, TSOEN: {=bool:?}, TXCHCNT: {=u8:?}, DBGMEMA: {=bool:?}, AVSEL: {=bool:?}, ESTWID: {=u8:?}, RAVSEL: {=bool:?}, TDCSZ: {=u8:?}, MACADR32SEL: {=bool:?}, POUOST: {=bool:?}, HASHTBLSZ: {=u8:?}, MACADR64SEL: {=bool:?}, PPSOUTNUM: {=u8:?}, TSSTSSEL: {=u8:?}, FPESEL: {=bool:?}, L3L4FNUM: {=u8:?}, SAVLANINS: {=bool:?}, TBSSEL: {=bool:?}, ACTPHYSEL: {=u8:?}, ASP: {=u8:?}, AUXSNAPNUM: {=u8:?} }}" , self . MIISEL () , self . NRVF () , self . RXFIFOSIZE () , self . RXQCNT () , self . GMIISEL () , self . HDSEL () , self . PCSSEL () , self . CBTISEL () , self . VLHASH () , self . DVLAN () , self . SMASEL () , self . SPRAM () , self . RWKSEL () , self . TXFIFOSIZE () , self . TXQCNT () , self . MGKSEL () , self . MMCSEL () , self . ARPOFFSEL () , self . PDUPSEL () , self . FRPSEL () , self . FRPBS () , self . OSTEN () , self . PTOEN () , self . RXCHCNT () , self . TSSEL () , self . ADVTHWORD () , self . EEESEL () , self . FRPES () , self . ADDR64 () , self . TXCOESEL () , self . DCBEN () , self . ESTSEL () , self . RDCSZ () , self . RXCOESEL () , self . ESTDEP () , self . SPHEN () , self . ADDMACADRSEL () , self . TSOEN () , self . TXCHCNT () , self . DBGMEMA () , self . AVSEL () , self . ESTWID () , self . RAVSEL () , self . TDCSZ () , self . MACADR32SEL () , self . POUOST () , self . HASHTBLSZ () , self . MACADR64SEL () , self . PPSOUTNUM () , self . TSSTSSEL () , self . FPESEL () , self . L3L4FNUM () , self . SAVLANINS () , self . TBSSEL () , self . ACTPHYSEL () , self . ASP () , self . AUXSNAPNUM ())
        }
    }
    #[doc = "MAC Inner VLAN Tag Inclusion or Replacement"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_INNER_VLAN_INCL(pub u32);
    impl MAC_INNER_VLAN_INCL {
        #[must_use]
        #[inline(always)]
        pub const fn VLT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_VLT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VLC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VLC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VLP(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VLP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CSVL(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CSVL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VLTI(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VLTI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for MAC_INNER_VLAN_INCL {
        #[inline(always)]
        fn default() -> MAC_INNER_VLAN_INCL {
            MAC_INNER_VLAN_INCL(0)
        }
    }
    impl core::fmt::Debug for MAC_INNER_VLAN_INCL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_INNER_VLAN_INCL")
                .field("VLT", &self.VLT())
                .field("VLC", &self.VLC())
                .field("VLP", &self.VLP())
                .field("CSVL", &self.CSVL())
                .field("VLTI", &self.VLTI())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_INNER_VLAN_INCL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_INNER_VLAN_INCL {{ VLT: {=u16:?}, VLC: {=u8:?}, VLP: {=bool:?}, CSVL: {=bool:?}, VLTI: {=bool:?} }}" , self . VLT () , self . VLC () , self . VLP () , self . CSVL () , self . VLTI ())
        }
    }
    #[doc = "Interrupt Enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_INTERRUPT_ENABLE(pub u32);
    impl MAC_INTERRUPT_ENABLE {
        #[must_use]
        #[inline(always)]
        pub const fn PHYIE(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PHYIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PMTIE(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PMTIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPIIE(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPIIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSIE(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXSTSIE(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXSTSIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXSTSIE(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXSTSIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MDIOIE(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MDIOIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for MAC_INTERRUPT_ENABLE {
        #[inline(always)]
        fn default() -> MAC_INTERRUPT_ENABLE {
            MAC_INTERRUPT_ENABLE(0)
        }
    }
    impl core::fmt::Debug for MAC_INTERRUPT_ENABLE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_INTERRUPT_ENABLE")
                .field("PHYIE", &self.PHYIE())
                .field("PMTIE", &self.PMTIE())
                .field("LPIIE", &self.LPIIE())
                .field("TSIE", &self.TSIE())
                .field("TXSTSIE", &self.TXSTSIE())
                .field("RXSTSIE", &self.RXSTSIE())
                .field("MDIOIE", &self.MDIOIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_INTERRUPT_ENABLE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_INTERRUPT_ENABLE {{ PHYIE: {=bool:?}, PMTIE: {=bool:?}, LPIIE: {=bool:?}, TSIE: {=bool:?}, TXSTSIE: {=bool:?}, RXSTSIE: {=bool:?}, MDIOIE: {=bool:?} }}" , self . PHYIE () , self . PMTIE () , self . LPIIE () , self . TSIE () , self . TXSTSIE () , self . RXSTSIE () , self . MDIOIE ())
        }
    }
    #[doc = "Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_INTERRUPT_STATUS(pub u32);
    impl MAC_INTERRUPT_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn PHYIS(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PHYIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PMTIS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PMTIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPIIS(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPIIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSIS(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXSTSIS(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXSTSIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXSTSIS(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXSTSIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MDIOIS(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MDIOIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for MAC_INTERRUPT_STATUS {
        #[inline(always)]
        fn default() -> MAC_INTERRUPT_STATUS {
            MAC_INTERRUPT_STATUS(0)
        }
    }
    impl core::fmt::Debug for MAC_INTERRUPT_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_INTERRUPT_STATUS")
                .field("PHYIS", &self.PHYIS())
                .field("PMTIS", &self.PMTIS())
                .field("LPIIS", &self.LPIIS())
                .field("TSIS", &self.TSIS())
                .field("TXSTSIS", &self.TXSTSIS())
                .field("RXSTSIS", &self.RXSTSIS())
                .field("MDIOIS", &self.MDIOIS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_INTERRUPT_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_INTERRUPT_STATUS {{ PHYIS: {=bool:?}, PMTIS: {=bool:?}, LPIIS: {=bool:?}, TSIS: {=bool:?}, TXSTSIS: {=bool:?}, RXSTSIS: {=bool:?}, MDIOIS: {=bool:?} }}" , self . PHYIS () , self . PMTIS () , self . LPIIS () , self . TSIS () , self . TXSTSIS () , self . RXSTSIS () , self . MDIOIS ())
        }
    }
    #[doc = "LPI Control and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_LPI_CONTROL_STATUS(pub u32);
    impl MAC_LPI_CONTROL_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn TLPIEN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TLPIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TLPIEX(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TLPIEX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RLPIEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RLPIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RLPIEX(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RLPIEX(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TLPIST(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TLPIST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RLPIST(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RLPIST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPIEN(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPIEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PLS(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PLS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPITXA(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPITXA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPIATE(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPIATE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LPITCSE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LPITCSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for MAC_LPI_CONTROL_STATUS {
        #[inline(always)]
        fn default() -> MAC_LPI_CONTROL_STATUS {
            MAC_LPI_CONTROL_STATUS(0)
        }
    }
    impl core::fmt::Debug for MAC_LPI_CONTROL_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_LPI_CONTROL_STATUS")
                .field("TLPIEN", &self.TLPIEN())
                .field("TLPIEX", &self.TLPIEX())
                .field("RLPIEN", &self.RLPIEN())
                .field("RLPIEX", &self.RLPIEX())
                .field("TLPIST", &self.TLPIST())
                .field("RLPIST", &self.RLPIST())
                .field("LPIEN", &self.LPIEN())
                .field("PLS", &self.PLS())
                .field("LPITXA", &self.LPITXA())
                .field("LPIATE", &self.LPIATE())
                .field("LPITCSE", &self.LPITCSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_LPI_CONTROL_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_LPI_CONTROL_STATUS {{ TLPIEN: {=bool:?}, TLPIEX: {=bool:?}, RLPIEN: {=bool:?}, RLPIEX: {=bool:?}, TLPIST: {=bool:?}, RLPIST: {=bool:?}, LPIEN: {=bool:?}, PLS: {=bool:?}, LPITXA: {=bool:?}, LPIATE: {=bool:?}, LPITCSE: {=bool:?} }}" , self . TLPIEN () , self . TLPIEX () , self . RLPIEN () , self . RLPIEX () , self . TLPIST () , self . RLPIST () , self . LPIEN () , self . PLS () , self . LPITXA () , self . LPIATE () , self . LPITCSE ())
        }
    }
    #[doc = "Tx LPI Entry Timer Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_LPI_ENTRY_TIMER(pub u32);
    impl MAC_LPI_ENTRY_TIMER {
        #[must_use]
        #[inline(always)]
        pub const fn LPIET(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x0001_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_LPIET(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 3usize)) | (((val as u32) & 0x0001_ffff) << 3usize);
        }
    }
    impl Default for MAC_LPI_ENTRY_TIMER {
        #[inline(always)]
        fn default() -> MAC_LPI_ENTRY_TIMER {
            MAC_LPI_ENTRY_TIMER(0)
        }
    }
    impl core::fmt::Debug for MAC_LPI_ENTRY_TIMER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_LPI_ENTRY_TIMER")
                .field("LPIET", &self.LPIET())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_LPI_ENTRY_TIMER {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MAC_LPI_ENTRY_TIMER {{ LPIET: {=u32:?} }}", self.LPIET())
        }
    }
    #[doc = "LPI Timers Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_LPI_TIMERS_CONTROL(pub u32);
    impl MAC_LPI_TIMERS_CONTROL {
        #[must_use]
        #[inline(always)]
        pub const fn TWT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TWT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LST(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_LST(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
    }
    impl Default for MAC_LPI_TIMERS_CONTROL {
        #[inline(always)]
        fn default() -> MAC_LPI_TIMERS_CONTROL {
            MAC_LPI_TIMERS_CONTROL(0)
        }
    }
    impl core::fmt::Debug for MAC_LPI_TIMERS_CONTROL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_LPI_TIMERS_CONTROL")
                .field("TWT", &self.TWT())
                .field("LST", &self.LST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_LPI_TIMERS_CONTROL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_LPI_TIMERS_CONTROL {{ TWT: {=u16:?}, LST: {=u16:?} }}",
                self.TWT(),
                self.LST()
            )
        }
    }
    #[doc = "MDIO Address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_MDIO_ADDRESS(pub u32);
    impl MAC_MDIO_ADDRESS {
        #[must_use]
        #[inline(always)]
        pub const fn GB(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn C45E(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_C45E(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GOC_0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GOC_0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GOC_1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GOC_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SKAP(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_SKAP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CR(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_CR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NTC(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_NTC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RDA(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RDA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PA(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PA(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 21usize)) | (((val as u32) & 0x1f) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BTB(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BTB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PSE(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PSE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MAC_MDIO_ADDRESS {
        #[inline(always)]
        fn default() -> MAC_MDIO_ADDRESS {
            MAC_MDIO_ADDRESS(0)
        }
    }
    impl core::fmt::Debug for MAC_MDIO_ADDRESS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_MDIO_ADDRESS")
                .field("GB", &self.GB())
                .field("C45E", &self.C45E())
                .field("GOC_0", &self.GOC_0())
                .field("GOC_1", &self.GOC_1())
                .field("SKAP", &self.SKAP())
                .field("CR", &self.CR())
                .field("NTC", &self.NTC())
                .field("RDA", &self.RDA())
                .field("PA", &self.PA())
                .field("BTB", &self.BTB())
                .field("PSE", &self.PSE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_MDIO_ADDRESS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_MDIO_ADDRESS {{ GB: {=bool:?}, C45E: {=bool:?}, GOC_0: {=bool:?}, GOC_1: {=bool:?}, SKAP: {=bool:?}, CR: {=u8:?}, NTC: {=u8:?}, RDA: {=u8:?}, PA: {=u8:?}, BTB: {=bool:?}, PSE: {=bool:?} }}" , self . GB () , self . C45E () , self . GOC_0 () , self . GOC_1 () , self . SKAP () , self . CR () , self . NTC () , self . RDA () , self . PA () , self . BTB () , self . PSE ())
        }
    }
    #[doc = "MAC MDIO Data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_MDIO_DATA(pub u32);
    impl MAC_MDIO_DATA {
        #[must_use]
        #[inline(always)]
        pub const fn GD(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_GD(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RA(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_RA(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MAC_MDIO_DATA {
        #[inline(always)]
        fn default() -> MAC_MDIO_DATA {
            MAC_MDIO_DATA(0)
        }
    }
    impl core::fmt::Debug for MAC_MDIO_DATA {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_MDIO_DATA")
                .field("GD", &self.GD())
                .field("RA", &self.RA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_MDIO_DATA {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_MDIO_DATA {{ GD: {=u16:?}, RA: {=u16:?} }}",
                self.GD(),
                self.RA()
            )
        }
    }
    #[doc = "One-microsecond Reference Timer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_ONEUS_TIC_COUNTER(pub u32);
    impl MAC_ONEUS_TIC_COUNTER {
        #[must_use]
        #[inline(always)]
        pub const fn TIC_1US_CNTR(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_TIC_1US_CNTR(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for MAC_ONEUS_TIC_COUNTER {
        #[inline(always)]
        fn default() -> MAC_ONEUS_TIC_COUNTER {
            MAC_ONEUS_TIC_COUNTER(0)
        }
    }
    impl core::fmt::Debug for MAC_ONEUS_TIC_COUNTER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_ONEUS_TIC_COUNTER")
                .field("TIC_1US_CNTR", &self.TIC_1US_CNTR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_ONEUS_TIC_COUNTER {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_ONEUS_TIC_COUNTER {{ TIC_1US_CNTR: {=u16:?} }}",
                self.TIC_1US_CNTR()
            )
        }
    }
    #[doc = "MAC Packet Filter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_PACKET_FILTER(pub u32);
    impl MAC_PACKET_FILTER {
        #[must_use]
        #[inline(always)]
        pub const fn PR(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DAIF(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DAIF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PM(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DBF(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DBF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PCF(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PCF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VTFE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VTFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RA(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MAC_PACKET_FILTER {
        #[inline(always)]
        fn default() -> MAC_PACKET_FILTER {
            MAC_PACKET_FILTER(0)
        }
    }
    impl core::fmt::Debug for MAC_PACKET_FILTER {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_PACKET_FILTER")
                .field("PR", &self.PR())
                .field("DAIF", &self.DAIF())
                .field("PM", &self.PM())
                .field("DBF", &self.DBF())
                .field("PCF", &self.PCF())
                .field("VTFE", &self.VTFE())
                .field("RA", &self.RA())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_PACKET_FILTER {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_PACKET_FILTER {{ PR: {=bool:?}, DAIF: {=bool:?}, PM: {=bool:?}, DBF: {=bool:?}, PCF: {=u8:?}, VTFE: {=bool:?}, RA: {=bool:?} }}" , self . PR () , self . DAIF () , self . PM () , self . DBF () , self . PCF () , self . VTFE () , self . RA ())
        }
    }
    #[doc = "PMT Control and Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_PMT_CONTROL_STATUS(pub u32);
    impl MAC_PMT_CONTROL_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn PWRDWN(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWRDWN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MGKPKTEN(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MGKPKTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RWKPKTEN(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RWKPKTEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MGKPRCVD(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MGKPRCVD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RWKPRCVD(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RWKPRCVD(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn GLBLUCAST(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_GLBLUCAST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RWKPFE(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RWKPFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RWKPTR(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RWKPTR(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RWKFILTRST(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RWKFILTRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MAC_PMT_CONTROL_STATUS {
        #[inline(always)]
        fn default() -> MAC_PMT_CONTROL_STATUS {
            MAC_PMT_CONTROL_STATUS(0)
        }
    }
    impl core::fmt::Debug for MAC_PMT_CONTROL_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_PMT_CONTROL_STATUS")
                .field("PWRDWN", &self.PWRDWN())
                .field("MGKPKTEN", &self.MGKPKTEN())
                .field("RWKPKTEN", &self.RWKPKTEN())
                .field("MGKPRCVD", &self.MGKPRCVD())
                .field("RWKPRCVD", &self.RWKPRCVD())
                .field("GLBLUCAST", &self.GLBLUCAST())
                .field("RWKPFE", &self.RWKPFE())
                .field("RWKPTR", &self.RWKPTR())
                .field("RWKFILTRST", &self.RWKFILTRST())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_PMT_CONTROL_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_PMT_CONTROL_STATUS {{ PWRDWN: {=bool:?}, MGKPKTEN: {=bool:?}, RWKPKTEN: {=bool:?}, MGKPRCVD: {=bool:?}, RWKPRCVD: {=bool:?}, GLBLUCAST: {=bool:?}, RWKPFE: {=bool:?}, RWKPTR: {=u8:?}, RWKFILTRST: {=bool:?} }}" , self . PWRDWN () , self . MGKPKTEN () , self . RWKPKTEN () , self . MGKPRCVD () , self . RWKPRCVD () , self . GLBLUCAST () , self . RWKPFE () , self . RWKPTR () , self . RWKFILTRST ())
        }
    }
    #[doc = "PPS Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_PPS_CONTROL(pub u32);
    impl MAC_PPS_CONTROL {
        #[must_use]
        #[inline(always)]
        pub const fn PPSCTRL_PPSCMD(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PPSCTRL_PPSCMD(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for MAC_PPS_CONTROL {
        #[inline(always)]
        fn default() -> MAC_PPS_CONTROL {
            MAC_PPS_CONTROL(0)
        }
    }
    impl core::fmt::Debug for MAC_PPS_CONTROL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_PPS_CONTROL")
                .field("PPSCTRL_PPSCMD", &self.PPSCTRL_PPSCMD())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_PPS_CONTROL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_PPS_CONTROL {{ PPSCTRL_PPSCMD: {=u8:?} }}",
                self.PPSCTRL_PPSCMD()
            )
        }
    }
    #[doc = "Receive Queue Control 0..Receive Queue Control 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_RXQ_CTRL(pub u32);
    impl MAC_RXQ_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn AVCPQ(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_AVCPQ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PSRQ0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PSRQ0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXQ0EN(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RXQ0EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXQ1EN(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RXQ1EN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PTPQ(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PTPQ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PSRQ1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PSRQ1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UPQ(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_UPQ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MCBCQ(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_MCBCQ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MCBCQEN(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MCBCQEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TACPQE(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TACPQE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TPQC(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TPQC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OMCBCQ(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OMCBCQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TBRQE(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TBRQE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for MAC_RXQ_CTRL {
        #[inline(always)]
        fn default() -> MAC_RXQ_CTRL {
            MAC_RXQ_CTRL(0)
        }
    }
    impl core::fmt::Debug for MAC_RXQ_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_RXQ_CTRL")
                .field("AVCPQ", &self.AVCPQ())
                .field("PSRQ0", &self.PSRQ0())
                .field("RXQ0EN", &self.RXQ0EN())
                .field("RXQ1EN", &self.RXQ1EN())
                .field("PTPQ", &self.PTPQ())
                .field("PSRQ1", &self.PSRQ1())
                .field("UPQ", &self.UPQ())
                .field("MCBCQ", &self.MCBCQ())
                .field("MCBCQEN", &self.MCBCQEN())
                .field("TACPQE", &self.TACPQE())
                .field("TPQC", &self.TPQC())
                .field("OMCBCQ", &self.OMCBCQ())
                .field("TBRQE", &self.TBRQE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_RXQ_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_RXQ_CTRL {{ AVCPQ: {=u8:?}, PSRQ0: {=u8:?}, RXQ0EN: {=u8:?}, RXQ1EN: {=u8:?}, PTPQ: {=u8:?}, PSRQ1: {=u8:?}, UPQ: {=u8:?}, MCBCQ: {=u8:?}, MCBCQEN: {=bool:?}, TACPQE: {=bool:?}, TPQC: {=u8:?}, OMCBCQ: {=bool:?}, TBRQE: {=bool:?} }}" , self . AVCPQ () , self . PSRQ0 () , self . RXQ0EN () , self . RXQ1EN () , self . PTPQ () , self . PSRQ1 () , self . UPQ () , self . MCBCQ () , self . MCBCQEN () , self . TACPQE () , self . TPQC () , self . OMCBCQ () , self . TBRQE ())
        }
    }
    #[doc = "Receive Queue Control 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_RXQ_CTRL4(pub u32);
    impl MAC_RXQ_CTRL4 {
        #[must_use]
        #[inline(always)]
        pub const fn UFFQE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UFFQE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UFFQ(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UFFQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MFFQE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MFFQE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MFFQ(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MFFQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VFFQE(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VFFQE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VFFQ(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VFFQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for MAC_RXQ_CTRL4 {
        #[inline(always)]
        fn default() -> MAC_RXQ_CTRL4 {
            MAC_RXQ_CTRL4(0)
        }
    }
    impl core::fmt::Debug for MAC_RXQ_CTRL4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_RXQ_CTRL4")
                .field("UFFQE", &self.UFFQE())
                .field("UFFQ", &self.UFFQ())
                .field("MFFQE", &self.MFFQE())
                .field("MFFQ", &self.MFFQ())
                .field("VFFQE", &self.VFFQE())
                .field("VFFQ", &self.VFFQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_RXQ_CTRL4 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_RXQ_CTRL4 {{ UFFQE: {=bool:?}, UFFQ: {=bool:?}, MFFQE: {=bool:?}, MFFQ: {=bool:?}, VFFQE: {=bool:?}, VFFQ: {=bool:?} }}" , self . UFFQE () , self . UFFQ () , self . MFFQE () , self . MFFQ () , self . VFFQE () , self . VFFQ ())
        }
    }
    #[doc = "MAC Rx Flow Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_RX_FLOW_CTRL(pub u32);
    impl MAC_RX_FLOW_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn RFE(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UP(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for MAC_RX_FLOW_CTRL {
        #[inline(always)]
        fn default() -> MAC_RX_FLOW_CTRL {
            MAC_RX_FLOW_CTRL(0)
        }
    }
    impl core::fmt::Debug for MAC_RX_FLOW_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_RX_FLOW_CTRL")
                .field("RFE", &self.RFE())
                .field("UP", &self.UP())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_RX_FLOW_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_RX_FLOW_CTRL {{ RFE: {=bool:?}, UP: {=bool:?} }}",
                self.RFE(),
                self.UP()
            )
        }
    }
    #[doc = "Receive Transmit Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_RX_TX_STATUS(pub u32);
    impl MAC_RX_TX_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn TJT(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TJT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn NCARR(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_NCARR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCARR(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LCARR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXDEF(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EXDEF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn LCOL(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_LCOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EXCOL(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EXCOL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RWT(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RWT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for MAC_RX_TX_STATUS {
        #[inline(always)]
        fn default() -> MAC_RX_TX_STATUS {
            MAC_RX_TX_STATUS(0)
        }
    }
    impl core::fmt::Debug for MAC_RX_TX_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_RX_TX_STATUS")
                .field("TJT", &self.TJT())
                .field("NCARR", &self.NCARR())
                .field("LCARR", &self.LCARR())
                .field("EXDEF", &self.EXDEF())
                .field("LCOL", &self.LCOL())
                .field("EXCOL", &self.EXCOL())
                .field("RWT", &self.RWT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_RX_TX_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_RX_TX_STATUS {{ TJT: {=bool:?}, NCARR: {=bool:?}, LCARR: {=bool:?}, EXDEF: {=bool:?}, LCOL: {=bool:?}, EXCOL: {=bool:?}, RWT: {=bool:?} }}" , self . TJT () , self . NCARR () , self . LCARR () , self . EXDEF () , self . LCOL () , self . EXCOL () , self . RWT ())
        }
    }
    #[doc = "Subsecond Increment"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_SUB_SECOND_INCREMENT(pub u32);
    impl MAC_SUB_SECOND_INCREMENT {
        #[must_use]
        #[inline(always)]
        pub const fn SNSINC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SNSINC(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for MAC_SUB_SECOND_INCREMENT {
        #[inline(always)]
        fn default() -> MAC_SUB_SECOND_INCREMENT {
            MAC_SUB_SECOND_INCREMENT(0)
        }
    }
    impl core::fmt::Debug for MAC_SUB_SECOND_INCREMENT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_SUB_SECOND_INCREMENT")
                .field("SNSINC", &self.SNSINC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_SUB_SECOND_INCREMENT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_SUB_SECOND_INCREMENT {{ SNSINC: {=u8:?} }}",
                self.SNSINC()
            )
        }
    }
    #[doc = "System Time Nanoseconds"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_SYSTEM_TIME_NANOSECONDS(pub u32);
    impl MAC_SYSTEM_TIME_NANOSECONDS {
        #[must_use]
        #[inline(always)]
        pub const fn TSSS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_TSSS(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for MAC_SYSTEM_TIME_NANOSECONDS {
        #[inline(always)]
        fn default() -> MAC_SYSTEM_TIME_NANOSECONDS {
            MAC_SYSTEM_TIME_NANOSECONDS(0)
        }
    }
    impl core::fmt::Debug for MAC_SYSTEM_TIME_NANOSECONDS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_SYSTEM_TIME_NANOSECONDS")
                .field("TSSS", &self.TSSS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_SYSTEM_TIME_NANOSECONDS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_SYSTEM_TIME_NANOSECONDS {{ TSSS: {=u32:?} }}",
                self.TSSS()
            )
        }
    }
    #[doc = "System Time Nanoseconds Update"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_SYSTEM_TIME_NANOSECONDS_UPDATE(pub u32);
    impl MAC_SYSTEM_TIME_NANOSECONDS_UPDATE {
        #[must_use]
        #[inline(always)]
        pub const fn TSSS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_TSSS(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDSUB(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ADDSUB(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MAC_SYSTEM_TIME_NANOSECONDS_UPDATE {
        #[inline(always)]
        fn default() -> MAC_SYSTEM_TIME_NANOSECONDS_UPDATE {
            MAC_SYSTEM_TIME_NANOSECONDS_UPDATE(0)
        }
    }
    impl core::fmt::Debug for MAC_SYSTEM_TIME_NANOSECONDS_UPDATE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_SYSTEM_TIME_NANOSECONDS_UPDATE")
                .field("TSSS", &self.TSSS())
                .field("ADDSUB", &self.ADDSUB())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_SYSTEM_TIME_NANOSECONDS_UPDATE {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_SYSTEM_TIME_NANOSECONDS_UPDATE {{ TSSS: {=u32:?}, ADDSUB: {=bool:?} }}",
                self.TSSS(),
                self.ADDSUB()
            )
        }
    }
    #[doc = "Timestamp Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_TIMESTAMP_CONTROL(pub u32);
    impl MAC_TIMESTAMP_CONTROL {
        #[must_use]
        #[inline(always)]
        pub const fn TSENA(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSCFUPDT(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSCFUPDT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSINIT(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSINIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSUPDT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSUPDT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSTRIG(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSTRIG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSADDREG(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSADDREG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSENALL(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSENALL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSCTRLSSR(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSCTRLSSR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSVER2ENA(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSVER2ENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSIPENA(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSIPENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSIPV6ENA(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSIPV6ENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSIPV4ENA(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSIPV4ENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSEVNTENA(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSEVNTENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSMSTRENA(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSMSTRENA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SNAPTYPSEL(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SNAPTYPSEL(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSENMACADDR(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSENMACADDR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ESTI(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ESTI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXTSSTSM(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXTSSTSM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn AV8021ASMEN(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AV8021ASMEN(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for MAC_TIMESTAMP_CONTROL {
        #[inline(always)]
        fn default() -> MAC_TIMESTAMP_CONTROL {
            MAC_TIMESTAMP_CONTROL(0)
        }
    }
    impl core::fmt::Debug for MAC_TIMESTAMP_CONTROL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_TIMESTAMP_CONTROL")
                .field("TSENA", &self.TSENA())
                .field("TSCFUPDT", &self.TSCFUPDT())
                .field("TSINIT", &self.TSINIT())
                .field("TSUPDT", &self.TSUPDT())
                .field("TSTRIG", &self.TSTRIG())
                .field("TSADDREG", &self.TSADDREG())
                .field("TSENALL", &self.TSENALL())
                .field("TSCTRLSSR", &self.TSCTRLSSR())
                .field("TSVER2ENA", &self.TSVER2ENA())
                .field("TSIPENA", &self.TSIPENA())
                .field("TSIPV6ENA", &self.TSIPV6ENA())
                .field("TSIPV4ENA", &self.TSIPV4ENA())
                .field("TSEVNTENA", &self.TSEVNTENA())
                .field("TSMSTRENA", &self.TSMSTRENA())
                .field("SNAPTYPSEL", &self.SNAPTYPSEL())
                .field("TSENMACADDR", &self.TSENMACADDR())
                .field("ESTI", &self.ESTI())
                .field("TXTSSTSM", &self.TXTSSTSM())
                .field("AV8021ASMEN", &self.AV8021ASMEN())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_TIMESTAMP_CONTROL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_TIMESTAMP_CONTROL {{ TSENA: {=bool:?}, TSCFUPDT: {=bool:?}, TSINIT: {=bool:?}, TSUPDT: {=bool:?}, TSTRIG: {=bool:?}, TSADDREG: {=bool:?}, TSENALL: {=bool:?}, TSCTRLSSR: {=bool:?}, TSVER2ENA: {=bool:?}, TSIPENA: {=bool:?}, TSIPV6ENA: {=bool:?}, TSIPV4ENA: {=bool:?}, TSEVNTENA: {=bool:?}, TSMSTRENA: {=bool:?}, SNAPTYPSEL: {=u8:?}, TSENMACADDR: {=bool:?}, ESTI: {=bool:?}, TXTSSTSM: {=bool:?}, AV8021ASMEN: {=bool:?} }}" , self . TSENA () , self . TSCFUPDT () , self . TSINIT () , self . TSUPDT () , self . TSTRIG () , self . TSADDREG () , self . TSENALL () , self . TSCTRLSSR () , self . TSVER2ENA () , self . TSIPENA () , self . TSIPV6ENA () , self . TSIPV4ENA () , self . TSEVNTENA () , self . TSMSTRENA () , self . SNAPTYPSEL () , self . TSENMACADDR () , self . ESTI () , self . TXTSSTSM () , self . AV8021ASMEN ())
        }
    }
    #[doc = "Timestamp Egress Latency"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_TIMESTAMP_EGRESS_LATENCY(pub u32);
    impl MAC_TIMESTAMP_EGRESS_LATENCY {
        #[must_use]
        #[inline(always)]
        pub const fn ETLSNS(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ETLSNS(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ETLNS(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ETLNS(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for MAC_TIMESTAMP_EGRESS_LATENCY {
        #[inline(always)]
        fn default() -> MAC_TIMESTAMP_EGRESS_LATENCY {
            MAC_TIMESTAMP_EGRESS_LATENCY(0)
        }
    }
    impl core::fmt::Debug for MAC_TIMESTAMP_EGRESS_LATENCY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_TIMESTAMP_EGRESS_LATENCY")
                .field("ETLSNS", &self.ETLSNS())
                .field("ETLNS", &self.ETLNS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_TIMESTAMP_EGRESS_LATENCY {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_TIMESTAMP_EGRESS_LATENCY {{ ETLSNS: {=u8:?}, ETLNS: {=u16:?} }}",
                self.ETLSNS(),
                self.ETLNS()
            )
        }
    }
    #[doc = "Timestamp Ingress Latency"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_TIMESTAMP_INGRESS_LATENCY(pub u32);
    impl MAC_TIMESTAMP_INGRESS_LATENCY {
        #[must_use]
        #[inline(always)]
        pub const fn ITLSNS(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_ITLSNS(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ITLNS(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_ITLNS(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for MAC_TIMESTAMP_INGRESS_LATENCY {
        #[inline(always)]
        fn default() -> MAC_TIMESTAMP_INGRESS_LATENCY {
            MAC_TIMESTAMP_INGRESS_LATENCY(0)
        }
    }
    impl core::fmt::Debug for MAC_TIMESTAMP_INGRESS_LATENCY {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_TIMESTAMP_INGRESS_LATENCY")
                .field("ITLSNS", &self.ITLSNS())
                .field("ITLNS", &self.ITLNS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_TIMESTAMP_INGRESS_LATENCY {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_TIMESTAMP_INGRESS_LATENCY {{ ITLSNS: {=u8:?}, ITLNS: {=u16:?} }}",
                self.ITLSNS(),
                self.ITLNS()
            )
        }
    }
    #[doc = "Timestamp Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_TIMESTAMP_STATUS(pub u32);
    impl MAC_TIMESTAMP_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn TSSOVF(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSSOVF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSTARGT0(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSTARGT0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSTRGTERR0(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSTRGTERR0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXTSSIS(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXTSSIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for MAC_TIMESTAMP_STATUS {
        #[inline(always)]
        fn default() -> MAC_TIMESTAMP_STATUS {
            MAC_TIMESTAMP_STATUS(0)
        }
    }
    impl core::fmt::Debug for MAC_TIMESTAMP_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_TIMESTAMP_STATUS")
                .field("TSSOVF", &self.TSSOVF())
                .field("TSTARGT0", &self.TSTARGT0())
                .field("TSTRGTERR0", &self.TSTRGTERR0())
                .field("TXTSSIS", &self.TXTSSIS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_TIMESTAMP_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_TIMESTAMP_STATUS {{ TSSOVF: {=bool:?}, TSTARGT0: {=bool:?}, TSTRGTERR0: {=bool:?}, TXTSSIS: {=bool:?} }}" , self . TSSOVF () , self . TSTARGT0 () , self . TSTRGTERR0 () , self . TXTSSIS ())
        }
    }
    #[doc = "MAC Q0 Tx Flow Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_TX_FLOW_CTRL_Q(pub u32);
    impl MAC_TX_FLOW_CTRL_Q {
        #[must_use]
        #[inline(always)]
        pub const fn FCB_BPA(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FCB_BPA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TFE(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TFE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PLT(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PLT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DZPQ(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DZPQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_PT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for MAC_TX_FLOW_CTRL_Q {
        #[inline(always)]
        fn default() -> MAC_TX_FLOW_CTRL_Q {
            MAC_TX_FLOW_CTRL_Q(0)
        }
    }
    impl core::fmt::Debug for MAC_TX_FLOW_CTRL_Q {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_TX_FLOW_CTRL_Q")
                .field("FCB_BPA", &self.FCB_BPA())
                .field("TFE", &self.TFE())
                .field("PLT", &self.PLT())
                .field("DZPQ", &self.DZPQ())
                .field("PT", &self.PT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_TX_FLOW_CTRL_Q {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_TX_FLOW_CTRL_Q {{ FCB_BPA: {=bool:?}, TFE: {=bool:?}, PLT: {=u8:?}, DZPQ: {=bool:?}, PT: {=u16:?} }}" , self . FCB_BPA () , self . TFE () , self . PLT () , self . DZPQ () , self . PT ())
        }
    }
    #[doc = "Transmit Timestamp Status Nanoseconds"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_TX_TIMESTAMP_STATUS_NANOSECONDS(pub u32);
    impl MAC_TX_TIMESTAMP_STATUS_NANOSECONDS {
        #[must_use]
        #[inline(always)]
        pub const fn TXTSSLO(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_TXTSSLO(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXTSSMIS(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXTSSMIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MAC_TX_TIMESTAMP_STATUS_NANOSECONDS {
        #[inline(always)]
        fn default() -> MAC_TX_TIMESTAMP_STATUS_NANOSECONDS {
            MAC_TX_TIMESTAMP_STATUS_NANOSECONDS(0)
        }
    }
    impl core::fmt::Debug for MAC_TX_TIMESTAMP_STATUS_NANOSECONDS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_TX_TIMESTAMP_STATUS_NANOSECONDS")
                .field("TXTSSLO", &self.TXTSSLO())
                .field("TXTSSMIS", &self.TXTSSMIS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_TX_TIMESTAMP_STATUS_NANOSECONDS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_TX_TIMESTAMP_STATUS_NANOSECONDS {{ TXTSSLO: {=u32:?}, TXTSSMIS: {=bool:?} }}",
                self.TXTSSLO(),
                self.TXTSSMIS()
            )
        }
    }
    #[doc = "MAC Version"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_VERSION(pub u32);
    impl MAC_VERSION {
        #[must_use]
        #[inline(always)]
        pub const fn SNPSVER(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SNPSVER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn USERVER(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[inline(always)]
        pub const fn set_USERVER(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for MAC_VERSION {
        #[inline(always)]
        fn default() -> MAC_VERSION {
            MAC_VERSION(0)
        }
    }
    impl core::fmt::Debug for MAC_VERSION {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_VERSION")
                .field("SNPSVER", &self.SNPSVER())
                .field("USERVER", &self.USERVER())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_VERSION {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_VERSION {{ SNPSVER: {=u8:?}, USERVER: {=u8:?} }}",
                self.SNPSVER(),
                self.USERVER()
            )
        }
    }
    #[doc = "VLAN Tag Inclusion or Replacement"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_VLAN_INCL(pub u32);
    impl MAC_VLAN_INCL {
        #[must_use]
        #[inline(always)]
        pub const fn VLT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_VLT(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VLC(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_VLC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VLP(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VLP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CSVL(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CSVL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VLTI(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VLTI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CBTI(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CBTI(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ADDR(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ADDR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RDWR(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RDWR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn BUSY(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_BUSY(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MAC_VLAN_INCL {
        #[inline(always)]
        fn default() -> MAC_VLAN_INCL {
            MAC_VLAN_INCL(0)
        }
    }
    impl core::fmt::Debug for MAC_VLAN_INCL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_VLAN_INCL")
                .field("VLT", &self.VLT())
                .field("VLC", &self.VLC())
                .field("VLP", &self.VLP())
                .field("CSVL", &self.CSVL())
                .field("VLTI", &self.VLTI())
                .field("CBTI", &self.CBTI())
                .field("ADDR", &self.ADDR())
                .field("RDWR", &self.RDWR())
                .field("BUSY", &self.BUSY())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_VLAN_INCL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_VLAN_INCL {{ VLT: {=u16:?}, VLC: {=u8:?}, VLP: {=bool:?}, CSVL: {=bool:?}, VLTI: {=bool:?}, CBTI: {=bool:?}, ADDR: {=bool:?}, RDWR: {=bool:?}, BUSY: {=bool:?} }}" , self . VLT () , self . VLC () , self . VLP () , self . CSVL () , self . VLTI () , self . CBTI () , self . ADDR () , self . RDWR () , self . BUSY ())
        }
    }
    #[doc = "MAC VLAN Tag Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_VLAN_TAG_CTRL(pub u32);
    impl MAC_VLAN_TAG_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn VL(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_VL(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ETV(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ETV(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn VTIM(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_VTIM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ESVL(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ESVL(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERSVLM(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERSVLM(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DOVLTC(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DOVLTC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVLS(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EVLS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EVLRXS(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EVLRXS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EDVLP(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EDVLP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ERIVLT(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ERIVLT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EIVLS(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_EIVLS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn EIVLRXS(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_EIVLRXS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MAC_VLAN_TAG_CTRL {
        #[inline(always)]
        fn default() -> MAC_VLAN_TAG_CTRL {
            MAC_VLAN_TAG_CTRL(0)
        }
    }
    impl core::fmt::Debug for MAC_VLAN_TAG_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_VLAN_TAG_CTRL")
                .field("VL", &self.VL())
                .field("ETV", &self.ETV())
                .field("VTIM", &self.VTIM())
                .field("ESVL", &self.ESVL())
                .field("ERSVLM", &self.ERSVLM())
                .field("DOVLTC", &self.DOVLTC())
                .field("EVLS", &self.EVLS())
                .field("EVLRXS", &self.EVLRXS())
                .field("EDVLP", &self.EDVLP())
                .field("ERIVLT", &self.ERIVLT())
                .field("EIVLS", &self.EIVLS())
                .field("EIVLRXS", &self.EIVLRXS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_VLAN_TAG_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MAC_VLAN_TAG_CTRL {{ VL: {=u16:?}, ETV: {=bool:?}, VTIM: {=bool:?}, ESVL: {=bool:?}, ERSVLM: {=bool:?}, DOVLTC: {=bool:?}, EVLS: {=u8:?}, EVLRXS: {=bool:?}, EDVLP: {=bool:?}, ERIVLT: {=bool:?}, EIVLS: {=u8:?}, EIVLRXS: {=bool:?} }}" , self . VL () , self . ETV () , self . VTIM () , self . ESVL () , self . ERSVLM () , self . DOVLTC () , self . EVLS () , self . EVLRXS () , self . EDVLP () , self . ERIVLT () , self . EIVLS () , self . EIVLRXS ())
        }
    }
    #[doc = "Watchdog Timeout"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MAC_WATCHDOG_TIMEOUT(pub u32);
    impl MAC_WATCHDOG_TIMEOUT {
        #[must_use]
        #[inline(always)]
        pub const fn WTO(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[inline(always)]
        pub const fn set_WTO(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PWE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_PWE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for MAC_WATCHDOG_TIMEOUT {
        #[inline(always)]
        fn default() -> MAC_WATCHDOG_TIMEOUT {
            MAC_WATCHDOG_TIMEOUT(0)
        }
    }
    impl core::fmt::Debug for MAC_WATCHDOG_TIMEOUT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MAC_WATCHDOG_TIMEOUT")
                .field("WTO", &self.WTO())
                .field("PWE", &self.PWE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MAC_WATCHDOG_TIMEOUT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MAC_WATCHDOG_TIMEOUT {{ WTO: {=u8:?}, PWE: {=bool:?} }}",
                self.WTO(),
                self.PWE()
            )
        }
    }
    #[doc = "MTL Interrupt Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_INTERRUPT_STATUS(pub u32);
    impl MTL_INTERRUPT_STATUS {
        #[must_use]
        #[inline(always)]
        pub const fn Q0IS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Q0IS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Q1IS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Q1IS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for MTL_INTERRUPT_STATUS {
        #[inline(always)]
        fn default() -> MTL_INTERRUPT_STATUS {
            MTL_INTERRUPT_STATUS(0)
        }
    }
    impl core::fmt::Debug for MTL_INTERRUPT_STATUS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_INTERRUPT_STATUS")
                .field("Q0IS", &self.Q0IS())
                .field("Q1IS", &self.Q1IS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_INTERRUPT_STATUS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MTL_INTERRUPT_STATUS {{ Q0IS: {=bool:?}, Q1IS: {=bool:?} }}",
                self.Q0IS(),
                self.Q1IS()
            )
        }
    }
    #[doc = "MTL Operation Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_OPERATION_MODE(pub u32);
    impl MTL_OPERATION_MODE {
        #[must_use]
        #[inline(always)]
        pub const fn DTXSTS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DTXSTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RAA(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RAA(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SCHALG(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SCHALG(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CNTPRST(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CNTPRST(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CNTCLR(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CNTCLR(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for MTL_OPERATION_MODE {
        #[inline(always)]
        fn default() -> MTL_OPERATION_MODE {
            MTL_OPERATION_MODE(0)
        }
    }
    impl core::fmt::Debug for MTL_OPERATION_MODE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_OPERATION_MODE")
                .field("DTXSTS", &self.DTXSTS())
                .field("RAA", &self.RAA())
                .field("SCHALG", &self.SCHALG())
                .field("CNTPRST", &self.CNTPRST())
                .field("CNTCLR", &self.CNTCLR())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_OPERATION_MODE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MTL_OPERATION_MODE {{ DTXSTS: {=bool:?}, RAA: {=bool:?}, SCHALG: {=u8:?}, CNTPRST: {=bool:?}, CNTCLR: {=bool:?} }}" , self . DTXSTS () , self . RAA () , self . SCHALG () , self . CNTPRST () , self . CNTCLR ())
        }
    }
    #[doc = "Queue 0 Interrupt Control Status..Queue 1 Interrupt Control Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_QX_INTCTRL_STAT(pub u32);
    impl MTL_QUEUE_MTL_QX_INTCTRL_STAT {
        #[must_use]
        #[inline(always)]
        pub const fn TXUNFIS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXUNFIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ABPSIS(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ABPSIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXUIE(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXUIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn ABPSIE(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ABPSIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXOVFIS(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXOVFIS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXOIE(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXOIE(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_QX_INTCTRL_STAT {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_QX_INTCTRL_STAT {
            MTL_QUEUE_MTL_QX_INTCTRL_STAT(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_QX_INTCTRL_STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_QX_INTCTRL_STAT")
                .field("TXUNFIS", &self.TXUNFIS())
                .field("ABPSIS", &self.ABPSIS())
                .field("TXUIE", &self.TXUIE())
                .field("ABPSIE", &self.ABPSIE())
                .field("RXOVFIS", &self.RXOVFIS())
                .field("RXOIE", &self.RXOIE())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_QX_INTCTRL_STAT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MTL_QUEUE_MTL_QX_INTCTRL_STAT {{ TXUNFIS: {=bool:?}, ABPSIS: {=bool:?}, TXUIE: {=bool:?}, ABPSIE: {=bool:?}, RXOVFIS: {=bool:?}, RXOIE: {=bool:?} }}" , self . TXUNFIS () , self . ABPSIS () , self . TXUIE () , self . ABPSIE () , self . RXOVFIS () , self . RXOIE ())
        }
    }
    #[doc = "Queue 0 Receive Control..Queue 1 Receive Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_RXQX_CTRL(pub u32);
    impl MTL_QUEUE_MTL_RXQX_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn RXQ_WEGT(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RXQ_WEGT(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXQ_FRM_ARBIT(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RXQ_FRM_ARBIT(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_RXQX_CTRL {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_RXQX_CTRL {
            MTL_QUEUE_MTL_RXQX_CTRL(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_RXQX_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_RXQX_CTRL")
                .field("RXQ_WEGT", &self.RXQ_WEGT())
                .field("RXQ_FRM_ARBIT", &self.RXQ_FRM_ARBIT())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_RXQX_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MTL_QUEUE_MTL_RXQX_CTRL {{ RXQ_WEGT: {=u8:?}, RXQ_FRM_ARBIT: {=bool:?} }}",
                self.RXQ_WEGT(),
                self.RXQ_FRM_ARBIT()
            )
        }
    }
    #[doc = "Queue 0 Receive Debug..Queue 1 Receive Debug"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_RXQX_DBG(pub u32);
    impl MTL_QUEUE_MTL_RXQX_DBG {
        #[must_use]
        #[inline(always)]
        pub const fn RWCSTS(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RWCSTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RRCSTS(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RRCSTS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RXQSTS(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RXQSTS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PRXQ(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_PRXQ(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_RXQX_DBG {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_RXQX_DBG {
            MTL_QUEUE_MTL_RXQX_DBG(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_RXQX_DBG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_RXQX_DBG")
                .field("RWCSTS", &self.RWCSTS())
                .field("RRCSTS", &self.RRCSTS())
                .field("RXQSTS", &self.RXQSTS())
                .field("PRXQ", &self.PRXQ())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_RXQX_DBG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MTL_QUEUE_MTL_RXQX_DBG {{ RWCSTS: {=bool:?}, RRCSTS: {=u8:?}, RXQSTS: {=u8:?}, PRXQ: {=u16:?} }}" , self . RWCSTS () , self . RRCSTS () , self . RXQSTS () , self . PRXQ ())
        }
    }
    #[doc = "Queue 0 Missed Packet and Overflow Counter..Queue 1 Missed Packet and Overflow Counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_RXQX_MISSPKT_OVRFLW_CNT(pub u32);
    impl MTL_QUEUE_MTL_RXQX_MISSPKT_OVRFLW_CNT {
        #[must_use]
        #[inline(always)]
        pub const fn OVFPKTCNT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_OVFPKTCNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn OVFCNTOVF(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_OVFCNTOVF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MISPKTCNT(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_MISPKTCNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn MISCNTOVF(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_MISCNTOVF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_RXQX_MISSPKT_OVRFLW_CNT {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_RXQX_MISSPKT_OVRFLW_CNT {
            MTL_QUEUE_MTL_RXQX_MISSPKT_OVRFLW_CNT(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_RXQX_MISSPKT_OVRFLW_CNT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_RXQX_MISSPKT_OVRFLW_CNT")
                .field("OVFPKTCNT", &self.OVFPKTCNT())
                .field("OVFCNTOVF", &self.OVFCNTOVF())
                .field("MISPKTCNT", &self.MISPKTCNT())
                .field("MISCNTOVF", &self.MISCNTOVF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_RXQX_MISSPKT_OVRFLW_CNT {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MTL_QUEUE_MTL_RXQX_MISSPKT_OVRFLW_CNT {{ OVFPKTCNT: {=u16:?}, OVFCNTOVF: {=bool:?}, MISPKTCNT: {=u16:?}, MISCNTOVF: {=bool:?} }}" , self . OVFPKTCNT () , self . OVFCNTOVF () , self . MISPKTCNT () , self . MISCNTOVF ())
        }
    }
    #[doc = "Queue 0 Receive Operation Mode..Queue 1 Receive Operation Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_RXQX_OP_MODE(pub u32);
    impl MTL_QUEUE_MTL_RXQX_OP_MODE {
        #[must_use]
        #[inline(always)]
        pub const fn RTC(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RTC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FUP(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FUP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn FEP(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FEP(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RSF(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_RSF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn DIS_TCP_EF(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_DIS_TCP_EF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn RQS(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_RQS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_RXQX_OP_MODE {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_RXQX_OP_MODE {
            MTL_QUEUE_MTL_RXQX_OP_MODE(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_RXQX_OP_MODE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_RXQX_OP_MODE")
                .field("RTC", &self.RTC())
                .field("FUP", &self.FUP())
                .field("FEP", &self.FEP())
                .field("RSF", &self.RSF())
                .field("DIS_TCP_EF", &self.DIS_TCP_EF())
                .field("RQS", &self.RQS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_RXQX_OP_MODE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MTL_QUEUE_MTL_RXQX_OP_MODE {{ RTC: {=u8:?}, FUP: {=bool:?}, FEP: {=bool:?}, RSF: {=bool:?}, DIS_TCP_EF: {=bool:?}, RQS: {=u8:?} }}" , self . RTC () , self . FUP () , self . FEP () , self . RSF () , self . DIS_TCP_EF () , self . RQS ())
        }
    }
    #[doc = "Queue 0 Transmit Debug..Queue 1 Transmit Debug"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_TXQX_DBG(pub u32);
    impl MTL_QUEUE_MTL_TXQX_DBG {
        #[must_use]
        #[inline(always)]
        pub const fn TXQPAUSED(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXQPAUSED(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TRCSTS(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TRCSTS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TWCSTS(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TWCSTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXQSTS(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXQSTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXSTSFSTS(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TXSTSFSTS(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn PTXQ(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_PTXQ(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn STXSTSF(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_STXSTSF(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_TXQX_DBG {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_TXQX_DBG {
            MTL_QUEUE_MTL_TXQX_DBG(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_TXQX_DBG {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_TXQX_DBG")
                .field("TXQPAUSED", &self.TXQPAUSED())
                .field("TRCSTS", &self.TRCSTS())
                .field("TWCSTS", &self.TWCSTS())
                .field("TXQSTS", &self.TXQSTS())
                .field("TXSTSFSTS", &self.TXSTSFSTS())
                .field("PTXQ", &self.PTXQ())
                .field("STXSTSF", &self.STXSTSF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_TXQX_DBG {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MTL_QUEUE_MTL_TXQX_DBG {{ TXQPAUSED: {=bool:?}, TRCSTS: {=u8:?}, TWCSTS: {=bool:?}, TXQSTS: {=bool:?}, TXSTSFSTS: {=bool:?}, PTXQ: {=u8:?}, STXSTSF: {=u8:?} }}" , self . TXQPAUSED () , self . TRCSTS () , self . TWCSTS () , self . TXQSTS () , self . TXSTSFSTS () , self . PTXQ () , self . STXSTSF ())
        }
    }
    #[doc = "Queue 1 ETS Control"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_TXQX_ETS_CTRL(pub u32);
    impl MTL_QUEUE_MTL_TXQX_ETS_CTRL {
        #[must_use]
        #[inline(always)]
        pub const fn AVALG(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_AVALG(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn CC(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_CC(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn SLC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_SLC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_TXQX_ETS_CTRL {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_TXQX_ETS_CTRL {
            MTL_QUEUE_MTL_TXQX_ETS_CTRL(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_TXQX_ETS_CTRL {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_TXQX_ETS_CTRL")
                .field("AVALG", &self.AVALG())
                .field("CC", &self.CC())
                .field("SLC", &self.SLC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_TXQX_ETS_CTRL {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MTL_QUEUE_MTL_TXQX_ETS_CTRL {{ AVALG: {=bool:?}, CC: {=bool:?}, SLC: {=u8:?} }}",
                self.AVALG(),
                self.CC(),
                self.SLC()
            )
        }
    }
    #[doc = "Queue 0 ETS Status..Queue 1 ETS Status"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_TXQX_ETS_STAT(pub u32);
    impl MTL_QUEUE_MTL_TXQX_ETS_STAT {
        #[must_use]
        #[inline(always)]
        pub const fn ABS(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ABS(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_TXQX_ETS_STAT {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_TXQX_ETS_STAT {
            MTL_QUEUE_MTL_TXQX_ETS_STAT(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_TXQX_ETS_STAT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_TXQX_ETS_STAT")
                .field("ABS", &self.ABS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_TXQX_ETS_STAT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MTL_QUEUE_MTL_TXQX_ETS_STAT {{ ABS: {=u32:?} }}",
                self.ABS()
            )
        }
    }
    #[doc = "Queue 1 hiCredit"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_TXQX_HI_CRDT(pub u32);
    impl MTL_QUEUE_MTL_TXQX_HI_CRDT {
        #[must_use]
        #[inline(always)]
        pub const fn HC(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_HC(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_TXQX_HI_CRDT {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_TXQX_HI_CRDT {
            MTL_QUEUE_MTL_TXQX_HI_CRDT(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_TXQX_HI_CRDT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_TXQX_HI_CRDT")
                .field("HC", &self.HC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_TXQX_HI_CRDT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MTL_QUEUE_MTL_TXQX_HI_CRDT {{ HC: {=u32:?} }}",
                self.HC()
            )
        }
    }
    #[doc = "Queue 1 loCredit"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_TXQX_LO_CRDT(pub u32);
    impl MTL_QUEUE_MTL_TXQX_LO_CRDT {
        #[must_use]
        #[inline(always)]
        pub const fn LC(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x1fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_LC(&mut self, val: u32) {
            self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_TXQX_LO_CRDT {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_TXQX_LO_CRDT {
            MTL_QUEUE_MTL_TXQX_LO_CRDT(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_TXQX_LO_CRDT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_TXQX_LO_CRDT")
                .field("LC", &self.LC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_TXQX_LO_CRDT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MTL_QUEUE_MTL_TXQX_LO_CRDT {{ LC: {=u32:?} }}",
                self.LC()
            )
        }
    }
    #[doc = "Queue 0 Transmit Operation Mode..Queue 1 Transmit Operation Mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_TXQX_OP_MODE(pub u32);
    impl MTL_QUEUE_MTL_TXQX_OP_MODE {
        #[must_use]
        #[inline(always)]
        pub const fn FTQ(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_FTQ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TSF(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_TSF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TXQEN(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TXQEN(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TTC(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TTC(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn TQS(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[inline(always)]
        pub const fn set_TQS(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_TXQX_OP_MODE {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_TXQX_OP_MODE {
            MTL_QUEUE_MTL_TXQX_OP_MODE(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_TXQX_OP_MODE {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_TXQX_OP_MODE")
                .field("FTQ", &self.FTQ())
                .field("TSF", &self.TSF())
                .field("TXQEN", &self.TXQEN())
                .field("TTC", &self.TTC())
                .field("TQS", &self.TQS())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_TXQX_OP_MODE {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MTL_QUEUE_MTL_TXQX_OP_MODE {{ FTQ: {=bool:?}, TSF: {=bool:?}, TXQEN: {=u8:?}, TTC: {=u8:?}, TQS: {=u8:?} }}" , self . FTQ () , self . TSF () , self . TXQEN () , self . TTC () , self . TQS ())
        }
    }
    #[doc = "Queue 0 Quantum or Weights..Queue 1 idleSlopeCredit, Quantum or Weights"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_TXQX_QNTM_WGHT(pub u32);
    impl MTL_QUEUE_MTL_TXQX_QNTM_WGHT {
        #[must_use]
        #[inline(always)]
        pub const fn ISCQW(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x001f_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ISCQW(&mut self, val: u32) {
            self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_TXQX_QNTM_WGHT {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_TXQX_QNTM_WGHT {
            MTL_QUEUE_MTL_TXQX_QNTM_WGHT(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_TXQX_QNTM_WGHT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_TXQX_QNTM_WGHT")
                .field("ISCQW", &self.ISCQW())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_TXQX_QNTM_WGHT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MTL_QUEUE_MTL_TXQX_QNTM_WGHT {{ ISCQW: {=u32:?} }}",
                self.ISCQW()
            )
        }
    }
    #[doc = "Queue 1 sendSlopeCredit"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_TXQX_SNDSLP_CRDT(pub u32);
    impl MTL_QUEUE_MTL_TXQX_SNDSLP_CRDT {
        #[must_use]
        #[inline(always)]
        pub const fn SSC(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_SSC(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_TXQX_SNDSLP_CRDT {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_TXQX_SNDSLP_CRDT {
            MTL_QUEUE_MTL_TXQX_SNDSLP_CRDT(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_TXQX_SNDSLP_CRDT {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_TXQX_SNDSLP_CRDT")
                .field("SSC", &self.SSC())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_TXQX_SNDSLP_CRDT {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MTL_QUEUE_MTL_TXQX_SNDSLP_CRDT {{ SSC: {=u16:?} }}",
                self.SSC()
            )
        }
    }
    #[doc = "Queue 0 Underflow Counter..Queue 1 Underflow Counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_QUEUE_MTL_TXQX_UNDRFLW(pub u32);
    impl MTL_QUEUE_MTL_TXQX_UNDRFLW {
        #[must_use]
        #[inline(always)]
        pub const fn UFFRMCNT(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[inline(always)]
        pub const fn set_UFFRMCNT(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn UFCNTOVF(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_UFCNTOVF(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for MTL_QUEUE_MTL_TXQX_UNDRFLW {
        #[inline(always)]
        fn default() -> MTL_QUEUE_MTL_TXQX_UNDRFLW {
            MTL_QUEUE_MTL_TXQX_UNDRFLW(0)
        }
    }
    impl core::fmt::Debug for MTL_QUEUE_MTL_TXQX_UNDRFLW {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_QUEUE_MTL_TXQX_UNDRFLW")
                .field("UFFRMCNT", &self.UFFRMCNT())
                .field("UFCNTOVF", &self.UFCNTOVF())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_QUEUE_MTL_TXQX_UNDRFLW {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MTL_QUEUE_MTL_TXQX_UNDRFLW {{ UFFRMCNT: {=u16:?}, UFCNTOVF: {=bool:?} }}",
                self.UFFRMCNT(),
                self.UFCNTOVF()
            )
        }
    }
    #[doc = "Receive Queue and DMA Channel Mapping 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MTL_RXQ_DMA_MAP0(pub u32);
    impl MTL_RXQ_DMA_MAP0 {
        #[must_use]
        #[inline(always)]
        pub const fn Q0MDMACH(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Q0MDMACH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Q0DDMACH(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Q0DDMACH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Q1MDMACH(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Q1MDMACH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn Q1DDMACH(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_Q1DDMACH(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for MTL_RXQ_DMA_MAP0 {
        #[inline(always)]
        fn default() -> MTL_RXQ_DMA_MAP0 {
            MTL_RXQ_DMA_MAP0(0)
        }
    }
    impl core::fmt::Debug for MTL_RXQ_DMA_MAP0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MTL_RXQ_DMA_MAP0")
                .field("Q0MDMACH", &self.Q0MDMACH())
                .field("Q0DDMACH", &self.Q0DDMACH())
                .field("Q1MDMACH", &self.Q1MDMACH())
                .field("Q1DDMACH", &self.Q1DDMACH())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MTL_RXQ_DMA_MAP0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "MTL_RXQ_DMA_MAP0 {{ Q0MDMACH: {=bool:?}, Q0DDMACH: {=bool:?}, Q1MDMACH: {=bool:?}, Q1DDMACH: {=bool:?} }}" , self . Q0MDMACH () , self . Q0DDMACH () , self . Q1MDMACH () , self . Q1DDMACH ())
        }
    }
    #[doc = "PPS0 Target Time Nanoseconds"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PPS0_TARGET_TIME_NANOSECONDS(pub u32);
    impl PPS0_TARGET_TIME_NANOSECONDS {
        #[must_use]
        #[inline(always)]
        pub const fn TTSL0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_TTSL0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for PPS0_TARGET_TIME_NANOSECONDS {
        #[inline(always)]
        fn default() -> PPS0_TARGET_TIME_NANOSECONDS {
            PPS0_TARGET_TIME_NANOSECONDS(0)
        }
    }
    impl core::fmt::Debug for PPS0_TARGET_TIME_NANOSECONDS {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PPS0_TARGET_TIME_NANOSECONDS")
                .field("TTSL0", &self.TTSL0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for PPS0_TARGET_TIME_NANOSECONDS {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "PPS0_TARGET_TIME_NANOSECONDS {{ TTSL0: {=u32:?} }}",
                self.TTSL0()
            )
        }
    }
}
