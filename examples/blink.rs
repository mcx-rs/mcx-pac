#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::delay::Delay;
use mcx_pac as pac;

#[cortex_m_rt::entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();

    pac::SYSCON0.ahbclkctrl0().modify(|w| {
        w.set_gpio0(true);
        w.set_port0(true);
    });
    pac::GPIO0.pddr().modify(|w| {
        w.set_pdd(10, true);
    });

    let mut delay = Delay::new(cp.SYST, 40_000_000u32);
    loop {
        delay.delay_ms(1000u32);
        pac::GPIO0.ptor().modify(|w| {
            w.set_ptto(10, true);
        });
    }
}
