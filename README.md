
# Raspberry Arduino Rust

Welcome to the Raspberry Arduino Rust repo, in this repo we are going to explore the basics of Rust and the Arduino platform all from a Raspberry pi 4.

The rasberry pi use in this project is running: `Raspberry pi os lite 64-bits (Debian GNU/Linux 11 (bullseye))`.

We are going to use the [avr-hal](https://github.com/Rahix/avr-hal) library to compile rust to avr.

## Prerequisites

Update an upgrade you pi.
```bash
sudo apt update && sudo apt upgrade
```

Install Rust compiler.
```bash
#Rust lang
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# When prompted if raspberry pi os is 64 bits use option 1
1

# Else if raspberry pi os is 32 bits select option "2) Custom installation"
2
# When prompted for "Default host triple?" enter ""
arm-unknown-linux-gnueabihf
```

You can check that Rust is install properly using the following commands:
```console
$ cargo --version
cargo 1.58.0 (f01b232bc 2022-01-19)

$ rustc --version
rustc 1.58.1 (db9d1b20b 2022-01-20)
```

Then we need to set the build chain of rust to a specific version that is compatible with the rust-avr runtime:

```bash
rustup override set nightly-2021-01-07
```

If you don't you will get the next error:
```bash
error[E0554]: `#![feature]` may not be used on the stable release channel
  --> /home/ichi0915/.cargo/registry/src/github.com-1ecc6299db9ec823/avr-device-0.3.2/src/lib.rs:53:12
   |
53 | #![feature(llvm_asm)]
```

We also need to install this programs:
```bash
sudo apt install avr-libc gcc-avr pkg-config avrdude
```

* avr-libc: A C library to use with GCC on Atmel AVR.
* gcc-avr: The arduino uses an avr chip so we need to have the gcc toolchain that can compile c code for rust down to avr.
* pkg-config: To help compile libraries.
* avrdude: Is the programmer we are going to use to flash the elf code to the Arduino.

## Repo structure

This repo will contain different programs that will be label from 0 to f64::INFINITY, they might be in increase difficulty but as we progress we are going to explain the new code/functionality in the form of comments example in project 3 we use a new function and comment its usage, so the project 4 will not explain that section of the code because the previous one explained it.
