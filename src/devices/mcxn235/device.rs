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
#[path = "../../peripherals/n1"]
pub mod adc {
    use core::marker::PhantomData;
    #[path = "adc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_d000usize, 0x4010_e000usize];
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
    pub mod instances {
        use super::Instance;
        pub type ADC0 = Instance<0u8>;
        pub type ADC1 = Instance<1u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod ahbsc {
    use core::marker::PhantomData;
    #[path = "ahbsc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4012_0000usize];
    pub type Instance<const N: u8> = crate::Instance<AHBSC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> AHBSC {
            unsafe { AHBSC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type AHBSC0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod can {
    use core::marker::PhantomData;
    #[path = "can.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400d_4000usize, 0x400d_8000usize];
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
    pub mod instances {
        use super::Instance;
        pub type CAN0 = Instance<0u8>;
        pub type CAN1 = Instance<1u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod cdog {
    use core::marker::PhantomData;
    #[path = "cdog.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400b_b000usize, 0x400b_c000usize];
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
    pub mod instances {
        use super::Instance;
        pub type CDOG0 = Instance<0u8>;
        pub type CDOG1 = Instance<1u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod cmc {
    use core::marker::PhantomData;
    #[path = "cmc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_8000usize];
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
    pub mod instances {
        use super::Instance;
        pub type CMC0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod crc {
    use core::marker::PhantomData;
    #[path = "crc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_b000usize];
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
    pub mod instances {
        use super::Instance;
        pub type CRC0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod ctimer {
    use core::marker::PhantomData;
    #[path = "ctimer.rs"]
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
    pub mod instances {
        use super::Instance;
        pub type CTIMER0 = Instance<0u8>;
        pub type CTIMER1 = Instance<1u8>;
        pub type CTIMER2 = Instance<2u8>;
        pub type CTIMER3 = Instance<3u8>;
        pub type CTIMER4 = Instance<4u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod digtmp {
    use core::marker::PhantomData;
    #[path = "digtmp.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_8000usize];
    pub type Instance<const N: u8> = crate::Instance<DIGTMP, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> DIGTMP {
            unsafe { DIGTMP::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type TDET0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod dm {
    use core::marker::PhantomData;
    #[path = "dm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400b_d000usize];
    pub type Instance<const N: u8> = crate::Instance<DM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> DM {
            unsafe { DM::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type DM0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod dma {
    use core::marker::PhantomData;
    #[path = "dma.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4008_0000usize, 0x400a_0000usize];
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
    pub mod instances {
        use super::Instance;
        pub type DMA0 = Instance<0u8>;
        pub type DMA1 = Instance<1u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod eim {
    use core::marker::PhantomData;
    #[path = "eim.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_b000usize];
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
    pub mod instances {
        use super::Instance;
        pub type EIM0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod erm {
    use core::marker::PhantomData;
    #[path = "erm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_c000usize];
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
    pub mod instances {
        use super::Instance;
        pub type ERM0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod evtg {
    use core::marker::PhantomData;
    #[path = "evtg.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400d_2000usize];
    pub type Instance<const N: u8> = crate::Instance<EVTG, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> EVTG {
            unsafe { EVTG::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type EVTG0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod ewm {
    use core::marker::PhantomData;
    #[path = "ewm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_0000usize];
    pub type Instance<const N: u8> = crate::Instance<EWM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> EWM {
            unsafe { EWM::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type EWM0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod flexio {
    use core::marker::PhantomData;
    #[path = "flexio.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_5000usize];
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
    pub mod instances {
        use super::Instance;
        pub type FLEXIO0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod fmu {
    use core::marker::PhantomData;
    #[path = "fmu.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_3000usize];
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
    pub mod instances {
        use super::Instance;
        pub type FMU0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod fmutest {
    use core::marker::PhantomData;
    #[path = "fmutest.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_3000usize];
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
    pub mod instances {
        use super::Instance;
        pub type FMU0TEST = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod freqme {
    use core::marker::PhantomData;
    #[path = "freqme.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4001_1000usize];
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
    pub mod instances {
        use super::Instance;
        pub type FREQME0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod gdet {
    use core::marker::PhantomData;
    #[path = "gdet.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4002_4000usize, 0x4002_5000usize];
    pub type Instance<const N: u8> = crate::Instance<GDET, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> GDET {
            unsafe { GDET::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type GDET0 = Instance<0u8>;
        pub type GDET1 = Instance<1u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod gpio {
    use core::marker::PhantomData;
    #[path = "gpio.rs"]
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
    pub mod instances {
        use super::Instance;
        pub type GPIO0 = Instance<0u8>;
        pub type GPIO1 = Instance<1u8>;
        pub type GPIO2 = Instance<2u8>;
        pub type GPIO3 = Instance<3u8>;
        pub type GPIO4 = Instance<4u8>;
        pub type GPIO5 = Instance<5u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod i2s {
    use core::marker::PhantomData;
    #[path = "i2s.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_6000usize, 0x4010_7000usize];
    pub type Instance<const N: u8> = crate::Instance<I2S, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> I2S {
            unsafe { I2S::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type SAI0 = Instance<0u8>;
        pub type SAI1 = Instance<1u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod i3c {
    use core::marker::PhantomData;
    #[path = "i3c.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4002_1000usize, 0x4002_2000usize];
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
    pub mod instances {
        use super::Instance;
        pub type I3C0 = Instance<0u8>;
        pub type I3C1 = Instance<1u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod inputmux {
    use core::marker::PhantomData;
    #[path = "inputmux.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_6000usize];
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
    pub mod instances {
        use super::Instance;
        pub type INPUTMUX0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod intm {
    use core::marker::PhantomData;
    #[path = "intm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_d000usize];
    pub type Instance<const N: u8> = crate::Instance<INTM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> INTM {
            unsafe { INTM::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type INTM0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod itrc {
    use core::marker::PhantomData;
    #[path = "itrc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4002_6000usize];
    pub type Instance<const N: u8> = crate::Instance<ITRC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> ITRC {
            unsafe { ITRC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type ITRC0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod lp_flexcomm {
    use core::marker::PhantomData;
    #[path = "lp_flexcomm.rs"]
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
    pub type Instance<const N: u8> = crate::Instance<LP_FLEXCOMM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> LP_FLEXCOMM {
            unsafe { LP_FLEXCOMM::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type LP_FLEXCOMM0 = Instance<0u8>;
        pub type LP_FLEXCOMM1 = Instance<1u8>;
        pub type LP_FLEXCOMM2 = Instance<2u8>;
        pub type LP_FLEXCOMM3 = Instance<3u8>;
        pub type LP_FLEXCOMM4 = Instance<4u8>;
        pub type LP_FLEXCOMM5 = Instance<5u8>;
        pub type LP_FLEXCOMM6 = Instance<6u8>;
        pub type LP_FLEXCOMM7 = Instance<7u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod lpcmp {
    use core::marker::PhantomData;
    #[path = "lpcmp.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_1000usize, 0x4005_2000usize];
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
    pub mod instances {
        use super::Instance;
        pub type CMP0 = Instance<0u8>;
        pub type CMP1 = Instance<1u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod lpi2c {
    use core::marker::PhantomData;
    #[path = "lpi2c.rs"]
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
    pub mod instances {
        use super::Instance;
        pub type LPI2C0 = Instance<0u8>;
        pub type LPI2C1 = Instance<1u8>;
        pub type LPI2C2 = Instance<2u8>;
        pub type LPI2C3 = Instance<3u8>;
        pub type LPI2C4 = Instance<4u8>;
        pub type LPI2C5 = Instance<5u8>;
        pub type LPI2C6 = Instance<6u8>;
        pub type LPI2C7 = Instance<7u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod lpspi {
    use core::marker::PhantomData;
    #[path = "lpspi.rs"]
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
    pub mod instances {
        use super::Instance;
        pub type LPSPI0 = Instance<0u8>;
        pub type LPSPI1 = Instance<1u8>;
        pub type LPSPI2 = Instance<2u8>;
        pub type LPSPI3 = Instance<3u8>;
        pub type LPSPI4 = Instance<4u8>;
        pub type LPSPI5 = Instance<5u8>;
        pub type LPSPI6 = Instance<6u8>;
        pub type LPSPI7 = Instance<7u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod lptmr {
    use core::marker::PhantomData;
    #[path = "lptmr.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_a000usize, 0x4004_b000usize];
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
    pub mod instances {
        use super::Instance;
        pub type LPTMR0 = Instance<0u8>;
        pub type LPTMR1 = Instance<1u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod lpuart {
    use core::marker::PhantomData;
    #[path = "lpuart.rs"]
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
    pub mod instances {
        use super::Instance;
        pub type LPUART0 = Instance<0u8>;
        pub type LPUART1 = Instance<1u8>;
        pub type LPUART2 = Instance<2u8>;
        pub type LPUART3 = Instance<3u8>;
        pub type LPUART4 = Instance<4u8>;
        pub type LPUART5 = Instance<5u8>;
        pub type LPUART6 = Instance<6u8>;
        pub type LPUART7 = Instance<7u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod mrt {
    use core::marker::PhantomData;
    #[path = "mrt.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4001_3000usize];
    pub type Instance<const N: u8> = crate::Instance<MRT, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> MRT {
            unsafe { MRT::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type MRT0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod npx {
    use core::marker::PhantomData;
    #[path = "npx.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_c000usize];
    pub type Instance<const N: u8> = crate::Instance<NPX, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> NPX {
            unsafe { NPX::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type NPX0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod ostimer {
    use core::marker::PhantomData;
    #[path = "ostimer.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_9000usize];
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
    pub mod instances {
        use super::Instance;
        pub type OSTIMER0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod otpc {
    use core::marker::PhantomData;
    #[path = "otpc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_9000usize];
    pub type Instance<const N: u8> = crate::Instance<OTPC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> OTPC {
            unsafe { OTPC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type OTPC0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod pdm {
    use core::marker::PhantomData;
    #[path = "pdm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_c000usize];
    pub type Instance<const N: u8> = crate::Instance<PDM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> PDM {
            unsafe { PDM::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type PDM0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod pint {
    use core::marker::PhantomData;
    #[path = "pint.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_4000usize];
    pub type Instance<const N: u8> = crate::Instance<PINT, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> PINT {
            unsafe { PINT::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type PINT0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod pkc {
    use core::marker::PhantomData;
    #[path = "pkc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4002_b000usize];
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
    pub mod instances {
        use super::Instance;
        pub type PKC0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod port {
    use core::marker::PhantomData;
    #[path = "port.rs"]
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
    pub mod instances {
        use super::Instance;
        pub type PORT0 = Instance<0u8>;
        pub type PORT1 = Instance<1u8>;
        pub type PORT2 = Instance<2u8>;
        pub type PORT3 = Instance<3u8>;
        pub type PORT4 = Instance<4u8>;
        pub type PORT5 = Instance<5u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod puf {
    use core::marker::PhantomData;
    #[path = "puf.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4002_c000usize];
    pub type Instance<const N: u8> = crate::Instance<PUF, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> PUF {
            unsafe { PUF::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type PUF0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod pwm {
    use core::marker::PhantomData;
    #[path = "pwm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_e000usize, 0x400d_0000usize];
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
    pub mod instances {
        use super::Instance;
        pub type PWM0 = Instance<0u8>;
        pub type PWM1 = Instance<1u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod qdc {
    use core::marker::PhantomData;
    #[path = "qdc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_f000usize, 0x400d_1000usize];
    pub type Instance<const N: u8> = crate::Instance<QDC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> QDC {
            unsafe { QDC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type QDC0 = Instance<0u8>;
        pub type QDC1 = Instance<1u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod rtc {
    use core::marker::PhantomData;
    #[path = "rtc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_c000usize];
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
    pub mod instances {
        use super::Instance;
        pub type RTC0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod s50 {
    use core::marker::PhantomData;
    #[path = "s50.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_4000usize];
    pub type Instance<const N: u8> = crate::Instance<S50, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> S50 {
            unsafe { S50::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type ELS = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod scg {
    use core::marker::PhantomData;
    #[path = "scg.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_4000usize];
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
    pub mod instances {
        use super::Instance;
        pub type SCG0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod smartdma {
    use core::marker::PhantomData;
    #[path = "smartdma.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4003_3000usize];
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
    pub mod instances {
        use super::Instance;
        pub type SMARTDMA0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod spc {
    use core::marker::PhantomData;
    #[path = "spc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_5000usize];
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
    pub mod instances {
        use super::Instance;
        pub type SPC0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod syscon {
    use core::marker::PhantomData;
    #[path = "syscon.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4000_0000usize];
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
    pub mod instances {
        use super::Instance;
        pub type SYSCON0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod syspm {
    use core::marker::PhantomData;
    #[path = "syspm.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_1000usize];
    pub type Instance<const N: u8> = crate::Instance<SYSPM, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> SYSPM {
            unsafe { SYSPM::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type CMX_PERFMON0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod trdc {
    use core::marker::PhantomData;
    #[path = "trdc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x400c_7000usize];
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
    pub mod instances {
        use super::Instance;
        pub type TRDC0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod usbhs {
    use core::marker::PhantomData;
    #[path = "usbhs.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_b000usize];
    pub type Instance<const N: u8> = crate::Instance<USBHS, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> USBHS {
            unsafe { USBHS::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type USBHS1__USBC = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod usbhsdcd {
    use core::marker::PhantomData;
    #[path = "usbhsdcd.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_a800usize];
    pub type Instance<const N: u8> = crate::Instance<USBHSDCD, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> USBHSDCD {
            unsafe { USBHSDCD::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type USBHS1_PHY_DCD = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod usbnc {
    use core::marker::PhantomData;
    #[path = "usbnc.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_b200usize];
    pub type Instance<const N: u8> = crate::Instance<USBNC, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> USBNC {
            unsafe { USBNC::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type USBHS1__USBNC = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod usbphy {
    use core::marker::PhantomData;
    #[path = "usbphy.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4010_a000usize];
    pub type Instance<const N: u8> = crate::Instance<USBPHY, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> USBPHY {
            unsafe { USBPHY::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type USBPHY0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod utick {
    use core::marker::PhantomData;
    #[path = "utick.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4001_2000usize];
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
    pub mod instances {
        use super::Instance;
        pub type UTICK0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod vbat {
    use core::marker::PhantomData;
    #[path = "vbat.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4005_9000usize];
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
    pub mod instances {
        use super::Instance;
        pub type VBAT0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod vref {
    use core::marker::PhantomData;
    #[path = "vref.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4011_1000usize];
    pub type Instance<const N: u8> = crate::Instance<VREF, N>;
    impl<const N: u8> Instance<N> {
        const CHECK: () = assert!((N as usize) < LEN);
        #[inline(always)]
        pub const unsafe fn instance() -> Self {
            Self { _t: PhantomData }
        }
        #[inline(always)]
        pub const fn regs(&self) -> VREF {
            unsafe { VREF::from_ptr(self.address() as _) }
        }
        #[inline(always)]
        pub const fn address(&self) -> usize {
            let _ = Self::CHECK;
            ADDRESSES[N as usize]
        }
    }
    pub mod instances {
        use super::Instance;
        pub type VREF0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod wuu {
    use core::marker::PhantomData;
    #[path = "wuu.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 1usize;
    pub const ADDRESSES: [usize; LEN] = [0x4004_6000usize];
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
    pub mod instances {
        use super::Instance;
        pub type WUU0 = Instance<0u8>;
    }
    pub use instances::*;
}
#[path = "../../peripherals/n1"]
pub mod wwdt {
    use core::marker::PhantomData;
    #[path = "wwdt.rs"]
    mod _block;
    pub use _block::*;
    pub const LEN: usize = 2usize;
    pub const ADDRESSES: [usize; LEN] = [0x4001_6000usize, 0x4001_7000usize];
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
    pub mod instances {
        use super::Instance;
        pub type WWDT0 = Instance<0u8>;
        pub type WWDT1 = Instance<1u8>;
    }
    pub use instances::*;
}
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
