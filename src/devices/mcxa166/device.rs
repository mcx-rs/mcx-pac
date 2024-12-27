#![no_std]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (d5ec99b 2024-12-16))"]
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
pub const INPUTMUX0: inputmux::INPUTMUX =
    unsafe { inputmux::INPUTMUX::from_ptr(0x4000_1000usize as _) };
pub const CTIMER0: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4000_4000usize as _) };
pub const CTIMER1: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4000_5000usize as _) };
pub const CTIMER2: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4000_6000usize as _) };
pub const CTIMER3: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4000_7000usize as _) };
pub const CTIMER4: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4000_8000usize as _) };
pub const FREQME0: freqme::FREQME = unsafe { freqme::FREQME::from_ptr(0x4000_9000usize as _) };
pub const UTICK0: utick::UTICK = unsafe { utick::UTICK::from_ptr(0x4000_b000usize as _) };
pub const WWDT0: wwdt::WWDT = unsafe { wwdt::WWDT::from_ptr(0x4000_c000usize as _) };
pub const SMARTDMA0: smartdma::SMARTDMA =
    unsafe { smartdma::SMARTDMA::from_ptr(0x4000_e000usize as _) };
pub const DMA0: dma::DMA = unsafe { dma::DMA::from_ptr(0x4008_0000usize as _) };
pub const AOI0: aoi::AOI = unsafe { aoi::AOI::from_ptr(0x4008_9000usize as _) };
pub const CRC0: crc::CRC = unsafe { crc::CRC::from_ptr(0x4008_a000usize as _) };
pub const CMC: cmc::CMC = unsafe { cmc::CMC::from_ptr(0x4008_b000usize as _) };
pub const EIM0: eim::EIM = unsafe { eim::EIM::from_ptr(0x4008_c000usize as _) };
pub const ERM0: erm::ERM = unsafe { erm::ERM::from_ptr(0x4008_d000usize as _) };
pub const MBC0: trdc::TRDC = unsafe { trdc::TRDC::from_ptr(0x4008_e000usize as _) };
pub const SCG0: scg::SCG = unsafe { scg::SCG::from_ptr(0x4008_f000usize as _) };
pub const SPC0: spc::SPC = unsafe { spc::SPC::from_ptr(0x4009_0000usize as _) };
pub const MRCC0: mrcc::MRCC = unsafe { mrcc::MRCC::from_ptr(0x4009_1000usize as _) };
pub const SYSCON: syscon::SYSCON = unsafe { syscon::SYSCON::from_ptr(0x4009_1000usize as _) };
pub const GLIKEY0: glikey::GLIKEY = unsafe { glikey::GLIKEY::from_ptr(0x4009_1d00usize as _) };
pub const WUU0: wuu::WUU = unsafe { wuu::WUU::from_ptr(0x4009_2000usize as _) };
pub const VBAT0: vbat::VBAT = unsafe { vbat::VBAT::from_ptr(0x4009_3000usize as _) };
pub const FMC0: fmc::FMC = unsafe { fmc::FMC::from_ptr(0x4009_4000usize as _) };
pub const FMU0: fmu::FMU = unsafe { fmu::FMU::from_ptr(0x4009_5000usize as _) };
pub const AOI1: aoi::AOI = unsafe { aoi::AOI::from_ptr(0x4009_7000usize as _) };
pub const LPI2C0: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x4009_a000usize as _) };
pub const LPI2C1: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x4009_b000usize as _) };
pub const LPSPI0: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x4009_c000usize as _) };
pub const LPSPI1: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x4009_d000usize as _) };
pub const LPUART0: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x4009_f000usize as _) };
pub const LPUART1: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400a_0000usize as _) };
pub const LPUART2: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400a_1000usize as _) };
pub const LPUART3: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400a_2000usize as _) };
pub const LPUART4: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400a_3000usize as _) };
pub const EQDC0: eqdc::EQDC = unsafe { eqdc::EQDC::from_ptr(0x400a_7000usize as _) };
pub const EQDC1: eqdc::EQDC = unsafe { eqdc::EQDC::from_ptr(0x400a_8000usize as _) };
pub const FLEXPWM0: pwm::PWM = unsafe { pwm::PWM::from_ptr(0x400a_9000usize as _) };
pub const FLEXPWM1: pwm::PWM = unsafe { pwm::PWM::from_ptr(0x400a_a000usize as _) };
pub const LPTMR0: lptmr::LPTMR = unsafe { lptmr::LPTMR::from_ptr(0x400a_b000usize as _) };
pub const OSTIMER0: ostimer::OSTIMER = unsafe { ostimer::OSTIMER::from_ptr(0x400a_d000usize as _) };
pub const WAKETIMER0: waketimer::WAKETIMER =
    unsafe { waketimer::WAKETIMER::from_ptr(0x400a_e000usize as _) };
pub const ADC0: adc::ADC = unsafe { adc::ADC::from_ptr(0x400a_f000usize as _) };
pub const ADC1: adc::ADC = unsafe { adc::ADC::from_ptr(0x400b_0000usize as _) };
pub const CMP0: lpcmp::LPCMP = unsafe { lpcmp::LPCMP::from_ptr(0x400b_1000usize as _) };
pub const CMP1: lpcmp::LPCMP = unsafe { lpcmp::LPCMP::from_ptr(0x400b_2000usize as _) };
pub const CMP2: lpcmp::LPCMP = unsafe { lpcmp::LPCMP::from_ptr(0x400b_3000usize as _) };
pub const DAC0: lpdac::LPDAC = unsafe { lpdac::LPDAC::from_ptr(0x400b_4000usize as _) };
pub const OPAMP0: opamp::OPAMP = unsafe { opamp::OPAMP::from_ptr(0x400b_7000usize as _) };
pub const OPAMP1: opamp::OPAMP = unsafe { opamp::OPAMP::from_ptr(0x400b_8000usize as _) };
pub const OPAMP2: opamp::OPAMP = unsafe { opamp::OPAMP::from_ptr(0x400b_9000usize as _) };
pub const OPAMP3: opamp::OPAMP = unsafe { opamp::OPAMP::from_ptr(0x400b_a000usize as _) };
pub const PORT0: port::PORT = unsafe { port::PORT::from_ptr(0x400b_c000usize as _) };
pub const PORT1: port::PORT = unsafe { port::PORT::from_ptr(0x400b_d000usize as _) };
pub const PORT2: port::PORT = unsafe { port::PORT::from_ptr(0x400b_e000usize as _) };
pub const PORT3: port::PORT = unsafe { port::PORT::from_ptr(0x400b_f000usize as _) };
pub const PORT4: port::PORT = unsafe { port::PORT::from_ptr(0x400c_0000usize as _) };
pub const CAN0: can::CAN = unsafe { can::CAN::from_ptr(0x400c_c000usize as _) };
pub const LPI2C2: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x400d_4000usize as _) };
pub const LPI2C3: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x400d_5000usize as _) };
pub const LPUART5: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400d_a000usize as _) };
pub const UDF0: udf::UDF = unsafe { udf::UDF::from_ptr(0x400e_d000usize as _) };
pub const RTC0: rtc::RTC = unsafe { rtc::RTC::from_ptr(0x400e_e000usize as _) };
pub const ADC2: adc::ADC = unsafe { adc::ADC::from_ptr(0x400f_0000usize as _) };
pub const ADC3: adc::ADC = unsafe { adc::ADC::from_ptr(0x400f_1000usize as _) };
pub const CDOG0: cdog::CDOG = unsafe { cdog::CDOG::from_ptr(0x4010_0000usize as _) };
pub const DBGMAILBOX: debugmailbox::DEBUGMAILBOX =
    unsafe { debugmailbox::DEBUGMAILBOX::from_ptr(0x4010_1000usize as _) };
pub const GPIO0: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4010_2000usize as _) };
pub const GPIO1: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4010_3000usize as _) };
pub const GPIO2: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4010_4000usize as _) };
pub const GPIO3: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4010_5000usize as _) };
pub const GPIO4: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4010_6000usize as _) };
pub const MAU0: mau::MAU = unsafe { mau::MAU::from_ptr(0x4010_8000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod wuu {
    #[path = "../../../peripherals/a2/wuu.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpspi {
    #[path = "../../../peripherals/a2/lpspi.rs"]
    mod _block;
    pub use _block::*;
}
pub mod opamp {
    #[path = "../../../peripherals/a2/opamp.rs"]
    mod _block;
    pub use _block::*;
}
pub mod fmu {
    #[path = "../../../peripherals/a2/fmu.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpi2c {
    #[path = "../../../peripherals/a2/lpi2c.rs"]
    mod _block;
    pub use _block::*;
}
pub mod ostimer {
    #[path = "../../../peripherals/a2/ostimer.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpcmp {
    #[path = "../../../peripherals/a2/lpcmp.rs"]
    mod _block;
    pub use _block::*;
}
pub mod vbat {
    #[path = "../../../peripherals/a2/vbat.rs"]
    mod _block;
    pub use _block::*;
}
pub mod waketimer {
    #[path = "../../../peripherals/a2/waketimer.rs"]
    mod _block;
    pub use _block::*;
}
pub mod syscon {
    #[path = "../../../peripherals/a2/syscon.rs"]
    mod _block;
    pub use _block::*;
}
pub mod erm {
    #[path = "../../../peripherals/a2/erm.rs"]
    mod _block;
    pub use _block::*;
}
pub mod mau {
    #[path = "../../../peripherals/a2/mau.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpuart {
    #[path = "../../../peripherals/a2/lpuart.rs"]
    mod _block;
    pub use _block::*;
}
pub mod wwdt {
    #[path = "../../../peripherals/a2/wwdt.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpdac {
    #[path = "../../../peripherals/a2/lpdac.rs"]
    mod _block;
    pub use _block::*;
}
pub mod fmc {
    #[path = "../../../peripherals/a2/fmc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod ctimer {
    #[path = "../../../peripherals/a2/ctimer.rs"]
    mod _block;
    pub use _block::*;
}
pub mod adc {
    #[path = "../../../peripherals/a2/adc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod udf {
    #[path = "../../../peripherals/a2/udf.rs"]
    mod _block;
    pub use _block::*;
}
pub mod dma {
    #[path = "../../../peripherals/a2/dma.rs"]
    mod _block;
    pub use _block::*;
}
pub mod trdc {
    #[path = "../../../peripherals/a2/trdc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod glikey {
    #[path = "../../../peripherals/a2/glikey.rs"]
    mod _block;
    pub use _block::*;
}
pub mod aoi {
    #[path = "../../../peripherals/a2/aoi.rs"]
    mod _block;
    pub use _block::*;
}
pub mod gpio {
    #[path = "../../../peripherals/a2/gpio.rs"]
    mod _block;
    pub use _block::*;
}
pub mod cmc {
    #[path = "../../../peripherals/a2/cmc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod cdog {
    #[path = "../../../peripherals/a2/cdog.rs"]
    mod _block;
    pub use _block::*;
}
pub mod utick {
    #[path = "../../../peripherals/a2/utick.rs"]
    mod _block;
    pub use _block::*;
}
pub mod eqdc {
    #[path = "../../../peripherals/a2/eqdc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod freqme {
    #[path = "../../../peripherals/a2/freqme.rs"]
    mod _block;
    pub use _block::*;
}
pub mod spc {
    #[path = "../../../peripherals/a2/spc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod inputmux {
    #[path = "../../../peripherals/a2/inputmux.rs"]
    mod _block;
    pub use _block::*;
}
pub mod crc {
    #[path = "../../../peripherals/a2/crc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod can {
    #[path = "../../../peripherals/a2/can.rs"]
    mod _block;
    pub use _block::*;
}
pub mod eim {
    #[path = "../../../peripherals/a2/eim.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lptmr {
    #[path = "../../../peripherals/a2/lptmr.rs"]
    mod _block;
    pub use _block::*;
}
pub mod rtc {
    #[path = "../../../peripherals/a2/rtc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod smartdma {
    #[path = "../../../peripherals/a2/smartdma.rs"]
    mod _block;
    pub use _block::*;
}
pub mod debugmailbox {
    #[path = "../../../peripherals/a2/debugmailbox.rs"]
    mod _block;
    pub use _block::*;
}
pub mod scg {
    #[path = "../../../peripherals/a2/scg.rs"]
    mod _block;
    pub use _block::*;
}
pub mod port {
    #[path = "../../../peripherals/a2/port.rs"]
    mod _block;
    pub use _block::*;
}
pub mod pwm {
    #[path = "../../../peripherals/a2/pwm.rs"]
    mod _block;
    pub use _block::*;
}
pub mod mrcc {
    #[path = "../../../peripherals/a2/mrcc.rs"]
    mod _block;
    pub use _block::*;
}
