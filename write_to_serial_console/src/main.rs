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

	let mut serial = arduino_hal::default_serial!(peripherals, pins, 9600);				//The Last parametert is the baud rate

	loop {
		ufmt::uwriteln!(&mut serial, "Hello World");
		arduino_hal::delay_ms(1000);
	}
}
