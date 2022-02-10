
# Button Toggle LED

In this code is to show how to write to the serial console.

## Hardware Required:
* Raspberry pi
* Arduino Uno
* USB 2.0 cable type A/B

## Compiling the projects and flashing to Arduino
```bash
cargo build
sudo avrdude -p m328p -c arduino -P /dev/ttyACM0 -b 115200 -U flash:w:target/avr-atmega328p/debug/write_to_serial_console.elf
```

## Result:
![Serial Message](imgs/SerialMessage.gif)
