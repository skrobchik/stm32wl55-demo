#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _; // global logger
use panic_probe as _; // panic handler
use stm32wlxx_hal::{
    cortex_m,
    gpio::{pins, Input, PinState, PortC, Pull},
    pac,
};

#[entry]
fn main() -> ! {
    let mut dp: pac::Peripherals = defmt::unwrap!(pac::Peripherals::take());

    let gpioc: PortC = PortC::split(dp.GPIOC, &mut dp.RCC);
    let button_1: Input<pins::C6> =
        cortex_m::interrupt::free(|cs| Input::new(gpioc.c6, Pull::Up, cs));

    let mut prev_level: PinState = button_1.level();
    loop {
        let level: PinState = button_1.level();
        if level != prev_level {
            prev_level = level;
            defmt::info!(
                "Button {}",
                match level {
                    PinState::Low => "down",
                    PinState::High => "up",
                }
            );
        }
    }
}
