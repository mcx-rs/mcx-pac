#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - OR IRQ"]
    OR = 0,
    #[doc = "1 - eDMA_0_CH0 error or transfer complete"]
    EDMA_0_CH0 = 1,
    #[doc = "2 - eDMA_0_CH1 error or transfer complete"]
    EDMA_0_CH1 = 2,
    #[doc = "3 - eDMA_0_CH2 error or transfer complete"]
    EDMA_0_CH2 = 3,
    #[doc = "4 - eDMA_0_CH3 error or transfer complete"]
    EDMA_0_CH3 = 4,
    #[doc = "5 - eDMA_0_CH4 error or transfer complete"]
    EDMA_0_CH4 = 5,
    #[doc = "6 - eDMA_0_CH5 error or transfer complete"]
    EDMA_0_CH5 = 6,
    #[doc = "7 - eDMA_0_CH6 error or transfer complete"]
    EDMA_0_CH6 = 7,
    #[doc = "8 - eDMA_0_CH7 error or transfer complete"]
    EDMA_0_CH7 = 8,
    #[doc = "9 - eDMA_0_CH8 error or transfer complete"]
    EDMA_0_CH8 = 9,
    #[doc = "10 - eDMA_0_CH9 error or transfer complete"]
    EDMA_0_CH9 = 10,
    #[doc = "11 - eDMA_0_CH10 error or transfer complete"]
    EDMA_0_CH10 = 11,
    #[doc = "12 - eDMA_0_CH11 error or transfer complete"]
    EDMA_0_CH11 = 12,
    #[doc = "13 - eDMA_0_CH12 error or transfer complete"]
    EDMA_0_CH12 = 13,
    #[doc = "14 - eDMA_0_CH13 error or transfer complete"]
    EDMA_0_CH13 = 14,
    #[doc = "15 - eDMA_0_CH14 error or transfer complete"]
    EDMA_0_CH14 = 15,
    #[doc = "16 - eDMA_0_CH15 error or transfer complete"]
    EDMA_0_CH15 = 16,
    #[doc = "17 - GPIO0 interrupt 0"]
    GPIO00 = 17,
    #[doc = "18 - GPIO0 interrupt 1"]
    GPIO01 = 18,
    #[doc = "19 - GPIO1 interrupt 0"]
    GPIO10 = 19,
    #[doc = "20 - GPIO1 interrupt 1"]
    GPIO11 = 20,
    #[doc = "21 - GPIO2 interrupt 0"]
    GPIO20 = 21,
    #[doc = "22 - GPIO2 interrupt 1"]
    GPIO21 = 22,
    #[doc = "23 - GPIO3 interrupt 0"]
    GPIO30 = 23,
    #[doc = "24 - GPIO3 interrupt 1"]
    GPIO31 = 24,
    #[doc = "25 - GPIO4 interrupt 0"]
    GPIO40 = 25,
    #[doc = "26 - GPIO4 interrupt 1"]
    GPIO41 = 26,
    #[doc = "27 - GPIO5 interrupt 0"]
    GPIO50 = 27,
    #[doc = "28 - GPIO5 interrupt 1"]
    GPIO51 = 28,
    #[doc = "29 - Micro-Tick Timer interrupt"]
    UTICK0 = 29,
    #[doc = "30 - Multi-Rate Timer interrupt"]
    MRT0 = 30,
    #[doc = "31 - Standard counter/timer 0 interrupt"]
    CTIMER0 = 31,
    #[doc = "32 - Standard counter/timer 1 interrupt"]
    CTIMER1 = 32,
    #[doc = "34 - Standard counter/timer 2 interrupt"]
    CTIMER2 = 34,
    #[doc = "35 - LP_FLEXCOMM0 (LPSPI interrupt or LPI2C interrupt or LPUART Receive/Transmit interrupt)"]
    LP_FLEXCOMM0 = 35,
    #[doc = "36 - LP_FLEXCOMM1 (LPSPI interrupt or LPI2C interrupt or LPUART Receive/Transmit interrupt)"]
    LP_FLEXCOMM1 = 36,
    #[doc = "37 - LP_FLEXCOMM2 (LPSPI interrupt or LPI2C interrupt or LPUART Receive/Transmit interrupt)"]
    LP_FLEXCOMM2 = 37,
    #[doc = "38 - LP_FLEXCOMM3 (LPSPI interrupt or LPI2C interrupt or LPUART Receive/Transmit interrupt)"]
    LP_FLEXCOMM3 = 38,
    #[doc = "39 - LP_FLEXCOMM4 (LPSPI interrupt or LPI2C interrupt or LPUART Receive/Transmit interrupt)"]
    LP_FLEXCOMM4 = 39,
    #[doc = "40 - LP_FLEXCOMM5 (LPSPI interrupt or LPI2C interrupt or LPUART Receive/Transmit interrupt)"]
    LP_FLEXCOMM5 = 40,
    #[doc = "41 - LP_FLEXCOMM6 (LPSPI interrupt or LPI2C interrupt or LPUART Receive/Transmit interrupt)"]
    LP_FLEXCOMM6 = 41,
    #[doc = "42 - LP_FLEXCOMM7 (LPSPI interrupt or LPI2C interrupt or LPUART Receive/Transmit interrupt)"]
    LP_FLEXCOMM7 = 42,
    #[doc = "45 - Analog-to-Digital Converter 0 - General Purpose interrupt"]
    ADC0 = 45,
    #[doc = "46 - Analog-to-Digital Converter 1 - General Purpose interrupt"]
    ADC1 = 46,
    #[doc = "47 - Pin Interrupt Pattern Match Interrupt"]
    PINT0 = 47,
    #[doc = "48 - Microphone Interface interrupt"]
    PDM_EVENT = 48,
    #[doc = "51 - Universal Serial Bus - Device Charge Detect interrupt"]
    USB0_DCD = 51,
    #[doc = "52 - RTC Subsystem interrupt (RTC interrupt or Wake timer interrupt)"]
    RTC = 52,
    #[doc = "53 - SmartDMA_IRQ"]
    SMARTDMA = 53,
    #[doc = "55 - Standard counter/timer 3 interrupt"]
    CTIMER3 = 55,
    #[doc = "56 - Standard counter/timer 4 interrupt"]
    CTIMER4 = 56,
    #[doc = "57 - OS event timer interrupt"]
    OS_EVENT = 57,
    #[doc = "59 - Serial Audio Interface 0 interrupt"]
    SAI0 = 59,
    #[doc = "60 - Serial Audio Interface 1 interrupt"]
    SAI1 = 60,
    #[doc = "62 - Controller Area Network 0 interrupt"]
    CAN0 = 62,
    #[doc = "63 - Controller Area Network 1 interrupt"]
    CAN1 = 63,
    #[doc = "66 - USBHS DCD or USBHS Phy interrupt"]
    USB1_HS_PHY = 66,
    #[doc = "67 - USB High Speed OTG Controller interrupt"]
    USB1_HS = 67,
    #[doc = "68 - AHB Secure Controller hypervisor call interrupt"]
    SEC_HYPERVISOR_CALL = 68,
    #[doc = "71 - Frequency Measurement interrupt"]
    FREQME = 71,
    #[doc = "72 - Secure violation interrupt (Memory Block Checker interrupt or secure AHB matrix violation interrupt)"]
    SEC_VIO = 72,
    #[doc = "73 - ELS interrupt"]
    ELS = 73,
    #[doc = "74 - PKC interrupt"]
    PKC = 74,
    #[doc = "75 - Physical Unclonable Function interrupt"]
    PUF = 75,
    #[doc = "77 - eDMA_1_CH0 error or transfer complete"]
    EDMA_1_CH0 = 77,
    #[doc = "78 - eDMA_1_CH1 error or transfer complete"]
    EDMA_1_CH1 = 78,
    #[doc = "79 - eDMA_1_CH2 error or transfer complete"]
    EDMA_1_CH2 = 79,
    #[doc = "80 - eDMA_1_CH3 error or transfer complete"]
    EDMA_1_CH3 = 80,
    #[doc = "81 - eDMA_1_CH4 error or transfer complete"]
    EDMA_1_CH4 = 81,
    #[doc = "82 - eDMA_1_CH5 error or transfer complete"]
    EDMA_1_CH5 = 82,
    #[doc = "83 - eDMA_1_CH6 error or transfer complete"]
    EDMA_1_CH6 = 83,
    #[doc = "84 - eDMA_1_CH7 error or transfer complete"]
    EDMA_1_CH7 = 84,
    #[doc = "93 - Code Watchdog Timer 0 interrupt"]
    CDOG0 = 93,
    #[doc = "94 - Code Watchdog Timer 1 interrupt"]
    CDOG1 = 94,
    #[doc = "95 - Improved Inter Integrated Circuit interrupt 0"]
    I3C0 = 95,
    #[doc = "96 - Improved Inter Integrated Circuit interrupt 1"]
    I3C1 = 96,
    #[doc = "98 - Digital Glitch Detect 0 interrupt or Digital Glitch Detect 1 interrupt"]
    GDET = 98,
    #[doc = "99 - VBAT interrupt( VBAT interrupt or digital tamper interrupt)"]
    VBAT0 = 99,
    #[doc = "100 - External Watchdog Monitor interrupt"]
    EWM0 = 100,
    #[doc = "105 - Flexible Input/Output interrupt"]
    FLEXIO = 105,
    #[doc = "109 - High-Speed comparator0 interrupt"]
    HSCMP0 = 109,
    #[doc = "110 - High-Speed comparator1 interrupt"]
    HSCMP1 = 110,
    #[doc = "112 - FlexPWM0_reload_error interrupt"]
    FLEXPWM0_RELOAD_ERROR = 112,
    #[doc = "113 - FlexPWM0_fault interrupt"]
    FLEXPWM0_FAULT = 113,
    #[doc = "114 - FlexPWM0 Submodule 0 capture/compare/reload interrupt"]
    FLEXPWM0_SUBMODULE0 = 114,
    #[doc = "115 - FlexPWM0 Submodule 1 capture/compare/reload interrupt"]
    FLEXPWM0_SUBMODULE1 = 115,
    #[doc = "116 - FlexPWM0 Submodule 2 capture/compare/reload interrupt"]
    FLEXPWM0_SUBMODULE2 = 116,
    #[doc = "117 - FlexPWM0 Submodule 3 capture/compare/reload interrupt"]
    FLEXPWM0_SUBMODULE3 = 117,
    #[doc = "118 - FlexPWM1_reload_error interrupt"]
    FLEXPWM1_RELOAD_ERROR = 118,
    #[doc = "119 - FlexPWM1_fault interrupt"]
    FLEXPWM1_FAULT = 119,
    #[doc = "120 - FlexPWM1 Submodule 0 capture/compare/reload interrupt"]
    FLEXPWM1_SUBMODULE0 = 120,
    #[doc = "121 - FlexPWM1 Submodule 1 capture/compare/reload interrupt"]
    FLEXPWM1_SUBMODULE1 = 121,
    #[doc = "122 - FlexPWM1 Submodule 2 capture/compare/reload interrupt"]
    FLEXPWM1_SUBMODULE2 = 122,
    #[doc = "123 - FlexPWM1 Submodule 3 capture/compare/reload interrupt"]
    FLEXPWM1_SUBMODULE3 = 123,
    #[doc = "124 - QDC0_Compare interrupt"]
    QDC0_COMPARE = 124,
    #[doc = "125 - QDC0_Home interrupt"]
    QDC0_HOME = 125,
    #[doc = "126 - QDC0_WDG_IRQ/SAB interrupt"]
    QDC0_WDG_SAB = 126,
    #[doc = "127 - QDC0_IDX interrupt"]
    QDC0_IDX = 127,
    #[doc = "128 - QDC1_Compare interrupt"]
    QDC1_COMPARE = 128,
    #[doc = "129 - QDC1_Home interrupt"]
    QDC1_HOME = 129,
    #[doc = "130 - QDC1_WDG_IRQ/SAB interrupt"]
    QDC1_WDG_SAB = 130,
    #[doc = "131 - QDC1_IDX interrupt"]
    QDC1_IDX = 131,
    #[doc = "132 - Intrusion and Tamper Response Controller interrupt"]
    ITRC0 = 132,
    #[doc = "134 - ELS error interrupt"]
    ELS_ERR = 134,
    #[doc = "135 - PKC error interrupt"]
    PKC_ERR = 135,
    #[doc = "136 - ERM Single Bit error interrupt"]
    ERM_SINGLE_BIT_ERROR = 136,
    #[doc = "137 - ERM Multi Bit error interrupt"]
    ERM_MULTI_BIT_ERROR = 137,
    #[doc = "138 - Flash Management Unit interrupt"]
    FMU0 = 138,
    #[doc = "143 - Low Power Timer 0 interrupt"]
    LPTMR0 = 143,
    #[doc = "144 - Low Power Timer 1 interrupt"]
    LPTMR1 = 144,
    #[doc = "145 - System Clock Generator interrupt"]
    SCG = 145,
    #[doc = "146 - System Power Controller interrupt"]
    SPC = 146,
    #[doc = "147 - Wake Up Unit interrupt"]
    WUU = 147,
    #[doc = "148 - PORT0~5 EFT interrupt"]
    PORT_EFT = 148,
    #[doc = "152 - Windowed Watchdog Timer 0 interrupt"]
    WWDT0 = 152,
    #[doc = "153 - Windowed Watchdog Timer 1 interrupt"]
    WWDT1 = 153,
    #[doc = "154 - Core Mode Controller interrupt"]
    CMC0 = 154,
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
        fn OR();
        fn EDMA_0_CH0();
        fn EDMA_0_CH1();
        fn EDMA_0_CH2();
        fn EDMA_0_CH3();
        fn EDMA_0_CH4();
        fn EDMA_0_CH5();
        fn EDMA_0_CH6();
        fn EDMA_0_CH7();
        fn EDMA_0_CH8();
        fn EDMA_0_CH9();
        fn EDMA_0_CH10();
        fn EDMA_0_CH11();
        fn EDMA_0_CH12();
        fn EDMA_0_CH13();
        fn EDMA_0_CH14();
        fn EDMA_0_CH15();
        fn GPIO00();
        fn GPIO01();
        fn GPIO10();
        fn GPIO11();
        fn GPIO20();
        fn GPIO21();
        fn GPIO30();
        fn GPIO31();
        fn GPIO40();
        fn GPIO41();
        fn GPIO50();
        fn GPIO51();
        fn UTICK0();
        fn MRT0();
        fn CTIMER0();
        fn CTIMER1();
        fn CTIMER2();
        fn LP_FLEXCOMM0();
        fn LP_FLEXCOMM1();
        fn LP_FLEXCOMM2();
        fn LP_FLEXCOMM3();
        fn LP_FLEXCOMM4();
        fn LP_FLEXCOMM5();
        fn LP_FLEXCOMM6();
        fn LP_FLEXCOMM7();
        fn ADC0();
        fn ADC1();
        fn PINT0();
        fn PDM_EVENT();
        fn USB0_DCD();
        fn RTC();
        fn SMARTDMA();
        fn CTIMER3();
        fn CTIMER4();
        fn OS_EVENT();
        fn SAI0();
        fn SAI1();
        fn CAN0();
        fn CAN1();
        fn USB1_HS_PHY();
        fn USB1_HS();
        fn SEC_HYPERVISOR_CALL();
        fn FREQME();
        fn SEC_VIO();
        fn ELS();
        fn PKC();
        fn PUF();
        fn EDMA_1_CH0();
        fn EDMA_1_CH1();
        fn EDMA_1_CH2();
        fn EDMA_1_CH3();
        fn EDMA_1_CH4();
        fn EDMA_1_CH5();
        fn EDMA_1_CH6();
        fn EDMA_1_CH7();
        fn CDOG0();
        fn CDOG1();
        fn I3C0();
        fn I3C1();
        fn GDET();
        fn VBAT0();
        fn EWM0();
        fn FLEXIO();
        fn HSCMP0();
        fn HSCMP1();
        fn FLEXPWM0_RELOAD_ERROR();
        fn FLEXPWM0_FAULT();
        fn FLEXPWM0_SUBMODULE0();
        fn FLEXPWM0_SUBMODULE1();
        fn FLEXPWM0_SUBMODULE2();
        fn FLEXPWM0_SUBMODULE3();
        fn FLEXPWM1_RELOAD_ERROR();
        fn FLEXPWM1_FAULT();
        fn FLEXPWM1_SUBMODULE0();
        fn FLEXPWM1_SUBMODULE1();
        fn FLEXPWM1_SUBMODULE2();
        fn FLEXPWM1_SUBMODULE3();
        fn QDC0_COMPARE();
        fn QDC0_HOME();
        fn QDC0_WDG_SAB();
        fn QDC0_IDX();
        fn QDC1_COMPARE();
        fn QDC1_HOME();
        fn QDC1_WDG_SAB();
        fn QDC1_IDX();
        fn ITRC0();
        fn ELS_ERR();
        fn PKC_ERR();
        fn ERM_SINGLE_BIT_ERROR();
        fn ERM_MULTI_BIT_ERROR();
        fn FMU0();
        fn LPTMR0();
        fn LPTMR1();
        fn SCG();
        fn SPC();
        fn WUU();
        fn PORT_EFT();
        fn WWDT0();
        fn WWDT1();
        fn CMC0();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 155] = [
        Vector { _handler: OR },
        Vector {
            _handler: EDMA_0_CH0,
        },
        Vector {
            _handler: EDMA_0_CH1,
        },
        Vector {
            _handler: EDMA_0_CH2,
        },
        Vector {
            _handler: EDMA_0_CH3,
        },
        Vector {
            _handler: EDMA_0_CH4,
        },
        Vector {
            _handler: EDMA_0_CH5,
        },
        Vector {
            _handler: EDMA_0_CH6,
        },
        Vector {
            _handler: EDMA_0_CH7,
        },
        Vector {
            _handler: EDMA_0_CH8,
        },
        Vector {
            _handler: EDMA_0_CH9,
        },
        Vector {
            _handler: EDMA_0_CH10,
        },
        Vector {
            _handler: EDMA_0_CH11,
        },
        Vector {
            _handler: EDMA_0_CH12,
        },
        Vector {
            _handler: EDMA_0_CH13,
        },
        Vector {
            _handler: EDMA_0_CH14,
        },
        Vector {
            _handler: EDMA_0_CH15,
        },
        Vector { _handler: GPIO00 },
        Vector { _handler: GPIO01 },
        Vector { _handler: GPIO10 },
        Vector { _handler: GPIO11 },
        Vector { _handler: GPIO20 },
        Vector { _handler: GPIO21 },
        Vector { _handler: GPIO30 },
        Vector { _handler: GPIO31 },
        Vector { _handler: GPIO40 },
        Vector { _handler: GPIO41 },
        Vector { _handler: GPIO50 },
        Vector { _handler: GPIO51 },
        Vector { _handler: UTICK0 },
        Vector { _handler: MRT0 },
        Vector { _handler: CTIMER0 },
        Vector { _handler: CTIMER1 },
        Vector { _reserved: 0 },
        Vector { _handler: CTIMER2 },
        Vector {
            _handler: LP_FLEXCOMM0,
        },
        Vector {
            _handler: LP_FLEXCOMM1,
        },
        Vector {
            _handler: LP_FLEXCOMM2,
        },
        Vector {
            _handler: LP_FLEXCOMM3,
        },
        Vector {
            _handler: LP_FLEXCOMM4,
        },
        Vector {
            _handler: LP_FLEXCOMM5,
        },
        Vector {
            _handler: LP_FLEXCOMM6,
        },
        Vector {
            _handler: LP_FLEXCOMM7,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: ADC0 },
        Vector { _handler: ADC1 },
        Vector { _handler: PINT0 },
        Vector {
            _handler: PDM_EVENT,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: USB0_DCD },
        Vector { _handler: RTC },
        Vector { _handler: SMARTDMA },
        Vector { _reserved: 0 },
        Vector { _handler: CTIMER3 },
        Vector { _handler: CTIMER4 },
        Vector { _handler: OS_EVENT },
        Vector { _reserved: 0 },
        Vector { _handler: SAI0 },
        Vector { _handler: SAI1 },
        Vector { _reserved: 0 },
        Vector { _handler: CAN0 },
        Vector { _handler: CAN1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector {
            _handler: USB1_HS_PHY,
        },
        Vector { _handler: USB1_HS },
        Vector {
            _handler: SEC_HYPERVISOR_CALL,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FREQME },
        Vector { _handler: SEC_VIO },
        Vector { _handler: ELS },
        Vector { _handler: PKC },
        Vector { _handler: PUF },
        Vector { _reserved: 0 },
        Vector {
            _handler: EDMA_1_CH0,
        },
        Vector {
            _handler: EDMA_1_CH1,
        },
        Vector {
            _handler: EDMA_1_CH2,
        },
        Vector {
            _handler: EDMA_1_CH3,
        },
        Vector {
            _handler: EDMA_1_CH4,
        },
        Vector {
            _handler: EDMA_1_CH5,
        },
        Vector {
            _handler: EDMA_1_CH6,
        },
        Vector {
            _handler: EDMA_1_CH7,
        },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: CDOG0 },
        Vector { _handler: CDOG1 },
        Vector { _handler: I3C0 },
        Vector { _handler: I3C1 },
        Vector { _reserved: 0 },
        Vector { _handler: GDET },
        Vector { _handler: VBAT0 },
        Vector { _handler: EWM0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: FLEXIO },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: HSCMP0 },
        Vector { _handler: HSCMP1 },
        Vector { _reserved: 0 },
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
            _handler: QDC0_COMPARE,
        },
        Vector {
            _handler: QDC0_HOME,
        },
        Vector {
            _handler: QDC0_WDG_SAB,
        },
        Vector { _handler: QDC0_IDX },
        Vector {
            _handler: QDC1_COMPARE,
        },
        Vector {
            _handler: QDC1_HOME,
        },
        Vector {
            _handler: QDC1_WDG_SAB,
        },
        Vector { _handler: QDC1_IDX },
        Vector { _handler: ITRC0 },
        Vector { _reserved: 0 },
        Vector { _handler: ELS_ERR },
        Vector { _handler: PKC_ERR },
        Vector {
            _handler: ERM_SINGLE_BIT_ERROR,
        },
        Vector {
            _handler: ERM_MULTI_BIT_ERROR,
        },
        Vector { _handler: FMU0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: LPTMR0 },
        Vector { _handler: LPTMR1 },
        Vector { _handler: SCG },
        Vector { _handler: SPC },
        Vector { _handler: WUU },
        Vector { _handler: PORT_EFT },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: WWDT0 },
        Vector { _handler: WWDT1 },
        Vector { _handler: CMC0 },
    ];
}
pub mod usbhsdcd {
    #[path = "../../../peripherals/n1/usbhsdcd.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_a000usize];
    pub const unsafe fn instance(n: u8) -> Option<USBHSDCD> {
        if n >= LEN as u8 {
            None
        } else {
            Some(USBHSDCD::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<USBHSDCD, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> USBHSDCD {
            let _ = Self::CHECK;
            USBHSDCD::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type USBHS1_PHY_DCD = Instance<0u8>;
}
pub mod i3c {
    #[path = "../../../peripherals/n1/i3c.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4002_1000usize, 0x4002_2000usize];
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
    pub type I3C1 = Instance<1u8>;
}
pub mod npx {
    #[path = "../../../peripherals/n1/npx.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_c000usize];
    pub const unsafe fn instance(n: u8) -> Option<NPX> {
        if n >= LEN as u8 {
            None
        } else {
            Some(NPX::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<NPX, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> NPX {
            let _ = Self::CHECK;
            NPX::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type NPX0 = Instance<0u8>;
}
pub mod ahbsc {
    #[path = "../../../peripherals/n1/ahbsc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4012_0000usize];
    pub const unsafe fn instance(n: u8) -> Option<AHBSC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(AHBSC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<AHBSC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> AHBSC {
            let _ = Self::CHECK;
            AHBSC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type AHBSC0 = Instance<0u8>;
}
pub mod wuu {
    #[path = "../../../peripherals/n1/wuu.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_6000usize];
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
pub mod otpc {
    #[path = "../../../peripherals/n1/otpc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_9000usize];
    pub const unsafe fn instance(n: u8) -> Option<OTPC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(OTPC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<OTPC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> OTPC {
            let _ = Self::CHECK;
            OTPC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type OTPC0 = Instance<0u8>;
}
pub mod utick {
    #[path = "../../../peripherals/n1/utick.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4001_2000usize];
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
pub mod fmu {
    #[path = "../../../peripherals/n1/fmu.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_3000usize];
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
pub mod eim {
    #[path = "../../../peripherals/n1/eim.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_b000usize];
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
pub mod syspm {
    #[path = "../../../peripherals/n1/syspm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_1000usize];
    pub const unsafe fn instance(n: u8) -> Option<SYSPM> {
        if n >= LEN as u8 {
            None
        } else {
            Some(SYSPM::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<SYSPM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> SYSPM {
            let _ = Self::CHECK;
            SYSPM::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type CMX_PERFMON0 = Instance<0u8>;
}
pub mod intm {
    #[path = "../../../peripherals/n1/intm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_d000usize];
    pub const unsafe fn instance(n: u8) -> Option<INTM> {
        if n >= LEN as u8 {
            None
        } else {
            Some(INTM::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<INTM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> INTM {
            let _ = Self::CHECK;
            INTM::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type INTM0 = Instance<0u8>;
}
pub mod crc {
    #[path = "../../../peripherals/n1/crc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_b000usize];
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
pub mod pwm {
    #[path = "../../../peripherals/n1/pwm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_e000usize, 0x400d_0000usize];
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
    pub type PWM0 = Instance<0u8>;
    pub type PWM1 = Instance<1u8>;
}
pub mod inputmux {
    #[path = "../../../peripherals/n1/inputmux.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_6000usize];
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
pub mod vbat {
    #[path = "../../../peripherals/n1/vbat.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_9000usize];
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
pub mod ewm {
    #[path = "../../../peripherals/n1/ewm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_0000usize];
    pub const unsafe fn instance(n: u8) -> Option<EWM> {
        if n >= LEN as u8 {
            None
        } else {
            Some(EWM::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<EWM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> EWM {
            let _ = Self::CHECK;
            EWM::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type EWM0 = Instance<0u8>;
}
pub mod ctimer {
    #[path = "../../../peripherals/n1/ctimer.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 5usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4000_c000usize,
        0x4000_d000usize,
        0x4000_e000usize,
        0x4000_f000usize,
        0x4001_0000usize,
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
pub mod puf {
    #[path = "../../../peripherals/n1/puf.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4002_c000usize];
    pub const unsafe fn instance(n: u8) -> Option<PUF> {
        if n >= LEN as u8 {
            None
        } else {
            Some(PUF::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<PUF, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> PUF {
            let _ = Self::CHECK;
            PUF::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type PUF0 = Instance<0u8>;
}
pub mod fmutest {
    #[path = "../../../peripherals/n1/fmutest.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_3000usize];
    pub const unsafe fn instance(n: u8) -> Option<FMUTEST> {
        if n >= LEN as u8 {
            None
        } else {
            Some(FMUTEST::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<FMUTEST, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> FMUTEST {
            let _ = Self::CHECK;
            FMUTEST::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type FMU0TEST = Instance<0u8>;
}
pub mod spc {
    #[path = "../../../peripherals/n1/spc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_5000usize];
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
pub mod usbphy {
    #[path = "../../../peripherals/n1/usbphy.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_a000usize];
    pub const unsafe fn instance(n: u8) -> Option<USBPHY> {
        if n >= LEN as u8 {
            None
        } else {
            Some(USBPHY::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<USBPHY, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> USBPHY {
            let _ = Self::CHECK;
            USBPHY::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type USBPHY0 = Instance<0u8>;
}
pub mod lpspi {
    #[path = "../../../peripherals/n1/lpspi.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 8usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4009_2000usize,
        0x4009_3000usize,
        0x4009_4000usize,
        0x4009_5000usize,
        0x400b_4000usize,
        0x400b_5000usize,
        0x400b_6000usize,
        0x400b_7000usize,
    ];
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
    pub type LPSPI2 = Instance<2u8>;
    pub type LPSPI3 = Instance<3u8>;
    pub type LPSPI4 = Instance<4u8>;
    pub type LPSPI5 = Instance<5u8>;
    pub type LPSPI6 = Instance<6u8>;
    pub type LPSPI7 = Instance<7u8>;
}
pub mod lptmr {
    #[path = "../../../peripherals/n1/lptmr.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_a000usize, 0x4004_b000usize];
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
    pub type LPTMR1 = Instance<1u8>;
}
pub mod dma {
    #[path = "../../../peripherals/n1/dma.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_0000usize, 0x400a_0000usize];
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
    pub type DMA1 = Instance<1u8>;
}
pub mod scg {
    #[path = "../../../peripherals/n1/scg.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_4000usize];
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
pub mod lpcmp {
    #[path = "../../../peripherals/n1/lpcmp.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_1000usize, 0x4005_2000usize];
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
}
pub mod syscon {
    #[path = "../../../peripherals/n1/syscon.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_0000usize];
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
pub mod lpi2c {
    #[path = "../../../peripherals/n1/lpi2c.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 8usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4009_2800usize,
        0x4009_3800usize,
        0x4009_4800usize,
        0x4009_5800usize,
        0x400b_4800usize,
        0x400b_5800usize,
        0x400b_6800usize,
        0x400b_7800usize,
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
    pub type LPI2C4 = Instance<4u8>;
    pub type LPI2C5 = Instance<5u8>;
    pub type LPI2C6 = Instance<6u8>;
    pub type LPI2C7 = Instance<7u8>;
}
pub mod gpio {
    #[path = "../../../peripherals/n1/gpio.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 6usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4009_6000usize,
        0x4009_8000usize,
        0x4009_a000usize,
        0x4009_c000usize,
        0x4009_e000usize,
        0x4004_0000usize,
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
    pub type GPIO5 = Instance<5u8>;
}
pub mod freqme {
    #[path = "../../../peripherals/n1/freqme.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4001_1000usize];
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
pub mod flexio {
    #[path = "../../../peripherals/n1/flexio.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_5000usize];
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
pub mod evtg {
    #[path = "../../../peripherals/n1/evtg.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400d_2000usize];
    pub const unsafe fn instance(n: u8) -> Option<EVTG> {
        if n >= LEN as u8 {
            None
        } else {
            Some(EVTG::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<EVTG, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> EVTG {
            let _ = Self::CHECK;
            EVTG::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type EVTG0 = Instance<0u8>;
}
pub mod lp_flexcomm {
    #[path = "../../../peripherals/n1/lp_flexcomm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 8usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4009_2000usize,
        0x4009_3000usize,
        0x4009_4000usize,
        0x4009_5000usize,
        0x400b_4000usize,
        0x400b_5000usize,
        0x400b_6000usize,
        0x400b_7000usize,
    ];
    pub const unsafe fn instance(n: u8) -> Option<LP_FLEXCOMM> {
        if n >= LEN as u8 {
            None
        } else {
            Some(LP_FLEXCOMM::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<LP_FLEXCOMM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> LP_FLEXCOMM {
            let _ = Self::CHECK;
            LP_FLEXCOMM::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type LP_FLEXCOMM0 = Instance<0u8>;
    pub type LP_FLEXCOMM1 = Instance<1u8>;
    pub type LP_FLEXCOMM2 = Instance<2u8>;
    pub type LP_FLEXCOMM3 = Instance<3u8>;
    pub type LP_FLEXCOMM4 = Instance<4u8>;
    pub type LP_FLEXCOMM5 = Instance<5u8>;
    pub type LP_FLEXCOMM6 = Instance<6u8>;
    pub type LP_FLEXCOMM7 = Instance<7u8>;
}
pub mod dm {
    #[path = "../../../peripherals/n1/dm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400b_d000usize];
    pub const unsafe fn instance(n: u8) -> Option<DM> {
        if n >= LEN as u8 {
            None
        } else {
            Some(DM::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<DM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> DM {
            let _ = Self::CHECK;
            DM::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type DM0 = Instance<0u8>;
}
pub mod itrc {
    #[path = "../../../peripherals/n1/itrc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4002_6000usize];
    pub const unsafe fn instance(n: u8) -> Option<ITRC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(ITRC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<ITRC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> ITRC {
            let _ = Self::CHECK;
            ITRC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type ITRC0 = Instance<0u8>;
}
pub mod pint {
    #[path = "../../../peripherals/n1/pint.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_4000usize];
    pub const unsafe fn instance(n: u8) -> Option<PINT> {
        if n >= LEN as u8 {
            None
        } else {
            Some(PINT::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<PINT, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> PINT {
            let _ = Self::CHECK;
            PINT::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type PINT0 = Instance<0u8>;
}
pub mod trdc {
    #[path = "../../../peripherals/n1/trdc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_7000usize];
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
    pub type TRDC0 = Instance<0u8>;
}
pub mod port {
    #[path = "../../../peripherals/n1/port.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 6usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4011_6000usize,
        0x4011_7000usize,
        0x4011_8000usize,
        0x4011_9000usize,
        0x4011_a000usize,
        0x4004_2000usize,
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
    pub type PORT5 = Instance<5u8>;
}
pub mod mrt {
    #[path = "../../../peripherals/n1/mrt.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4001_3000usize];
    pub const unsafe fn instance(n: u8) -> Option<MRT> {
        if n >= LEN as u8 {
            None
        } else {
            Some(MRT::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<MRT, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> MRT {
            let _ = Self::CHECK;
            MRT::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type MRT0 = Instance<0u8>;
}
pub mod s50 {
    #[path = "../../../peripherals/n1/s50.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_4000usize];
    pub const unsafe fn instance(n: u8) -> Option<S50> {
        if n >= LEN as u8 {
            None
        } else {
            Some(S50::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<S50, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> S50 {
            let _ = Self::CHECK;
            S50::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type ELS = Instance<0u8>;
}
pub mod smartdma {
    #[path = "../../../peripherals/n1/smartdma.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4003_3000usize];
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
pub mod gdet {
    #[path = "../../../peripherals/n1/gdet.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4002_4000usize, 0x4002_5000usize];
    pub const unsafe fn instance(n: u8) -> Option<GDET> {
        if n >= LEN as u8 {
            None
        } else {
            Some(GDET::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<GDET, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> GDET {
            let _ = Self::CHECK;
            GDET::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type GDET0 = Instance<0u8>;
    pub type GDET1 = Instance<1u8>;
}
pub mod i2s {
    #[path = "../../../peripherals/n1/i2s.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_6000usize, 0x4010_7000usize];
    pub const unsafe fn instance(n: u8) -> Option<I2S> {
        if n >= LEN as u8 {
            None
        } else {
            Some(I2S::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<I2S, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> I2S {
            let _ = Self::CHECK;
            I2S::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type SAI0 = Instance<0u8>;
    pub type SAI1 = Instance<1u8>;
}
pub mod ostimer {
    #[path = "../../../peripherals/n1/ostimer.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_9000usize];
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
pub mod vref {
    #[path = "../../../peripherals/n1/vref.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4011_1000usize];
    pub const unsafe fn instance(n: u8) -> Option<VREF> {
        if n >= LEN as u8 {
            None
        } else {
            Some(VREF::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<VREF, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> VREF {
            let _ = Self::CHECK;
            VREF::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type VREF0 = Instance<0u8>;
}
pub mod rtc {
    #[path = "../../../peripherals/n1/rtc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_c000usize];
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
pub mod erm {
    #[path = "../../../peripherals/n1/erm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_c000usize];
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
pub mod lpuart {
    #[path = "../../../peripherals/n1/lpuart.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 8usize;
    pub const ADDRESSES: [usize; LEN] = [
        0x4009_2000usize,
        0x4009_3000usize,
        0x4009_4000usize,
        0x4009_5000usize,
        0x400b_4000usize,
        0x400b_5000usize,
        0x400b_6000usize,
        0x400b_7000usize,
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
    pub type LPUART6 = Instance<6u8>;
    pub type LPUART7 = Instance<7u8>;
}
pub mod cmc {
    #[path = "../../../peripherals/n1/cmc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_8000usize];
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
pub mod usbnc {
    #[path = "../../../peripherals/n1/usbnc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_b200usize];
    pub const unsafe fn instance(n: u8) -> Option<USBNC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(USBNC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<USBNC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> USBNC {
            let _ = Self::CHECK;
            USBNC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type USBHS1__USBNC = Instance<0u8>;
}
pub mod pdm {
    #[path = "../../../peripherals/n1/pdm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_c000usize];
    pub const unsafe fn instance(n: u8) -> Option<PDM> {
        if n >= LEN as u8 {
            None
        } else {
            Some(PDM::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<PDM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> PDM {
            let _ = Self::CHECK;
            PDM::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type PDM0 = Instance<0u8>;
}
pub mod wwdt {
    #[path = "../../../peripherals/n1/wwdt.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4001_6000usize, 0x4001_7000usize];
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
    pub type WWDT1 = Instance<1u8>;
}
pub mod qdc {
    #[path = "../../../peripherals/n1/qdc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_f000usize, 0x400d_1000usize];
    pub const unsafe fn instance(n: u8) -> Option<QDC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(QDC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<QDC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> QDC {
            let _ = Self::CHECK;
            QDC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type QDC0 = Instance<0u8>;
    pub type QDC1 = Instance<1u8>;
}
pub mod usbhs {
    #[path = "../../../peripherals/n1/usbhs.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_b000usize];
    pub const unsafe fn instance(n: u8) -> Option<USBHS> {
        if n >= LEN as u8 {
            None
        } else {
            Some(USBHS::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<USBHS, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> USBHS {
            let _ = Self::CHECK;
            USBHS::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type USBHS1__USBC = Instance<0u8>;
}
pub mod digtmp {
    #[path = "../../../peripherals/n1/digtmp.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_8000usize];
    pub const unsafe fn instance(n: u8) -> Option<DIGTMP> {
        if n >= LEN as u8 {
            None
        } else {
            Some(DIGTMP::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<DIGTMP, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> DIGTMP {
            let _ = Self::CHECK;
            DIGTMP::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type TDET0 = Instance<0u8>;
}
pub mod can {
    #[path = "../../../peripherals/n1/can.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400d_4000usize, 0x400d_8000usize];
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
pub mod cdog {
    #[path = "../../../peripherals/n1/cdog.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400b_b000usize, 0x400b_c000usize];
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
    pub type CDOG1 = Instance<1u8>;
}
pub mod pkc {
    #[path = "../../../peripherals/n1/pkc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4002_b000usize];
    pub const unsafe fn instance(n: u8) -> Option<PKC> {
        if n >= LEN as u8 {
            None
        } else {
            Some(PKC::from_ptr(ADDRESSES[n as usize] as _))
        }
    }
    pub type Instance<const N: u8> = crate::Instance<PKC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn regs() -> PKC {
            let _ = Self::CHECK;
            PKC::from_ptr(Self::address() as _)
        }
        #[inline(always)]
        pub const fn address() -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub type PKC0 = Instance<0u8>;
}
pub mod adc {
    #[path = "../../../peripherals/n1/adc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_d000usize, 0x4010_e000usize];
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
}
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
