//Puts the compiler in embedded mode (it only brings lib core) and allows to write code for kernel and firmware apps
#![no_std]
//It dosen't expect a main function, so we can specify it manually
#![no_main]

use core::panic::PanicInfo;

use arduino_hal::Peripherals;

//Define the panic handler function
//The return type "!" is equal to "Never" it means this function is not allow to return and checks it at compile time
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

//Define the entry point of the program
#[arduino_hal::entry]
fn main() -> ! {
	let peripherals: Peripherals = arduino_hal::Peripherals::take().unwrap();	//Use the struct Peripherals to get access to the hardware
	let pins: Pins = arduino_hal::pins!(peripherals);							//Use macro function to get the pins from the peripherals

	let mut led = pins.d13.into_output();

	led.set_high();

	loop {
		led.toggle();
		arduino_hal::delay_ms(1000);
	}
}
