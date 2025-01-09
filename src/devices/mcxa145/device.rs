#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "1 - Core Mode Controller interrupt"]
    CMC = 1,
    #[doc = "2 - DMA3_0_CH0 error or transfer complete"]
    DMA_CH0 = 2,
    #[doc = "3 - DMA3_0_CH1 error or transfer complete"]
    DMA_CH1 = 3,
    #[doc = "4 - DMA3_0_CH2 error or transfer complete"]
    DMA_CH2 = 4,
    #[doc = "5 - DMA3_0_CH3 error or transfer complete"]
    DMA_CH3 = 5,
    #[doc = "6 - DMA3_0_CH4 error or transfer complete"]
    DMA_CH4 = 6,
    #[doc = "7 - DMA3_0_CH5 error or transfer complete"]
    DMA_CH5 = 7,
    #[doc = "8 - DMA3_0_CH6 error or transfer complete"]
    DMA_CH6 = 8,
    #[doc = "9 - DMA3_0_CH7 error or transfer complete"]
    DMA_CH7 = 9,
    #[doc = "10 - ERM Single Bit error interrupt"]
    ERM0_SINGLE_BIT = 10,
    #[doc = "11 - ERM Multi Bit error interrupt"]
    ERM0_MULTI_BIT = 11,
    #[doc = "12 - Flash Management Unit interrupt"]
    FMU0 = 12,
    #[doc = "13 - GLIKEY Interrupt"]
    GLIKEY0 = 13,
    #[doc = "14 - MBC secure violation interrupt"]
    MBC0 = 14,
    #[doc = "15 - System Clock Generator interrupt"]
    SCG0 = 15,
    #[doc = "16 - System Power Controller interrupt"]
    SPC0 = 16,
    #[doc = "17 - VBAT interrupt"]
    VBAT0 = 17,
    #[doc = "18 - Wake Up Unit interrupt"]
    WUU0 = 18,
    #[doc = "19 - Controller Area Network 0 interrupt"]
    CAN0 = 19,
    #[doc = "23 - Flexible Input/Output interrupt"]
    FLEXIO = 23,
    #[doc = "24 - Improved Inter Integrated Circuit interrupt 0"]
    I3C0 = 24,
    #[doc = "26 - Low-Power Inter Integrated Circuit interrupt"]
    LPI2C0 = 26,
    #[doc = "27 - Low-Power Inter Integrated Circuit interrupt"]
    LPI2C1 = 27,
    #[doc = "28 - Low-Power Serial Peripheral Interface interrupt"]
    LPSPI0 = 28,
    #[doc = "29 - Low-Power Serial Peripheral Interface interrupt"]
    LPSPI1 = 29,
    #[doc = "31 - Low-Power Universal Asynchronous Receive/Transmit interrupt"]
    LPUART0 = 31,
    #[doc = "32 - Low-Power Universal Asynchronous Receive/Transmit interrupt"]
    LPUART1 = 32,
    #[doc = "33 - Low-Power Universal Asynchronous Receive/Transmit interrupt"]
    LPUART2 = 33,
    #[doc = "34 - Low-Power Universal Asynchronous Receive/Transmit interrupt"]
    LPUART3 = 34,
    #[doc = "35 - Low-Power Universal Asynchronous Receive/Transmit interrupt"]
    LPUART4 = 35,
    #[doc = "36 - Universal Serial Bus - Full Speed interrupt"]
    USB0 = 36,
    #[doc = "38 - Code Watchdog Timer 0 interrupt"]
    CDOG0 = 38,
    #[doc = "39 - Standard counter/timer 0 interrupt"]
    CTIMER0 = 39,
    #[doc = "40 - Standard counter/timer 1 interrupt"]
    CTIMER1 = 40,
    #[doc = "41 - Standard counter/timer 2 interrupt"]
    CTIMER2 = 41,
    #[doc = "42 - Standard counter/timer 3 interrupt"]
    CTIMER3 = 42,
    #[doc = "43 - Standard counter/timer 4 interrupt"]
    CTIMER4 = 43,
    #[doc = "44 - FlexPWM0_reload_error interrupt"]
    FLEXPWM0_RELOAD_ERROR = 44,
    #[doc = "45 - FlexPWM0_fault interrupt"]
    FLEXPWM0_FAULT = 45,
    #[doc = "46 - FlexPWM0 Submodule 0 capture/compare/reload interrupt"]
    FLEXPWM0_SUBMODULE0 = 46,
    #[doc = "47 - FlexPWM0 Submodule 1 capture/compare/reload interrupt"]
    FLEXPWM0_SUBMODULE1 = 47,
    #[doc = "48 - FlexPWM0 Submodule 2 capture/compare/reload interrupt"]
    FLEXPWM0_SUBMODULE2 = 48,
    #[doc = "50 - Compare"]
    QDC0_COMPARE = 50,
    #[doc = "51 - Home"]
    QDC0_HOME = 51,
    #[doc = "52 - Watchdog / Simultaneous A and B Change"]
    QDC0_WATCHDOG = 52,
    #[doc = "53 - Index / Roll Over / Roll Under"]
    QDC0_INDEX = 53,
    #[doc = "54 - Frequency Measurement interrupt"]
    FREQME0 = 54,
    #[doc = "55 - Low Power Timer 0 interrupt"]
    LPTMR0 = 55,
    #[doc = "57 - OS event timer interrupt"]
    OS_EVENT = 57,
    #[doc = "58 - Wake Timer Interrupt"]
    WAKETIMER0 = 58,
    #[doc = "59 - Micro-Tick Timer interrupt"]
    UTICK0 = 59,
    #[doc = "60 - Windowed Watchdog Timer 0 interrupt"]
    WWDT0 = 60,
    #[doc = "62 - Analog-to-Digital Converter interrupt"]
    ADC0 = 62,
    #[doc = "63 - Analog-to-Digital Converter interrupt"]
    ADC1 = 63,
    #[doc = "64 - Comparator interrupt"]
    CMP0 = 64,
    #[doc = "65 - Comparator interrupt"]
    CMP1 = 65,
    #[doc = "71 - General Purpose Input/Output interrupt 0"]
    GPIO0 = 71,
    #[doc = "72 - General Purpose Input/Output interrupt 1"]
    GPIO1 = 72,
    #[doc = "73 - General Purpose Input/Output interrupt 2"]
    GPIO2 = 73,
    #[doc = "74 - General Purpose Input/Output interrupt 3"]
    GPIO3 = 74,
    #[doc = "75 - General Purpose Input/Output interrupt 4"]
    GPIO4 = 75,
    #[doc = "77 - Low-Power Inter Integrated Circuit interrupt"]
    LPI2C2 = 77,
    #[doc = "78 - Low-Power Inter Integrated Circuit interrupt"]
    LPI2C3 = 78,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    extern "C" {
        fn CMC();
        fn DMA_CH0();
        fn DMA_CH1();
        fn DMA_CH2();
        fn DMA_CH3();
        fn DMA_CH4();
        fn DMA_CH5();
        fn DMA_CH6();
        fn DMA_CH7();
        fn ERM0_SINGLE_BIT();
        fn ERM0_MULTI_BIT();
        fn FMU0();
        fn GLIKEY0();
        fn MBC0();
        fn SCG0();
        fn SPC0();
        fn VBAT0();
        fn WUU0();
        fn CAN0();
        fn FLEXIO();
        fn I3C0();
        fn LPI2C0();
        fn LPI2C1();
        fn LPSPI0();
        fn LPSPI1();
        fn LPUART0();
        fn LPUART1();
        fn LPUART2();
        fn LPUART3();
        fn LPUART4();
        fn USB0();
        fn CDOG0();
        fn CTIMER0();
        fn CTIMER1();
        fn CTIMER2();
        fn CTIMER3();
        fn CTIMER4();
        fn FLEXPWM0_RELOAD_ERROR();
        fn FLEXPWM0_FAULT();
        fn FLEXPWM0_SUBMODULE0();
        fn FLEXPWM0_SUBMODULE1();
        fn FLEXPWM0_SUBMODULE2();
        fn QDC0_COMPARE();
        fn QDC0_HOME();
        fn QDC0_WATCHDOG();
        fn QDC0_INDEX();
        fn FREQME0();
        fn LPTMR0();
        fn OS_EVENT();
        fn WAKETIMER0();
        fn UTICK0();
        fn WWDT0();
        fn ADC0();
        fn ADC1();
        fn CMP0();
        fn CMP1();
        fn GPIO0();
        fn GPIO1();
        fn GPIO2();
        fn GPIO3();
        fn GPIO4();
        fn LPI2C2();
        fn LPI2C3();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 79] = [
        Vector { _reserved: 0 },
        Vector { _handler: CMC },
        Vector { _handler: DMA_CH0 },
        Vector { _handler: DMA_CH1 },
        Vector { _handler: DMA_CH2 },
        Vector { _handler: DMA_CH3 },
        Vector { _handler: DMA_CH4 },
        Vector { _handler: DMA_CH5 },
        Vector { _handler: DMA_CH6 },
        Vector { _handler: DMA_CH7 },
        Vector {
            _handler: ERM0_SINGLE_BIT,
        },
        Vector {
            _handler: ERM0_MULTI_BIT,
        },
        Vector { _handler: FMU0 },
        Vector { _handler: GLIKEY0 },
        Vector { _handler: MBC0 },
        Vector { _handler: SCG0 },
        Vector { _handler: SPC0 },
        Vector { _handler: VBAT0 },
        Vector { _handler: WUU0 },
        Vector { _handler: CAN0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FLEXIO },
        Vector { _handler: I3C0 },
        Vector { _reserved: 0 },
        Vector { _handler: LPI2C0 },
        Vector { _handler: LPI2C1 },
        Vector { _handler: LPSPI0 },
        Vector { _handler: LPSPI1 },
        Vector { _reserved: 0 },
        Vector { _handler: LPUART0 },
        Vector { _handler: LPUART1 },
        Vector { _handler: LPUART2 },
        Vector { _handler: LPUART3 },
        Vector { _handler: LPUART4 },
        Vector { _handler: USB0 },
        Vector { _reserved: 0 },
        Vector { _handler: CDOG0 },
        Vector { _handler: CTIMER0 },
        Vector { _handler: CTIMER1 },
        Vector { _handler: CTIMER2 },
        Vector { _handler: CTIMER3 },
        Vector { _handler: CTIMER4 },
        Vector {
            _handler: FLEXPWM0_RELOAD_ERROR,
        },
        Vector {
            _handler: FLEXPWM0_FAULT,
        },
        Vector {
            _handler: FLEXPWM0_SUBMODULE0,
        },
        Vector {
            _handler: FLEXPWM0_SUBMODULE1,
        },
        Vector {
            _handler: FLEXPWM0_SUBMODULE2,
        },
        Vector { _reserved: 0 },
        Vector {
            _handler: QDC0_COMPARE,
        },
        Vector {
            _handler: QDC0_HOME,
        },
        Vector {
            _handler: QDC0_WATCHDOG,
        },
        Vector {
            _handler: QDC0_INDEX,
        },
        Vector { _handler: FREQME0 },
        Vector { _handler: LPTMR0 },
        Vector { _reserved: 0 },
        Vector { _handler: OS_EVENT },
        Vector {
            _handler: WAKETIMER0,
        },
        Vector { _handler: UTICK0 },
        Vector { _handler: WWDT0 },
        Vector { _reserved: 0 },
        Vector { _handler: ADC0 },
        Vector { _handler: ADC1 },
        Vector { _handler: CMP0 },
        Vector { _handler: CMP1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: GPIO0 },
        Vector { _handler: GPIO1 },
        Vector { _handler: GPIO2 },
        Vector { _handler: GPIO3 },
        Vector { _handler: GPIO4 },
        Vector { _reserved: 0 },
        Vector { _handler: LPI2C2 },
        Vector { _handler: LPI2C3 },
    ];
}
#[path = "../../peripherals/a1"]
pub mod adc {
    use core::marker::PhantomData;
    #[path = "adc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_f000usize, 0x400b_0000usize];
    pub type Instance<const N: u8> = crate::Instance<ADC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> ADC {
            unsafe { ADC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type ADC0 = Instance<0u8>;
    pub type ADC1 = Instance<1u8>;
}
#[path = "../../peripherals/a1"]
pub mod aoi {
    use core::marker::PhantomData;
    #[path = "aoi.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_9000usize, 0x4009_7000usize];
    pub type Instance<const N: u8> = crate::Instance<AOI, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> AOI {
            unsafe { AOI::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type AOI0 = Instance<0u8>;
    pub type AOI1 = Instance<1u8>;
}
#[path = "../../peripherals/a1"]
pub mod can {
    use core::marker::PhantomData;
    #[path = "can.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_c000usize];
    pub type Instance<const N: u8> = crate::Instance<CAN, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> CAN {
            unsafe { CAN::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type CAN0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod cdog {
    use core::marker::PhantomData;
    #[path = "cdog.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_0000usize];
    pub type Instance<const N: u8> = crate::Instance<CDOG, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> CDOG {
            unsafe { CDOG::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type CDOG0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod cmc {
    use core::marker::PhantomData;
    #[path = "cmc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_b000usize];
    pub type Instance<const N: u8> = crate::Instance<CMC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> CMC {
            unsafe { CMC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type CMC0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod crc {
    use core::marker::PhantomData;
    #[path = "crc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_a000usize];
    pub type Instance<const N: u8> = crate::Instance<CRC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> CRC {
            unsafe { CRC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type CRC0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod ctimer {
    use core::marker::PhantomData;
    #[path = "ctimer.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 5usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4000_4000usize,
        0x4000_5000usize,
        0x4000_6000usize,
        0x4000_7000usize,
        0x4000_8000usize,
    ];
    pub type Instance<const N: u8> = crate::Instance<CTIMER, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> CTIMER {
            unsafe { CTIMER::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type CTIMER0 = Instance<0u8>;
    pub type CTIMER1 = Instance<1u8>;
    pub type CTIMER2 = Instance<2u8>;
    pub type CTIMER3 = Instance<3u8>;
    pub type CTIMER4 = Instance<4u8>;
}
#[path = "../../peripherals/a1"]
pub mod debugmailbox {
    use core::marker::PhantomData;
    #[path = "debugmailbox.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_1000usize];
    pub type Instance<const N: u8> = crate::Instance<DEBUGMAILBOX, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> DEBUGMAILBOX {
            unsafe { DEBUGMAILBOX::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type DBGMAILBOX = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod dma {
    use core::marker::PhantomData;
    #[path = "dma.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_0000usize];
    pub type Instance<const N: u8> = crate::Instance<DMA, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> DMA {
            unsafe { DMA::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type DMA0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod eim {
    use core::marker::PhantomData;
    #[path = "eim.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_c000usize];
    pub type Instance<const N: u8> = crate::Instance<EIM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> EIM {
            unsafe { EIM::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type EIM0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod eqdc {
    use core::marker::PhantomData;
    #[path = "eqdc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_7000usize];
    pub type Instance<const N: u8> = crate::Instance<EQDC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> EQDC {
            unsafe { EQDC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type QDC0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod erm {
    use core::marker::PhantomData;
    #[path = "erm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_d000usize];
    pub type Instance<const N: u8> = crate::Instance<ERM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> ERM {
            unsafe { ERM::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type ERM0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod flexio {
    use core::marker::PhantomData;
    #[path = "flexio.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_9000usize];
    pub type Instance<const N: u8> = crate::Instance<FLEXIO, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> FLEXIO {
            unsafe { FLEXIO::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type FLEXIO0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod fmc {
    use core::marker::PhantomData;
    #[path = "fmc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_4000usize];
    pub type Instance<const N: u8> = crate::Instance<FMC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> FMC {
            unsafe { FMC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type FMC0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod fmu {
    use core::marker::PhantomData;
    #[path = "fmu.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_5000usize];
    pub type Instance<const N: u8> = crate::Instance<FMU, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> FMU {
            unsafe { FMU::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type FMU0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod fmutest {
    use core::marker::PhantomData;
    #[path = "fmutest.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_6000usize];
    pub type Instance<const N: u8> = crate::Instance<FMUTEST, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> FMUTEST {
            unsafe { FMUTEST::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type FMU0TEST = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod freqme {
    use core::marker::PhantomData;
    #[path = "freqme.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_9000usize];
    pub type Instance<const N: u8> = crate::Instance<FREQME, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> FREQME {
            unsafe { FREQME::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type FREQME0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod glikey {
    use core::marker::PhantomData;
    #[path = "glikey.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_1d00usize];
    pub type Instance<const N: u8> = crate::Instance<GLIKEY, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> GLIKEY {
            unsafe { GLIKEY::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type GLIKEY0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod gpio {
    use core::marker::PhantomData;
    #[path = "gpio.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 5usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4010_2000usize,
        0x4010_3000usize,
        0x4010_4000usize,
        0x4010_5000usize,
        0x4010_6000usize,
    ];
    pub type Instance<const N: u8> = crate::Instance<GPIO, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> GPIO {
            unsafe { GPIO::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type GPIO0 = Instance<0u8>;
    pub type GPIO1 = Instance<1u8>;
    pub type GPIO2 = Instance<2u8>;
    pub type GPIO3 = Instance<3u8>;
    pub type GPIO4 = Instance<4u8>;
}
#[path = "../../peripherals/a1"]
pub mod i3c {
    use core::marker::PhantomData;
    #[path = "i3c.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_2000usize];
    pub type Instance<const N: u8> = crate::Instance<I3C, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> I3C {
            unsafe { I3C::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type I3C0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod inputmux {
    use core::marker::PhantomData;
    #[path = "inputmux.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_1000usize];
    pub type Instance<const N: u8> = crate::Instance<INPUTMUX, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> INPUTMUX {
            unsafe { INPUTMUX::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type INPUTMUX0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod lpcmp {
    use core::marker::PhantomData;
    #[path = "lpcmp.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400b_1000usize, 0x400b_2000usize];
    pub type Instance<const N: u8> = crate::Instance<LPCMP, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> LPCMP {
            unsafe { LPCMP::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type CMP0 = Instance<0u8>;
    pub type CMP1 = Instance<1u8>;
}
#[path = "../../peripherals/a1"]
pub mod lpi2c {
    use core::marker::PhantomData;
    #[path = "lpi2c.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 4usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4009_a000usize,
        0x4009_b000usize,
        0x400d_4000usize,
        0x400d_5000usize,
    ];
    pub type Instance<const N: u8> = crate::Instance<LPI2C, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> LPI2C {
            unsafe { LPI2C::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type LPI2C0 = Instance<0u8>;
    pub type LPI2C1 = Instance<1u8>;
    pub type LPI2C2 = Instance<2u8>;
    pub type LPI2C3 = Instance<3u8>;
}
#[path = "../../peripherals/a1"]
pub mod lpspi {
    use core::marker::PhantomData;
    #[path = "lpspi.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_c000usize, 0x4009_d000usize];
    pub type Instance<const N: u8> = crate::Instance<LPSPI, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> LPSPI {
            unsafe { LPSPI::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type LPSPI0 = Instance<0u8>;
    pub type LPSPI1 = Instance<1u8>;
}
#[path = "../../peripherals/a1"]
pub mod lptmr {
    use core::marker::PhantomData;
    #[path = "lptmr.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_b000usize];
    pub type Instance<const N: u8> = crate::Instance<LPTMR, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> LPTMR {
            unsafe { LPTMR::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type LPTMR0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod lpuart {
    use core::marker::PhantomData;
    #[path = "lpuart.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 5usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4009_f000usize,
        0x400a_0000usize,
        0x400a_1000usize,
        0x400a_2000usize,
        0x400a_3000usize,
    ];
    pub type Instance<const N: u8> = crate::Instance<LPUART, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> LPUART {
            unsafe { LPUART::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type LPUART0 = Instance<0u8>;
    pub type LPUART1 = Instance<1u8>;
    pub type LPUART2 = Instance<2u8>;
    pub type LPUART3 = Instance<3u8>;
    pub type LPUART4 = Instance<4u8>;
}
#[path = "../../peripherals/a1"]
pub mod mrcc {
    use core::marker::PhantomData;
    #[path = "mrcc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_1000usize];
    pub type Instance<const N: u8> = crate::Instance<MRCC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> MRCC {
            unsafe { MRCC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type MRCC0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod ostimer {
    use core::marker::PhantomData;
    #[path = "ostimer.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_d000usize];
    pub type Instance<const N: u8> = crate::Instance<OSTIMER, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> OSTIMER {
            unsafe { OSTIMER::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type OSTIMER0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod port {
    use core::marker::PhantomData;
    #[path = "port.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 5usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x400b_c000usize,
        0x400b_d000usize,
        0x400b_e000usize,
        0x400b_f000usize,
        0x400c_0000usize,
    ];
    pub type Instance<const N: u8> = crate::Instance<PORT, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> PORT {
            unsafe { PORT::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type PORT0 = Instance<0u8>;
    pub type PORT1 = Instance<1u8>;
    pub type PORT2 = Instance<2u8>;
    pub type PORT3 = Instance<3u8>;
    pub type PORT4 = Instance<4u8>;
}
#[path = "../../peripherals/a1"]
pub mod pwm {
    use core::marker::PhantomData;
    #[path = "pwm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_9000usize];
    pub type Instance<const N: u8> = crate::Instance<PWM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> PWM {
            unsafe { PWM::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type FLEXPWM0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod scg {
    use core::marker::PhantomData;
    #[path = "scg.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_f000usize];
    pub type Instance<const N: u8> = crate::Instance<SCG, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> SCG {
            unsafe { SCG::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type SCG0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod spc {
    use core::marker::PhantomData;
    #[path = "spc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_0000usize];
    pub type Instance<const N: u8> = crate::Instance<SPC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> SPC {
            unsafe { SPC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type SPC0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod syscon {
    use core::marker::PhantomData;
    #[path = "syscon.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_1000usize];
    pub type Instance<const N: u8> = crate::Instance<SYSCON, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> SYSCON {
            unsafe { SYSCON::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type SYSCON0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod trdc {
    use core::marker::PhantomData;
    #[path = "trdc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_e000usize];
    pub type Instance<const N: u8> = crate::Instance<TRDC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> TRDC {
            unsafe { TRDC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type MBC0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod usb {
    use core::marker::PhantomData;
    #[path = "usb.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_4000usize];
    pub type Instance<const N: u8> = crate::Instance<USB, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> USB {
            unsafe { USB::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type USB0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod utick {
    use core::marker::PhantomData;
    #[path = "utick.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_b000usize];
    pub type Instance<const N: u8> = crate::Instance<UTICK, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> UTICK {
            unsafe { UTICK::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type UTICK0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod vbat {
    use core::marker::PhantomData;
    #[path = "vbat.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_3000usize];
    pub type Instance<const N: u8> = crate::Instance<VBAT, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> VBAT {
            unsafe { VBAT::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type VBAT0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod waketimer {
    use core::marker::PhantomData;
    #[path = "waketimer.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_e000usize];
    pub type Instance<const N: u8> = crate::Instance<WAKETIMER, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> WAKETIMER {
            unsafe { WAKETIMER::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type WAKETIMER0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod wuu {
    use core::marker::PhantomData;
    #[path = "wuu.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_2000usize];
    pub type Instance<const N: u8> = crate::Instance<WUU, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> WUU {
            unsafe { WUU::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type WUU0 = Instance<0u8>;
}
#[path = "../../peripherals/a1"]
pub mod wwdt {
    use core::marker::PhantomData;
    #[path = "wwdt.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_c000usize];
    pub type Instance<const N: u8> = crate::Instance<WWDT, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> WWDT {
            unsafe { WWDT::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type WWDT0 = Instance<0u8>;
}
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
