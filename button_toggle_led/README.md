
# Button Toggle LED

In this code we set the pin 7 to be a pullup input and we monitor it until it changes state to toggle the led on pin 3.

## Hardware Required:
* Raspberry pi
* Arduino Uno
* USB 2.0 cable type A/B
* Button
* LED
* 220 or 330 ohm resistor
* Breadboard and jumpers.

## Diagram:
You can download the fritzing diagram to take a closer look to the schematic [here](fritzing_diagram/button_toggle_led_diagram.fzz)

![Toggle LED](imgs/FritzingDiagram.png)

## Compiling the projects and flashing to Arduino
```bash
cargo build
sudo avrdude -p m328p -c arduino -P /dev/ttyACM0 -b 115200 -U flash:w:target/avr-atmega328p/debug/button_toggle_led.elf
```

## Result:
![Toggle LED](imgs/ToggleLed.gif)
