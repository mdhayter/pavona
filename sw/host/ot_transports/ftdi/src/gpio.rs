// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use std::cell::{Cell, RefCell};

use anyhow::Result;
use embedded_hal::digital::{InputPin, OutputPin};

use opentitanlib::io::gpio::{GpioError, GpioPin, PinMode, PullMode};

use super::{Channel, Channels, Chip};

#[derive(Default)]
enum PinType {
    Output(ftdi_embedded_hal::OutputPin<ftdi::Device>),
    Input(ftdi_embedded_hal::InputPin<ftdi::Device>),
    #[default]
    None,
}

/// Where a pin is: its MPSSE channel, which GPIO byte (low: `xdbus`, high:
/// `xcbus`) and its bit in that byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PinLocation {
    pub interface: ftdi::Interface,
    pub upper: bool,
    pub bit: u8,
}

pub struct Pin {
    pin: RefCell<PinType>,
    mode: Cell<Option<PinMode>>,
    ftdi: Channels,
    pinname: String,
    has_cbus: bool,
}

impl Pin {
    pub fn open<C: Chip>(ftdi: &Channels, pinname: String) -> Result<Self> {
        Ok(Self {
            pin: RefCell::new(PinType::None),
            mode: Cell::new(None),
            ftdi: ftdi.clone(),
            pinname,
            has_cbus: C::HAS_CBUS,
        })
    }

    /// The channel, byte and bit of the pin named `xdbusN` or `xcbusN`. The
    /// high-byte names (`xcbus0-7`) exist only on chips that have them.
    pub(crate) fn location(&self) -> Result<PinLocation> {
        let pinname = self.pinname.to_lowercase();
        let invalid = || GpioError::InvalidPinName(self.pinname.to_owned());
        let interface = match pinname.get(0..1) {
            Some("a") => ftdi::Interface::A,
            Some("b") => ftdi::Interface::B,
            Some("c") => ftdi::Interface::C,
            Some("d") => ftdi::Interface::D,
            _ => return Err(invalid().into()),
        };
        let (upper, bitstr) = match pinname.get(1..) {
            Some(rest) if rest.starts_with("dbus") => (false, &rest[4..]),
            Some(rest) if rest.starts_with("cbus") && self.has_cbus => (true, &rest[4..]),
            _ => return Err(invalid().into()),
        };
        // A single digit 0-7.
        let bit = match bitstr.as_bytes() {
            [digit @ b'0'..=b'7'] => digit - b'0',
            _ => return Err(invalid().into()),
        };
        Ok(PinLocation {
            interface,
            upper,
            bit,
        })
    }

    /// The mode last set with `set_mode`, if any.
    pub(crate) fn mode(&self) -> Option<PinMode> {
        self.mode.get()
    }

    fn channel(&self, interface: ftdi::Interface) -> Result<&Channel> {
        Ok(self
            .ftdi
            .get(&interface)
            .ok_or_else(|| GpioError::InvalidPinName(self.pinname.to_owned()))?)
    }

    /// Saves the pin's direction in the channel's copy, after the HAL has set
    /// it on the chip.
    fn save_direction(&self, loc: PinLocation, output: bool) {
        let mask = 1 << loc.bit;
        self.ftdi[&loc.interface]
            .saved_direction
            .update(|mut saved| {
                let byte = &mut saved[loc.upper as usize];
                if output {
                    *byte |= mask;
                } else {
                    *byte &= !mask;
                }
                saved
            });
    }

    fn map_outpin(&self) -> Result<ftdi_embedded_hal::OutputPin<ftdi::Device>> {
        let loc = self.location()?;
        let hal = self.channel(loc.interface)?.hal.borrow();
        let pin = match (loc.upper, loc.bit) {
            (false, 0) => hal.ad0(),
            (false, 1) => hal.ad1(),
            (false, 2) => hal.ad2(),
            (false, 3) => hal.ad3(),
            (false, 4) => hal.ad4(),
            (false, 5) => hal.ad5(),
            (false, 6) => hal.ad6(),
            (false, 7) => hal.ad7(),
            (true, 0) => hal.c0(),
            (true, 1) => hal.c1(),
            (true, 2) => hal.c2(),
            (true, 3) => hal.c3(),
            (true, 4) => hal.c4(),
            (true, 5) => hal.c5(),
            (true, 6) => hal.c6(),
            (true, 7) => hal.c7(),
            _ => unreachable!(),
        }?;
        self.save_direction(loc, true);
        Ok(pin)
    }

    fn map_inpin(&self) -> Result<ftdi_embedded_hal::InputPin<ftdi::Device>> {
        let loc = self.location()?;
        let hal = self.channel(loc.interface)?.hal.borrow();
        let pin = match (loc.upper, loc.bit) {
            (false, 0) => hal.adi0(),
            (false, 1) => hal.adi1(),
            (false, 2) => hal.adi2(),
            (false, 3) => hal.adi3(),
            (false, 4) => hal.adi4(),
            (false, 5) => hal.adi5(),
            (false, 6) => hal.adi6(),
            (false, 7) => hal.adi7(),
            (true, 0) => hal.ci0(),
            (true, 1) => hal.ci1(),
            (true, 2) => hal.ci2(),
            (true, 3) => hal.ci3(),
            (true, 4) => hal.ci4(),
            (true, 5) => hal.ci5(),
            (true, 6) => hal.ci6(),
            (true, 7) => hal.ci7(),
            _ => unreachable!(),
        }?;
        self.save_direction(loc, false);
        Ok(pin)
    }
}

impl GpioPin for Pin {
    fn read(&self) -> Result<bool> {
        let mut pin = self.pin.borrow_mut();
        match *pin {
            PinType::Input(ref mut pin) => Ok(pin.is_high()?),
            _ => Err(GpioError::InvalidPinMode(0).into()),
        }
    }

    fn write(&self, value: bool) -> Result<()> {
        let mut pin = self.pin.borrow_mut();
        match *pin {
            PinType::Output(ref mut pin) => {
                if value {
                    pin.set_high()?;
                } else {
                    pin.set_low()?;
                }
                Ok(())
            }
            // An open-drain pin is released (an input) outside bit-banging,
            // which chooses each pin's direction for a whole waveform.
            PinType::Input(_) if self.mode.get() == Some(PinMode::OpenDrain) => {
                if value {
                    Ok(())
                } else {
                    Err(GpioError::UnsupportedPinMode(PinMode::OpenDrain).into())
                }
            }
            _ => Err(GpioError::InvalidPinMode(0).into()),
        }
    }

    fn set_mode(&self, mode: PinMode) -> Result<()> {
        let mut pin = self.pin.borrow_mut();
        *pin = match (&*pin, mode) {
            (PinType::None, PinMode::Input | PinMode::OpenDrain) => {
                PinType::Input(self.map_inpin()?)
            }
            (PinType::None, PinMode::PushPull) => PinType::Output(self.map_outpin()?),
            _ => return Ok(()),
        };
        self.mode.set(Some(mode));
        Ok(())
    }

    fn set_pull_mode(&self, _mode: PullMode) -> Result<()> {
        Ok(())
    }

    fn get_internal_pin_name(&self) -> Option<&str> {
        Some(&self.pinname)
    }
}
