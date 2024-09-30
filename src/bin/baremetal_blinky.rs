#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

const RCC_AHB2ENR: *mut u32 = (0x5800_0000 + 0x04C) as *mut u32;
const GPIOB_BASE: usize = 0x4800_0400;
const GPIOB_MODER: *mut u32 = GPIOB_BASE as *mut u32;
const GPIOB_ODR: *mut u32 = (GPIOB_BASE + 0x14) as *mut u32;

fn delay(mut ticks: u32) {
    while ticks > 0 {
        unsafe {
            ((&mut ticks) as *mut u32).write_volatile(ticks - 1);
        }
    }
}

#[entry]
fn main() -> ! {
    // Enable clock for GPIO Port B
    unsafe {
        RCC_AHB2ENR.write_volatile(0b010);
    }

    // Configure PB9 as output
    let pin = 9;
    let mut val: u32 = unsafe { GPIOB_MODER.read_volatile() };
    val &= !(0b11 << pin);
    val |= 0b01 << (pin * 2); // 0b01 is output
    unsafe { GPIOB_MODER.write_volatile(val); }

    let mut val = unsafe { GPIOB_ODR.read_volatile() };
    loop {
        // Set pin to high
        val |= 0b1 << pin;
        unsafe { GPIOB_ODR.write_volatile(val) };
        delay(10_000);

        // Set pin to low
        val &= !(0b1 << pin);
        unsafe { GPIOB_ODR.write_volatile(val) };
        delay(10_000);
    }
}
