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
pub mod slcd {
    #[path = "../../../peripherals/a2/slcd.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_2000usize];
    pub const unsafe fn instance(n: u8) -> Option<SLCD> {
        if n >= LEN as u8 {
            None
        } else {
            Some(SLCD::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<SLCD, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> SLCD {
            let _ = Self::CHECK;
            SLCD::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type SLCD0 = Instance<0u8>;
}
pub mod smartdma {
    #[path = "../../../peripherals/a2/smartdma.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_e000usize];
    pub const unsafe fn instance(n: u8) -> Option<SMARTDMA> {
        if n >= LEN as u8 {
            None
        } else {
            Some(SMARTDMA::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<SMARTDMA, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> SMARTDMA {
            let _ = Self::CHECK;
            SMARTDMA::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type SMARTDMA0 = Instance<0u8>;
}
pub mod cmc {
    #[path = "../../../peripherals/a2/cmc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_b000usize];
    pub const unsafe fn instance(n: u8) -> Option<CMC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(CMC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<CMC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> CMC {
            let _ = Self::CHECK;
            CMC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type CMC0 = Instance<0u8>;
}
pub mod utick {
    #[path = "../../../peripherals/a2/utick.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_b000usize];
    pub const unsafe fn instance(n: u8) -> Option<UTICK> {
        if n >= LEN as u8 {
            None
        } else {
            Some(UTICK::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<UTICK, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> UTICK {
            let _ = Self::CHECK;
            UTICK::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type UTICK0 = Instance<0u8>;
}
pub mod flexio {
    #[path = "../../../peripherals/a2/flexio.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_9000usize];
    pub const unsafe fn instance(n: u8) -> Option<FLEXIO> {
        if n >= LEN as u8 {
            None
        } else {
            Some(FLEXIO::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<FLEXIO, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> FLEXIO {
            let _ = Self::CHECK;
            FLEXIO::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type FLEXIO0 = Instance<0u8>;
}
pub mod pwm {
    #[path = "../../../peripherals/a2/pwm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_9000usize, 0x400a_a000usize];
    pub const unsafe fn instance(n: u8) -> Option<PWM> {
        if n >= LEN as u8 {
            None
        } else {
            Some(PWM::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<PWM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> PWM {
            let _ = Self::CHECK;
            PWM::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type FLEXPWM0 = Instance<0u8>;
    pub type FLEXPWM1 = Instance<1u8>;
}
pub mod udf {
    #[path = "../../../peripherals/a2/udf.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400e_d000usize];
    pub const unsafe fn instance(n: u8) -> Option<UDF> {
        if n >= LEN as u8 {
            None
        } else {
            Some(UDF::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<UDF, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> UDF {
            let _ = Self::CHECK;
            UDF::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type UDF0 = Instance<0u8>;
}
pub mod lpuart {
    #[path = "../../../peripherals/a2/lpuart.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 6usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4009_f000usize,
        0x400a_0000usize,
        0x400a_1000usize,
        0x400a_2000usize,
        0x400a_3000usize,
        0x400d_a000usize,
    ];
    pub const unsafe fn instance(n: u8) -> Option<LPUART> {
        if n >= LEN as u8 {
            None
        } else {
            Some(LPUART::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<LPUART, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> LPUART {
            let _ = Self::CHECK;
            LPUART::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type LPUART0 = Instance<0u8>;
    pub type LPUART1 = Instance<1u8>;
    pub type LPUART2 = Instance<2u8>;
    pub type LPUART3 = Instance<3u8>;
    pub type LPUART4 = Instance<4u8>;
    pub type LPUART5 = Instance<5u8>;
}
pub mod lpspi {
    #[path = "../../../peripherals/a2/lpspi.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_c000usize, 0x4009_d000usize];
    pub const unsafe fn instance(n: u8) -> Option<LPSPI> {
        if n >= LEN as u8 {
            None
        } else {
            Some(LPSPI::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<LPSPI, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> LPSPI {
            let _ = Self::CHECK;
            LPSPI::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type LPSPI0 = Instance<0u8>;
    pub type LPSPI1 = Instance<1u8>;
}
pub mod rtc {
    #[path = "../../../peripherals/a2/rtc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400e_e000usize];
    pub const unsafe fn instance(n: u8) -> Option<RTC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(RTC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<RTC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> RTC {
            let _ = Self::CHECK;
            RTC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type RTC0 = Instance<0u8>;
}
pub mod gpio {
    #[path = "../../../peripherals/a2/gpio.rs"]
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
    pub const unsafe fn instance(n: u8) -> Option<GPIO> {
        if n >= LEN as u8 {
            None
        } else {
            Some(GPIO::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<GPIO, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> GPIO {
            let _ = Self::CHECK;
            GPIO::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
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
pub mod spc {
    #[path = "../../../peripherals/a2/spc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_0000usize];
    pub const unsafe fn instance(n: u8) -> Option<SPC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(SPC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<SPC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> SPC {
            let _ = Self::CHECK;
            SPC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type SPC0 = Instance<0u8>;
}
pub mod ctimer {
    #[path = "../../../peripherals/a2/ctimer.rs"]
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
    pub const unsafe fn instance(n: u8) -> Option<CTIMER> {
        if n >= LEN as u8 {
            None
        } else {
            Some(CTIMER::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<CTIMER, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> CTIMER {
            let _ = Self::CHECK;
            CTIMER::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
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
pub mod lpi2c {
    #[path = "../../../peripherals/a2/lpi2c.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 4usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4009_a000usize,
        0x4009_b000usize,
        0x400d_4000usize,
        0x400d_5000usize,
    ];
    pub const unsafe fn instance(n: u8) -> Option<LPI2C> {
        if n >= LEN as u8 {
            None
        } else {
            Some(LPI2C::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<LPI2C, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> LPI2C {
            let _ = Self::CHECK;
            LPI2C::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type LPI2C0 = Instance<0u8>;
    pub type LPI2C1 = Instance<1u8>;
    pub type LPI2C2 = Instance<2u8>;
    pub type LPI2C3 = Instance<3u8>;
}
pub mod aoi {
    #[path = "../../../peripherals/a2/aoi.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_9000usize, 0x4009_7000usize];
    pub const unsafe fn instance(n: u8) -> Option<AOI> {
        if n >= LEN as u8 {
            None
        } else {
            Some(AOI::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<AOI, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> AOI {
            let _ = Self::CHECK;
            AOI::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type AOI0 = Instance<0u8>;
    pub type AOI1 = Instance<1u8>;
}
pub mod vbat {
    #[path = "../../../peripherals/a2/vbat.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_3000usize];
    pub const unsafe fn instance(n: u8) -> Option<VBAT> {
        if n >= LEN as u8 {
            None
        } else {
            Some(VBAT::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<VBAT, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> VBAT {
            let _ = Self::CHECK;
            VBAT::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type VBAT0 = Instance<0u8>;
}
pub mod eim {
    #[path = "../../../peripherals/a2/eim.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_c000usize];
    pub const unsafe fn instance(n: u8) -> Option<EIM> {
        if n >= LEN as u8 {
            None
        } else {
            Some(EIM::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<EIM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> EIM {
            let _ = Self::CHECK;
            EIM::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type EIM0 = Instance<0u8>;
}
pub mod inputmux {
    #[path = "../../../peripherals/a2/inputmux.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_1000usize];
    pub const unsafe fn instance(n: u8) -> Option<INPUTMUX> {
        if n >= LEN as u8 {
            None
        } else {
            Some(INPUTMUX::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<INPUTMUX, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> INPUTMUX {
            let _ = Self::CHECK;
            INPUTMUX::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type INPUTMUX0 = Instance<0u8>;
}
pub mod mau {
    #[path = "../../../peripherals/a2/mau.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_8000usize];
    pub const unsafe fn instance(n: u8) -> Option<MAU> {
        if n >= LEN as u8 {
            None
        } else {
            Some(MAU::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<MAU, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> MAU {
            let _ = Self::CHECK;
            MAU::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type MAU0 = Instance<0u8>;
}
pub mod lpcmp {
    #[path = "../../../peripherals/a2/lpcmp.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 3usize;
    pub const ADDRESSES: [usize; LEN] = [0x400b_1000usize, 0x400b_2000usize, 0x400b_3000usize];
    pub const unsafe fn instance(n: u8) -> Option<LPCMP> {
        if n >= LEN as u8 {
            None
        } else {
            Some(LPCMP::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<LPCMP, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> LPCMP {
            let _ = Self::CHECK;
            LPCMP::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type CMP0 = Instance<0u8>;
    pub type CMP1 = Instance<1u8>;
    pub type CMP2 = Instance<2u8>;
}
pub mod usb {
    #[path = "../../../peripherals/a2/usb.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_4000usize];
    pub const unsafe fn instance(n: u8) -> Option<USB> {
        if n >= LEN as u8 {
            None
        } else {
            Some(USB::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<USB, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> USB {
            let _ = Self::CHECK;
            USB::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type USB0 = Instance<0u8>;
}
pub mod fmc {
    #[path = "../../../peripherals/a2/fmc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_4000usize];
    pub const unsafe fn instance(n: u8) -> Option<FMC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(FMC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<FMC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> FMC {
            let _ = Self::CHECK;
            FMC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type FMC0 = Instance<0u8>;
}
pub mod trdc {
    #[path = "../../../peripherals/a2/trdc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_e000usize];
    pub const unsafe fn instance(n: u8) -> Option<TRDC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(TRDC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<TRDC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> TRDC {
            let _ = Self::CHECK;
            TRDC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type MBC0 = Instance<0u8>;
}
pub mod waketimer {
    #[path = "../../../peripherals/a2/waketimer.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_e000usize];
    pub const unsafe fn instance(n: u8) -> Option<WAKETIMER> {
        if n >= LEN as u8 {
            None
        } else {
            Some(WAKETIMER::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<WAKETIMER, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> WAKETIMER {
            let _ = Self::CHECK;
            WAKETIMER::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type WAKETIMER0 = Instance<0u8>;
}
pub mod cdog {
    #[path = "../../../peripherals/a2/cdog.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_0000usize];
    pub const unsafe fn instance(n: u8) -> Option<CDOG> {
        if n >= LEN as u8 {
            None
        } else {
            Some(CDOG::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<CDOG, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> CDOG {
            let _ = Self::CHECK;
            CDOG::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type CDOG0 = Instance<0u8>;
}
pub mod debugmailbox {
    #[path = "../../../peripherals/a2/debugmailbox.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_1000usize];
    pub const unsafe fn instance(n: u8) -> Option<DEBUGMAILBOX> {
        if n >= LEN as u8 {
            None
        } else {
            Some(DEBUGMAILBOX::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<DEBUGMAILBOX, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> DEBUGMAILBOX {
            let _ = Self::CHECK;
            DEBUGMAILBOX::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type DBGMAILBOX = Instance<0u8>;
}
pub mod ostimer {
    #[path = "../../../peripherals/a2/ostimer.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_d000usize];
    pub const unsafe fn instance(n: u8) -> Option<OSTIMER> {
        if n >= LEN as u8 {
            None
        } else {
            Some(OSTIMER::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<OSTIMER, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> OSTIMER {
            let _ = Self::CHECK;
            OSTIMER::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type OSTIMER0 = Instance<0u8>;
}
pub mod mrcc {
    #[path = "../../../peripherals/a2/mrcc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_1000usize];
    pub const unsafe fn instance(n: u8) -> Option<MRCC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(MRCC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<MRCC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> MRCC {
            let _ = Self::CHECK;
            MRCC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type MRCC0 = Instance<0u8>;
}
pub mod syscon {
    #[path = "../../../peripherals/a2/syscon.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_1000usize];
    pub const unsafe fn instance(n: u8) -> Option<SYSCON> {
        if n >= LEN as u8 {
            None
        } else {
            Some(SYSCON::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<SYSCON, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> SYSCON {
            let _ = Self::CHECK;
            SYSCON::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type SYSCON0 = Instance<0u8>;
}
pub mod wwdt {
    #[path = "../../../peripherals/a2/wwdt.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_c000usize];
    pub const unsafe fn instance(n: u8) -> Option<WWDT> {
        if n >= LEN as u8 {
            None
        } else {
            Some(WWDT::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<WWDT, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> WWDT {
            let _ = Self::CHECK;
            WWDT::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type WWDT0 = Instance<0u8>;
}
pub mod crc {
    #[path = "../../../peripherals/a2/crc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_a000usize];
    pub const unsafe fn instance(n: u8) -> Option<CRC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(CRC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<CRC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> CRC {
            let _ = Self::CHECK;
            CRC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type CRC0 = Instance<0u8>;
}
pub mod eqdc {
    #[path = "../../../peripherals/a2/eqdc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_7000usize, 0x400a_8000usize];
    pub const unsafe fn instance(n: u8) -> Option<EQDC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(EQDC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<EQDC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> EQDC {
            let _ = Self::CHECK;
            EQDC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type EQDC0 = Instance<0u8>;
    pub type EQDC1 = Instance<1u8>;
}
pub mod opamp {
    #[path = "../../../peripherals/a2/opamp.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 4usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x400b_7000usize,
        0x400b_8000usize,
        0x400b_9000usize,
        0x400b_a000usize,
    ];
    pub const unsafe fn instance(n: u8) -> Option<OPAMP> {
        if n >= LEN as u8 {
            None
        } else {
            Some(OPAMP::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<OPAMP, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> OPAMP {
            let _ = Self::CHECK;
            OPAMP::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type OPAMP0 = Instance<0u8>;
    pub type OPAMP1 = Instance<1u8>;
    pub type OPAMP2 = Instance<2u8>;
    pub type OPAMP3 = Instance<3u8>;
}
pub mod wuu {
    #[path = "../../../peripherals/a2/wuu.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_2000usize];
    pub const unsafe fn instance(n: u8) -> Option<WUU> {
        if n >= LEN as u8 {
            None
        } else {
            Some(WUU::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<WUU, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> WUU {
            let _ = Self::CHECK;
            WUU::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type WUU0 = Instance<0u8>;
}
pub mod lpdac {
    #[path = "../../../peripherals/a2/lpdac.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400b_4000usize];
    pub const unsafe fn instance(n: u8) -> Option<LPDAC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(LPDAC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<LPDAC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> LPDAC {
            let _ = Self::CHECK;
            LPDAC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type DAC0 = Instance<0u8>;
}
pub mod lptmr {
    #[path = "../../../peripherals/a2/lptmr.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400a_b000usize];
    pub const unsafe fn instance(n: u8) -> Option<LPTMR> {
        if n >= LEN as u8 {
            None
        } else {
            Some(LPTMR::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<LPTMR, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> LPTMR {
            let _ = Self::CHECK;
            LPTMR::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type LPTMR0 = Instance<0u8>;
}
pub mod i3c {
    #[path = "../../../peripherals/a2/i3c.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_2000usize];
    pub const unsafe fn instance(n: u8) -> Option<I3C> {
        if n >= LEN as u8 {
            None
        } else {
            Some(I3C::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<I3C, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> I3C {
            let _ = Self::CHECK;
            I3C::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type I3C0 = Instance<0u8>;
}
pub mod fmu {
    #[path = "../../../peripherals/a2/fmu.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_5000usize];
    pub const unsafe fn instance(n: u8) -> Option<FMU> {
        if n >= LEN as u8 {
            None
        } else {
            Some(FMU::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<FMU, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> FMU {
            let _ = Self::CHECK;
            FMU::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type FMU0 = Instance<0u8>;
}
pub mod can {
    #[path = "../../../peripherals/a2/can.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_c000usize, 0x400d_0000usize];
    pub const unsafe fn instance(n: u8) -> Option<CAN> {
        if n >= LEN as u8 {
            None
        } else {
            Some(CAN::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<CAN, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> CAN {
            let _ = Self::CHECK;
            CAN::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type CAN0 = Instance<0u8>;
    pub type CAN1 = Instance<1u8>;
}
pub mod freqme {
    #[path = "../../../peripherals/a2/freqme.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_9000usize];
    pub const unsafe fn instance(n: u8) -> Option<FREQME> {
        if n >= LEN as u8 {
            None
        } else {
            Some(FREQME::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<FREQME, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> FREQME {
            let _ = Self::CHECK;
            FREQME::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type FREQME0 = Instance<0u8>;
}
pub mod port {
    #[path = "../../../peripherals/a2/port.rs"]
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
    pub const unsafe fn instance(n: u8) -> Option<PORT> {
        if n >= LEN as u8 {
            None
        } else {
            Some(PORT::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<PORT, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> PORT {
            let _ = Self::CHECK;
            PORT::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
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
pub mod scg {
    #[path = "../../../peripherals/a2/scg.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_f000usize];
    pub const unsafe fn instance(n: u8) -> Option<SCG> {
        if n >= LEN as u8 {
            None
        } else {
            Some(SCG::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<SCG, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> SCG {
            let _ = Self::CHECK;
            SCG::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type SCG0 = Instance<0u8>;
}
pub mod adc {
    #[path = "../../../peripherals/a2/adc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 4usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x400a_f000usize,
        0x400b_0000usize,
        0x400f_0000usize,
        0x400f_1000usize,
    ];
    pub const unsafe fn instance(n: u8) -> Option<ADC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(ADC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<ADC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> ADC {
            let _ = Self::CHECK;
            ADC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type ADC0 = Instance<0u8>;
    pub type ADC1 = Instance<1u8>;
    pub type ADC2 = Instance<2u8>;
    pub type ADC3 = Instance<3u8>;
}
pub mod glikey {
    #[path = "../../../peripherals/a2/glikey.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4009_1d00usize];
    pub const unsafe fn instance(n: u8) -> Option<GLIKEY> {
        if n >= LEN as u8 {
            None
        } else {
            Some(GLIKEY::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<GLIKEY, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> GLIKEY {
            let _ = Self::CHECK;
            GLIKEY::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type GLIKEY0 = Instance<0u8>;
}
pub mod erm {
    #[path = "../../../peripherals/a2/erm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_d000usize];
    pub const unsafe fn instance(n: u8) -> Option<ERM> {
        if n >= LEN as u8 {
            None
        } else {
            Some(ERM::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<ERM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> ERM {
            let _ = Self::CHECK;
            ERM::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type ERM0 = Instance<0u8>;
}
pub mod dma {
    #[path = "../../../peripherals/a2/dma.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_0000usize];
    pub const unsafe fn instance(n: u8) -> Option<DMA> {
        if n >= LEN as u8 {
            None
        } else {
            Some(DMA::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<DMA, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> DMA {
            let _ = Self::CHECK;
            DMA::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type DMA0 = Instance<0u8>;
}
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
