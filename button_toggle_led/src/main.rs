#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
	let peripherals = arduino_hal::Peripherals::take().unwrap();
	let pins = arduino_hal::pins!(peripherals);

	let mut led = pins.d3.into_output();
        let button = pins.d7.into_pull_up_input();				//We set the pin 7 to pullup to keep the signal high by default

	loop {
		if button.is_low() {						//If the button is press we toggle the LED 
			led.toggle();
			arduino_hal::delay_ms(300);
		}
	}
}
