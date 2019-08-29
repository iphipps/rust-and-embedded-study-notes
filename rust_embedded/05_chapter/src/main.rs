#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();
    let half_period = 50_u16;

    loop {
        for led in 0..8 {
            leds[led].on();
            delay.delay_ms(half_period);

            leds[led].off();
            delay.delay_ms(half_period);
        }
    }
}
