Параметр           |  Характеристика 
-------------------|--------------------
Power Supply Pins  | VDD: Core power supply (3.3V). Connect to a 3.3V regulated power source. <br> VSS: Core ground. Connect to the ground of the power source. <br> VDDA: Analog power supply (3.3V). Connect to the same 3.3V source as VDD. <br> VSSA: Analog ground. Connect to ground. <br> VREF+ / VREF-: Reference voltage for ADC. Typically connect VREF+ to a stable voltage reference (e.g., 3.3V) and VREF- to ground.
RS232 (USART1)     | TX (PA9): Transmit data pin. <br> RX (PA10): Receive data pin. <br> GND: Ground pin. Connect to the ground of the RS232 interface.
RS485 (USART2)     | TX (PA2): Transmit data pin. <br> RX (PA3): Receive data pin. <br> DE (PA1): Driver Enable pin. Used to control the direction of data flow in RS485 communication. <br> RE (PA1): Receiver Enable pin. Connect to the same pin as DE for half-duplex operation. <br> GND: Ground pin. Connect to the ground of the RS485 interface.
CAN1:              | CAN1_H (PB9): CAN High pin. <br> CAN1_L (PB8): CAN Low pin. <br> GND: Ground pin.
CAN2:              | CAN2_H (PD1): CAN High pin. <br> CAN2_L (PD0): CAN Low pin. <br> GND: Ground pin.
Ethernet           | ETH_TX+ (PA1): Ethernet Transmit Positive pin. <br> ETH_TX- (PA2): Ethernet Transmit Negative pin. <br> ETH_RX+ (PA7): Ethernet Receive Positive pin. <br> ETH_RX- (PA6): Ethernet Receive Negative pin. <br> ETH_CLK (PC10): Ethernet Clock pin. <br> ETH_MDC (PC1): Management Data Clock pin. <br> ETH_MDIO (PC2): Management Data Input/Output pin. <br> ETH_RST (PF0): Ethernet Reset pin. <br> GND: Ground pin. 
USB Host           | VBUS (PA9): USB power supply pin. <br> D+ (PA12): USB Data Positive pin. <br> D- (PA11): USB Data Negative pin. <br> GND: Ground pin.

<details>
<summary><b>![Board pic](../doc/pic/stm32f407vet6-development-board.jpg)</b></summary>
  
</details>

# `stm32-template`

> A template for building applications for STM32 microcontrollers

## Dependencies

To build embedded programs using this template you'll need:

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/cargo-generate/cargo-generate#installation).
``` console
$ cargo install cargo-generate
```

- Flash and run/debug tools:
``` console
$ cargo install probe-rs --features cli
```

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:
  
``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```

## Instantiate the template.

1. Run and enter project name
``` console
$ cargo generate --git https://github.com/burrbull/stm32-template/
 Project Name: app
```

2. Specify **chip product name** and answer on several other guide questions.

3. Your program is ready to compile:
``` console
$ cargo build --release
```

## Flash and run/debug

You can flash your firmware using one of those tools:

- `cargo flash --release` — just flash
- `cargo run --release` — flash and run using `probe-rs run` runner or `probe-run` runner (deprecated) which you can set in `.cargo/config.toml`
- `cargo embed --release` — multifunctional tool for flash and debug

You also can debug your firmware on device from VS Code with [probe-rs](https://probe.rs/docs/tools/vscode/) extention or with `probe-rs gdb` command.
You will need SVD specification for your chip for this. You can load patched SVD files [here](https://stm32-rs.github.io/stm32-rs/).

## Contribution
<details>
<summary>Details</summary>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

### Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, promises
to intervene to uphold that code of conduct.

[CoC]: https://www.rust-lang.org/policies/code-of-conduct
</details>
