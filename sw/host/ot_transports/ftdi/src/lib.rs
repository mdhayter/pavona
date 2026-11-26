// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::rc::Rc;

use anyhow::Result;
use serialport::SerialPortType;

use opentitanlib::backend::{Backend, BackendOpts, define_interface};
use opentitanlib::io::gpio::{GpioBitbanging, GpioPin, PinMode};
use opentitanlib::io::spi::Target;
use opentitanlib::io::uart::flow::SoftwareFlowControl;
use opentitanlib::io::uart::serial::SerialPortUart;
use opentitanlib::io::uart::{Uart, UartError};
use opentitanlib::transport::{
    Capabilities, Capability, FpgaOps, ProgressIndicator, Transport, TransportError,
    TransportInterfaceType,
};
use opentitanlib::util::fs::builtin_file;
use opentitanlib::util::parse_int::ParseInt;

pub mod bitbang;
pub mod chip;
pub mod fpga;
pub mod gpio;
pub mod spi;

use chip::{Chip, Ft2232hGenesys2, Ft4232hq};

#[derive(Default)]
struct Inner {
    spi: Option<Rc<dyn Target>>,
    uart: HashMap<String, Rc<dyn Uart>>,
}

/// One open MPSSE channel.
pub struct Channel {
    pub(crate) hal: RefCell<ftdi_embedded_hal::FtHal<ftdi::Device>>,
    /// The transport's copy of the direction (1: output) of the low and high
    /// GPIO bytes, saved after each change made through the HAL, which keeps
    /// its own private. Bit-banging needs it to drive the pins it is not given
    /// and to restore them afterwards.
    pub(crate) saved_direction: Cell<[u8; 2]>,
}

/// The open MPSSE channels.
pub(crate) type Channels = Rc<HashMap<ftdi::Interface, Channel>>;

/// The MPSSE settings the channels are opened with, and restored to after
/// bit-banging.
pub(crate) fn mpsse_settings() -> ftdi_mpsse::MpsseSettings {
    ftdi_mpsse::MpsseSettings {
        clock_frequency: Some(8_000_000),
        ..Default::default()
    }
}

pub struct Ftdi<C: Chip> {
    pub(crate) ftdi_interfaces: Channels,
    gpio: bitbang::Pins,
    inner: RefCell<Inner>,
    phantom: std::marker::PhantomData<C>,
}

impl<C: Chip> Ftdi<C> {
    pub fn new() -> anyhow::Result<Self> {
        let mut ftdi_interfaces = HashMap::new();
        for interface in C::INTERFACES {
            let device = ftdi::find_by_vid_pid(C::VENDOR_ID, C::PRODUCT_ID)
                .interface(*interface)
                .open()?;
            ftdi_interfaces.insert(
                *interface,
                Channel {
                    hal: RefCell::new(ftdi_embedded_hal::FtHal::init(device, &mpsse_settings())?),
                    saved_direction: Cell::new([0, 0]),
                },
            );
        }

        let ftdi_dev = Ftdi {
            ftdi_interfaces: Rc::new(ftdi_interfaces),
            gpio: Rc::default(),
            inner: RefCell::default(),
            phantom: std::marker::PhantomData,
        };
        Ok(ftdi_dev)
    }

    /// Opens a UART. The instance is either the index of a serial port with
    /// the chip's UART product ID, or the path of a serial port device, such as
    /// `/dev/serial/by-id/...`, for boards with more than one such port.
    fn open_uart(&self, instance: &str) -> Result<SoftwareFlowControl<SerialPortUart>> {
        if instance.starts_with('/') {
            return Ok(SoftwareFlowControl::new(SerialPortUart::open(
                instance,
                C::UART_BAUD,
            )?));
        }
        let index = u32::from_str(instance).ok().ok_or_else(|| {
            TransportError::InvalidInstance(TransportInterfaceType::Uart, instance.to_string())
        })?;
        let mut ports = serialport::available_ports()
            .map_err(|e| UartError::EnumerationError(e.to_string()))?;

        ports.retain(|port| {
            if let SerialPortType::UsbPort(info) = &port.port_type
                && info.vid == C::VENDOR_ID
                && info.pid == C::UART_PRODUCT_ID
            {
                return true;
            }
            false
        });

        let port = ports.get(index as usize).ok_or_else(|| {
            TransportError::InvalidInstance(TransportInterfaceType::Uart, instance.to_string())
        })?;

        Ok(SoftwareFlowControl::new(SerialPortUart::open(
            &port.port_name,
            C::UART_BAUD,
        )?))
    }
}

impl<C: Chip> Transport for Ftdi<C> {
    fn capabilities(&self) -> Result<Capabilities> {
        Ok(Capabilities::new(
            Capability::SPI
                | Capability::GPIO
                | Capability::GPIO_BITBANGING
                | Capability::UART
                | Capability::UART_NONBLOCKING,
        ))
    }

    fn uart(&self, instance: &str) -> Result<Rc<dyn Uart>> {
        let mut inner = self.inner.borrow_mut();
        let uart = match inner.uart.entry(instance.to_string()) {
            Entry::Vacant(v) => v.insert(Rc::new(self.open_uart(instance)?)).clone(),
            Entry::Occupied(o) => o.get().clone(),
        };
        Ok(uart)
    }

    fn gpio_pin(&self, pinname: &str) -> Result<Rc<dyn GpioPin>> {
        let mut gpio = self.gpio.borrow_mut();
        Ok(match gpio.entry(pinname.to_string()) {
            Entry::Vacant(v) => v
                .insert(Rc::new(gpio::Pin::open::<C>(
                    &self.ftdi_interfaces,
                    pinname.to_string(),
                )?))
                .clone(),
            Entry::Occupied(o) => o.get().clone(),
        })
    }

    fn spi(&self, _instance: &str) -> Result<Rc<dyn Target>> {
        let spi_cs = gpio::Pin::open::<C>(&self.ftdi_interfaces, C::SPI_CS_PIN.to_string())?;
        spi_cs.set_mode(PinMode::PushPull)?;
        let mut inner = self.inner.borrow_mut();
        if inner.spi.is_none() {
            inner.spi = Some(Rc::new(spi::Spi::open(
                &self.ftdi_interfaces,
                C::SPI_INTERFACE,
                spi_cs,
            )?));
        }
        Ok(inner.spi.as_ref().unwrap().clone())
    }

    fn gpio_bitbanging(&self) -> Result<Rc<dyn GpioBitbanging>> {
        Ok(Rc::new(bitbang::Bitbanging::new(
            &self.ftdi_interfaces,
            &self.gpio,
        )))
    }

    fn fpga_ops(&self) -> Result<&dyn FpgaOps> {
        if C::FPGA_JTAG.is_some() {
            Ok(self)
        } else {
            Err(TransportError::UnsupportedOperation.into())
        }
    }
}

impl<C: Chip> Ftdi<C> {
    fn fpga_jtag(&self) -> Result<fpga::Jtag> {
        let cable = C::FPGA_JTAG
            .as_ref()
            .ok_or(TransportError::UnsupportedOperation)?;
        fpga::Jtag::open(C::VENDOR_ID, C::PRODUCT_ID, cable)
    }
}

impl<C: Chip> FpgaOps for Ftdi<C> {
    fn load_bitstream(&self, bitstream: &[u8], progress: &dyn ProgressIndicator) -> Result<()> {
        log::info!("Programming the FPGA bitstream over JTAG.");
        self.fpga_jtag()?
            .program(bitstream, progress)
            .map_err(|e| TransportError::FpgaProgramFailed(e.to_string()).into())
    }

    fn clear_bitstream(&self) -> Result<()> {
        self.fpga_jtag()?.clear_bitstream()
    }
}

struct FtdiBackend<C>(C);

impl<C: Chip + 'static> Backend for FtdiBackend<C> {
    type Opts = ();

    fn create_transport(_: &BackendOpts, _: &()) -> Result<Box<dyn Transport>> {
        Ok(Box::new(Ftdi::<C>::new()?))
    }
}

define_interface!(
    "ftdi",
    FtdiBackend<Ft4232hq>,
    "/__builtin__/pavona_ftdi_voyager.json5"
);
builtin_file!(
    "pavona_ftdi_voyager.json5",
    include_str!("../config/pavona_ftdi_voyager.json5")
);

define_interface!(
    "ftdi_genesys2",
    FtdiBackend<Ft2232hGenesys2>,
    "/__builtin__/pavona_ftdi_genesys2.json5"
);
builtin_file!(
    "pavona_ftdi_genesys2.json5",
    include_str!("../config/pavona_ftdi_genesys2.json5")
);
