
# Raspberry Arduino Rust

Welcome to the Raspberry Arduino Rust repo, in this repo we are going to explore the basics of Rust and the Arduino platform all from a Raspberry pi 4.

The rasberry pi use in this project is running: `Raspberry pi os lite 64-bits (Debian GNU/Linux 11 (bullseye))`.

We are going to use the [avr-hal](https://github.com/Rahix/avr-hal) library to compile rust to avr.

## Prerequisites

Update an upgrade you pi
```bash
sudo apt update && sudo apt upgrade
```

Install Rust compiler
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



