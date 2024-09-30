#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _; // global logger
use panic_probe as _; // panic handler
use stm32wlxx_hal::{
    cortex_m,
    gpio::{pins, Input, Output, OutputArgs, PinState, PortB, PortC, Pull},
    pac,
};

#[entry]
fn main() -> ! {
    let mut dp: pac::Peripherals = defmt::unwrap!(pac::Peripherals::take());

    let gpioc: PortC = PortC::split(dp.GPIOC, &mut dp.RCC);
    let button: Input<pins::C6> =
        cortex_m::interrupt::free(|cs| Input::new(gpioc.c6, Pull::Up, cs));

    let gpiob: PortB = PortB::split(dp.GPIOB, &mut dp.RCC);
    let mut led: Output<pins::B9> = cortex_m::interrupt::free(|cs| {
        Output::new(
            gpiob.b9,
            &OutputArgs::default(),
            cs,
        )
    });

    let mut prev_level: PinState = button.level();
    loop {
        let level: PinState = button.level();
        if level != prev_level {
            prev_level = level;
            led.set_level(!level);
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
