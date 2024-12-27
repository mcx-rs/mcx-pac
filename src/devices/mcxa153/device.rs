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
    #[doc = "18 - Wake Up Unit interrupt"]
    WUU0 = 18,
    #[doc = "24 - Improved Inter Integrated Circuit interrupt 0"]
    I3C0 = 24,
    #[doc = "26 - Low-Power Inter Integrated Circuit interrupt"]
    LPI2C0 = 26,
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
        fn ERM0_SINGLE_BIT();
        fn ERM0_MULTI_BIT();
        fn FMU0();
        fn GLIKEY0();
        fn MBC0();
        fn SCG0();
        fn SPC0();
        fn WUU0();
        fn I3C0();
        fn LPI2C0();
        fn LPSPI0();
        fn LPSPI1();
        fn LPUART0();
        fn LPUART1();
        fn LPUART2();
        fn USB0();
        fn CDOG0();
        fn CTIMER0();
        fn CTIMER1();
        fn CTIMER2();
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
        fn CMP0();
        fn CMP1();
        fn GPIO0();
        fn GPIO1();
        fn GPIO2();
        fn GPIO3();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __INTERRUPTS: [Vector; 75] = [
        Vector { _reserved: 0 },
        Vector { _handler: CMC },
        Vector { _handler: DMA_CH0 },
        Vector { _handler: DMA_CH1 },
        Vector { _handler: DMA_CH2 },
        Vector { _handler: DMA_CH3 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
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
        Vector { _reserved: 0 },
        Vector { _handler: WUU0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: I3C0 },
        Vector { _reserved: 0 },
        Vector { _handler: LPI2C0 },
        Vector { _reserved: 0 },
        Vector { _handler: LPSPI0 },
        Vector { _handler: LPSPI1 },
        Vector { _reserved: 0 },
        Vector { _handler: LPUART0 },
        Vector { _handler: LPUART1 },
        Vector { _handler: LPUART2 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: USB0 },
        Vector { _reserved: 0 },
        Vector { _handler: CDOG0 },
        Vector { _handler: CTIMER0 },
        Vector { _handler: CTIMER1 },
        Vector { _handler: CTIMER2 },
        Vector { _reserved: 0 },
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
        Vector { _reserved: 0 },
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
    ];
}
pub const INPUTMUX0: inputmux::INPUTMUX =
    unsafe { inputmux::INPUTMUX::from_ptr(0x4000_1000usize as _) };
pub const I3C0: i3c::I3C = unsafe { i3c::I3C::from_ptr(0x4000_2000usize as _) };
pub const CTIMER0: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4000_4000usize as _) };
pub const CTIMER1: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4000_5000usize as _) };
pub const CTIMER2: ctimer::CTIMER = unsafe { ctimer::CTIMER::from_ptr(0x4000_6000usize as _) };
pub const FREQME0: freqme::FREQME = unsafe { freqme::FREQME::from_ptr(0x4000_9000usize as _) };
pub const UTICK0: utick::UTICK = unsafe { utick::UTICK::from_ptr(0x4000_b000usize as _) };
pub const WWDT0: wwdt::WWDT = unsafe { wwdt::WWDT::from_ptr(0x4000_c000usize as _) };
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
pub const FMU0TEST: fmutest::FMUTEST = unsafe { fmutest::FMUTEST::from_ptr(0x4009_6000usize as _) };
pub const LPI2C0: lpi2c::LPI2C = unsafe { lpi2c::LPI2C::from_ptr(0x4009_a000usize as _) };
pub const LPSPI0: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x4009_c000usize as _) };
pub const LPSPI1: lpspi::LPSPI = unsafe { lpspi::LPSPI::from_ptr(0x4009_d000usize as _) };
pub const LPUART0: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x4009_f000usize as _) };
pub const LPUART1: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400a_0000usize as _) };
pub const LPUART2: lpuart::LPUART = unsafe { lpuart::LPUART::from_ptr(0x400a_1000usize as _) };
pub const USB0: usb::USB = unsafe { usb::USB::from_ptr(0x400a_4000usize as _) };
pub const QDC0: eqdc::EQDC = unsafe { eqdc::EQDC::from_ptr(0x400a_7000usize as _) };
pub const FLEXPWM0: pwm::PWM = unsafe { pwm::PWM::from_ptr(0x400a_9000usize as _) };
pub const LPTMR0: lptmr::LPTMR = unsafe { lptmr::LPTMR::from_ptr(0x400a_b000usize as _) };
pub const OSTIMER0: ostimer::OSTIMER = unsafe { ostimer::OSTIMER::from_ptr(0x400a_d000usize as _) };
pub const WAKETIMER0: waketimer::WAKETIMER =
    unsafe { waketimer::WAKETIMER::from_ptr(0x400a_e000usize as _) };
pub const ADC0: adc::ADC = unsafe { adc::ADC::from_ptr(0x400a_f000usize as _) };
pub const CMP0: lpcmp::LPCMP = unsafe { lpcmp::LPCMP::from_ptr(0x400b_1000usize as _) };
pub const CMP1: lpcmp::LPCMP = unsafe { lpcmp::LPCMP::from_ptr(0x400b_2000usize as _) };
pub const PORT0: port::PORT = unsafe { port::PORT::from_ptr(0x400b_c000usize as _) };
pub const PORT1: port::PORT = unsafe { port::PORT::from_ptr(0x400b_d000usize as _) };
pub const PORT2: port::PORT = unsafe { port::PORT::from_ptr(0x400b_e000usize as _) };
pub const PORT3: port::PORT = unsafe { port::PORT::from_ptr(0x400b_f000usize as _) };
pub const CDOG: cdog::CDOG = unsafe { cdog::CDOG::from_ptr(0x4010_0000usize as _) };
pub const DBGMAILBOX: debugmailbox::DEBUGMAILBOX =
    unsafe { debugmailbox::DEBUGMAILBOX::from_ptr(0x4010_1000usize as _) };
pub const GPIO0: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4010_2000usize as _) };
pub const GPIO1: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4010_3000usize as _) };
pub const GPIO2: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4010_4000usize as _) };
pub const GPIO3: gpio::GPIO = unsafe { gpio::GPIO::from_ptr(0x4010_5000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod port {
    #[path = "../../../peripherals/a0/port.rs"]
    mod _block;
    pub use _block::*;
}
pub mod trdc {
    #[path = "../../../peripherals/a0/trdc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod syscon {
    #[path = "../../../peripherals/a0/syscon.rs"]
    mod _block;
    pub use _block::*;
}
pub mod gpio {
    #[path = "../../../peripherals/a0/gpio.rs"]
    mod _block;
    pub use _block::*;
}
pub mod wwdt {
    #[path = "../../../peripherals/a0/wwdt.rs"]
    mod _block;
    pub use _block::*;
}
pub mod debugmailbox {
    #[path = "../../../peripherals/a0/debugmailbox.rs"]
    mod _block;
    pub use _block::*;
}
pub mod crc {
    #[path = "../../../peripherals/a0/crc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod fmu {
    #[path = "../../../peripherals/a0/fmu.rs"]
    mod _block;
    pub use _block::*;
}
pub mod inputmux {
    #[path = "../../../peripherals/a0/inputmux.rs"]
    mod _block;
    pub use _block::*;
}
pub mod i3c {
    #[path = "../../../peripherals/a0/i3c.rs"]
    mod _block;
    pub use _block::*;
}
pub mod pwm {
    #[path = "../../../peripherals/a0/pwm.rs"]
    mod _block;
    pub use _block::*;
}
pub mod dma {
    #[path = "../../../peripherals/a0/dma.rs"]
    mod _block;
    pub use _block::*;
}
pub mod ctimer {
    #[path = "../../../peripherals/a0/ctimer.rs"]
    mod _block;
    pub use _block::*;
}
pub mod vbat {
    #[path = "../../../peripherals/a0/vbat.rs"]
    mod _block;
    pub use _block::*;
}
pub mod cdog {
    #[path = "../../../peripherals/a0/cdog.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpcmp {
    #[path = "../../../peripherals/a0/lpcmp.rs"]
    mod _block;
    pub use _block::*;
}
pub mod usb {
    #[path = "../../../peripherals/a0/usb.rs"]
    mod _block;
    pub use _block::*;
}
pub mod waketimer {
    #[path = "../../../peripherals/a0/waketimer.rs"]
    mod _block;
    pub use _block::*;
}
pub mod fmc {
    #[path = "../../../peripherals/a0/fmc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpuart {
    #[path = "../../../peripherals/a0/lpuart.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpspi {
    #[path = "../../../peripherals/a0/lpspi.rs"]
    mod _block;
    pub use _block::*;
}
pub mod utick {
    #[path = "../../../peripherals/a0/utick.rs"]
    mod _block;
    pub use _block::*;
}
pub mod mrcc {
    #[path = "../../../peripherals/a0/mrcc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod scg {
    #[path = "../../../peripherals/a0/scg.rs"]
    mod _block;
    pub use _block::*;
}
pub mod aoi {
    #[path = "../../../peripherals/a0/aoi.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lpi2c {
    #[path = "../../../peripherals/a0/lpi2c.rs"]
    mod _block;
    pub use _block::*;
}
pub mod adc {
    #[path = "../../../peripherals/a0/adc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod spc {
    #[path = "../../../peripherals/a0/spc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod fmutest {
    #[path = "../../../peripherals/a0/fmutest.rs"]
    mod _block;
    pub use _block::*;
}
pub mod lptmr {
    #[path = "../../../peripherals/a0/lptmr.rs"]
    mod _block;
    pub use _block::*;
}
pub mod eim {
    #[path = "../../../peripherals/a0/eim.rs"]
    mod _block;
    pub use _block::*;
}
pub mod erm {
    #[path = "../../../peripherals/a0/erm.rs"]
    mod _block;
    pub use _block::*;
}
pub mod cmc {
    #[path = "../../../peripherals/a0/cmc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod eqdc {
    #[path = "../../../peripherals/a0/eqdc.rs"]
    mod _block;
    pub use _block::*;
}
pub mod wuu {
    #[path = "../../../peripherals/a0/wuu.rs"]
    mod _block;
    pub use _block::*;
}
pub mod ostimer {
    #[path = "../../../peripherals/a0/ostimer.rs"]
    mod _block;
    pub use _block::*;
}
pub mod glikey {
    #[path = "../../../peripherals/a0/glikey.rs"]
    mod _block;
    pub use _block::*;
}
pub mod freqme {
    #[path = "../../../peripherals/a0/freqme.rs"]
    mod _block;
    pub use _block::*;
}
