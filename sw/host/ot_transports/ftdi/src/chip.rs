// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

pub trait Chip {
    const VENDOR_ID: u16 = 0x0403;
    const PRODUCT_ID: u16;

    const UART_BAUD: u32 = 115200;

    /// USB product ID of the serial ports used as UARTs. Numbered UART
    /// instances count the ports with this product ID and `VENDOR_ID`.
    const UART_PRODUCT_ID: u16 = Self::PRODUCT_ID;

    /// The interfaces to open. Interfaces left out stay free for other tools,
    /// such as a JTAG cable on the same chip.
    const INTERFACES: &'static [ftdi::Interface];

    /// The interface whose MPSSE runs the SPI host.
    const SPI_INTERFACE: ftdi::Interface;

    /// The GPIO pin used as the SPI chip select.
    const SPI_CS_PIN: &'static str;
}

pub struct Ft4232hq {}

impl Chip for Ft4232hq {
    const PRODUCT_ID: u16 = 0x6011;
    const INTERFACES: &'static [ftdi::Interface] = &[
        ftdi::Interface::A,
        ftdi::Interface::B,
        // ftdi::Interface::C,
        ftdi::Interface::D,
    ];
    const SPI_INTERFACE: ftdi::Interface = ftdi::Interface::B;
    const SPI_CS_PIN: &'static str = "bdbus3";
}

/// The FT2232H on a Digilent Genesys 2 (USB PROG port). Channel B is the
/// board's JTAG cable, so only channel A is opened; its ADBUS0-3 carry SPI and
/// ADBUS4-7 are GPIOs. The board's console UART is a separate FT232R.
pub struct Ft2232hGenesys2 {}

impl Chip for Ft2232hGenesys2 {
    const PRODUCT_ID: u16 = 0x6010;
    const UART_PRODUCT_ID: u16 = 0x6001;
    const INTERFACES: &'static [ftdi::Interface] = &[ftdi::Interface::A];
    const SPI_INTERFACE: ftdi::Interface = ftdi::Interface::A;
    const SPI_CS_PIN: &'static str = "adbus3";
}
