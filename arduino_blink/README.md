
# Arduino Blink

This the "Hello World" for arduino, the idea is to blink the builtin LED of the arduino.

The builtin LED is mapped to the pin 13 of the arduino.

## Hardware Required:
* Raspberry pi
* Arduino Uno
* USB 2.0 cable type A/B
* LED
* Breadboard and jumpers.

## Compiling the projects and flashing to Arduino
```bash
cargo build
sudo avrdude -p m328p -c arduino -P /dev/ttyACM0 -b 115200 -U flash:w:target/avr-atmega328p/debug/arduino_blink.elf
```

## Result:
![Blink LED](imgs/BlinkLED.gif)
