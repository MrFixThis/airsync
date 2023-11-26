#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![allow(unused_macros)]

#[macro_use]
mod sprint;
pub mod timer;

use arduino_hal::{prelude::_void_ResultVoidExt, Peripherals};

use panic_halt as _;

const PAUSE_RATE: u32 = 10_000;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    sprintln!(
        serial,
        "[<( AirSync )>]\n=> By MrFixThis <mrfixthis55@gmail.com>\n"
    );
    timer::millis_init(dp.TC0);
    unsafe {
        avr_device::interrupt::enable();
    }
    sprintln!(serial, "> Global interrupts enabled.");

    _ = pins.d13.into_output_high();
    let pir_pin = pins.d12.into_pull_up_input();
    let mut fan_pin = pins.d3.into_output();
    sprintln!(serial, "> Pins configured.");

    sprintln!(serial, "> Waiting for sensor to be active...");
    while pir_pin.is_high() {}
    sprintln!(serial, "> Sensor active.\n");

    let mut lock_low = true;
    let mut take_low_time = false;
    let mut low_in = 0;

    loop {
        if pir_pin.is_high() && lock_low {
            sprintln!(serial, "> Motion detected.");

            lock_low = false;
            take_low_time = true;

            fan_pin.set_high();
            sprintln!(
                serial,
                "> Fan turned ON at second {}. \n> (X) Ventilating...",
                timer::millis() / 1000
            );
            arduino_hal::delay_ms(PAUSE_RATE as u16);
        }

        if pir_pin.is_low() {
            if take_low_time {
                low_in = timer::millis();
                take_low_time = false;
            }

            if !lock_low && timer::millis() - low_in > PAUSE_RATE {
                lock_low = true;
                fan_pin.set_low();
                sprintln!(
                    serial,
                    "> Fan turned OFF at second {}.\n",
                    (timer::millis() - PAUSE_RATE) / 1000
                );
                arduino_hal::delay_ms(50);
            }
        }
    }
}
