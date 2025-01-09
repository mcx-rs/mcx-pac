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
    #[doc = "17 - TDET interrrupt"]
    TDET = 17,
    #[doc = "18 - Wake Up Unit interrupt"]
    WUU0 = 18,
    #[doc = "19 - Controller Area Network 0 interrupt"]
    CAN0 = 19,
    #[doc = "20 - Controller Area Network 1 interrupt"]
    CAN1 = 20,
    #[doc = "23 - Flexible Input/Output interrupt"]
    FLEXIO = 23,
    #[doc = "24 - Improved Inter Integrated Circuit interrupt 0"]
    I3C0 = 24,
    #[doc = "26 - Low-Power Inter Integrated Circuit 0 interrupt"]
    LPI2C0 = 26,
    #[doc = "27 - Low-Power Inter Integrated Circuit 1 interrupt"]
    LPI2C1 = 27,
    #[doc = "28 - Low-Power Serial Peripheral Interface 0 interrupt"]
    LPSPI0 = 28,
    #[doc = "29 - Low-Power Serial Peripheral Interface 1 interrupt"]
    LPSPI1 = 29,
    #[doc = "31 - Low-Power Universal Asynchronous Receive/Transmit 0 interrupt"]
    LPUART0 = 31,
    #[doc = "32 - Low-Power Universal Asynchronous Receive/Transmit 1 interrupt"]
    LPUART1 = 32,
    #[doc = "33 - Low-Power Universal Asynchronous Receive/Transmit 2 interrupt"]
    LPUART2 = 33,
    #[doc = "34 - Low-Power Universal Asynchronous Receive/Transmit 3 interrupt"]
    LPUART3 = 34,
    #[doc = "35 - Low-Power Universal Asynchronous Receive/Transmit 4 interrupt"]
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
    #[doc = "49 - FlexPWM0 Submodule 3 capture/compare/reload interrupt"]
    FLEXPWM0_SUBMODULE3 = 49,
    #[doc = "50 - Compare"]
    EQDC0_COMPARE = 50,
    #[doc = "51 - Home"]
    EQDC0_HOME = 51,
    #[doc = "52 - Watchdog / Simultaneous A and B Change"]
    EQDC0_WATCHDOG = 52,
    #[doc = "53 - Index / Roll Over / Roll Under"]
    EQDC0_INDEX = 53,
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
    #[doc = "62 - Analog-to-Digital Converter 0 interrupt"]
    ADC0 = 62,
    #[doc = "63 - Analog-to-Digital Converter 1 interrupt"]
    ADC1 = 63,
    #[doc = "64 - Comparator 0 interrupt"]
    CMP0 = 64,
    #[doc = "65 - Comparator 1 interrupt"]
    CMP1 = 65,
    #[doc = "66 - Comparator 2 interrupt"]
    CMP2 = 66,
    #[doc = "67 - Digital-to-Analog Converter 0 - General Purpose interrupt"]
    DAC0 = 67,
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
    #[doc = "77 - Low-Power Inter Integrated Circuit 2 interrupt"]
    LPI2C2 = 77,
    #[doc = "78 - Low-Power Inter Integrated Circuit 3 interrupt"]
    LPI2C3 = 78,
    #[doc = "79 - FlexPWM1_reload_error interrupt"]
    FLEXPWM1_RELOAD_ERROR = 79,
    #[doc = "80 - FlexPWM1_fault interrupt"]
    FLEXPWM1_FAULT = 80,
    #[doc = "81 - FlexPWM1 Submodule 0 capture/compare/reload interrupt"]
    FLEXPWM1_SUBMODULE0 = 81,
    #[doc = "82 - FlexPWM1 Submodule 1 capture/compare/reload interrupt"]
    FLEXPWM1_SUBMODULE1 = 82,
    #[doc = "83 - FlexPWM1 Submodule 2 capture/compare/reload interrupt"]
    FLEXPWM1_SUBMODULE2 = 83,
    #[doc = "84 - FlexPWM1 Submodule 3 capture/compare/reload interrupt"]
    FLEXPWM1_SUBMODULE3 = 84,
    #[doc = "85 - Compare"]
    EQDC1_COMPARE = 85,
    #[doc = "86 - Home"]
    EQDC1_HOME = 86,
    #[doc = "87 - Watchdog / Simultaneous A and B Change"]
    EQDC1_WATCHDOG = 87,
    #[doc = "88 - Index / Roll Over / Roll Under"]
    EQDC1_INDEX = 88,
    #[doc = "95 - Low-Power Universal Asynchronous Receive/Transmit interrupt"]
    LPUART5 = 95,
    #[doc = "107 - MAU interrupt"]
    MAU = 107,
    #[doc = "108 - SmartDMA interrupt"]
    SMARTDMA = 108,
    #[doc = "109 - Code Watchdog Timer 1 interrupt"]
    CDOG1 = 109,
    #[doc = "110 - PKC interrupt"]
    PKC = 110,
    #[doc = "111 - SGI interrupt"]
    SGI = 111,
    #[doc = "113 - True Random Number Generator interrupt"]
    TRNG0 = 113,
    #[doc = "116 - Analog-to-Digital Converter 2 interrupt"]
    ADC2 = 116,
    #[doc = "117 - Analog-to-Digital Converter 3 interrupt"]
    ADC3 = 117,
    #[doc = "119 - RTC alarm interrupt"]
    RTC = 119,
    #[doc = "120 - RTC 1Hz interrupt"]
    RTC_1HZ = 120,
    #[doc = "121 - SLCD frame start interrupt"]
    SLCD = 121,
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
        fn TDET();
        fn WUU0();
        fn CAN0();
        fn CAN1();
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
        fn FLEXPWM0_SUBMODULE3();
        fn EQDC0_COMPARE();
        fn EQDC0_HOME();
        fn EQDC0_WATCHDOG();
        fn EQDC0_INDEX();
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
        fn CMP2();
        fn DAC0();
        fn GPIO0();
        fn GPIO1();
        fn GPIO2();
        fn GPIO3();
        fn GPIO4();
        fn LPI2C2();
        fn LPI2C3();
        fn FLEXPWM1_RELOAD_ERROR();
        fn FLEXPWM1_FAULT();
        fn FLEXPWM1_SUBMODULE0();
        fn FLEXPWM1_SUBMODULE1();
        fn FLEXPWM1_SUBMODULE2();
        fn FLEXPWM1_SUBMODULE3();
        fn EQDC1_COMPARE();
        fn EQDC1_HOME();
        fn EQDC1_WATCHDOG();
        fn EQDC1_INDEX();
        fn LPUART5();
        fn MAU();
        fn SMARTDMA();
        fn CDOG1();
        fn PKC();
        fn SGI();
        fn TRNG0();
        fn ADC2();
        fn ADC3();
        fn RTC();
        fn RTC_1HZ();
        fn SLCD();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 122] = [
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
        Vector { _handler: TDET },
        Vector { _handler: WUU0 },
        Vector { _handler: CAN0 },
        Vector { _handler: CAN1 },
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
        Vector {
            _handler: FLEXPWM0_SUBMODULE3,
        },
        Vector {
            _handler: EQDC0_COMPARE,
        },
        Vector {
            _handler: EQDC0_HOME,
        },
        Vector {
            _handler: EQDC0_WATCHDOG,
        },
        Vector {
            _handler: EQDC0_INDEX,
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
        Vector { _handler: CMP2 },
        Vector { _handler: DAC0 },
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
        Vector {
            _handler: FLEXPWM1_RELOAD_ERROR,
        },
        Vector {
            _handler: FLEXPWM1_FAULT,
        },
        Vector {
            _handler: FLEXPWM1_SUBMODULE0,
        },
        Vector {
            _handler: FLEXPWM1_SUBMODULE1,
        },
        Vector {
            _handler: FLEXPWM1_SUBMODULE2,
        },
        Vector {
            _handler: FLEXPWM1_SUBMODULE3,
        },
        Vector {
            _handler: EQDC1_COMPARE,
        },
        Vector {
            _handler: EQDC1_HOME,
        },
        Vector {
            _handler: EQDC1_WATCHDOG,
        },
        Vector {
            _handler: EQDC1_INDEX,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: LPUART5 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: MAU },
        Vector { _handler: SMARTDMA },
        Vector { _handler: CDOG1 },
        Vector { _handler: PKC },
        Vector { _handler: SGI },
        Vector { _reserved: 0 },
        Vector { _handler: TRNG0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: ADC2 },
        Vector { _handler: ADC3 },
        Vector { _reserved: 0 },
        Vector { _handler: RTC },
        Vector { _handler: RTC_1HZ },
        Vector { _handler: SLCD },
    ];
}
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
pub mod can {
    use core::marker::PhantomData;
    #[path = "can.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_c000usize, 0x400d_0000usize];
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
    pub type CAN1 = Instance<1u8>;
}
#[path = "../../peripherals/a2"]
pub mod cdog {
    use core::marker::PhantomData;
    #[path = "cdog.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_0000usize, 0x4010_7000usize];
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
    pub type CDOG1 = Instance<1u8>;
}
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
pub mod eqdc {
    use core::marker::PhantomData;
    #[path = "eqdc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_7000usize, 0x400a_8000usize];
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
    pub type EQDC0 = Instance<0u8>;
    pub type EQDC1 = Instance<1u8>;
}
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
pub mod lpdac {
    use core::marker::PhantomData;
    #[path = "lpdac.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400b_4000usize];
    pub type Instance<const N: u8> = crate::Instance<LPDAC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> LPDAC {
            unsafe { LPDAC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type DAC0 = Instance<0u8>;
}
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
pub mod mau {
    use core::marker::PhantomData;
    #[path = "mau.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_8000usize];
    pub type Instance<const N: u8> = crate::Instance<MAU, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> MAU {
            unsafe { MAU::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type MAU0 = Instance<0u8>;
}
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
pub mod opamp {
    use core::marker::PhantomData;
    #[path = "opamp.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400b_7000usize];
    pub type Instance<const N: u8> = crate::Instance<OPAMP, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> OPAMP {
            unsafe { OPAMP::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type OPAMP0 = Instance<0u8>;
}
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
pub mod pkc {
    use core::marker::PhantomData;
    #[path = "pkc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400e_a000usize];
    pub type Instance<const N: u8> = crate::Instance<PKC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> PKC {
            unsafe { PKC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type PKC0 = Instance<0u8>;
}
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
pub mod pwm {
    use core::marker::PhantomData;
    #[path = "pwm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_9000usize, 0x400a_a000usize];
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
    pub type FLEXPWM1 = Instance<1u8>;
}
#[path = "../../peripherals/a2"]
pub mod rtc {
    use core::marker::PhantomData;
    #[path = "rtc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400e_e000usize];
    pub type Instance<const N: u8> = crate::Instance<RTC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> RTC {
            unsafe { RTC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type RTC0 = Instance<0u8>;
}
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
pub mod sgi {
    use core::marker::PhantomData;
    #[path = "sgi.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400e_b000usize];
    pub type Instance<const N: u8> = crate::Instance<SGI, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> SGI {
            unsafe { SGI::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type SGI0 = Instance<0u8>;
}
#[path = "../../peripherals/a2"]
pub mod smartdma {
    use core::marker::PhantomData;
    #[path = "smartdma.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_e000usize];
    pub type Instance<const N: u8> = crate::Instance<SMARTDMA, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> SMARTDMA {
            unsafe { SMARTDMA::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type SMARTDMA0 = Instance<0u8>;
}
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
pub mod tdet {
    use core::marker::PhantomData;
    #[path = "tdet.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400e_9000usize];
    pub type Instance<const N: u8> = crate::Instance<TDET, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> TDET {
            unsafe { TDET::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type TDET0 = Instance<0u8>;
}
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
pub mod trng {
    use core::marker::PhantomData;
    #[path = "trng.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400e_c000usize];
    pub type Instance<const N: u8> = crate::Instance<TRNG, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> TRNG {
            unsafe { TRNG::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type TRNG0 = Instance<0u8>;
}
#[path = "../../peripherals/a2"]
pub mod udf {
    use core::marker::PhantomData;
    #[path = "udf.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400e_d000usize];
    pub type Instance<const N: u8> = crate::Instance<UDF, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> UDF {
            unsafe { UDF::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type UDF0 = Instance<0u8>;
}
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
#[path = "../../peripherals/a2"]
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
