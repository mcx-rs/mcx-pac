#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code
)]

mod irq;
pub use irq::*;
#[path = "."]
pub mod syscon {

    pub const SYSCON0: *const RegisterBlock = 0x40000000 as *const RegisterBlock;

    #[path = "../../peripherals/syscon_n94x.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SYSCON0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SYSCON0 {}
    impl SYSCON0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SYSCON0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SYSCON0).then_some(0)
    }
}
#[path = "."]
pub mod pint {

    pub const PINT0: *const RegisterBlock = 0x40004000 as *const RegisterBlock;

    #[path = "../../peripherals/pint.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PINT0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PINT0 {}
    impl PINT0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PINT0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PINT0).then_some(0)
    }
}
#[path = "."]
pub mod inputmux {

    pub const INPUTMUX0: *const RegisterBlock = 0x40006000 as *const RegisterBlock;

    #[path = "../../peripherals/inputmux.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type INPUTMUX0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for INPUTMUX0 {}
    impl INPUTMUX0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(INPUTMUX0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, INPUTMUX0).then_some(0)
    }
}
#[path = "."]
pub mod ctimer {

    pub const CTIMER0: *const RegisterBlock = 0x4000c000 as *const RegisterBlock;

    pub const CTIMER1: *const RegisterBlock = 0x4000d000 as *const RegisterBlock;

    pub const CTIMER2: *const RegisterBlock = 0x4000e000 as *const RegisterBlock;

    pub const CTIMER3: *const RegisterBlock = 0x4000f000 as *const RegisterBlock;

    pub const CTIMER4: *const RegisterBlock = 0x40010000 as *const RegisterBlock;

    #[path = "../../peripherals/ctimer.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CTIMER0 = Instance<0>;
    impl crate::sealed::Sealed for CTIMER0 {}
    impl CTIMER0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CTIMER0)
        }
    }

    pub type CTIMER1 = Instance<1>;
    impl crate::sealed::Sealed for CTIMER1 {}
    impl CTIMER1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CTIMER1)
        }
    }

    pub type CTIMER2 = Instance<2>;
    impl crate::sealed::Sealed for CTIMER2 {}
    impl CTIMER2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CTIMER2)
        }
    }

    pub type CTIMER3 = Instance<3>;
    impl crate::sealed::Sealed for CTIMER3 {}
    impl CTIMER3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CTIMER3)
        }
    }

    pub type CTIMER4 = Instance<4>;
    impl crate::sealed::Sealed for CTIMER4 {}
    impl CTIMER4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CTIMER4)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (CTIMER0, 0),
            (CTIMER1, 1),
            (CTIMER2, 2),
            (CTIMER3, 3),
            (CTIMER4, 4),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod freqme {

    pub const FREQME0: *const RegisterBlock = 0x40011000 as *const RegisterBlock;

    #[path = "../../peripherals/freqme.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type FREQME0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for FREQME0 {}
    impl FREQME0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FREQME0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, FREQME0).then_some(0)
    }
}
#[path = "."]
pub mod utick {

    pub const UTICK0: *const RegisterBlock = 0x40012000 as *const RegisterBlock;

    #[path = "../../peripherals/utick.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type UTICK0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for UTICK0 {}
    impl UTICK0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UTICK0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, UTICK0).then_some(0)
    }
}
#[path = "."]
pub mod mrt {

    pub const MRT0: *const RegisterBlock = 0x40013000 as *const RegisterBlock;

    #[path = "../../peripherals/mrt.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type MRT0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for MRT0 {}
    impl MRT0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MRT0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, MRT0).then_some(0)
    }
}
#[path = "."]
pub mod wwdt {

    pub const WWDT0: *const RegisterBlock = 0x40016000 as *const RegisterBlock;

    pub const WWDT1: *const RegisterBlock = 0x40017000 as *const RegisterBlock;

    #[path = "../../peripherals/wwdt.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type WWDT0 = Instance<0>;
    impl crate::sealed::Sealed for WWDT0 {}
    impl WWDT0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(WWDT0)
        }
    }

    pub type WWDT1 = Instance<1>;
    impl crate::sealed::Sealed for WWDT1 {}
    impl WWDT1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(WWDT1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(WWDT0, 0), (WWDT1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod cache64_ctrl {

    pub const CACHE64_CTRL0: *const RegisterBlock = 0x4001b000 as *const RegisterBlock;

    #[path = "../../peripherals/cache64_ctrl.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CACHE64_CTRL0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for CACHE64_CTRL0 {}
    impl CACHE64_CTRL0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CACHE64_CTRL0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CACHE64_CTRL0).then_some(0)
    }
}
#[path = "."]
pub mod cache64_polsel {

    pub const CACHE64_POLSEL0: *const RegisterBlock = 0x4001b000 as *const RegisterBlock;

    #[path = "../../peripherals/cache64_polsel.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CACHE64_POLSEL0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for CACHE64_POLSEL0 {}
    impl CACHE64_POLSEL0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CACHE64_POLSEL0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CACHE64_POLSEL0).then_some(0)
    }
}
#[path = "."]
pub mod i3c {

    pub const I3C0: *const RegisterBlock = 0x40021000 as *const RegisterBlock;

    pub const I3C1: *const RegisterBlock = 0x40022000 as *const RegisterBlock;

    #[path = "../../peripherals/i3c.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type I3C0 = Instance<0>;
    impl crate::sealed::Sealed for I3C0 {}
    impl I3C0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I3C0)
        }
    }

    pub type I3C1 = Instance<1>;
    impl crate::sealed::Sealed for I3C1 {}
    impl I3C1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I3C1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(I3C0, 0), (I3C1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod gdet {

    pub const GDET0: *const RegisterBlock = 0x40024000 as *const RegisterBlock;

    pub const GDET1: *const RegisterBlock = 0x40025000 as *const RegisterBlock;

    #[path = "../../peripherals/gdet.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type GDET0 = Instance<0>;
    impl crate::sealed::Sealed for GDET0 {}
    impl GDET0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GDET0)
        }
    }

    pub type GDET1 = Instance<1>;
    impl crate::sealed::Sealed for GDET1 {}
    impl GDET1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GDET1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(GDET0, 0), (GDET1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod itrc {

    pub const ITRC0: *const RegisterBlock = 0x40026000 as *const RegisterBlock;

    #[path = "../../peripherals/itrc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type ITRC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for ITRC0 {}
    impl ITRC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ITRC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ITRC0).then_some(0)
    }
}
#[path = "."]
pub mod pkc {

    pub const PKC0: *const RegisterBlock = 0x4002b000 as *const RegisterBlock;

    #[path = "../../peripherals/pkc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PKC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PKC0 {}
    impl PKC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PKC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PKC0).then_some(0)
    }
}
#[path = "."]
pub mod puf {

    pub const PUF: *const RegisterBlock = 0x4002c000 as *const RegisterBlock;

    #[path = "../../peripherals/puf.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PUF = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PUF {}
    impl PUF {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PUF)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PUF).then_some(0)
    }
}
#[path = "."]
pub mod puf_ctrl {

    pub const PUF_CTRL: *const RegisterBlock = 0x4002c000 as *const RegisterBlock;

    #[path = "../../peripherals/puf_ctrl.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PUF_CTRL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PUF_CTRL {}
    impl PUF_CTRL {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PUF_CTRL)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PUF_CTRL).then_some(0)
    }
}
#[path = "."]
pub mod sm3 {

    pub const SM3_0: *const RegisterBlock = 0x40031000 as *const RegisterBlock;

    #[path = "../../peripherals/sm3.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SM3_0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SM3_0 {}
    impl SM3_0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SM3_0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SM3_0).then_some(0)
    }
}
#[path = "."]
pub mod bsp32 {

    pub const BSP32_0: *const RegisterBlock = 0x40032000 as *const RegisterBlock;

    #[path = "../../peripherals/bsp32.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type BSP32_0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for BSP32_0 {}
    impl BSP32_0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BSP32_0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BSP32_0).then_some(0)
    }
}
#[path = "."]
pub mod smartdma {

    pub const SMARTDMA0: *const RegisterBlock = 0x40033000 as *const RegisterBlock;

    #[path = "../../peripherals/smartdma.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SMARTDMA0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SMARTDMA0 {}
    impl SMARTDMA0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SMARTDMA0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SMARTDMA0).then_some(0)
    }
}
#[path = "."]
pub mod plu {

    pub const PLU0: *const RegisterBlock = 0x40034000 as *const RegisterBlock;

    #[path = "../../peripherals/plu.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PLU0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PLU0 {}
    impl PLU0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PLU0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PLU0).then_some(0)
    }
}
#[path = "."]
pub mod port {

    pub const PORT0: *const RegisterBlock = 0x40116000 as *const RegisterBlock;

    pub const PORT1: *const RegisterBlock = 0x40117000 as *const RegisterBlock;

    pub const PORT2: *const RegisterBlock = 0x40118000 as *const RegisterBlock;

    pub const PORT3: *const RegisterBlock = 0x40119000 as *const RegisterBlock;

    pub const PORT4: *const RegisterBlock = 0x4011a000 as *const RegisterBlock;

    pub const PORT5: *const RegisterBlock = 0x40042000 as *const RegisterBlock;

    #[path = "../../peripherals/port.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PORT0 = Instance<0>;
    impl crate::sealed::Sealed for PORT0 {}
    impl PORT0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PORT0)
        }
    }

    pub type PORT1 = Instance<1>;
    impl crate::sealed::Sealed for PORT1 {}
    impl PORT1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PORT1)
        }
    }

    pub type PORT2 = Instance<2>;
    impl crate::sealed::Sealed for PORT2 {}
    impl PORT2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PORT2)
        }
    }

    pub type PORT3 = Instance<3>;
    impl crate::sealed::Sealed for PORT3 {}
    impl PORT3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PORT3)
        }
    }

    pub type PORT4 = Instance<4>;
    impl crate::sealed::Sealed for PORT4 {}
    impl PORT4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PORT4)
        }
    }

    pub type PORT5 = Instance<5>;
    impl crate::sealed::Sealed for PORT5 {}
    impl PORT5 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PORT5)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (PORT0, 0),
            (PORT1, 1),
            (PORT2, 2),
            (PORT3, 3),
            (PORT4, 4),
            (PORT5, 5),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod msf1_b {

    pub const FMU0: *const RegisterBlock = 0x40043000 as *const RegisterBlock;

    #[path = "../../peripherals/msf1_b.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type FMU0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for FMU0 {}
    impl FMU0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FMU0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, FMU0).then_some(0)
    }
}
#[path = "."]
pub mod scg {

    pub const SCG0: *const RegisterBlock = 0x40044000 as *const RegisterBlock;

    #[path = "../../peripherals/scg.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SCG0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SCG0 {}
    impl SCG0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SCG0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SCG0).then_some(0)
    }
}
#[path = "."]
pub mod spc {

    pub const SPC0: *const RegisterBlock = 0x40045000 as *const RegisterBlock;

    #[path = "../../peripherals/spc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SPC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SPC0 {}
    impl SPC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SPC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SPC0).then_some(0)
    }
}
#[path = "."]
pub mod wuu {

    pub const WUU0: *const RegisterBlock = 0x40046000 as *const RegisterBlock;

    #[path = "../../peripherals/wuu.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type WUU0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for WUU0 {}
    impl WUU0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(WUU0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, WUU0).then_some(0)
    }
}
#[path = "."]
pub mod cmc {

    pub const CMC0: *const RegisterBlock = 0x40048000 as *const RegisterBlock;

    #[path = "../../peripherals/cmc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CMC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for CMC0 {}
    impl CMC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CMC0).then_some(0)
    }
}
#[path = "."]
pub mod ostimer {

    pub const OSTIMER0: *const RegisterBlock = 0x40049000 as *const RegisterBlock;

    #[path = "../../peripherals/ostimer.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type OSTIMER0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for OSTIMER0 {}
    impl OSTIMER0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OSTIMER0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, OSTIMER0).then_some(0)
    }
}
#[path = "."]
pub mod lptmr {

    pub const LPTMR0: *const RegisterBlock = 0x4004a000 as *const RegisterBlock;

    pub const LPTMR1: *const RegisterBlock = 0x4004b000 as *const RegisterBlock;

    #[path = "../../peripherals/lptmr.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type LPTMR0 = Instance<0>;
    impl crate::sealed::Sealed for LPTMR0 {}
    impl LPTMR0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPTMR0)
        }
    }

    pub type LPTMR1 = Instance<1>;
    impl crate::sealed::Sealed for LPTMR1 {}
    impl LPTMR1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPTMR1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(LPTMR0, 0), (LPTMR1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod rtc {

    pub const RTC0: *const RegisterBlock = 0x4004c000 as *const RegisterBlock;

    #[path = "../../peripherals/rtc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type RTC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for RTC0 {}
    impl RTC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, RTC0).then_some(0)
    }
}
#[path = "."]
pub mod rtc_subsystem {

    pub const RTC_SUBSYSTEM0: *const RegisterBlock = 0x4004c000 as *const RegisterBlock;

    #[path = "../../peripherals/rtc_subsystem.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type RTC_SUBSYSTEM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for RTC_SUBSYSTEM0 {}
    impl RTC_SUBSYSTEM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTC_SUBSYSTEM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, RTC_SUBSYSTEM0).then_some(0)
    }
}
#[path = "."]
pub mod tsi {

    pub const TSI0: *const RegisterBlock = 0x40050000 as *const RegisterBlock;

    #[path = "../../peripherals/tsi.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type TSI0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for TSI0 {}
    impl TSI0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TSI0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TSI0).then_some(0)
    }
}
#[path = "."]
pub mod cmp {

    pub const CMP0: *const RegisterBlock = 0x40051000 as *const RegisterBlock;

    pub const CMP1: *const RegisterBlock = 0x40052000 as *const RegisterBlock;

    pub const CMP2: *const RegisterBlock = 0x40053000 as *const RegisterBlock;

    #[path = "../../peripherals/cmp.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CMP0 = Instance<0>;
    impl crate::sealed::Sealed for CMP0 {}
    impl CMP0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMP0)
        }
    }

    pub type CMP1 = Instance<1>;
    impl crate::sealed::Sealed for CMP1 {}
    impl CMP1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMP1)
        }
    }

    pub type CMP2 = Instance<2>;
    impl crate::sealed::Sealed for CMP2 {}
    impl CMP2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMP2)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CMP0, 0), (CMP1, 1), (CMP2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod s50 {

    pub const ELS: *const RegisterBlock = 0x40054000 as *const RegisterBlock;

    #[path = "../../peripherals/s50.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type ELS = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for ELS {}
    impl ELS {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ELS)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ELS).then_some(0)
    }
}
#[path = "."]
pub mod digtmp {

    pub const TDET0: *const RegisterBlock = 0x40058000 as *const RegisterBlock;

    #[path = "../../peripherals/digtmp.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type TDET0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for TDET0 {}
    impl TDET0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TDET0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TDET0).then_some(0)
    }
}
#[path = "."]
pub mod vbat {

    pub const VBAT0: *const RegisterBlock = 0x40059000 as *const RegisterBlock;

    #[path = "../../peripherals/vbat.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type VBAT0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for VBAT0 {}
    impl VBAT0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(VBAT0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, VBAT0).then_some(0)
    }
}
#[path = "."]
pub mod trng {

    pub const TRNG0: *const RegisterBlock = 0x4005a000 as *const RegisterBlock;

    #[path = "../../peripherals/trng.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type TRNG0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for TRNG0 {}
    impl TRNG0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TRNG0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TRNG0).then_some(0)
    }
}
#[path = "."]
pub mod eim {

    pub const EIM0: *const RegisterBlock = 0x4005b000 as *const RegisterBlock;

    #[path = "../../peripherals/eim.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type EIM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for EIM0 {}
    impl EIM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EIM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, EIM0).then_some(0)
    }
}
#[path = "."]
pub mod erm {

    pub const ERM0: *const RegisterBlock = 0x4005c000 as *const RegisterBlock;

    #[path = "../../peripherals/erm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type ERM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for ERM0 {}
    impl ERM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ERM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ERM0).then_some(0)
    }
}
#[path = "."]
pub mod intm {

    pub const INTM0: *const RegisterBlock = 0x4005d000 as *const RegisterBlock;

    #[path = "../../peripherals/intm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type INTM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for INTM0 {}
    impl INTM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(INTM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, INTM0).then_some(0)
    }
}
#[path = "."]
pub mod dma {

    pub const DMA0: *const RegisterBlock = 0x40080000 as *const RegisterBlock;

    pub const DMA1: *const RegisterBlock = 0x400a0000 as *const RegisterBlock;

    #[path = "../../peripherals/dma.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type DMA0 = Instance<0>;
    impl crate::sealed::Sealed for DMA0 {}
    impl DMA0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DMA0)
        }
    }

    pub type DMA1 = Instance<1>;
    impl crate::sealed::Sealed for DMA1 {}
    impl DMA1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DMA1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(DMA0, 0), (DMA1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod dma_tcd {

    pub const EDMA_TCD0: *const RegisterBlock = 0x40081000 as *const RegisterBlock;

    pub const EDMA_TCD1: *const RegisterBlock = 0x400a1000 as *const RegisterBlock;

    #[path = "../../peripherals/dma_tcd.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type EDMA_TCD0 = Instance<0>;
    impl crate::sealed::Sealed for EDMA_TCD0 {}
    impl EDMA_TCD0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EDMA_TCD0)
        }
    }

    pub type EDMA_TCD1 = Instance<1>;
    impl crate::sealed::Sealed for EDMA_TCD1 {}
    impl EDMA_TCD1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EDMA_TCD1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(EDMA_TCD0, 0), (EDMA_TCD1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod sct {

    pub const SCT0: *const RegisterBlock = 0x40091000 as *const RegisterBlock;

    #[path = "../../peripherals/sct.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SCT0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SCT0 {}
    impl SCT0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SCT0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SCT0).then_some(0)
    }
}
#[path = "."]
pub mod lpspi {

    pub const LPSPI0: *const RegisterBlock = 0x40092000 as *const RegisterBlock;

    pub const LPSPI1: *const RegisterBlock = 0x40093000 as *const RegisterBlock;

    pub const LPSPI2: *const RegisterBlock = 0x40094000 as *const RegisterBlock;

    pub const LPSPI3: *const RegisterBlock = 0x40095000 as *const RegisterBlock;

    pub const LPSPI4: *const RegisterBlock = 0x400b4000 as *const RegisterBlock;

    pub const LPSPI5: *const RegisterBlock = 0x400b5000 as *const RegisterBlock;

    pub const LPSPI6: *const RegisterBlock = 0x400b6000 as *const RegisterBlock;

    pub const LPSPI7: *const RegisterBlock = 0x400b7000 as *const RegisterBlock;

    pub const LPSPI8: *const RegisterBlock = 0x400b8000 as *const RegisterBlock;

    pub const LPSPI9: *const RegisterBlock = 0x400b9000 as *const RegisterBlock;

    #[path = "../../peripherals/lpspi.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type LPSPI0 = Instance<0>;
    impl crate::sealed::Sealed for LPSPI0 {}
    impl LPSPI0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI0)
        }
    }

    pub type LPSPI1 = Instance<1>;
    impl crate::sealed::Sealed for LPSPI1 {}
    impl LPSPI1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI1)
        }
    }

    pub type LPSPI2 = Instance<2>;
    impl crate::sealed::Sealed for LPSPI2 {}
    impl LPSPI2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI2)
        }
    }

    pub type LPSPI3 = Instance<3>;
    impl crate::sealed::Sealed for LPSPI3 {}
    impl LPSPI3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI3)
        }
    }

    pub type LPSPI4 = Instance<4>;
    impl crate::sealed::Sealed for LPSPI4 {}
    impl LPSPI4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI4)
        }
    }

    pub type LPSPI5 = Instance<5>;
    impl crate::sealed::Sealed for LPSPI5 {}
    impl LPSPI5 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI5)
        }
    }

    pub type LPSPI6 = Instance<6>;
    impl crate::sealed::Sealed for LPSPI6 {}
    impl LPSPI6 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI6)
        }
    }

    pub type LPSPI7 = Instance<7>;
    impl crate::sealed::Sealed for LPSPI7 {}
    impl LPSPI7 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI7)
        }
    }

    pub type LPSPI8 = Instance<8>;
    impl crate::sealed::Sealed for LPSPI8 {}
    impl LPSPI8 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI8)
        }
    }

    pub type LPSPI9 = Instance<9>;
    impl crate::sealed::Sealed for LPSPI9 {}
    impl LPSPI9 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI9)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LPSPI0, 0),
            (LPSPI1, 1),
            (LPSPI2, 2),
            (LPSPI3, 3),
            (LPSPI4, 4),
            (LPSPI5, 5),
            (LPSPI6, 6),
            (LPSPI7, 7),
            (LPSPI8, 8),
            (LPSPI9, 9),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpuart {

    pub const LPUART0: *const RegisterBlock = 0x40092000 as *const RegisterBlock;

    pub const LPUART1: *const RegisterBlock = 0x40093000 as *const RegisterBlock;

    pub const LPUART2: *const RegisterBlock = 0x40094000 as *const RegisterBlock;

    pub const LPUART3: *const RegisterBlock = 0x40095000 as *const RegisterBlock;

    pub const LPUART4: *const RegisterBlock = 0x400b4000 as *const RegisterBlock;

    pub const LPUART5: *const RegisterBlock = 0x400b5000 as *const RegisterBlock;

    pub const LPUART6: *const RegisterBlock = 0x400b6000 as *const RegisterBlock;

    pub const LPUART7: *const RegisterBlock = 0x400b7000 as *const RegisterBlock;

    pub const LPUART8: *const RegisterBlock = 0x400b8000 as *const RegisterBlock;

    pub const LPUART9: *const RegisterBlock = 0x400b9000 as *const RegisterBlock;

    #[path = "../../peripherals/lpuart.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type LPUART0 = Instance<0>;
    impl crate::sealed::Sealed for LPUART0 {}
    impl LPUART0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART0)
        }
    }

    pub type LPUART1 = Instance<1>;
    impl crate::sealed::Sealed for LPUART1 {}
    impl LPUART1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART1)
        }
    }

    pub type LPUART2 = Instance<2>;
    impl crate::sealed::Sealed for LPUART2 {}
    impl LPUART2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART2)
        }
    }

    pub type LPUART3 = Instance<3>;
    impl crate::sealed::Sealed for LPUART3 {}
    impl LPUART3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART3)
        }
    }

    pub type LPUART4 = Instance<4>;
    impl crate::sealed::Sealed for LPUART4 {}
    impl LPUART4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART4)
        }
    }

    pub type LPUART5 = Instance<5>;
    impl crate::sealed::Sealed for LPUART5 {}
    impl LPUART5 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART5)
        }
    }

    pub type LPUART6 = Instance<6>;
    impl crate::sealed::Sealed for LPUART6 {}
    impl LPUART6 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART6)
        }
    }

    pub type LPUART7 = Instance<7>;
    impl crate::sealed::Sealed for LPUART7 {}
    impl LPUART7 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART7)
        }
    }

    pub type LPUART8 = Instance<8>;
    impl crate::sealed::Sealed for LPUART8 {}
    impl LPUART8 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART8)
        }
    }

    pub type LPUART9 = Instance<9>;
    impl crate::sealed::Sealed for LPUART9 {}
    impl LPUART9 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART9)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LPUART0, 0),
            (LPUART1, 1),
            (LPUART2, 2),
            (LPUART3, 3),
            (LPUART4, 4),
            (LPUART5, 5),
            (LPUART6, 6),
            (LPUART7, 7),
            (LPUART8, 8),
            (LPUART9, 9),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lp_flexcomm {

    pub const LP_FLEXCOMM0: *const RegisterBlock = 0x40092000 as *const RegisterBlock;

    pub const LP_FLEXCOMM1: *const RegisterBlock = 0x40093000 as *const RegisterBlock;

    pub const LP_FLEXCOMM2: *const RegisterBlock = 0x40094000 as *const RegisterBlock;

    pub const LP_FLEXCOMM3: *const RegisterBlock = 0x40095000 as *const RegisterBlock;

    pub const LP_FLEXCOMM4: *const RegisterBlock = 0x400b4000 as *const RegisterBlock;

    pub const LP_FLEXCOMM5: *const RegisterBlock = 0x400b5000 as *const RegisterBlock;

    pub const LP_FLEXCOMM6: *const RegisterBlock = 0x400b6000 as *const RegisterBlock;

    pub const LP_FLEXCOMM7: *const RegisterBlock = 0x400b7000 as *const RegisterBlock;

    pub const LP_FLEXCOMM8: *const RegisterBlock = 0x400b8000 as *const RegisterBlock;

    pub const LP_FLEXCOMM9: *const RegisterBlock = 0x400b9000 as *const RegisterBlock;

    #[path = "../../peripherals/lp_flexcomm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type LP_FLEXCOMM0 = Instance<0>;
    impl crate::sealed::Sealed for LP_FLEXCOMM0 {}
    impl LP_FLEXCOMM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM0)
        }
    }

    pub type LP_FLEXCOMM1 = Instance<1>;
    impl crate::sealed::Sealed for LP_FLEXCOMM1 {}
    impl LP_FLEXCOMM1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM1)
        }
    }

    pub type LP_FLEXCOMM2 = Instance<2>;
    impl crate::sealed::Sealed for LP_FLEXCOMM2 {}
    impl LP_FLEXCOMM2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM2)
        }
    }

    pub type LP_FLEXCOMM3 = Instance<3>;
    impl crate::sealed::Sealed for LP_FLEXCOMM3 {}
    impl LP_FLEXCOMM3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM3)
        }
    }

    pub type LP_FLEXCOMM4 = Instance<4>;
    impl crate::sealed::Sealed for LP_FLEXCOMM4 {}
    impl LP_FLEXCOMM4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM4)
        }
    }

    pub type LP_FLEXCOMM5 = Instance<5>;
    impl crate::sealed::Sealed for LP_FLEXCOMM5 {}
    impl LP_FLEXCOMM5 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM5)
        }
    }

    pub type LP_FLEXCOMM6 = Instance<6>;
    impl crate::sealed::Sealed for LP_FLEXCOMM6 {}
    impl LP_FLEXCOMM6 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM6)
        }
    }

    pub type LP_FLEXCOMM7 = Instance<7>;
    impl crate::sealed::Sealed for LP_FLEXCOMM7 {}
    impl LP_FLEXCOMM7 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM7)
        }
    }

    pub type LP_FLEXCOMM8 = Instance<8>;
    impl crate::sealed::Sealed for LP_FLEXCOMM8 {}
    impl LP_FLEXCOMM8 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM8)
        }
    }

    pub type LP_FLEXCOMM9 = Instance<9>;
    impl crate::sealed::Sealed for LP_FLEXCOMM9 {}
    impl LP_FLEXCOMM9 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM9)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LP_FLEXCOMM0, 0),
            (LP_FLEXCOMM1, 1),
            (LP_FLEXCOMM2, 2),
            (LP_FLEXCOMM3, 3),
            (LP_FLEXCOMM4, 4),
            (LP_FLEXCOMM5, 5),
            (LP_FLEXCOMM6, 6),
            (LP_FLEXCOMM7, 7),
            (LP_FLEXCOMM8, 8),
            (LP_FLEXCOMM9, 9),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpi2c {

    pub const LPI2C0: *const RegisterBlock = 0x40092800 as *const RegisterBlock;

    pub const LPI2C1: *const RegisterBlock = 0x40093800 as *const RegisterBlock;

    pub const LPI2C2: *const RegisterBlock = 0x40094800 as *const RegisterBlock;

    pub const LPI2C3: *const RegisterBlock = 0x40095800 as *const RegisterBlock;

    pub const LPI2C4: *const RegisterBlock = 0x400b4800 as *const RegisterBlock;

    pub const LPI2C5: *const RegisterBlock = 0x400b5800 as *const RegisterBlock;

    pub const LPI2C6: *const RegisterBlock = 0x400b6800 as *const RegisterBlock;

    pub const LPI2C7: *const RegisterBlock = 0x400b7800 as *const RegisterBlock;

    pub const LPI2C8: *const RegisterBlock = 0x400b8800 as *const RegisterBlock;

    pub const LPI2C9: *const RegisterBlock = 0x400b9800 as *const RegisterBlock;

    #[path = "../../peripherals/lpi2c.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type LPI2C0 = Instance<0>;
    impl crate::sealed::Sealed for LPI2C0 {}
    impl LPI2C0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C0)
        }
    }

    pub type LPI2C1 = Instance<1>;
    impl crate::sealed::Sealed for LPI2C1 {}
    impl LPI2C1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C1)
        }
    }

    pub type LPI2C2 = Instance<2>;
    impl crate::sealed::Sealed for LPI2C2 {}
    impl LPI2C2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C2)
        }
    }

    pub type LPI2C3 = Instance<3>;
    impl crate::sealed::Sealed for LPI2C3 {}
    impl LPI2C3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C3)
        }
    }

    pub type LPI2C4 = Instance<4>;
    impl crate::sealed::Sealed for LPI2C4 {}
    impl LPI2C4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C4)
        }
    }

    pub type LPI2C5 = Instance<5>;
    impl crate::sealed::Sealed for LPI2C5 {}
    impl LPI2C5 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C5)
        }
    }

    pub type LPI2C6 = Instance<6>;
    impl crate::sealed::Sealed for LPI2C6 {}
    impl LPI2C6 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C6)
        }
    }

    pub type LPI2C7 = Instance<7>;
    impl crate::sealed::Sealed for LPI2C7 {}
    impl LPI2C7 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C7)
        }
    }

    pub type LPI2C8 = Instance<8>;
    impl crate::sealed::Sealed for LPI2C8 {}
    impl LPI2C8 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C8)
        }
    }

    pub type LPI2C9 = Instance<9>;
    impl crate::sealed::Sealed for LPI2C9 {}
    impl LPI2C9 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C9)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LPI2C0, 0),
            (LPI2C1, 1),
            (LPI2C2, 2),
            (LPI2C3, 3),
            (LPI2C4, 4),
            (LPI2C5, 5),
            (LPI2C6, 6),
            (LPI2C7, 7),
            (LPI2C8, 8),
            (LPI2C9, 9),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod gpio {

    pub const GPIO0: *const RegisterBlock = 0x40096000 as *const RegisterBlock;

    pub const GPIO1: *const RegisterBlock = 0x40098000 as *const RegisterBlock;

    pub const GPIO2: *const RegisterBlock = 0x4009a000 as *const RegisterBlock;

    pub const GPIO3: *const RegisterBlock = 0x4009c000 as *const RegisterBlock;

    pub const GPIO4: *const RegisterBlock = 0x4009e000 as *const RegisterBlock;

    pub const GPIO5: *const RegisterBlock = 0x40040000 as *const RegisterBlock;

    #[path = "../../peripherals/gpio.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type GPIO0 = Instance<0>;
    impl crate::sealed::Sealed for GPIO0 {}
    impl GPIO0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO0)
        }
    }

    pub type GPIO1 = Instance<1>;
    impl crate::sealed::Sealed for GPIO1 {}
    impl GPIO1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO1)
        }
    }

    pub type GPIO2 = Instance<2>;
    impl crate::sealed::Sealed for GPIO2 {}
    impl GPIO2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO2)
        }
    }

    pub type GPIO3 = Instance<3>;
    impl crate::sealed::Sealed for GPIO3 {}
    impl GPIO3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO3)
        }
    }

    pub type GPIO4 = Instance<4>;
    impl crate::sealed::Sealed for GPIO4 {}
    impl GPIO4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO4)
        }
    }

    pub type GPIO5 = Instance<5>;
    impl crate::sealed::Sealed for GPIO5 {}
    impl GPIO5 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO5)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (GPIO0, 0),
            (GPIO1, 1),
            (GPIO2, 2),
            (GPIO3, 3),
            (GPIO4, 4),
            (GPIO5, 5),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod sema42 {

    pub const SEMA42_0: *const RegisterBlock = 0x400b1000 as *const RegisterBlock;

    #[path = "../../peripherals/sema42.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SEMA42_0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SEMA42_0 {}
    impl SEMA42_0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SEMA42_0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SEMA42_0).then_some(0)
    }
}
#[path = "."]
pub mod mailbox {

    pub const MAILBOX: *const RegisterBlock = 0x400b2000 as *const RegisterBlock;

    #[path = "../../peripherals/mailbox.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type MAILBOX = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for MAILBOX {}
    impl MAILBOX {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MAILBOX)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, MAILBOX).then_some(0)
    }
}
#[path = "."]
pub mod cdog {

    pub const CDOG0: *const RegisterBlock = 0x400bb000 as *const RegisterBlock;

    pub const CDOG1: *const RegisterBlock = 0x400bc000 as *const RegisterBlock;

    #[path = "../../peripherals/cdog.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CDOG0 = Instance<0>;
    impl crate::sealed::Sealed for CDOG0 {}
    impl CDOG0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CDOG0)
        }
    }

    pub type CDOG1 = Instance<1>;
    impl crate::sealed::Sealed for CDOG1 {}
    impl CDOG1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CDOG1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CDOG0, 0), (CDOG1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod dm {

    pub const DM0: *const RegisterBlock = 0x400bd000 as *const RegisterBlock;

    #[path = "../../peripherals/dm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type DM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for DM0 {}
    impl DM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DM0).then_some(0)
    }
}
#[path = "."]
pub mod powerquad {

    pub const POWERQUAD: *const RegisterBlock = 0x400bf000 as *const RegisterBlock;

    #[path = "../../peripherals/powerquad.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type POWERQUAD = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for POWERQUAD {}
    impl POWERQUAD {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(POWERQUAD)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, POWERQUAD).then_some(0)
    }
}
#[path = "."]
pub mod ewm {

    pub const EWM0: *const RegisterBlock = 0x400c0000 as *const RegisterBlock;

    #[path = "../../peripherals/ewm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type EWM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for EWM0 {}
    impl EWM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EWM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, EWM0).then_some(0)
    }
}
#[path = "."]
pub mod syspm {

    pub const CMX_PERFMON0: *const RegisterBlock = 0x400c1000 as *const RegisterBlock;

    pub const CMX_PERFMON1: *const RegisterBlock = 0x400c2000 as *const RegisterBlock;

    #[path = "../../peripherals/syspm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CMX_PERFMON0 = Instance<0>;
    impl crate::sealed::Sealed for CMX_PERFMON0 {}
    impl CMX_PERFMON0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMX_PERFMON0)
        }
    }

    pub type CMX_PERFMON1 = Instance<1>;
    impl crate::sealed::Sealed for CMX_PERFMON1 {}
    impl CMX_PERFMON1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMX_PERFMON1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CMX_PERFMON0, 0), (CMX_PERFMON1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod mbc {

    pub const TRDC: *const RegisterBlock = 0x400c6000 as *const RegisterBlock;

    #[path = "../../peripherals/mbc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type TRDC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for TRDC {}
    impl TRDC {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TRDC)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TRDC).then_some(0)
    }
}
#[path = "."]
pub mod flexspi {

    pub const FLEXSPI0: *const RegisterBlock = 0x400c8000 as *const RegisterBlock;

    #[path = "../../peripherals/flexspi.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type FLEXSPI0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for FLEXSPI0 {}
    impl FLEXSPI0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FLEXSPI0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, FLEXSPI0).then_some(0)
    }
}
#[path = "."]
pub mod otpc {

    pub const OTPC0: *const RegisterBlock = 0x400c9000 as *const RegisterBlock;

    #[path = "../../peripherals/otpc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type OTPC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for OTPC0 {}
    impl OTPC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OTPC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, OTPC0).then_some(0)
    }
}
#[path = "."]
pub mod crc {

    pub const CRC0: *const RegisterBlock = 0x400cb000 as *const RegisterBlock;

    #[path = "../../peripherals/crc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CRC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for CRC0 {}
    impl CRC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CRC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CRC0).then_some(0)
    }
}
#[path = "."]
pub mod npx {

    pub const NPX0: *const RegisterBlock = 0x400cc000 as *const RegisterBlock;

    #[path = "../../peripherals/npx.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type NPX0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for NPX0 {}
    impl NPX0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(NPX0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, NPX0).then_some(0)
    }
}
#[path = "."]
pub mod pwm {

    pub const PWM0: *const RegisterBlock = 0x400ce000 as *const RegisterBlock;

    pub const PWM1: *const RegisterBlock = 0x400d0000 as *const RegisterBlock;

    #[path = "../../peripherals/pwm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PWM0 = Instance<0>;
    impl crate::sealed::Sealed for PWM0 {}
    impl PWM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PWM0)
        }
    }

    pub type PWM1 = Instance<1>;
    impl crate::sealed::Sealed for PWM1 {}
    impl PWM1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PWM1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(PWM0, 0), (PWM1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod enc {

    pub const ENC0: *const RegisterBlock = 0x400cf000 as *const RegisterBlock;

    pub const ENC1: *const RegisterBlock = 0x400d1000 as *const RegisterBlock;

    #[path = "../../peripherals/enc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type ENC0 = Instance<0>;
    impl crate::sealed::Sealed for ENC0 {}
    impl ENC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENC0)
        }
    }

    pub type ENC1 = Instance<1>;
    impl crate::sealed::Sealed for ENC1 {}
    impl ENC1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENC1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(ENC0, 0), (ENC1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod evtg {

    pub const EVTG0: *const RegisterBlock = 0x400d2000 as *const RegisterBlock;

    #[path = "../../peripherals/evtg.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type EVTG0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for EVTG0 {}
    impl EVTG0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EVTG0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, EVTG0).then_some(0)
    }
}
#[path = "."]
pub mod can {

    pub const CAN0: *const RegisterBlock = 0x400d4000 as *const RegisterBlock;

    pub const CAN1: *const RegisterBlock = 0x400d8000 as *const RegisterBlock;

    #[path = "../../peripherals/can.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CAN0 = Instance<0>;
    impl crate::sealed::Sealed for CAN0 {}
    impl CAN0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN0)
        }
    }

    pub type CAN1 = Instance<1>;
    impl crate::sealed::Sealed for CAN1 {}
    impl CAN1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CAN0, 0), (CAN1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod usbdcd {

    pub const USBDCD0: *const RegisterBlock = 0x400dc000 as *const RegisterBlock;

    #[path = "../../peripherals/usbdcd.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USBDCD0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USBDCD0 {}
    impl USBDCD0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBDCD0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBDCD0).then_some(0)
    }
}
#[path = "."]
pub mod usb_fs {

    pub const USBFS0: *const RegisterBlock = 0x400dd000 as *const RegisterBlock;

    #[path = "../../peripherals/usb_fs.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USBFS0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USBFS0 {}
    impl USBFS0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBFS0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBFS0).then_some(0)
    }
}
#[path = "."]
pub mod enet {

    pub const ENET0: *const RegisterBlock = 0x40100000 as *const RegisterBlock;

    #[path = "../../peripherals/enet.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type ENET0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for ENET0 {}
    impl ENET0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENET0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENET0).then_some(0)
    }
}
#[path = "."]
pub mod emvsim {

    pub const EMVSIM0: *const RegisterBlock = 0x40103000 as *const RegisterBlock;

    pub const EMVSIM1: *const RegisterBlock = 0x40104000 as *const RegisterBlock;

    #[path = "../../peripherals/emvsim.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type EMVSIM0 = Instance<0>;
    impl crate::sealed::Sealed for EMVSIM0 {}
    impl EMVSIM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EMVSIM0)
        }
    }

    pub type EMVSIM1 = Instance<1>;
    impl crate::sealed::Sealed for EMVSIM1 {}
    impl EMVSIM1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EMVSIM1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(EMVSIM0, 0), (EMVSIM1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod flexio {

    pub const FLEXIO0: *const RegisterBlock = 0x40105000 as *const RegisterBlock;

    #[path = "../../peripherals/flexio.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type FLEXIO0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for FLEXIO0 {}
    impl FLEXIO0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FLEXIO0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, FLEXIO0).then_some(0)
    }
}
#[path = "."]
pub mod sai {

    pub const SAI0: *const RegisterBlock = 0x40106000 as *const RegisterBlock;

    pub const SAI1: *const RegisterBlock = 0x40107000 as *const RegisterBlock;

    #[path = "../../peripherals/sai.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SAI0 = Instance<0>;
    impl crate::sealed::Sealed for SAI0 {}
    impl SAI0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SAI0)
        }
    }

    pub type SAI1 = Instance<1>;
    impl crate::sealed::Sealed for SAI1 {}
    impl SAI1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SAI1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(SAI0, 0), (SAI1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod sinc {

    pub const SINC0: *const RegisterBlock = 0x40108000 as *const RegisterBlock;

    #[path = "../../peripherals/sinc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SINC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SINC0 {}
    impl SINC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SINC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SINC0).then_some(0)
    }
}
#[path = "."]
pub mod usdhc {

    pub const USDHC0: *const RegisterBlock = 0x40109000 as *const RegisterBlock;

    #[path = "../../peripherals/usdhc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USDHC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USDHC0 {}
    impl USDHC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USDHC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USDHC0).then_some(0)
    }
}
#[path = "."]
pub mod usbphy {

    pub const USBPHY: *const RegisterBlock = 0x4010a000 as *const RegisterBlock;

    #[path = "../../peripherals/usbphy.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USBPHY = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USBPHY {}
    impl USBPHY {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBPHY)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBPHY).then_some(0)
    }
}
#[path = "."]
pub mod usbhsdcd {

    pub const USBHS1_PHY_DCD: *const RegisterBlock = 0x4010a800 as *const RegisterBlock;

    #[path = "../../peripherals/usbhsdcd.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USBHS1_PHY_DCD = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USBHS1_PHY_DCD {}
    impl USBHS1_PHY_DCD {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBHS1_PHY_DCD)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBHS1_PHY_DCD).then_some(0)
    }
}
#[path = "."]
pub mod usbhs {

    pub const USBHS1__USBC: *const RegisterBlock = 0x4010b000 as *const RegisterBlock;

    #[path = "../../peripherals/usbhs.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USBHS1__USBC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USBHS1__USBC {}
    impl USBHS1__USBC {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBHS1__USBC)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBHS1__USBC).then_some(0)
    }
}
#[path = "."]
pub mod usbnc {

    pub const USBHS1__USBNC: *const RegisterBlock = 0x4010b200 as *const RegisterBlock;

    #[path = "../../peripherals/usbnc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USBHS1__USBNC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USBHS1__USBNC {}
    impl USBHS1__USBNC {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBHS1__USBNC)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBHS1__USBNC).then_some(0)
    }
}
#[path = "."]
pub mod micfil {

    pub const PDM: *const RegisterBlock = 0x4010c000 as *const RegisterBlock;

    #[path = "../../peripherals/micfil.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PDM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PDM {}
    impl PDM {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PDM)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PDM).then_some(0)
    }
}
#[path = "."]
pub mod adc {

    pub const ADC0: *const RegisterBlock = 0x4010d000 as *const RegisterBlock;

    pub const ADC1: *const RegisterBlock = 0x4010e000 as *const RegisterBlock;

    #[path = "../../peripherals/adc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type ADC0 = Instance<0>;
    impl crate::sealed::Sealed for ADC0 {}
    impl ADC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ADC0)
        }
    }

    pub type ADC1 = Instance<1>;
    impl crate::sealed::Sealed for ADC1 {}
    impl ADC1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ADC1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(ADC0, 0), (ADC1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod dac {

    pub const DAC0: *const RegisterBlock = 0x4010f000 as *const RegisterBlock;

    pub const DAC1: *const RegisterBlock = 0x40112000 as *const RegisterBlock;

    #[path = "../../peripherals/dac.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type DAC0 = Instance<0>;
    impl crate::sealed::Sealed for DAC0 {}
    impl DAC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DAC0)
        }
    }

    pub type DAC1 = Instance<1>;
    impl crate::sealed::Sealed for DAC1 {}
    impl DAC1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DAC1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(DAC0, 0), (DAC1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod opamp {

    pub const OPAMP0: *const RegisterBlock = 0x40110000 as *const RegisterBlock;

    pub const OPAMP1: *const RegisterBlock = 0x40113000 as *const RegisterBlock;

    pub const OPAMP2: *const RegisterBlock = 0x40115000 as *const RegisterBlock;

    #[path = "../../peripherals/opamp.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type OPAMP0 = Instance<0>;
    impl crate::sealed::Sealed for OPAMP0 {}
    impl OPAMP0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OPAMP0)
        }
    }

    pub type OPAMP1 = Instance<1>;
    impl crate::sealed::Sealed for OPAMP1 {}
    impl OPAMP1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OPAMP1)
        }
    }

    pub type OPAMP2 = Instance<2>;
    impl crate::sealed::Sealed for OPAMP2 {}
    impl OPAMP2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OPAMP2)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(OPAMP0, 0), (OPAMP1, 1), (OPAMP2, 2)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod vref {

    pub const VREF0: *const RegisterBlock = 0x40111000 as *const RegisterBlock;

    #[path = "../../peripherals/vref.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type VREF0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for VREF0 {}
    impl VREF0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(VREF0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, VREF0).then_some(0)
    }
}
#[path = "."]
pub mod hpdac {

    pub const DAC2: *const RegisterBlock = 0x40114000 as *const RegisterBlock;

    #[path = "../../peripherals/hpdac.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type DAC2 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for DAC2 {}
    impl DAC2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DAC2)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DAC2).then_some(0)
    }
}
#[path = "."]
pub mod syscon {

    pub const SYSCON0: *const RegisterBlock = 0x40000000 as *const RegisterBlock;

    #[path = "../../peripherals/syscon_n54x.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SYSCON0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SYSCON0 {}
    impl SYSCON0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SYSCON0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SYSCON0).then_some(0)
    }
}
#[path = "."]
pub mod pint {

    pub const PINT0: *const RegisterBlock = 0x40004000 as *const RegisterBlock;

    #[path = "../../peripherals/pint.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PINT0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PINT0 {}
    impl PINT0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PINT0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PINT0).then_some(0)
    }
}
#[path = "."]
pub mod inputmux {

    pub const INPUTMUX0: *const RegisterBlock = 0x40006000 as *const RegisterBlock;

    #[path = "../../peripherals/inputmux.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type INPUTMUX0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for INPUTMUX0 {}
    impl INPUTMUX0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(INPUTMUX0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, INPUTMUX0).then_some(0)
    }
}
#[path = "."]
pub mod ctimer {

    pub const CTIMER0: *const RegisterBlock = 0x4000c000 as *const RegisterBlock;

    pub const CTIMER1: *const RegisterBlock = 0x4000d000 as *const RegisterBlock;

    pub const CTIMER2: *const RegisterBlock = 0x4000e000 as *const RegisterBlock;

    pub const CTIMER3: *const RegisterBlock = 0x4000f000 as *const RegisterBlock;

    pub const CTIMER4: *const RegisterBlock = 0x40010000 as *const RegisterBlock;

    #[path = "../../peripherals/ctimer.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CTIMER0 = Instance<0>;
    impl crate::sealed::Sealed for CTIMER0 {}
    impl CTIMER0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CTIMER0)
        }
    }

    pub type CTIMER1 = Instance<1>;
    impl crate::sealed::Sealed for CTIMER1 {}
    impl CTIMER1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CTIMER1)
        }
    }

    pub type CTIMER2 = Instance<2>;
    impl crate::sealed::Sealed for CTIMER2 {}
    impl CTIMER2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CTIMER2)
        }
    }

    pub type CTIMER3 = Instance<3>;
    impl crate::sealed::Sealed for CTIMER3 {}
    impl CTIMER3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CTIMER3)
        }
    }

    pub type CTIMER4 = Instance<4>;
    impl crate::sealed::Sealed for CTIMER4 {}
    impl CTIMER4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CTIMER4)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (CTIMER0, 0),
            (CTIMER1, 1),
            (CTIMER2, 2),
            (CTIMER3, 3),
            (CTIMER4, 4),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod freqme {

    pub const FREQME0: *const RegisterBlock = 0x40011000 as *const RegisterBlock;

    #[path = "../../peripherals/freqme.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type FREQME0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for FREQME0 {}
    impl FREQME0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FREQME0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, FREQME0).then_some(0)
    }
}
#[path = "."]
pub mod utick {

    pub const UTICK0: *const RegisterBlock = 0x40012000 as *const RegisterBlock;

    #[path = "../../peripherals/utick.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type UTICK0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for UTICK0 {}
    impl UTICK0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(UTICK0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, UTICK0).then_some(0)
    }
}
#[path = "."]
pub mod mrt {

    pub const MRT0: *const RegisterBlock = 0x40013000 as *const RegisterBlock;

    #[path = "../../peripherals/mrt.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type MRT0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for MRT0 {}
    impl MRT0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MRT0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, MRT0).then_some(0)
    }
}
#[path = "."]
pub mod wwdt {

    pub const WWDT0: *const RegisterBlock = 0x40016000 as *const RegisterBlock;

    pub const WWDT1: *const RegisterBlock = 0x40017000 as *const RegisterBlock;

    #[path = "../../peripherals/wwdt.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type WWDT0 = Instance<0>;
    impl crate::sealed::Sealed for WWDT0 {}
    impl WWDT0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(WWDT0)
        }
    }

    pub type WWDT1 = Instance<1>;
    impl crate::sealed::Sealed for WWDT1 {}
    impl WWDT1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(WWDT1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(WWDT0, 0), (WWDT1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod cache64_ctrl {

    pub const CACHE64_CTRL0: *const RegisterBlock = 0x4001b000 as *const RegisterBlock;

    #[path = "../../peripherals/cache64_ctrl.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CACHE64_CTRL0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for CACHE64_CTRL0 {}
    impl CACHE64_CTRL0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CACHE64_CTRL0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CACHE64_CTRL0).then_some(0)
    }
}
#[path = "."]
pub mod cache64_polsel {

    pub const CACHE64_POLSEL0: *const RegisterBlock = 0x4001b000 as *const RegisterBlock;

    #[path = "../../peripherals/cache64_polsel.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CACHE64_POLSEL0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for CACHE64_POLSEL0 {}
    impl CACHE64_POLSEL0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CACHE64_POLSEL0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CACHE64_POLSEL0).then_some(0)
    }
}
#[path = "."]
pub mod i3c {

    pub const I3C0: *const RegisterBlock = 0x40021000 as *const RegisterBlock;

    pub const I3C1: *const RegisterBlock = 0x40022000 as *const RegisterBlock;

    #[path = "../../peripherals/i3c.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type I3C0 = Instance<0>;
    impl crate::sealed::Sealed for I3C0 {}
    impl I3C0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I3C0)
        }
    }

    pub type I3C1 = Instance<1>;
    impl crate::sealed::Sealed for I3C1 {}
    impl I3C1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(I3C1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(I3C0, 0), (I3C1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod gdet {

    pub const GDET0: *const RegisterBlock = 0x40024000 as *const RegisterBlock;

    pub const GDET1: *const RegisterBlock = 0x40025000 as *const RegisterBlock;

    #[path = "../../peripherals/gdet.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type GDET0 = Instance<0>;
    impl crate::sealed::Sealed for GDET0 {}
    impl GDET0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GDET0)
        }
    }

    pub type GDET1 = Instance<1>;
    impl crate::sealed::Sealed for GDET1 {}
    impl GDET1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GDET1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(GDET0, 0), (GDET1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod itrc {

    pub const ITRC0: *const RegisterBlock = 0x40026000 as *const RegisterBlock;

    #[path = "../../peripherals/itrc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type ITRC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for ITRC0 {}
    impl ITRC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ITRC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ITRC0).then_some(0)
    }
}
#[path = "."]
pub mod pkc {

    pub const PKC0: *const RegisterBlock = 0x4002b000 as *const RegisterBlock;

    #[path = "../../peripherals/pkc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PKC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PKC0 {}
    impl PKC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PKC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PKC0).then_some(0)
    }
}
#[path = "."]
pub mod puf {

    pub const PUF: *const RegisterBlock = 0x4002c000 as *const RegisterBlock;

    #[path = "../../peripherals/puf.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PUF = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PUF {}
    impl PUF {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PUF)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PUF).then_some(0)
    }
}
#[path = "."]
pub mod puf_ctrl {

    pub const PUF_CTRL: *const RegisterBlock = 0x4002c000 as *const RegisterBlock;

    #[path = "../../peripherals/puf_ctrl.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PUF_CTRL = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PUF_CTRL {}
    impl PUF_CTRL {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PUF_CTRL)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PUF_CTRL).then_some(0)
    }
}
#[path = "."]
pub mod sm3 {

    pub const SM3_0: *const RegisterBlock = 0x40031000 as *const RegisterBlock;

    #[path = "../../peripherals/sm3.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SM3_0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SM3_0 {}
    impl SM3_0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SM3_0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SM3_0).then_some(0)
    }
}
#[path = "."]
pub mod bsp32 {

    pub const BSP32_0: *const RegisterBlock = 0x40032000 as *const RegisterBlock;

    #[path = "../../peripherals/bsp32.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type BSP32_0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for BSP32_0 {}
    impl BSP32_0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(BSP32_0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, BSP32_0).then_some(0)
    }
}
#[path = "."]
pub mod smartdma {

    pub const SMARTDMA0: *const RegisterBlock = 0x40033000 as *const RegisterBlock;

    #[path = "../../peripherals/smartdma.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SMARTDMA0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SMARTDMA0 {}
    impl SMARTDMA0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SMARTDMA0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SMARTDMA0).then_some(0)
    }
}
#[path = "."]
pub mod plu {

    pub const PLU0: *const RegisterBlock = 0x40034000 as *const RegisterBlock;

    #[path = "../../peripherals/plu.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PLU0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PLU0 {}
    impl PLU0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PLU0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PLU0).then_some(0)
    }
}
#[path = "."]
pub mod port {

    pub const PORT0: *const RegisterBlock = 0x40116000 as *const RegisterBlock;

    pub const PORT1: *const RegisterBlock = 0x40117000 as *const RegisterBlock;

    pub const PORT2: *const RegisterBlock = 0x40118000 as *const RegisterBlock;

    pub const PORT3: *const RegisterBlock = 0x40119000 as *const RegisterBlock;

    pub const PORT4: *const RegisterBlock = 0x4011a000 as *const RegisterBlock;

    pub const PORT5: *const RegisterBlock = 0x40042000 as *const RegisterBlock;

    #[path = "../../peripherals/port.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PORT0 = Instance<0>;
    impl crate::sealed::Sealed for PORT0 {}
    impl PORT0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PORT0)
        }
    }

    pub type PORT1 = Instance<1>;
    impl crate::sealed::Sealed for PORT1 {}
    impl PORT1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PORT1)
        }
    }

    pub type PORT2 = Instance<2>;
    impl crate::sealed::Sealed for PORT2 {}
    impl PORT2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PORT2)
        }
    }

    pub type PORT3 = Instance<3>;
    impl crate::sealed::Sealed for PORT3 {}
    impl PORT3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PORT3)
        }
    }

    pub type PORT4 = Instance<4>;
    impl crate::sealed::Sealed for PORT4 {}
    impl PORT4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PORT4)
        }
    }

    pub type PORT5 = Instance<5>;
    impl crate::sealed::Sealed for PORT5 {}
    impl PORT5 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PORT5)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (PORT0, 0),
            (PORT1, 1),
            (PORT2, 2),
            (PORT3, 3),
            (PORT4, 4),
            (PORT5, 5),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod msf1_b {

    pub const FMU0: *const RegisterBlock = 0x40043000 as *const RegisterBlock;

    #[path = "../../peripherals/msf1_b.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type FMU0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for FMU0 {}
    impl FMU0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FMU0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, FMU0).then_some(0)
    }
}
#[path = "."]
pub mod scg {

    pub const SCG0: *const RegisterBlock = 0x40044000 as *const RegisterBlock;

    #[path = "../../peripherals/scg.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SCG0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SCG0 {}
    impl SCG0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SCG0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SCG0).then_some(0)
    }
}
#[path = "."]
pub mod spc {

    pub const SPC0: *const RegisterBlock = 0x40045000 as *const RegisterBlock;

    #[path = "../../peripherals/spc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SPC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SPC0 {}
    impl SPC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SPC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SPC0).then_some(0)
    }
}
#[path = "."]
pub mod wuu {

    pub const WUU0: *const RegisterBlock = 0x40046000 as *const RegisterBlock;

    #[path = "../../peripherals/wuu.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type WUU0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for WUU0 {}
    impl WUU0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(WUU0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, WUU0).then_some(0)
    }
}
#[path = "."]
pub mod cmc {

    pub const CMC0: *const RegisterBlock = 0x40048000 as *const RegisterBlock;

    #[path = "../../peripherals/cmc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CMC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for CMC0 {}
    impl CMC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CMC0).then_some(0)
    }
}
#[path = "."]
pub mod ostimer {

    pub const OSTIMER0: *const RegisterBlock = 0x40049000 as *const RegisterBlock;

    #[path = "../../peripherals/ostimer.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type OSTIMER0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for OSTIMER0 {}
    impl OSTIMER0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OSTIMER0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, OSTIMER0).then_some(0)
    }
}
#[path = "."]
pub mod lptmr {

    pub const LPTMR0: *const RegisterBlock = 0x4004a000 as *const RegisterBlock;

    pub const LPTMR1: *const RegisterBlock = 0x4004b000 as *const RegisterBlock;

    #[path = "../../peripherals/lptmr.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type LPTMR0 = Instance<0>;
    impl crate::sealed::Sealed for LPTMR0 {}
    impl LPTMR0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPTMR0)
        }
    }

    pub type LPTMR1 = Instance<1>;
    impl crate::sealed::Sealed for LPTMR1 {}
    impl LPTMR1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPTMR1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(LPTMR0, 0), (LPTMR1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod rtc {

    pub const RTC0: *const RegisterBlock = 0x4004c000 as *const RegisterBlock;

    #[path = "../../peripherals/rtc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type RTC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for RTC0 {}
    impl RTC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, RTC0).then_some(0)
    }
}
#[path = "."]
pub mod rtc_subsystem {

    pub const RTC_SUBSYSTEM0: *const RegisterBlock = 0x4004c000 as *const RegisterBlock;

    #[path = "../../peripherals/rtc_subsystem.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type RTC_SUBSYSTEM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for RTC_SUBSYSTEM0 {}
    impl RTC_SUBSYSTEM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(RTC_SUBSYSTEM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, RTC_SUBSYSTEM0).then_some(0)
    }
}
#[path = "."]
pub mod tsi {

    pub const TSI0: *const RegisterBlock = 0x40050000 as *const RegisterBlock;

    #[path = "../../peripherals/tsi.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type TSI0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for TSI0 {}
    impl TSI0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TSI0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TSI0).then_some(0)
    }
}
#[path = "."]
pub mod cmp {

    pub const CMP0: *const RegisterBlock = 0x40051000 as *const RegisterBlock;

    pub const CMP1: *const RegisterBlock = 0x40052000 as *const RegisterBlock;

    #[path = "../../peripherals/cmp.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CMP0 = Instance<0>;
    impl crate::sealed::Sealed for CMP0 {}
    impl CMP0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMP0)
        }
    }

    pub type CMP1 = Instance<1>;
    impl crate::sealed::Sealed for CMP1 {}
    impl CMP1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMP1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CMP0, 0), (CMP1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod s50 {

    pub const ELS: *const RegisterBlock = 0x40054000 as *const RegisterBlock;

    #[path = "../../peripherals/s50.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type ELS = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for ELS {}
    impl ELS {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ELS)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ELS).then_some(0)
    }
}
#[path = "."]
pub mod digtmp {

    pub const TDET0: *const RegisterBlock = 0x40058000 as *const RegisterBlock;

    #[path = "../../peripherals/digtmp.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type TDET0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for TDET0 {}
    impl TDET0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TDET0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TDET0).then_some(0)
    }
}
#[path = "."]
pub mod vbat {

    pub const VBAT0: *const RegisterBlock = 0x40059000 as *const RegisterBlock;

    #[path = "../../peripherals/vbat.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type VBAT0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for VBAT0 {}
    impl VBAT0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(VBAT0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, VBAT0).then_some(0)
    }
}
#[path = "."]
pub mod trng {

    pub const TRNG0: *const RegisterBlock = 0x4005a000 as *const RegisterBlock;

    #[path = "../../peripherals/trng.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type TRNG0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for TRNG0 {}
    impl TRNG0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TRNG0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TRNG0).then_some(0)
    }
}
#[path = "."]
pub mod eim {

    pub const EIM0: *const RegisterBlock = 0x4005b000 as *const RegisterBlock;

    #[path = "../../peripherals/eim.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type EIM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for EIM0 {}
    impl EIM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EIM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, EIM0).then_some(0)
    }
}
#[path = "."]
pub mod erm {

    pub const ERM0: *const RegisterBlock = 0x4005c000 as *const RegisterBlock;

    #[path = "../../peripherals/erm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type ERM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for ERM0 {}
    impl ERM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ERM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ERM0).then_some(0)
    }
}
#[path = "."]
pub mod intm {

    pub const INTM0: *const RegisterBlock = 0x4005d000 as *const RegisterBlock;

    #[path = "../../peripherals/intm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type INTM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for INTM0 {}
    impl INTM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(INTM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, INTM0).then_some(0)
    }
}
#[path = "."]
pub mod dma {

    pub const DMA0: *const RegisterBlock = 0x40080000 as *const RegisterBlock;

    pub const DMA1: *const RegisterBlock = 0x400a0000 as *const RegisterBlock;

    #[path = "../../peripherals/dma.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type DMA0 = Instance<0>;
    impl crate::sealed::Sealed for DMA0 {}
    impl DMA0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DMA0)
        }
    }

    pub type DMA1 = Instance<1>;
    impl crate::sealed::Sealed for DMA1 {}
    impl DMA1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DMA1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(DMA0, 0), (DMA1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod dma_tcd {

    pub const EDMA_TCD0: *const RegisterBlock = 0x40081000 as *const RegisterBlock;

    pub const EDMA_TCD1: *const RegisterBlock = 0x400a1000 as *const RegisterBlock;

    #[path = "../../peripherals/dma_tcd.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type EDMA_TCD0 = Instance<0>;
    impl crate::sealed::Sealed for EDMA_TCD0 {}
    impl EDMA_TCD0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EDMA_TCD0)
        }
    }

    pub type EDMA_TCD1 = Instance<1>;
    impl crate::sealed::Sealed for EDMA_TCD1 {}
    impl EDMA_TCD1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EDMA_TCD1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(EDMA_TCD0, 0), (EDMA_TCD1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod sct {

    pub const SCT0: *const RegisterBlock = 0x40091000 as *const RegisterBlock;

    #[path = "../../peripherals/sct.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SCT0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SCT0 {}
    impl SCT0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SCT0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SCT0).then_some(0)
    }
}
#[path = "."]
pub mod lpspi {

    pub const LPSPI0: *const RegisterBlock = 0x40092000 as *const RegisterBlock;

    pub const LPSPI1: *const RegisterBlock = 0x40093000 as *const RegisterBlock;

    pub const LPSPI2: *const RegisterBlock = 0x40094000 as *const RegisterBlock;

    pub const LPSPI3: *const RegisterBlock = 0x40095000 as *const RegisterBlock;

    pub const LPSPI4: *const RegisterBlock = 0x400b4000 as *const RegisterBlock;

    pub const LPSPI5: *const RegisterBlock = 0x400b5000 as *const RegisterBlock;

    pub const LPSPI6: *const RegisterBlock = 0x400b6000 as *const RegisterBlock;

    pub const LPSPI7: *const RegisterBlock = 0x400b7000 as *const RegisterBlock;

    pub const LPSPI8: *const RegisterBlock = 0x400b8000 as *const RegisterBlock;

    pub const LPSPI9: *const RegisterBlock = 0x400b9000 as *const RegisterBlock;

    #[path = "../../peripherals/lpspi.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type LPSPI0 = Instance<0>;
    impl crate::sealed::Sealed for LPSPI0 {}
    impl LPSPI0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI0)
        }
    }

    pub type LPSPI1 = Instance<1>;
    impl crate::sealed::Sealed for LPSPI1 {}
    impl LPSPI1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI1)
        }
    }

    pub type LPSPI2 = Instance<2>;
    impl crate::sealed::Sealed for LPSPI2 {}
    impl LPSPI2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI2)
        }
    }

    pub type LPSPI3 = Instance<3>;
    impl crate::sealed::Sealed for LPSPI3 {}
    impl LPSPI3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI3)
        }
    }

    pub type LPSPI4 = Instance<4>;
    impl crate::sealed::Sealed for LPSPI4 {}
    impl LPSPI4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI4)
        }
    }

    pub type LPSPI5 = Instance<5>;
    impl crate::sealed::Sealed for LPSPI5 {}
    impl LPSPI5 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI5)
        }
    }

    pub type LPSPI6 = Instance<6>;
    impl crate::sealed::Sealed for LPSPI6 {}
    impl LPSPI6 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI6)
        }
    }

    pub type LPSPI7 = Instance<7>;
    impl crate::sealed::Sealed for LPSPI7 {}
    impl LPSPI7 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI7)
        }
    }

    pub type LPSPI8 = Instance<8>;
    impl crate::sealed::Sealed for LPSPI8 {}
    impl LPSPI8 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI8)
        }
    }

    pub type LPSPI9 = Instance<9>;
    impl crate::sealed::Sealed for LPSPI9 {}
    impl LPSPI9 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPSPI9)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LPSPI0, 0),
            (LPSPI1, 1),
            (LPSPI2, 2),
            (LPSPI3, 3),
            (LPSPI4, 4),
            (LPSPI5, 5),
            (LPSPI6, 6),
            (LPSPI7, 7),
            (LPSPI8, 8),
            (LPSPI9, 9),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpuart {

    pub const LPUART0: *const RegisterBlock = 0x40092000 as *const RegisterBlock;

    pub const LPUART1: *const RegisterBlock = 0x40093000 as *const RegisterBlock;

    pub const LPUART2: *const RegisterBlock = 0x40094000 as *const RegisterBlock;

    pub const LPUART3: *const RegisterBlock = 0x40095000 as *const RegisterBlock;

    pub const LPUART4: *const RegisterBlock = 0x400b4000 as *const RegisterBlock;

    pub const LPUART5: *const RegisterBlock = 0x400b5000 as *const RegisterBlock;

    pub const LPUART6: *const RegisterBlock = 0x400b6000 as *const RegisterBlock;

    pub const LPUART7: *const RegisterBlock = 0x400b7000 as *const RegisterBlock;

    pub const LPUART8: *const RegisterBlock = 0x400b8000 as *const RegisterBlock;

    pub const LPUART9: *const RegisterBlock = 0x400b9000 as *const RegisterBlock;

    #[path = "../../peripherals/lpuart.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type LPUART0 = Instance<0>;
    impl crate::sealed::Sealed for LPUART0 {}
    impl LPUART0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART0)
        }
    }

    pub type LPUART1 = Instance<1>;
    impl crate::sealed::Sealed for LPUART1 {}
    impl LPUART1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART1)
        }
    }

    pub type LPUART2 = Instance<2>;
    impl crate::sealed::Sealed for LPUART2 {}
    impl LPUART2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART2)
        }
    }

    pub type LPUART3 = Instance<3>;
    impl crate::sealed::Sealed for LPUART3 {}
    impl LPUART3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART3)
        }
    }

    pub type LPUART4 = Instance<4>;
    impl crate::sealed::Sealed for LPUART4 {}
    impl LPUART4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART4)
        }
    }

    pub type LPUART5 = Instance<5>;
    impl crate::sealed::Sealed for LPUART5 {}
    impl LPUART5 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART5)
        }
    }

    pub type LPUART6 = Instance<6>;
    impl crate::sealed::Sealed for LPUART6 {}
    impl LPUART6 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART6)
        }
    }

    pub type LPUART7 = Instance<7>;
    impl crate::sealed::Sealed for LPUART7 {}
    impl LPUART7 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART7)
        }
    }

    pub type LPUART8 = Instance<8>;
    impl crate::sealed::Sealed for LPUART8 {}
    impl LPUART8 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART8)
        }
    }

    pub type LPUART9 = Instance<9>;
    impl crate::sealed::Sealed for LPUART9 {}
    impl LPUART9 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPUART9)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LPUART0, 0),
            (LPUART1, 1),
            (LPUART2, 2),
            (LPUART3, 3),
            (LPUART4, 4),
            (LPUART5, 5),
            (LPUART6, 6),
            (LPUART7, 7),
            (LPUART8, 8),
            (LPUART9, 9),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lp_flexcomm {

    pub const LP_FLEXCOMM0: *const RegisterBlock = 0x40092000 as *const RegisterBlock;

    pub const LP_FLEXCOMM1: *const RegisterBlock = 0x40093000 as *const RegisterBlock;

    pub const LP_FLEXCOMM2: *const RegisterBlock = 0x40094000 as *const RegisterBlock;

    pub const LP_FLEXCOMM3: *const RegisterBlock = 0x40095000 as *const RegisterBlock;

    pub const LP_FLEXCOMM4: *const RegisterBlock = 0x400b4000 as *const RegisterBlock;

    pub const LP_FLEXCOMM5: *const RegisterBlock = 0x400b5000 as *const RegisterBlock;

    pub const LP_FLEXCOMM6: *const RegisterBlock = 0x400b6000 as *const RegisterBlock;

    pub const LP_FLEXCOMM7: *const RegisterBlock = 0x400b7000 as *const RegisterBlock;

    pub const LP_FLEXCOMM8: *const RegisterBlock = 0x400b8000 as *const RegisterBlock;

    pub const LP_FLEXCOMM9: *const RegisterBlock = 0x400b9000 as *const RegisterBlock;

    #[path = "../../peripherals/lp_flexcomm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type LP_FLEXCOMM0 = Instance<0>;
    impl crate::sealed::Sealed for LP_FLEXCOMM0 {}
    impl LP_FLEXCOMM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM0)
        }
    }

    pub type LP_FLEXCOMM1 = Instance<1>;
    impl crate::sealed::Sealed for LP_FLEXCOMM1 {}
    impl LP_FLEXCOMM1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM1)
        }
    }

    pub type LP_FLEXCOMM2 = Instance<2>;
    impl crate::sealed::Sealed for LP_FLEXCOMM2 {}
    impl LP_FLEXCOMM2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM2)
        }
    }

    pub type LP_FLEXCOMM3 = Instance<3>;
    impl crate::sealed::Sealed for LP_FLEXCOMM3 {}
    impl LP_FLEXCOMM3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM3)
        }
    }

    pub type LP_FLEXCOMM4 = Instance<4>;
    impl crate::sealed::Sealed for LP_FLEXCOMM4 {}
    impl LP_FLEXCOMM4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM4)
        }
    }

    pub type LP_FLEXCOMM5 = Instance<5>;
    impl crate::sealed::Sealed for LP_FLEXCOMM5 {}
    impl LP_FLEXCOMM5 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM5)
        }
    }

    pub type LP_FLEXCOMM6 = Instance<6>;
    impl crate::sealed::Sealed for LP_FLEXCOMM6 {}
    impl LP_FLEXCOMM6 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM6)
        }
    }

    pub type LP_FLEXCOMM7 = Instance<7>;
    impl crate::sealed::Sealed for LP_FLEXCOMM7 {}
    impl LP_FLEXCOMM7 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM7)
        }
    }

    pub type LP_FLEXCOMM8 = Instance<8>;
    impl crate::sealed::Sealed for LP_FLEXCOMM8 {}
    impl LP_FLEXCOMM8 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM8)
        }
    }

    pub type LP_FLEXCOMM9 = Instance<9>;
    impl crate::sealed::Sealed for LP_FLEXCOMM9 {}
    impl LP_FLEXCOMM9 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LP_FLEXCOMM9)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LP_FLEXCOMM0, 0),
            (LP_FLEXCOMM1, 1),
            (LP_FLEXCOMM2, 2),
            (LP_FLEXCOMM3, 3),
            (LP_FLEXCOMM4, 4),
            (LP_FLEXCOMM5, 5),
            (LP_FLEXCOMM6, 6),
            (LP_FLEXCOMM7, 7),
            (LP_FLEXCOMM8, 8),
            (LP_FLEXCOMM9, 9),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod lpi2c {

    pub const LPI2C0: *const RegisterBlock = 0x40092800 as *const RegisterBlock;

    pub const LPI2C1: *const RegisterBlock = 0x40093800 as *const RegisterBlock;

    pub const LPI2C2: *const RegisterBlock = 0x40094800 as *const RegisterBlock;

    pub const LPI2C3: *const RegisterBlock = 0x40095800 as *const RegisterBlock;

    pub const LPI2C4: *const RegisterBlock = 0x400b4800 as *const RegisterBlock;

    pub const LPI2C5: *const RegisterBlock = 0x400b5800 as *const RegisterBlock;

    pub const LPI2C6: *const RegisterBlock = 0x400b6800 as *const RegisterBlock;

    pub const LPI2C7: *const RegisterBlock = 0x400b7800 as *const RegisterBlock;

    pub const LPI2C8: *const RegisterBlock = 0x400b8800 as *const RegisterBlock;

    pub const LPI2C9: *const RegisterBlock = 0x400b9800 as *const RegisterBlock;

    #[path = "../../peripherals/lpi2c.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type LPI2C0 = Instance<0>;
    impl crate::sealed::Sealed for LPI2C0 {}
    impl LPI2C0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C0)
        }
    }

    pub type LPI2C1 = Instance<1>;
    impl crate::sealed::Sealed for LPI2C1 {}
    impl LPI2C1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C1)
        }
    }

    pub type LPI2C2 = Instance<2>;
    impl crate::sealed::Sealed for LPI2C2 {}
    impl LPI2C2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C2)
        }
    }

    pub type LPI2C3 = Instance<3>;
    impl crate::sealed::Sealed for LPI2C3 {}
    impl LPI2C3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C3)
        }
    }

    pub type LPI2C4 = Instance<4>;
    impl crate::sealed::Sealed for LPI2C4 {}
    impl LPI2C4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C4)
        }
    }

    pub type LPI2C5 = Instance<5>;
    impl crate::sealed::Sealed for LPI2C5 {}
    impl LPI2C5 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C5)
        }
    }

    pub type LPI2C6 = Instance<6>;
    impl crate::sealed::Sealed for LPI2C6 {}
    impl LPI2C6 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C6)
        }
    }

    pub type LPI2C7 = Instance<7>;
    impl crate::sealed::Sealed for LPI2C7 {}
    impl LPI2C7 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C7)
        }
    }

    pub type LPI2C8 = Instance<8>;
    impl crate::sealed::Sealed for LPI2C8 {}
    impl LPI2C8 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C8)
        }
    }

    pub type LPI2C9 = Instance<9>;
    impl crate::sealed::Sealed for LPI2C9 {}
    impl LPI2C9 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(LPI2C9)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (LPI2C0, 0),
            (LPI2C1, 1),
            (LPI2C2, 2),
            (LPI2C3, 3),
            (LPI2C4, 4),
            (LPI2C5, 5),
            (LPI2C6, 6),
            (LPI2C7, 7),
            (LPI2C8, 8),
            (LPI2C9, 9),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod gpio {

    pub const GPIO0: *const RegisterBlock = 0x40096000 as *const RegisterBlock;

    pub const GPIO1: *const RegisterBlock = 0x40098000 as *const RegisterBlock;

    pub const GPIO2: *const RegisterBlock = 0x4009a000 as *const RegisterBlock;

    pub const GPIO3: *const RegisterBlock = 0x4009c000 as *const RegisterBlock;

    pub const GPIO4: *const RegisterBlock = 0x4009e000 as *const RegisterBlock;

    pub const GPIO5: *const RegisterBlock = 0x40040000 as *const RegisterBlock;

    #[path = "../../peripherals/gpio.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type GPIO0 = Instance<0>;
    impl crate::sealed::Sealed for GPIO0 {}
    impl GPIO0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO0)
        }
    }

    pub type GPIO1 = Instance<1>;
    impl crate::sealed::Sealed for GPIO1 {}
    impl GPIO1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO1)
        }
    }

    pub type GPIO2 = Instance<2>;
    impl crate::sealed::Sealed for GPIO2 {}
    impl GPIO2 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO2)
        }
    }

    pub type GPIO3 = Instance<3>;
    impl crate::sealed::Sealed for GPIO3 {}
    impl GPIO3 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO3)
        }
    }

    pub type GPIO4 = Instance<4>;
    impl crate::sealed::Sealed for GPIO4 {}
    impl GPIO4 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO4)
        }
    }

    pub type GPIO5 = Instance<5>;
    impl crate::sealed::Sealed for GPIO5 {}
    impl GPIO5 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(GPIO5)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [
            (GPIO0, 0),
            (GPIO1, 1),
            (GPIO2, 2),
            (GPIO3, 3),
            (GPIO4, 4),
            (GPIO5, 5),
        ]
        .into_iter()
        .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
        .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod sema42 {

    pub const SEMA42_0: *const RegisterBlock = 0x400b1000 as *const RegisterBlock;

    #[path = "../../peripherals/sema42.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SEMA42_0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for SEMA42_0 {}
    impl SEMA42_0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SEMA42_0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, SEMA42_0).then_some(0)
    }
}
#[path = "."]
pub mod mailbox {

    pub const MAILBOX: *const RegisterBlock = 0x400b2000 as *const RegisterBlock;

    #[path = "../../peripherals/mailbox.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type MAILBOX = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for MAILBOX {}
    impl MAILBOX {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(MAILBOX)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, MAILBOX).then_some(0)
    }
}
#[path = "."]
pub mod cdog {

    pub const CDOG0: *const RegisterBlock = 0x400bb000 as *const RegisterBlock;

    pub const CDOG1: *const RegisterBlock = 0x400bc000 as *const RegisterBlock;

    #[path = "../../peripherals/cdog.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CDOG0 = Instance<0>;
    impl crate::sealed::Sealed for CDOG0 {}
    impl CDOG0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CDOG0)
        }
    }

    pub type CDOG1 = Instance<1>;
    impl crate::sealed::Sealed for CDOG1 {}
    impl CDOG1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CDOG1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CDOG0, 0), (CDOG1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod dm {

    pub const DM0: *const RegisterBlock = 0x400bd000 as *const RegisterBlock;

    #[path = "../../peripherals/dm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type DM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for DM0 {}
    impl DM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DM0).then_some(0)
    }
}
#[path = "."]
pub mod powerquad {

    pub const POWERQUAD: *const RegisterBlock = 0x400bf000 as *const RegisterBlock;

    #[path = "../../peripherals/powerquad.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type POWERQUAD = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for POWERQUAD {}
    impl POWERQUAD {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(POWERQUAD)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, POWERQUAD).then_some(0)
    }
}
#[path = "."]
pub mod ewm {

    pub const EWM0: *const RegisterBlock = 0x400c0000 as *const RegisterBlock;

    #[path = "../../peripherals/ewm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type EWM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for EWM0 {}
    impl EWM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EWM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, EWM0).then_some(0)
    }
}
#[path = "."]
pub mod syspm {

    pub const CMX_PERFMON0: *const RegisterBlock = 0x400c1000 as *const RegisterBlock;

    pub const CMX_PERFMON1: *const RegisterBlock = 0x400c2000 as *const RegisterBlock;

    #[path = "../../peripherals/syspm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CMX_PERFMON0 = Instance<0>;
    impl crate::sealed::Sealed for CMX_PERFMON0 {}
    impl CMX_PERFMON0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMX_PERFMON0)
        }
    }

    pub type CMX_PERFMON1 = Instance<1>;
    impl crate::sealed::Sealed for CMX_PERFMON1 {}
    impl CMX_PERFMON1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CMX_PERFMON1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(CMX_PERFMON0, 0), (CMX_PERFMON1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod mbc {

    pub const TRDC: *const RegisterBlock = 0x400c6000 as *const RegisterBlock;

    #[path = "../../peripherals/mbc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type TRDC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for TRDC {}
    impl TRDC {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(TRDC)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, TRDC).then_some(0)
    }
}
#[path = "."]
pub mod flexspi {

    pub const FLEXSPI0: *const RegisterBlock = 0x400c8000 as *const RegisterBlock;

    #[path = "../../peripherals/flexspi.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type FLEXSPI0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for FLEXSPI0 {}
    impl FLEXSPI0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FLEXSPI0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, FLEXSPI0).then_some(0)
    }
}
#[path = "."]
pub mod otpc {

    pub const OTPC0: *const RegisterBlock = 0x400c9000 as *const RegisterBlock;

    #[path = "../../peripherals/otpc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type OTPC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for OTPC0 {}
    impl OTPC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(OTPC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, OTPC0).then_some(0)
    }
}
#[path = "."]
pub mod crc {

    pub const CRC0: *const RegisterBlock = 0x400cb000 as *const RegisterBlock;

    #[path = "../../peripherals/crc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CRC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for CRC0 {}
    impl CRC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CRC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CRC0).then_some(0)
    }
}
#[path = "."]
pub mod npx {

    pub const NPX0: *const RegisterBlock = 0x400cc000 as *const RegisterBlock;

    #[path = "../../peripherals/npx.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type NPX0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for NPX0 {}
    impl NPX0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(NPX0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, NPX0).then_some(0)
    }
}
#[path = "."]
pub mod pwm {

    pub const PWM0: *const RegisterBlock = 0x400ce000 as *const RegisterBlock;

    #[path = "../../peripherals/pwm.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PWM0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PWM0 {}
    impl PWM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PWM0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PWM0).then_some(0)
    }
}
#[path = "."]
pub mod enc {

    pub const ENC0: *const RegisterBlock = 0x400cf000 as *const RegisterBlock;

    pub const ENC1: *const RegisterBlock = 0x400d1000 as *const RegisterBlock;

    #[path = "../../peripherals/enc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type ENC0 = Instance<0>;
    impl crate::sealed::Sealed for ENC0 {}
    impl ENC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENC0)
        }
    }

    pub type ENC1 = Instance<1>;
    impl crate::sealed::Sealed for ENC1 {}
    impl ENC1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENC1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(ENC0, 0), (ENC1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod evtg {

    pub const EVTG0: *const RegisterBlock = 0x400d2000 as *const RegisterBlock;

    #[path = "../../peripherals/evtg.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type EVTG0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for EVTG0 {}
    impl EVTG0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EVTG0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, EVTG0).then_some(0)
    }
}
#[path = "."]
pub mod can {

    pub const CAN0: *const RegisterBlock = 0x400d4000 as *const RegisterBlock;

    #[path = "../../peripherals/can.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type CAN0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for CAN0 {}
    impl CAN0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(CAN0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, CAN0).then_some(0)
    }
}
#[path = "."]
pub mod usbdcd {

    pub const USBDCD0: *const RegisterBlock = 0x400dc000 as *const RegisterBlock;

    #[path = "../../peripherals/usbdcd.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USBDCD0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USBDCD0 {}
    impl USBDCD0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBDCD0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBDCD0).then_some(0)
    }
}
#[path = "."]
pub mod usb_fs {

    pub const USBFS0: *const RegisterBlock = 0x400dd000 as *const RegisterBlock;

    #[path = "../../peripherals/usb_fs.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USBFS0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USBFS0 {}
    impl USBFS0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBFS0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBFS0).then_some(0)
    }
}
#[path = "."]
pub mod enet {

    pub const ENET0: *const RegisterBlock = 0x40100000 as *const RegisterBlock;

    #[path = "../../peripherals/enet.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type ENET0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for ENET0 {}
    impl ENET0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ENET0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, ENET0).then_some(0)
    }
}
#[path = "."]
pub mod emvsim {

    pub const EMVSIM0: *const RegisterBlock = 0x40103000 as *const RegisterBlock;

    pub const EMVSIM1: *const RegisterBlock = 0x40104000 as *const RegisterBlock;

    #[path = "../../peripherals/emvsim.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type EMVSIM0 = Instance<0>;
    impl crate::sealed::Sealed for EMVSIM0 {}
    impl EMVSIM0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EMVSIM0)
        }
    }

    pub type EMVSIM1 = Instance<1>;
    impl crate::sealed::Sealed for EMVSIM1 {}
    impl EMVSIM1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(EMVSIM1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(EMVSIM0, 0), (EMVSIM1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod flexio {

    pub const FLEXIO0: *const RegisterBlock = 0x40105000 as *const RegisterBlock;

    #[path = "../../peripherals/flexio.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type FLEXIO0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for FLEXIO0 {}
    impl FLEXIO0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(FLEXIO0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, FLEXIO0).then_some(0)
    }
}
#[path = "."]
pub mod sai {

    pub const SAI0: *const RegisterBlock = 0x40106000 as *const RegisterBlock;

    pub const SAI1: *const RegisterBlock = 0x40107000 as *const RegisterBlock;

    #[path = "../../peripherals/sai.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type SAI0 = Instance<0>;
    impl crate::sealed::Sealed for SAI0 {}
    impl SAI0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SAI0)
        }
    }

    pub type SAI1 = Instance<1>;
    impl crate::sealed::Sealed for SAI1 {}
    impl SAI1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(SAI1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(SAI0, 0), (SAI1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod usdhc {

    pub const USDHC0: *const RegisterBlock = 0x40109000 as *const RegisterBlock;

    #[path = "../../peripherals/usdhc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USDHC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USDHC0 {}
    impl USDHC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USDHC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USDHC0).then_some(0)
    }
}
#[path = "."]
pub mod usbphy {

    pub const USBPHY: *const RegisterBlock = 0x4010a000 as *const RegisterBlock;

    #[path = "../../peripherals/usbphy.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USBPHY = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USBPHY {}
    impl USBPHY {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBPHY)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBPHY).then_some(0)
    }
}
#[path = "."]
pub mod usbhsdcd {

    pub const USBHS1_PHY_DCD: *const RegisterBlock = 0x4010a800 as *const RegisterBlock;

    #[path = "../../peripherals/usbhsdcd.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USBHS1_PHY_DCD = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USBHS1_PHY_DCD {}
    impl USBHS1_PHY_DCD {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBHS1_PHY_DCD)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBHS1_PHY_DCD).then_some(0)
    }
}
#[path = "."]
pub mod usbhs {

    pub const USBHS1__USBC: *const RegisterBlock = 0x4010b000 as *const RegisterBlock;

    #[path = "../../peripherals/usbhs.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USBHS1__USBC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USBHS1__USBC {}
    impl USBHS1__USBC {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBHS1__USBC)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBHS1__USBC).then_some(0)
    }
}
#[path = "."]
pub mod usbnc {

    pub const USBHS1__USBNC: *const RegisterBlock = 0x4010b200 as *const RegisterBlock;

    #[path = "../../peripherals/usbnc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type USBHS1__USBNC = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for USBHS1__USBNC {}
    impl USBHS1__USBNC {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(USBHS1__USBNC)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, USBHS1__USBNC).then_some(0)
    }
}
#[path = "."]
pub mod micfil {

    pub const PDM: *const RegisterBlock = 0x4010c000 as *const RegisterBlock;

    #[path = "../../peripherals/micfil.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type PDM = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for PDM {}
    impl PDM {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(PDM)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, PDM).then_some(0)
    }
}
#[path = "."]
pub mod adc {

    pub const ADC0: *const RegisterBlock = 0x4010d000 as *const RegisterBlock;

    pub const ADC1: *const RegisterBlock = 0x4010e000 as *const RegisterBlock;

    #[path = "../../peripherals/adc.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type ADC0 = Instance<0>;
    impl crate::sealed::Sealed for ADC0 {}
    impl ADC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ADC0)
        }
    }

    pub type ADC1 = Instance<1>;
    impl crate::sealed::Sealed for ADC1 {}
    impl ADC1 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(ADC1)
        }
    }

    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        [(ADC0, 0), (ADC1, 1)]
            .into_iter()
            .find(|(ptr, _)| core::ptr::eq(rb, *ptr))
            .map(|(_, inst)| inst)
    }
}
#[path = "."]
pub mod dac {

    pub const DAC0: *const RegisterBlock = 0x4010f000 as *const RegisterBlock;

    #[path = "../../peripherals/dac.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type DAC0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for DAC0 {}
    impl DAC0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(DAC0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, DAC0).then_some(0)
    }
}
#[path = "."]
pub mod vref {

    pub const VREF0: *const RegisterBlock = 0x40111000 as *const RegisterBlock;

    #[path = "../../peripherals/vref.rs"]
    mod blocks;
    pub use blocks::*;

    pub type Instance<const N: u8> = crate::Instance<RegisterBlock, N>;

    pub type VREF0 = Instance<{ crate::SOLE_INSTANCE }>;
    impl crate::sealed::Sealed for VREF0 {}
    impl VREF0 {
        #[inline]
        pub const unsafe fn instance() -> Self {
            Instance::new(VREF0)
        }
    }
    pub fn number(rb: *const RegisterBlock) -> Option<u8> {
        core::ptr::eq(rb, VREF0).then_some(0)
    }
}
