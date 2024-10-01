#![no_main]
#![no_std]

use core::panic;

use defmt_rtt as _;
use panic_probe as _;
use stm32wlxx_hal::{self as hal};

#[hal::cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello World!\n");

    for x in 0..=10 {
        defmt::info!("Here's a number: {}", x);
        defmt::debug!("BTW {} is {}", x, if x % 2 == 0 { "even" } else { "odd" });
    }

    panic!("Oh no!");
}
