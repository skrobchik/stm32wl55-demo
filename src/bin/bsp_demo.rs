#![no_std]
#![no_main]

use defmt_rtt as _; // global logger
use nucleo_wl55jc_bsp::led::Green;
use nucleo_wl55jc_bsp::pb::Pb3;
use nucleo_wl55jc_bsp::{
    hal::{
        cortex_m,
        gpio::{PortB, PortC},
        pac,
    },
    led::Led,
    pb::PushButton,
};
use panic_probe as _;
use stm32wlxx_hal::{self as hal}; // panic handler

#[hal::cortex_m_rt::entry]
fn main() -> ! {
    let mut dp: pac::Peripherals = defmt::unwrap!(pac::Peripherals::take());

    let gpiob: PortB = PortB::split(dp.GPIOB, &mut dp.RCC);
    let gpioc: PortC = PortC::split(dp.GPIOC, &mut dp.RCC);

    let button: Pb3 = cortex_m::interrupt::free(|cs| Pb3::new(gpioc.c6, cs));

    let mut led: Green = cortex_m::interrupt::free(|cs| Green::new(gpiob.b9, cs));

    let mut prev_btn_pushed: bool = button.is_pushed();
    loop {
        let btn_pushed = button.is_pushed();
        if btn_pushed != prev_btn_pushed {
            prev_btn_pushed = btn_pushed;
            if btn_pushed {
                led.set_on();
            } else {
                led.set_off();
            }
            defmt::info!("Button {}", if btn_pushed { "pushed" } else { "released" });
        }
    }
}
