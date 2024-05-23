#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use mcx_pac::device;
use ral_registers::{modify_reg, write_reg};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();

    let syscon = unsafe { device::syscon::SYSCON0::instance() };
    let gpio0 = unsafe { device::gpio::GPIO0::instance() };
    modify_reg!(device::syscon, syscon, AHBCLKCTRL0, GPIO0: device::syscon::AHBCLKCTRL0::GPIO0::RW::ENABLE,
                                                     PORT0: device::syscon::AHBCLKCTRL0::PORT0::RW::ENABLE);
    modify_reg!(device::gpio, gpio0, PDDR, PDD10: device::gpio::PDDR::PDD10::RW::PDD1);

    let mut delay = Delay::new(cp.SYST, 48_000_000u32);
    loop {
        delay.delay_ms(1000u32);
        write_reg!(device::gpio, gpio0, PTOR, PTTO10: device::gpio::PTOR::PTTO10::RW::PTTO1);
    }
}
