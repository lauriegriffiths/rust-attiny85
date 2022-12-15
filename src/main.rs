#![no_std]
#![no_main]

use panic_halt as _;
use attiny_hal::delay::*;
use attiny_hal::clock::*;

use attiny_hal::prelude::*;

#[attiny_hal::entry]
fn main() -> ! {
    let dp = attiny_hal::Peripherals::take().unwrap();
    let pins = attiny_hal::pins!(dp);
    let mut delay = Delay::<MHz1>::new();

    let mut led = pins.pb3.into_output();

    loop {
        led.toggle();
        delay.delay_ms(1000u16);
    }
}


//use attiny_hal::prelude::*;
//use attiny_hal::delay::Delay;
