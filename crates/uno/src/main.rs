/*!
 * Blink the builtin LED - the "Hello World" of embedded programming.
 */
#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();
    led.set_high();

    loop {
        //start on
        
        arduino_hal::delay_ms(1000);
        led.toggle();
        //off

        arduino_hal::delay_ms(200);
        led.toggle();
        //on
        
        arduino_hal::delay_ms(200);
        led.toggle();
        //off
        
        arduino_hal::delay_ms(200);
        led.toggle();
        //on
        
        arduino_hal::delay_ms(200);
        led.toggle();
        //off

        arduino_hal::delay_ms(200);
        led.toggle();

        //end on
    }
}