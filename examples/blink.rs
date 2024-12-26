#![no_std]
#![no_main]

extern crate mcx_pac;
extern crate panic_halt;

use cortex_m::{self, delay::Delay};
use cortex_m_rt::entry;
use mcx_pac as pac;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut delay = Delay::new(cp.SYST, 48_000_000);

    pac::MRCC0.MRCC_GLB_RST1().modify(|r| r.set_PORT3(true));
    pac::MRCC0.MRCC_GLB_ACC1().modify(|r| r.set_PORT3(true));
    pac::MRCC0.MRCC_GLB_CC1().modify(|r| r.set_PORT3(true));

    pac::MRCC0.MRCC_GLB_RST2().modify(|r| r.set_GPIO3(true));
    pac::MRCC0.MRCC_GLB_ACC2().modify(|r| r.set_GPIO3(true));
    pac::MRCC0.MRCC_GLB_CC2().modify(|r| r.set_GPIO3(true));

    pac::GPIO3.PDDR().modify(|r| r.set_PDD18(true));

    // configure LED RED
    pac::PORT3.PCR(18).modify(|r| {
        r.set_IBE(true);
        r.set_MUX(0);
    });

    loop {
        delay.delay_ms(1000);
        pac::GPIO3.PTOR().write(|r| r.set_PTTO18(true));
    }
}
