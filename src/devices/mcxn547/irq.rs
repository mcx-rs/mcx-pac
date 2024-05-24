#[repr(C)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

#[cfg(feature = "rt")]
extern "C" {

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

    fn USB1_HS_PHY();

    fn USB1_HS();

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

    fn GDET();

    fn VBAT0();

    fn EWM0();

    fn TSI_END_OF_SCAN();

    fn TSI_OUT_OF_SCAN();

    fn EMVSIM0();

    fn EMVSIM1();

    fn FLEXIO();

    fn DAC0();

    fn HSCMP0();

    fn HSCMP1();

    fn FLEXPWM0_RELOAD_ERROR();

    fn FLEXPWM0_FAULT();

    fn FLEXPWM0_SUBMODULE0();

    fn FLEXPWM0_SUBMODULE1();

    fn FLEXPWM0_SUBMODULE2();

    fn FLEXPWM0_SUBMODULE3();

    fn ENC0_COMPARE();

    fn ENC0_HOME();

    fn ENC0_WDG_SAB();

    fn ENC0_IDX();

    fn ENC1_COMPARE();

    fn ENC1_HOME();

    fn ENC1_WDG_SAB();

    fn ENC1_IDX();

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

    fn LPTMR0();

    fn LPTMR1();

    fn SCG();

    fn SPC();

    fn WUU();

    fn PORT_EFT();

    fn ETB0();

    fn SM3();

    fn TRNG0();

    fn WWDT0();

    fn WWDT1();

    fn CMC0();

    fn CTI0();

}

#[cfg(feature = "rt")]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 156] = [
    Vector { _reserved: 0 },
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
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: USB1_HS_PHY,
    },
    Vector { _handler: USB1_HS },
    Vector { _reserved: 0 },
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
    Vector { _reserved: 0 },
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
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: ENC0_COMPARE,
    },
    Vector {
        _handler: ENC0_HOME,
    },
    Vector {
        _handler: ENC0_WDG_SAB,
    },
    Vector { _handler: ENC0_IDX },
    Vector {
        _handler: ENC1_COMPARE,
    },
    Vector {
        _handler: ENC1_HOME,
    },
    Vector {
        _handler: ENC1_WDG_SAB,
    },
    Vector { _handler: ENC1_IDX },
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
    Vector { _reserved: 0 },
    Vector { _handler: LPTMR0 },
    Vector { _handler: LPTMR1 },
    Vector { _handler: SCG },
    Vector { _handler: SPC },
    Vector { _handler: WUU },
    Vector { _handler: PORT_EFT },
    Vector { _handler: ETB0 },
    Vector { _handler: SM3 },
    Vector { _handler: TRNG0 },
    Vector { _handler: WWDT0 },
    Vector { _handler: WWDT1 },
    Vector { _handler: CMC0 },
    Vector { _handler: CTI0 },
];

#[doc = r"Enumeration of all the interrupts."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "1 - EDMA_0_CH0"]
    EDMA_0_CH0 = 1,

    #[doc = "2 - EDMA_0_CH1"]
    EDMA_0_CH1 = 2,

    #[doc = "3 - EDMA_0_CH2"]
    EDMA_0_CH2 = 3,

    #[doc = "4 - EDMA_0_CH3"]
    EDMA_0_CH3 = 4,

    #[doc = "5 - EDMA_0_CH4"]
    EDMA_0_CH4 = 5,

    #[doc = "6 - EDMA_0_CH5"]
    EDMA_0_CH5 = 6,

    #[doc = "7 - EDMA_0_CH6"]
    EDMA_0_CH6 = 7,

    #[doc = "8 - EDMA_0_CH7"]
    EDMA_0_CH7 = 8,

    #[doc = "9 - EDMA_0_CH8"]
    EDMA_0_CH8 = 9,

    #[doc = "10 - EDMA_0_CH9"]
    EDMA_0_CH9 = 10,

    #[doc = "11 - EDMA_0_CH10"]
    EDMA_0_CH10 = 11,

    #[doc = "12 - EDMA_0_CH11"]
    EDMA_0_CH11 = 12,

    #[doc = "13 - EDMA_0_CH12"]
    EDMA_0_CH12 = 13,

    #[doc = "14 - EDMA_0_CH13"]
    EDMA_0_CH13 = 14,

    #[doc = "15 - EDMA_0_CH14"]
    EDMA_0_CH14 = 15,

    #[doc = "16 - EDMA_0_CH15"]
    EDMA_0_CH15 = 16,

    #[doc = "17 - GPIO00"]
    GPIO00 = 17,

    #[doc = "18 - GPIO01"]
    GPIO01 = 18,

    #[doc = "19 - GPIO10"]
    GPIO10 = 19,

    #[doc = "20 - GPIO11"]
    GPIO11 = 20,

    #[doc = "21 - GPIO20"]
    GPIO20 = 21,

    #[doc = "22 - GPIO21"]
    GPIO21 = 22,

    #[doc = "23 - GPIO30"]
    GPIO30 = 23,

    #[doc = "24 - GPIO31"]
    GPIO31 = 24,

    #[doc = "25 - GPIO40"]
    GPIO40 = 25,

    #[doc = "26 - GPIO41"]
    GPIO41 = 26,

    #[doc = "27 - GPIO50"]
    GPIO50 = 27,

    #[doc = "28 - GPIO51"]
    GPIO51 = 28,

    #[doc = "29 - UTICK0"]
    UTICK0 = 29,

    #[doc = "30 - MRT0"]
    MRT0 = 30,

    #[doc = "31 - CTIMER0"]
    CTIMER0 = 31,

    #[doc = "32 - CTIMER1"]
    CTIMER1 = 32,

    #[doc = "33 - SCT0"]
    SCT0 = 33,

    #[doc = "34 - CTIMER2"]
    CTIMER2 = 34,

    #[doc = "35 - LP_FLEXCOMM0"]
    LP_FLEXCOMM0 = 35,

    #[doc = "36 - LP_FLEXCOMM1"]
    LP_FLEXCOMM1 = 36,

    #[doc = "37 - LP_FLEXCOMM2"]
    LP_FLEXCOMM2 = 37,

    #[doc = "38 - LP_FLEXCOMM3"]
    LP_FLEXCOMM3 = 38,

    #[doc = "39 - LP_FLEXCOMM4"]
    LP_FLEXCOMM4 = 39,

    #[doc = "40 - LP_FLEXCOMM5"]
    LP_FLEXCOMM5 = 40,

    #[doc = "41 - LP_FLEXCOMM6"]
    LP_FLEXCOMM6 = 41,

    #[doc = "42 - LP_FLEXCOMM7"]
    LP_FLEXCOMM7 = 42,

    #[doc = "43 - LP_FLEXCOMM8"]
    LP_FLEXCOMM8 = 43,

    #[doc = "44 - LP_FLEXCOMM9"]
    LP_FLEXCOMM9 = 44,

    #[doc = "45 - ADC0"]
    ADC0 = 45,

    #[doc = "46 - ADC1"]
    ADC1 = 46,

    #[doc = "47 - PINT0"]
    PINT0 = 47,

    #[doc = "48 - PDM_EVENT"]
    PDM_EVENT = 48,

    #[doc = "50 - USB0_FS"]
    USB0_FS = 50,

    #[doc = "51 - USB0_DCD"]
    USB0_DCD = 51,

    #[doc = "52 - RTC"]
    RTC = 52,

    #[doc = "53 - SMARTDMA"]
    SMARTDMA = 53,

    #[doc = "54 - MAILBOX"]
    MAILBOX = 54,

    #[doc = "55 - CTIMER3"]
    CTIMER3 = 55,

    #[doc = "56 - CTIMER4"]
    CTIMER4 = 56,

    #[doc = "57 - OS_EVENT"]
    OS_EVENT = 57,

    #[doc = "58 - FLEXSPI0"]
    FLEXSPI0 = 58,

    #[doc = "59 - SAI0"]
    SAI0 = 59,

    #[doc = "60 - SAI1"]
    SAI1 = 60,

    #[doc = "61 - USDHC0"]
    USDHC0 = 61,

    #[doc = "62 - CAN0"]
    CAN0 = 62,

    #[doc = "66 - USB1_HS_PHY"]
    USB1_HS_PHY = 66,

    #[doc = "67 - USB1_HS"]
    USB1_HS = 67,

    #[doc = "70 - PLU"]
    PLU = 70,

    #[doc = "71 - FREQME"]
    FREQME = 71,

    #[doc = "72 - SEC_VIO"]
    SEC_VIO = 72,

    #[doc = "73 - ELS"]
    ELS = 73,

    #[doc = "74 - PKC"]
    PKC = 74,

    #[doc = "75 - PUF"]
    PUF = 75,

    #[doc = "76 - PQ"]
    PQ = 76,

    #[doc = "77 - EDMA_1_CH0"]
    EDMA_1_CH0 = 77,

    #[doc = "78 - EDMA_1_CH1"]
    EDMA_1_CH1 = 78,

    #[doc = "79 - EDMA_1_CH2"]
    EDMA_1_CH2 = 79,

    #[doc = "80 - EDMA_1_CH3"]
    EDMA_1_CH3 = 80,

    #[doc = "81 - EDMA_1_CH4"]
    EDMA_1_CH4 = 81,

    #[doc = "82 - EDMA_1_CH5"]
    EDMA_1_CH5 = 82,

    #[doc = "83 - EDMA_1_CH6"]
    EDMA_1_CH6 = 83,

    #[doc = "84 - EDMA_1_CH7"]
    EDMA_1_CH7 = 84,

    #[doc = "85 - EDMA_1_CH8"]
    EDMA_1_CH8 = 85,

    #[doc = "86 - EDMA_1_CH9"]
    EDMA_1_CH9 = 86,

    #[doc = "87 - EDMA_1_CH10"]
    EDMA_1_CH10 = 87,

    #[doc = "88 - EDMA_1_CH11"]
    EDMA_1_CH11 = 88,

    #[doc = "89 - EDMA_1_CH12"]
    EDMA_1_CH12 = 89,

    #[doc = "90 - EDMA_1_CH13"]
    EDMA_1_CH13 = 90,

    #[doc = "91 - EDMA_1_CH14"]
    EDMA_1_CH14 = 91,

    #[doc = "92 - EDMA_1_CH15"]
    EDMA_1_CH15 = 92,

    #[doc = "93 - CDOG0"]
    CDOG0 = 93,

    #[doc = "94 - CDOG1"]
    CDOG1 = 94,

    #[doc = "95 - I3C0"]
    I3C0 = 95,

    #[doc = "96 - I3C1"]
    I3C1 = 96,

    #[doc = "98 - GDET"]
    GDET = 98,

    #[doc = "99 - VBAT0"]
    VBAT0 = 99,

    #[doc = "100 - EWM0"]
    EWM0 = 100,

    #[doc = "101 - TSI_END_OF_SCAN"]
    TSI_END_OF_SCAN = 101,

    #[doc = "102 - TSI_OUT_OF_SCAN"]
    TSI_OUT_OF_SCAN = 102,

    #[doc = "103 - EMVSIM0"]
    EMVSIM0 = 103,

    #[doc = "104 - EMVSIM1"]
    EMVSIM1 = 104,

    #[doc = "105 - FLEXIO"]
    FLEXIO = 105,

    #[doc = "106 - DAC0"]
    DAC0 = 106,

    #[doc = "109 - HSCMP0"]
    HSCMP0 = 109,

    #[doc = "110 - HSCMP1"]
    HSCMP1 = 110,

    #[doc = "112 - FLEXPWM0_RELOAD_ERROR"]
    FLEXPWM0_RELOAD_ERROR = 112,

    #[doc = "113 - FLEXPWM0_FAULT"]
    FLEXPWM0_FAULT = 113,

    #[doc = "114 - FLEXPWM0_SUBMODULE0"]
    FLEXPWM0_SUBMODULE0 = 114,

    #[doc = "115 - FLEXPWM0_SUBMODULE1"]
    FLEXPWM0_SUBMODULE1 = 115,

    #[doc = "116 - FLEXPWM0_SUBMODULE2"]
    FLEXPWM0_SUBMODULE2 = 116,

    #[doc = "117 - FLEXPWM0_SUBMODULE3"]
    FLEXPWM0_SUBMODULE3 = 117,

    #[doc = "124 - ENC0_COMPARE"]
    ENC0_COMPARE = 124,

    #[doc = "125 - ENC0_HOME"]
    ENC0_HOME = 125,

    #[doc = "126 - ENC0_WDG_SAB"]
    ENC0_WDG_SAB = 126,

    #[doc = "127 - ENC0_IDX"]
    ENC0_IDX = 127,

    #[doc = "128 - ENC1_COMPARE"]
    ENC1_COMPARE = 128,

    #[doc = "129 - ENC1_HOME"]
    ENC1_HOME = 129,

    #[doc = "130 - ENC1_WDG_SAB"]
    ENC1_WDG_SAB = 130,

    #[doc = "131 - ENC1_IDX"]
    ENC1_IDX = 131,

    #[doc = "132 - ITRC0"]
    ITRC0 = 132,

    #[doc = "133 - BSP32"]
    BSP32 = 133,

    #[doc = "134 - ELS_ERR"]
    ELS_ERR = 134,

    #[doc = "135 - PKC_ERR"]
    PKC_ERR = 135,

    #[doc = "136 - ERM_SINGLE_BIT_ERROR"]
    ERM_SINGLE_BIT_ERROR = 136,

    #[doc = "137 - ERM_MULTI_BIT_ERROR"]
    ERM_MULTI_BIT_ERROR = 137,

    #[doc = "138 - FMU0"]
    FMU0 = 138,

    #[doc = "139 - ETHERNET"]
    ETHERNET = 139,

    #[doc = "140 - ETHERNET_PMT"]
    ETHERNET_PMT = 140,

    #[doc = "141 - ETHERNET_MACLP"]
    ETHERNET_MACLP = 141,

    #[doc = "143 - LPTMR0"]
    LPTMR0 = 143,

    #[doc = "144 - LPTMR1"]
    LPTMR1 = 144,

    #[doc = "145 - SCG"]
    SCG = 145,

    #[doc = "146 - SPC"]
    SPC = 146,

    #[doc = "147 - WUU"]
    WUU = 147,

    #[doc = "148 - PORT_EFT"]
    PORT_EFT = 148,

    #[doc = "149 - ETB0"]
    ETB0 = 149,

    #[doc = "150 - SM3"]
    SM3 = 150,

    #[doc = "151 - TRNG0"]
    TRNG0 = 151,

    #[doc = "152 - WWDT0"]
    WWDT0 = 152,

    #[doc = "153 - WWDT1"]
    WWDT1 = 153,

    #[doc = "154 - CMC0"]
    CMC0 = 154,

    #[doc = "155 - CTI0"]
    CTI0 = 155,
}

unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
