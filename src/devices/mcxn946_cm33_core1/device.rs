#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
    #[doc = "33 - SCTimer/PWM interrupt"]
    SCT0 = 33,
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
    #[doc = "43 - LP_FLEXCOMM8 (LPSPI interrupt or LPI2C interrupt or LPUART Receive/Transmit interrupt)"]
    LP_FLEXCOMM8 = 43,
    #[doc = "44 - LP_FLEXCOMM9 (LPSPI interrupt or LPI2C interrupt or LPUART Receive/Transmit interrupt)"]
    LP_FLEXCOMM9 = 44,
    #[doc = "45 - Analog-to-Digital Converter 0 - General Purpose interrupt"]
    ADC0 = 45,
    #[doc = "46 - Analog-to-Digital Converter 1 - General Purpose interrupt"]
    ADC1 = 46,
    #[doc = "47 - Pin Interrupt Pattern Match Interrupt"]
    PINT0 = 47,
    #[doc = "48 - Microphone Interface interrupt"]
    PDM_EVENT = 48,
    #[doc = "50 - Universal Serial Bus - Full Speed interrupt"]
    USB0_FS = 50,
    #[doc = "51 - Universal Serial Bus - Device Charge Detect interrupt"]
    USB0_DCD = 51,
    #[doc = "52 - RTC Subsystem interrupt (RTC interrupt or Wake timer interrupt)"]
    RTC = 52,
    #[doc = "53 - SmartDMA_IRQ"]
    SMARTDMA = 53,
    #[doc = "54 - Inter-CPU Mailbox interrupt0 for CPU0 Inter-CPU Mailbox interrupt1 for CPU1"]
    MAILBOX = 54,
    #[doc = "55 - Standard counter/timer 3 interrupt"]
    CTIMER3 = 55,
    #[doc = "56 - Standard counter/timer 4 interrupt"]
    CTIMER4 = 56,
    #[doc = "57 - OS event timer interrupt"]
    OS_EVENT = 57,
    #[doc = "58 - Flexible Serial Peripheral Interface interrupt"]
    FLEXSPI0 = 58,
    #[doc = "59 - Serial Audio Interface 0 interrupt"]
    SAI0 = 59,
    #[doc = "60 - Serial Audio Interface 1 interrupt"]
    SAI1 = 60,
    #[doc = "61 - Ultra Secured Digital Host Controller interrupt"]
    USDHC0 = 61,
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
    #[doc = "70 - Programmable Logic Unit interrupt"]
    PLU = 70,
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
    #[doc = "76 - Power Quad interrupt"]
    PQ = 76,
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
    #[doc = "85 - eDMA_1_CH8 error or transfer complete"]
    EDMA_1_CH8 = 85,
    #[doc = "86 - eDMA_1_CH9 error or transfer complete"]
    EDMA_1_CH9 = 86,
    #[doc = "87 - eDMA_1_CH10 error or transfer complete"]
    EDMA_1_CH10 = 87,
    #[doc = "88 - eDMA_1_CH11 error or transfer complete"]
    EDMA_1_CH11 = 88,
    #[doc = "89 - eDMA_1_CH12 error or transfer complete"]
    EDMA_1_CH12 = 89,
    #[doc = "90 - eDMA_1_CH13 error or transfer complete"]
    EDMA_1_CH13 = 90,
    #[doc = "91 - eDMA_1_CH14 error or transfer complete"]
    EDMA_1_CH14 = 91,
    #[doc = "92 - eDMA_1_CH15 error or transfer complete"]
    EDMA_1_CH15 = 92,
    #[doc = "93 - Code Watchdog Timer 0 interrupt"]
    CDOG0 = 93,
    #[doc = "94 - Code Watchdog Timer 1 interrupt"]
    CDOG1 = 94,
    #[doc = "95 - Improved Inter Integrated Circuit interrupt 0"]
    I3C0 = 95,
    #[doc = "96 - Improved Inter Integrated Circuit interrupt 1"]
    I3C1 = 96,
    #[doc = "97 - NPU interrupt"]
    NPU = 97,
    #[doc = "98 - Digital Glitch Detect 0 interrupt or Digital Glitch Detect 1 interrupt"]
    GDET = 98,
    #[doc = "99 - VBAT interrupt( VBAT interrupt or digital tamper interrupt)"]
    VBAT0 = 99,
    #[doc = "100 - External Watchdog Monitor interrupt"]
    EWM0 = 100,
    #[doc = "101 - TSI End of Scan interrupt"]
    TSI_END_OF_SCAN = 101,
    #[doc = "102 - TSI Out of Scan interrupt"]
    TSI_OUT_OF_SCAN = 102,
    #[doc = "103 - EMVSIM0 interrupt"]
    EMVSIM0 = 103,
    #[doc = "104 - EMVSIM1 interrupt"]
    EMVSIM1 = 104,
    #[doc = "105 - Flexible Input/Output interrupt"]
    FLEXIO = 105,
    #[doc = "106 - Digital-to-Analog Converter 0 - General Purpose interrupt"]
    DAC0 = 106,
    #[doc = "107 - Digital-to-Analog Converter 1 - General Purpose interrupt"]
    DAC1 = 107,
    #[doc = "108 - 14-bit Digital-to-Analog Converter interrupt"]
    DAC2 = 108,
    #[doc = "109 - High-Speed comparator0 interrupt"]
    HSCMP0 = 109,
    #[doc = "110 - High-Speed comparator1 interrupt"]
    HSCMP1 = 110,
    #[doc = "111 - High-Speed comparator2 interrupt"]
    HSCMP2 = 111,
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
    #[doc = "133 - CoolFlux BSP32 interrupt"]
    BSP32 = 133,
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
    #[doc = "139 - Ethernet QoS interrupt"]
    ETHERNET = 139,
    #[doc = "140 - Ethernet QoS power management interrupt"]
    ETHERNET_PMT = 140,
    #[doc = "141 - Ethernet QoS MAC interrupt"]
    ETHERNET_MACLP = 141,
    #[doc = "142 - SINC Filter interrupt"]
    SINC_FILTER = 142,
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
    #[doc = "149 - ETB counter expires interrupt"]
    ETB0 = 149,
    #[doc = "152 - Windowed Watchdog Timer 0 interrupt"]
    WWDT0 = 152,
    #[doc = "153 - Windowed Watchdog Timer 1 interrupt"]
    WWDT1 = 153,
    #[doc = "154 - Core Mode Controller interrupt"]
    CMC0 = 154,
    #[doc = "155 - Cross Trigger Interface interrupt"]
    CTI0 = 155,
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
        fn SCT0();
        fn CTIMER2();
        fn LP_FLEXCOMM0();
        fn LP_FLEXCOMM1();
        fn LP_FLEXCOMM2();
        fn LP_FLEXCOMM3();
        fn LP_FLEXCOMM4();
        fn LP_FLEXCOMM5();
        fn LP_FLEXCOMM6();
        fn LP_FLEXCOMM7();
        fn LP_FLEXCOMM8();
        fn LP_FLEXCOMM9();
        fn ADC0();
        fn ADC1();
        fn PINT0();
        fn PDM_EVENT();
        fn USB0_FS();
        fn USB0_DCD();
        fn RTC();
        fn SMARTDMA();
        fn MAILBOX();
        fn CTIMER3();
        fn CTIMER4();
        fn OS_EVENT();
        fn FLEXSPI0();
        fn SAI0();
        fn SAI1();
        fn USDHC0();
        fn CAN0();
        fn CAN1();
        fn USB1_HS_PHY();
        fn USB1_HS();
        fn SEC_HYPERVISOR_CALL();
        fn PLU();
        fn FREQME();
        fn SEC_VIO();
        fn ELS();
        fn PKC();
        fn PUF();
        fn PQ();
        fn EDMA_1_CH0();
        fn EDMA_1_CH1();
        fn EDMA_1_CH2();
        fn EDMA_1_CH3();
        fn EDMA_1_CH4();
        fn EDMA_1_CH5();
        fn EDMA_1_CH6();
        fn EDMA_1_CH7();
        fn EDMA_1_CH8();
        fn EDMA_1_CH9();
        fn EDMA_1_CH10();
        fn EDMA_1_CH11();
        fn EDMA_1_CH12();
        fn EDMA_1_CH13();
        fn EDMA_1_CH14();
        fn EDMA_1_CH15();
        fn CDOG0();
        fn CDOG1();
        fn I3C0();
        fn I3C1();
        fn NPU();
        fn GDET();
        fn VBAT0();
        fn EWM0();
        fn TSI_END_OF_SCAN();
        fn TSI_OUT_OF_SCAN();
        fn EMVSIM0();
        fn EMVSIM1();
        fn FLEXIO();
        fn DAC0();
        fn DAC1();
        fn DAC2();
        fn HSCMP0();
        fn HSCMP1();
        fn HSCMP2();
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
        fn BSP32();
        fn ELS_ERR();
        fn PKC_ERR();
        fn ERM_SINGLE_BIT_ERROR();
        fn ERM_MULTI_BIT_ERROR();
        fn FMU0();
        fn ETHERNET();
        fn ETHERNET_PMT();
        fn ETHERNET_MACLP();
        fn SINC_FILTER();
        fn LPTMR0();
        fn LPTMR1();
        fn SCG();
        fn SPC();
        fn WUU();
        fn PORT_EFT();
        fn ETB0();
        fn WWDT0();
        fn WWDT1();
        fn CMC0();
        fn CTI0();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 156] = [
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
        Vector { _handler: SCT0 },
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
        Vector {
            _handler: LP_FLEXCOMM8,
        },
        Vector {
            _handler: LP_FLEXCOMM9,
        },
        Vector { _handler: ADC0 },
        Vector { _handler: ADC1 },
        Vector { _handler: PINT0 },
        Vector {
            _handler: PDM_EVENT,
        },
        Vector { _reserved: 0 },
        Vector { _handler: USB0_FS },
        Vector { _handler: USB0_DCD },
        Vector { _handler: RTC },
        Vector { _handler: SMARTDMA },
        Vector { _handler: MAILBOX },
        Vector { _handler: CTIMER3 },
        Vector { _handler: CTIMER4 },
        Vector { _handler: OS_EVENT },
        Vector { _handler: FLEXSPI0 },
        Vector { _handler: SAI0 },
        Vector { _handler: SAI1 },
        Vector { _handler: USDHC0 },
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
        Vector { _handler: PLU },
        Vector { _handler: FREQME },
        Vector { _handler: SEC_VIO },
        Vector { _handler: ELS },
        Vector { _handler: PKC },
        Vector { _handler: PUF },
        Vector { _handler: PQ },
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
        Vector {
            _handler: EDMA_1_CH8,
        },
        Vector {
            _handler: EDMA_1_CH9,
        },
        Vector {
            _handler: EDMA_1_CH10,
        },
        Vector {
            _handler: EDMA_1_CH11,
        },
        Vector {
            _handler: EDMA_1_CH12,
        },
        Vector {
            _handler: EDMA_1_CH13,
        },
        Vector {
            _handler: EDMA_1_CH14,
        },
        Vector {
            _handler: EDMA_1_CH15,
        },
        Vector { _handler: CDOG0 },
        Vector { _handler: CDOG1 },
        Vector { _handler: I3C0 },
        Vector { _handler: I3C1 },
        Vector { _handler: NPU },
        Vector { _handler: GDET },
        Vector { _handler: VBAT0 },
        Vector { _handler: EWM0 },
        Vector {
            _handler: TSI_END_OF_SCAN,
        },
        Vector {
            _handler: TSI_OUT_OF_SCAN,
        },
        Vector { _handler: EMVSIM0 },
        Vector { _handler: EMVSIM1 },
        Vector { _handler: FLEXIO },
        Vector { _handler: DAC0 },
        Vector { _handler: DAC1 },
        Vector { _handler: DAC2 },
        Vector { _handler: HSCMP0 },
        Vector { _handler: HSCMP1 },
        Vector { _handler: HSCMP2 },
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
        Vector { _handler: BSP32 },
        Vector { _handler: ELS_ERR },
        Vector { _handler: PKC_ERR },
        Vector {
            _handler: ERM_SINGLE_BIT_ERROR,
        },
        Vector {
            _handler: ERM_MULTI_BIT_ERROR,
        },
        Vector { _handler: FMU0 },
        Vector { _handler: ETHERNET },
        Vector {
            _handler: ETHERNET_PMT,
        },
        Vector {
            _handler: ETHERNET_MACLP,
        },
        Vector {
            _handler: SINC_FILTER,
        },
        Vector { _handler: LPTMR0 },
        Vector { _handler: LPTMR1 },
        Vector { _handler: SCG },
        Vector { _handler: SPC },
        Vector { _handler: WUU },
        Vector { _handler: PORT_EFT },
        Vector { _handler: ETB0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: WWDT0 },
        Vector { _handler: WWDT1 },
        Vector { _handler: CMC0 },
        Vector { _handler: CTI0 },
    ];
}
pub const SYSCON0: syscon::SYSCON = unsafe { syscon::SYSCON::from_ptr(0x4000_0000usize as _) };
pub const PINT0: pint::PINT = unsafe { pint::PINT::from_ptr(0x4000_4000usize as _) };
pub const INPUTMUX0: inputmux::INPUTMUX =
    unsafe { inputmux::INPUTMUX::from_ptr(0x4000_6000usize as _) };
pub const CTIMER0: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4000_c000usize as _) };
pub const CTIMER1: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4000_d000usize as _) };
pub const CTIMER2: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4000_e000usize as _) };
pub const CTIMER3: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4000_f000usize as _) };
pub const CTIMER4: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4001_0000usize as _) };
pub const FREQME0: freqme::FREQME = unsafe { freqme::FREQME::from_ptr(0x4001_1000usize as _) };
pub const UTICK0: utick::UTICK = unsafe { utick::UTICK::from_ptr(0x4001_2000usize as _) };
pub const MRT0: mrt::MRT = unsafe { mrt::MRT::from_ptr(0x4001_3000usize as _) };
pub const WWDT0: wwdt::WWDT = unsafe { wwdt::WWDT::from_ptr(0x4001_6000usize as _) };
pub const WWDT1: wwdt::WWDT = unsafe { wwdt::WWDT::from_ptr(0x4001_7000usize as _) };
pub const CACHE64_CTRL0: cache64_ctrl::CACHE64_CTRL =
    unsafe { cache64_ctrl::CACHE64_CTRL::from_ptr(0x4001_b000usize as _) };
pub const CACHE64_POLSEL0: cache64_polsel::CACHE64_POLSEL =
    unsafe { cache64_polsel::CACHE64_POLSEL::from_ptr(0x4001_b000usize as _) };
pub const I3C0: i3c::I3C = unsafe { i3c::I3C::from_ptr(0x4002_1000usize as _) };
pub const I3C1: i3c::I3C = unsafe { i3c::I3C::from_ptr(0x4002_2000usize as _) };
pub const GDET0: gdet::GDET = unsafe { gdet::GDET::from_ptr(0x4002_4000usize as _) };
pub const GDET1: gdet::GDET = unsafe { gdet::GDET::from_ptr(0x4002_5000usize as _) };
pub const ITRC0: itrc::ITRC = unsafe { itrc::ITRC::from_ptr(0x4002_6000usize as _) };
pub const PKC0: pkc::PKC = unsafe { pkc::PKC::from_ptr(0x4002_b000usize as _) };
pub const PUF: puf::PUF = unsafe { puf::PUF::from_ptr(0x4002_c000usize as _) };
pub const PUF_ALIAS1: puf::PUF = unsafe { puf::PUF::from_ptr(0x4002_d000usize as _) };
pub const PUF_ALIAS2: puf::PUF = unsafe { puf::PUF::from_ptr(0x4002_e000usize as _) };
pub const PUF_ALIAS3: puf::PUF = unsafe { puf::PUF::from_ptr(0x4002_f000usize as _) };
pub const BSP32_0: bsp32::BSP32 = unsafe { bsp32::BSP32::from_ptr(0x4003_2000usize as _) };
pub const SMARTDMA0: smartdma::SMARTDMA =
    unsafe { smartdma::SMARTDMA::from_ptr(0x4003_3000usize as _) };
pub const PLU0: plu::PLU = unsafe { plu::PLU::from_ptr(0x4003_4000usize as _) };
pub const GPIO5: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4004_0000usize as _) };
pub const GPIO5_ALIAS1: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4004_1000usize as _) };
pub const PORT5: port::PORT = unsafe { port::PORT::from_ptr(0x4004_2000usize as _) };
pub const FMU0: fmu::FMU = unsafe { fmu::FMU::from_ptr(0x4004_3000usize as _) };
pub const FMU0TEST: fmutest::FMUTEST = unsafe { fmutest::FMUTEST::from_ptr(0x4004_3000usize as _) };
pub const SCG0: scg::SCG = unsafe { scg::SCG::from_ptr(0x4004_4000usize as _) };
pub const SPC0: spc::SPC = unsafe { spc::SPC::from_ptr(0x4004_5000usize as _) };
pub const WUU0: wuu::WUU = unsafe { wuu::WUU::from_ptr(0x4004_6000usize as _) };
pub const CMC0: cmc::CMC = unsafe { cmc::CMC::from_ptr(0x4004_8000usize as _) };
pub const OSTIMER0: ostimer::OSTIMER = unsafe { ostimer::OSTIMER::from_ptr(0x4004_9000usize as _) };
pub const LPTMR0: lptmr::LPTMR = unsafe { lptmr::LPTMR::from_ptr(0x4004_a000usize as _) };
pub const LPTMR1: lptmr::LPTMR = unsafe { lptmr::LPTMR::from_ptr(0x4004_b000usize as _) };
pub const RTC0: rtc::RTC = unsafe { rtc::RTC::from_ptr(0x4004_c000usize as _) };
pub const CMP0: lpcmp::LPCMP = unsafe { lpcmp::LPCMP::from_ptr(0x4005_1000usize as _) };
pub const CMP1: lpcmp::LPCMP = unsafe { lpcmp::LPCMP::from_ptr(0x4005_2000usize as _) };
pub const CMP2: lpcmp::LPCMP = unsafe { lpcmp::LPCMP::from_ptr(0x4005_3000usize as _) };
pub const ELS: s50::S50 = unsafe { s50::S50::from_ptr(0x4005_4000usize as _) };
pub const ELS_ALIAS1: s50::S50 = unsafe { s50::S50::from_ptr(0x4005_5000usize as _) };
pub const ELS_ALIAS2: s50::S50 = unsafe { s50::S50::from_ptr(0x4005_6000usize as _) };
pub const ELS_ALIAS3: s50::S50 = unsafe { s50::S50::from_ptr(0x4005_7000usize as _) };
pub const TDET0: digtmp::DIGTMP = unsafe { digtmp::DIGTMP::from_ptr(0x4005_8000usize as _) };
pub const VBAT0: vbat::VBAT = unsafe { vbat::VBAT::from_ptr(0x4005_9000usize as _) };
pub const EIM0: eim::EIM = unsafe { eim::EIM::from_ptr(0x4005_b000usize as _) };
pub const ERM0: erm::ERM = unsafe { erm::ERM::from_ptr(0x4005_c000usize as _) };
pub const INTM0: intm::INTM = unsafe { intm::INTM::from_ptr(0x4005_d000usize as _) };
pub const DMA0: dma::DMA = unsafe { dma::DMA::from_ptr(0x4008_0000usize as _) };
pub const SCT0: sct::SCT = unsafe { sct::SCT::from_ptr(0x4009_1000usize as _) };
pub const LPSPI0: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x4009_2000usize as _) };
pub const LPUART0: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x4009_2000usize as _) };
pub const LP_FLEXCOMM0: lp_flexcomm::LP_FLEXCOMM =
    unsafe { lp_flexcomm::LP_FLEXCOMM::from_ptr(0x4009_2000usize as _) };
pub const LPI2C0: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x4009_2800usize as _) };
pub const LPSPI1: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x4009_3000usize as _) };
pub const LPUART1: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x4009_3000usize as _) };
pub const LP_FLEXCOMM1: lp_flexcomm::LP_FLEXCOMM =
    unsafe { lp_flexcomm::LP_FLEXCOMM::from_ptr(0x4009_3000usize as _) };
pub const LPI2C1: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x4009_3800usize as _) };
pub const LPSPI2: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x4009_4000usize as _) };
pub const LPUART2: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x4009_4000usize as _) };
pub const LP_FLEXCOMM2: lp_flexcomm::LP_FLEXCOMM =
    unsafe { lp_flexcomm::LP_FLEXCOMM::from_ptr(0x4009_4000usize as _) };
pub const LPI2C2: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x4009_4800usize as _) };
pub const LPSPI3: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x4009_5000usize as _) };
pub const LPUART3: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x4009_5000usize as _) };
pub const LP_FLEXCOMM3: lp_flexcomm::LP_FLEXCOMM =
    unsafe { lp_flexcomm::LP_FLEXCOMM::from_ptr(0x4009_5000usize as _) };
pub const LPI2C3: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x4009_5800usize as _) };
pub const GPIO0: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4009_6000usize as _) };
pub const GPIO0_ALIAS1: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4009_7000usize as _) };
pub const GPIO1: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4009_8000usize as _) };
pub const GPIO1_ALIAS1: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4009_9000usize as _) };
pub const GPIO2: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4009_a000usize as _) };
pub const GPIO2_ALIAS1: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4009_b000usize as _) };
pub const GPIO3: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4009_c000usize as _) };
pub const GPIO3_ALIAS1: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4009_d000usize as _) };
pub const GPIO4: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4009_e000usize as _) };
pub const GPIO4_ALIAS1: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4009_f000usize as _) };
pub const DMA1: dma::DMA = unsafe { dma::DMA::from_ptr(0x400a_0000usize as _) };
pub const SEMA42_0: sema42::SEMA42 = unsafe { sema42::SEMA42::from_ptr(0x400b_1000usize as _) };
pub const MAILBOX: mailbox::MAILBOX = unsafe { mailbox::MAILBOX::from_ptr(0x400b_2000usize as _) };
pub const LPSPI4: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x400b_4000usize as _) };
pub const LPUART4: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400b_4000usize as _) };
pub const LP_FLEXCOMM4: lp_flexcomm::LP_FLEXCOMM =
    unsafe { lp_flexcomm::LP_FLEXCOMM::from_ptr(0x400b_4000usize as _) };
pub const LPI2C4: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x400b_4800usize as _) };
pub const LPSPI5: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x400b_5000usize as _) };
pub const LPUART5: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400b_5000usize as _) };
pub const LP_FLEXCOMM5: lp_flexcomm::LP_FLEXCOMM =
    unsafe { lp_flexcomm::LP_FLEXCOMM::from_ptr(0x400b_5000usize as _) };
pub const LPI2C5: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x400b_5800usize as _) };
pub const LPSPI6: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x400b_6000usize as _) };
pub const LPUART6: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400b_6000usize as _) };
pub const LP_FLEXCOMM6: lp_flexcomm::LP_FLEXCOMM =
    unsafe { lp_flexcomm::LP_FLEXCOMM::from_ptr(0x400b_6000usize as _) };
pub const LPI2C6: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x400b_6800usize as _) };
pub const LPSPI7: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x400b_7000usize as _) };
pub const LPUART7: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400b_7000usize as _) };
pub const LP_FLEXCOMM7: lp_flexcomm::LP_FLEXCOMM =
    unsafe { lp_flexcomm::LP_FLEXCOMM::from_ptr(0x400b_7000usize as _) };
pub const LPI2C7: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x400b_7800usize as _) };
pub const LPSPI8: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x400b_8000usize as _) };
pub const LPUART8: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400b_8000usize as _) };
pub const LP_FLEXCOMM8: lp_flexcomm::LP_FLEXCOMM =
    unsafe { lp_flexcomm::LP_FLEXCOMM::from_ptr(0x400b_8000usize as _) };
pub const LPI2C8: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x400b_8800usize as _) };
pub const LPSPI9: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x400b_9000usize as _) };
pub const LPUART9: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400b_9000usize as _) };
pub const LP_FLEXCOMM9: lp_flexcomm::LP_FLEXCOMM =
    unsafe { lp_flexcomm::LP_FLEXCOMM::from_ptr(0x400b_9000usize as _) };
pub const LPI2C9: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x400b_9800usize as _) };
pub const CDOG0: cdog::CDOG = unsafe { cdog::CDOG::from_ptr(0x400b_b000usize as _) };
pub const CDOG1: cdog::CDOG = unsafe { cdog::CDOG::from_ptr(0x400b_c000usize as _) };
pub const DM0: dm::DM = unsafe { dm::DM::from_ptr(0x400b_d000usize as _) };
pub const POWERQUAD: powerquad::POWERQUAD =
    unsafe { powerquad::POWERQUAD::from_ptr(0x400b_f000usize as _) };
pub const EWM0: ewm::EWM = unsafe { ewm::EWM::from_ptr(0x400c_0000usize as _) };
pub const CMX_PERFMON0: syspm::SYSPM = unsafe { syspm::SYSPM::from_ptr(0x400c_1000usize as _) };
pub const CMX_PERFMON1: syspm::SYSPM = unsafe { syspm::SYSPM::from_ptr(0x400c_2000usize as _) };
pub const TRDC: trdc::TRDC = unsafe { trdc::TRDC::from_ptr(0x400c_7000usize as _) };
pub const FLEXSPI0: flexspi::FLEXSPI = unsafe { flexspi::FLEXSPI::from_ptr(0x400c_8000usize as _) };
pub const OTPC0: otpc::OTPC = unsafe { otpc::OTPC::from_ptr(0x400c_9000usize as _) };
pub const CRC0: crc::CRC = unsafe { crc::CRC::from_ptr(0x400c_b000usize as _) };
pub const NPX0: npx::NPX = unsafe { npx::NPX::from_ptr(0x400c_c000usize as _) };
pub const PWM0: pwm::PWM = unsafe { pwm::PWM::from_ptr(0x400c_e000usize as _) };
pub const QDC0: qdc::QDC = unsafe { qdc::QDC::from_ptr(0x400c_f000usize as _) };
pub const PWM1: pwm::PWM = unsafe { pwm::PWM::from_ptr(0x400d_0000usize as _) };
pub const QDC1: qdc::QDC = unsafe { qdc::QDC::from_ptr(0x400d_1000usize as _) };
pub const EVTG0: evtg::EVTG = unsafe { evtg::EVTG::from_ptr(0x400d_2000usize as _) };
pub const CAN0: can::CAN = unsafe { can::CAN::from_ptr(0x400d_4000usize as _) };
pub const CAN1: can::CAN = unsafe { can::CAN::from_ptr(0x400d_8000usize as _) };
pub const USBDCD0: usbdcd::USBDCD = unsafe { usbdcd::USBDCD::from_ptr(0x400d_c000usize as _) };
pub const USBFS0: usb::USB = unsafe { usb::USB::from_ptr(0x400d_d000usize as _) };
pub const ENET0: enet::ENET = unsafe { enet::ENET::from_ptr(0x4010_0000usize as _) };
pub const FLEXIO0: flexio::FLEXIO = unsafe { flexio::FLEXIO::from_ptr(0x4010_5000usize as _) };
pub const SAI0: i2s::I2S = unsafe { i2s::I2S::from_ptr(0x4010_6000usize as _) };
pub const SAI1: i2s::I2S = unsafe { i2s::I2S::from_ptr(0x4010_7000usize as _) };
pub const SINC0: sinc::SINC = unsafe { sinc::SINC::from_ptr(0x4010_8000usize as _) };
pub const USBPHY: usbphy::USBPHY = unsafe { usbphy::USBPHY::from_ptr(0x4010_a000usize as _) };
pub const USBHS1_PHY_DCD: usbhsdcd::USBHSDCD =
    unsafe { usbhsdcd::USBHSDCD::from_ptr(0x4010_a800usize as _) };
pub const USBHS1__USBC: usbhs::USBHS = unsafe { usbhs::USBHS::from_ptr(0x4010_b000usize as _) };
pub const USBHS1__USBNC: usbnc::USBNC = unsafe { usbnc::USBNC::from_ptr(0x4010_b200usize as _) };
pub const ADC0: adc::ADC = unsafe { adc::ADC::from_ptr(0x4010_d000usize as _) };
pub const ADC1: adc::ADC = unsafe { adc::ADC::from_ptr(0x4010_e000usize as _) };
pub const DAC0: lpdac::LPDAC = unsafe { lpdac::LPDAC::from_ptr(0x4010_f000usize as _) };
pub const OPAMP0: opamp::OPAMP = unsafe { opamp::OPAMP::from_ptr(0x4011_0000usize as _) };
pub const VREF0: vref::VREF = unsafe { vref::VREF::from_ptr(0x4011_1000usize as _) };
pub const DAC1: lpdac::LPDAC = unsafe { lpdac::LPDAC::from_ptr(0x4011_2000usize as _) };
pub const OPAMP1: opamp::OPAMP = unsafe { opamp::OPAMP::from_ptr(0x4011_3000usize as _) };
pub const DAC2: hpdac::HPDAC = unsafe { hpdac::HPDAC::from_ptr(0x4011_4000usize as _) };
pub const OPAMP2: opamp::OPAMP = unsafe { opamp::OPAMP::from_ptr(0x4011_5000usize as _) };
pub const PORT0: port::PORT = unsafe { port::PORT::from_ptr(0x4011_6000usize as _) };
pub const PORT1: port::PORT = unsafe { port::PORT::from_ptr(0x4011_7000usize as _) };
pub const PORT2: port::PORT = unsafe { port::PORT::from_ptr(0x4011_8000usize as _) };
pub const PORT3: port::PORT = unsafe { port::PORT::from_ptr(0x4011_9000usize as _) };
pub const PORT4: port::PORT = unsafe { port::PORT::from_ptr(0x4011_a000usize as _) };
pub const AHBSC: ahbsc::AHBSC = unsafe { ahbsc::AHBSC::from_ptr(0x4012_0000usize as _) };
pub const AHBSC_ALIAS1: ahbsc::AHBSC = unsafe { ahbsc::AHBSC::from_ptr(0x4012_1000usize as _) };
pub const AHBSC_ALIAS2: ahbsc::AHBSC = unsafe { ahbsc::AHBSC::from_ptr(0x4012_2000usize as _) };
pub const AHBSC_ALIAS3: ahbsc::AHBSC = unsafe { ahbsc::AHBSC::from_ptr(0x4012_3000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod syscon {
    #[path = "../../../peripherals/n0/syscon.rs"]
    mod _block;
    pub use _block::*;
}
pub mod syspm {
    #[path = "../../../peripherals/n0/syspm.rs"]
    mod _block;
    pub use _block::*;
}
pub mod enet {
    #[path = "../../../peripherals/n0/enet.rs"]
    mod _block;
    pub use _block::*;
}
pub mod opamp {
    #[path = "../../../peripherals/n0/opamp.rs"]
    mod _block;
    pub use _block::*;
}
pub mod usb {
    #[path = "../../../peripherals/n0/usb.rs"]
    mod _block;
    pub use _block::*;
}
pub mod usbphy {
    #[path = "../../../peripherals/n0/usbphy.rs"]
    mod _block;
    pub use _block::*;
}
pub mod npx {
    #[path = "../../../peripherals/n0/npx.rs"]
    mod _block;
    pub use _block::*;
}
pub mod qdc {
    #[path = "../../../peripherals/n0/qdc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod usbnc {
    #[path = "../../../peripherals/n0/usbnc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod rtc {
    #[path = "../../../peripherals/n0/rtc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod plu {
    #[path = "../../../peripherals/n0/plu.rs"]
    mod _block;
    pub use _block::*;
}
pub mod smartdma {
    #[path = "../../../peripherals/n0/smartdma.rs"]
    mod _block;
    pub use _block::*;
}
pub mod cmc {
    #[path = "../../../peripherals/n0/cmc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod powerquad {
    #[path = "../../../peripherals/n0/powerquad.rs"]
    mod _block;
    pub use _block::*;
}
pub mod pint {
    #[path = "../../../peripherals/n0/pint.rs"]
    mod _block;
    pub use _block::*;
}
pub mod crc {
    #[path = "../../../peripherals/n0/crc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod fmutest {
    #[path = "../../../peripherals/n0/fmutest.rs"]
    mod _block;
    pub use _block::*;
}
pub mod puf {
    #[path = "../../../peripherals/n0/puf.rs"]
    mod _block;
    pub use _block::*;
}
pub mod erm {
    #[path = "../../../peripherals/n0/erm.rs"]
    mod _block;
    pub use _block::*;
}
pub mod intm {
    #[path = "../../../peripherals/n0/intm.rs"]
    mod _block;
    pub use _block::*;
}
pub mod gdet {
    #[path = "../../../peripherals/n0/gdet.rs"]
    mod _block;
    pub use _block::*;
}
pub mod port {
    #[path = "../../../peripherals/n0/port.rs"]
    mod _block;
    pub use _block::*;
}
pub mod dma {
    #[path = "../../../peripherals/n0/dma.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpdac {
    #[path = "../../../peripherals/n0/lpdac.rs"]
    mod _block;
    pub use _block::*;
}
pub mod utick {
    #[path = "../../../peripherals/n0/utick.rs"]
    mod _block;
    pub use _block::*;
}
pub mod vbat {
    #[path = "../../../peripherals/n0/vbat.rs"]
    mod _block;
    pub use _block::*;
}
pub mod vref {
    #[path = "../../../peripherals/n0/vref.rs"]
    mod _block;
    pub use _block::*;
}
pub mod wuu {
    #[path = "../../../peripherals/n0/wuu.rs"]
    mod _block;
    pub use _block::*;
}
pub mod pkc {
    #[path = "../../../peripherals/n0/pkc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod dm {
    #[path = "../../../peripherals/n0/dm.rs"]
    mod _block;
    pub use _block::*;
}
pub mod mrt {
    #[path = "../../../peripherals/n0/mrt.rs"]
    mod _block;
    pub use _block::*;
}
pub mod usbdcd {
    #[path = "../../../peripherals/n0/usbdcd.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpspi {
    #[path = "../../../peripherals/n0/lpspi.rs"]
    mod _block;
    pub use _block::*;
}
pub mod gpio {
    #[path = "../../../peripherals/n0/gpio.rs"]
    mod _block;
    pub use _block::*;
}
pub mod sema42 {
    #[path = "../../../peripherals/n0/sema42.rs"]
    mod _block;
    pub use _block::*;
}
pub mod hpdac {
    #[path = "../../../peripherals/n0/hpdac.rs"]
    mod _block;
    pub use _block::*;
}
pub mod scg {
    #[path = "../../../peripherals/n0/scg.rs"]
    mod _block;
    pub use _block::*;
}
pub mod freqme {
    #[path = "../../../peripherals/n0/freqme.rs"]
    mod _block;
    pub use _block::*;
}
pub mod adc {
    #[path = "../../../peripherals/n0/adc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpcmp {
    #[path = "../../../peripherals/n0/lpcmp.rs"]
    mod _block;
    pub use _block::*;
}
pub mod cache64_ctrl {
    #[path = "../../../peripherals/n0/cache64_ctrl.rs"]
    mod _block;
    pub use _block::*;
}
pub mod digtmp {
    #[path = "../../../peripherals/n0/digtmp.rs"]
    mod _block;
    pub use _block::*;
}
pub mod sct {
    #[path = "../../../peripherals/n0/sct.rs"]
    mod _block;
    pub use _block::*;
}
pub mod eim {
    #[path = "../../../peripherals/n0/eim.rs"]
    mod _block;
    pub use _block::*;
}
pub mod wwdt {
    #[path = "../../../peripherals/n0/wwdt.rs"]
    mod _block;
    pub use _block::*;
}
pub mod i3c {
    #[path = "../../../peripherals/n0/i3c.rs"]
    mod _block;
    pub use _block::*;
}
pub mod bsp32 {
    #[path = "../../../peripherals/n0/bsp32.rs"]
    mod _block;
    pub use _block::*;
}
pub mod ctimer {
    #[path = "../../../peripherals/n0/ctimer.rs"]
    mod _block;
    pub use _block::*;
}
pub mod i2s {
    #[path = "../../../peripherals/n0/i2s.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpi2c {
    #[path = "../../../peripherals/n0/lpi2c.rs"]
    mod _block;
    pub use _block::*;
}
pub mod ostimer {
    #[path = "../../../peripherals/n0/ostimer.rs"]
    mod _block;
    pub use _block::*;
}
pub mod ahbsc {
    #[path = "../../../peripherals/n0/ahbsc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod evtg {
    #[path = "../../../peripherals/n0/evtg.rs"]
    mod _block;
    pub use _block::*;
}
pub mod can {
    #[path = "../../../peripherals/n0/can.rs"]
    mod _block;
    pub use _block::*;
}
pub mod ewm {
    #[path = "../../../peripherals/n0/ewm.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lp_flexcomm {
    #[path = "../../../peripherals/n0/lp_flexcomm.rs"]
    mod _block;
    pub use _block::*;
}
pub mod s50 {
    #[path = "../../../peripherals/n0/s50.rs"]
    mod _block;
    pub use _block::*;
}
pub mod trdc {
    #[path = "../../../peripherals/n0/trdc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lptmr {
    #[path = "../../../peripherals/n0/lptmr.rs"]
    mod _block;
    pub use _block::*;
}
pub mod usbhsdcd {
    #[path = "../../../peripherals/n0/usbhsdcd.rs"]
    mod _block;
    pub use _block::*;
}
pub mod sinc {
    #[path = "../../../peripherals/n0/sinc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod flexspi {
    #[path = "../../../peripherals/n0/flexspi.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpuart {
    #[path = "../../../peripherals/n0/lpuart.rs"]
    mod _block;
    pub use _block::*;
}
pub mod mailbox {
    #[path = "../../../peripherals/n0/mailbox.rs"]
    mod _block;
    pub use _block::*;
}
pub mod otpc {
    #[path = "../../../peripherals/n0/otpc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod inputmux {
    #[path = "../../../peripherals/n0/inputmux.rs"]
    mod _block;
    pub use _block::*;
}
pub mod cache64_polsel {
    #[path = "../../../peripherals/n0/cache64_polsel.rs"]
    mod _block;
    pub use _block::*;
}
pub mod spc {
    #[path = "../../../peripherals/n0/spc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod usbhs {
    #[path = "../../../peripherals/n0/usbhs.rs"]
    mod _block;
    pub use _block::*;
}
pub mod itrc {
    #[path = "../../../peripherals/n0/itrc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod pwm {
    #[path = "../../../peripherals/n0/pwm.rs"]
    mod _block;
    pub use _block::*;
}
pub mod fmu {
    #[path = "../../../peripherals/n0/fmu.rs"]
    mod _block;
    pub use _block::*;
}
pub mod flexio {
    #[path = "../../../peripherals/n0/flexio.rs"]
    mod _block;
    pub use _block::*;
}
pub mod cdog {
    #[path = "../../../peripherals/n0/cdog.rs"]
    mod _block;
    pub use _block::*;
}
