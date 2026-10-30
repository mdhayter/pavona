// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::iter;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;

use opentitanlib::app::TransportWrapper;
use opentitanlib::execute_test;
use opentitanlib::io::gpio::{PinMode, PullMode};
use opentitanlib::io::uart::Uart;
use opentitanlib::test_utils::gpio::{GpioGet, GpioSet};
use opentitanlib::test_utils::init::InitializeTest;
use opentitanlib::test_utils::pinmux_config::PinmuxConfig;
use opentitanlib::uart::console::UartConsole;
use ot_hal::dif::pinmux::PinmuxPadAttr;

#[derive(Debug, Parser)]
struct Opts {
    #[command(flatten)]
    init: InitializeTest,

    /// Console receive timeout.
    #[arg(long, value_parser = humantime::parse_duration, default_value = "600s")]
    timeout: Duration,
}

/// Pinmux configuration, by the names in the top description: GPIO inputs to
/// pads, and pads to GPIO outputs.
struct Config {
    input: HashMap<String, String>,
    output: HashMap<String, String>,
}

/// Number of GPIOs.
const NUM_GPIOS: u32 = 32;

/// Returns the pinmux name of GPIO `index`, which is the same for its
/// peripheral input and its output selector.
fn gpio_name(index: u32) -> String {
    format!("GpioGpio{index}")
}

/// Returns the index of GPIO `name`.
fn gpio_index(name: &str) -> u32 {
    name.strip_prefix("GpioGpio")
        .and_then(|index| index.parse().ok())
        .unwrap_or_else(|| panic!("{name} is not a GPIO"))
}

static SUPPORTS_OD: [&str; 14] = [
    "Ioa6", "Ioa7", "Ioa8", "Iob9", "Iob10", "Iob11", "Iob12", "Ioc10", "Ioc11", "Ioc12", "Ior10",
    "Ior11", "Ior12", "Ior13",
];

static SUPPORTS_VIRT_OD: [&str; 33] = [
    "Ioa0", "Ioa1", "Ioa2", "Ioa3", "Ioa4", "Ioa5", "Iob0", "Iob1", "Iob2", "Iob3", "Iob4", "Iob5",
    "Iob6", "Iob7", "Iob8", "Ioc0", "Ioc1", "Ioc2", "Ioc3", "Ioc4", "Ioc5", "Ioc6", "Ioc7", "Ioc8",
    "Ioc9", "Ior0", "Ior1", "Ior2", "Ior3", "Ior4", "Ior5", "Ior6", "Ior7",
];

fn write_all_verify(
    transport: &TransportWrapper,
    uart: &dyn Uart,
    write: u32,
    read: u32,
    config: &HashMap<String, String>,
) -> Result<()> {
    let mask = config.values().fold(0, |acc, v| {
        let i = gpio_index(v);
        acc | 1u32 << i
    });

    let (masked_write, masked_read) = (write & mask, read & mask);

    log::info!(
        "write & verify pattern {write:08x} with mask {mask:08x}: {masked_write:08x} -> {masked_read:08x}"
    );

    if masked_write == 0 {
        log::info!(
            "skipping: {masked_write:08x} has no bits in common with the pinmux configuration"
        );
        return Ok(());
    }

    GpioSet::write_all(uart, masked_write)?;

    let mut result = 0u32;
    for (k, v) in config.iter() {
        let n = u32::from(transport.gpio_pin(k)?.read()?);
        let i = gpio_index(v);
        result |= n << i;
    }

    log::info!("result = {result:08x}");
    assert_eq!(masked_read, result);

    Ok(())
}

fn read_all_verify(
    transport: &TransportWrapper,
    uart: &dyn Uart,
    write: u32,
    read: u32,
    config: &HashMap<String, String>,
) -> Result<()> {
    let mask = config.keys().fold(0, |acc, k| {
        let i = gpio_index(k);
        acc | 1u32 << i
    });

    let (masked_write, masked_read) = (write & mask, read & mask);

    log::info!(
        "read & verify pattern with mask {mask:08x}: {masked_write:08x} -> {masked_read:08x}"
    );

    if masked_write == 0 {
        log::info!(
            "skipping: {masked_write:08x} has no bits in common with the pinmux configuration"
        );
        return Ok(());
    }

    for (k, v) in config.iter() {
        let i = gpio_index(k);
        transport.gpio_pin(v)?.write((masked_write >> i) & 1 != 0)?;
    }

    let result = GpioGet::read_all(uart)? & mask;
    log::info!("result = {result:08x}");
    assert_eq!(masked_read, result);

    Ok(())
}

fn get_config(transport: &TransportWrapper) -> Config {
    // The transport's GPIO pins are named after the pads they connect to.
    let gpio_pins = transport.gpios();
    let gpios = (0..NUM_GPIOS).map(gpio_name);

    let input: HashMap<_, _> = iter::zip(gpios.clone(), gpio_pins.iter().cloned()).collect();
    let output: HashMap<_, _> = iter::zip(gpio_pins.iter().cloned(), gpios).collect();

    Config { input, output }
}

fn test_gpio_outputs(transport: &TransportWrapper, config: &Config) -> Result<()> {
    let uart = transport.uart("console")?;
    PinmuxConfig::configure(&*uart, transport.top()?, None, Some(&config.output), None)?;

    log::info!("Configuring debugger GPIOs as inputs");
    // The outputs (with respect to pinmux config) correspond to the input pins on the debug board.
    for pin in config.output.keys() {
        transport.gpio_pin(pin)?.set_mode(PinMode::Input)?;
    }

    log::info!("Enabling outputs on the DUT");
    GpioSet::set_enabled_all(&*uart, 0xFFFFFFFF)?;
    write_all_verify(transport, &*uart, 0x5555_5555, 0x5555_5555, &config.output)?;
    write_all_verify(transport, &*uart, 0xAAAA_AAAA, 0xAAAA_AAAA, &config.output)?;

    for i in 0..32 {
        write_all_verify(transport, &*uart, 1 << i, 1 << i, &config.output)?;
    }

    Ok(())
}

fn test_gpio_inputs(transport: &TransportWrapper, config: &Config) -> Result<()> {
    let uart = transport.uart("console")?;

    PinmuxConfig::configure(&*uart, transport.top()?, Some(&config.input), None, None)?;

    log::info!("Configuring debugger GPIOs as outputs");
    // The inputs (with respect to pinmux config) correspond to the output pins on the debug board.
    for pin in config.input.values() {
        transport.gpio_pin(pin)?.set_mode(PinMode::PushPull)?;
    }

    log::info!("Disabling outputs on the DUT");
    GpioSet::set_enabled_all(&*uart, 0x0)?;

    read_all_verify(transport, &*uart, 0x5555_5555, 0x5555_5555, &config.input)?;
    read_all_verify(transport, &*uart, 0xAAAA_AAAA, 0xAAAA_AAAA, &config.input)?;

    for i in 0..32 {
        read_all_verify(transport, &*uart, 1 << i, 1 << i, &config.input)?;
    }
    Ok(())
}

fn test_pad_inversion(transport: &TransportWrapper, config: &Config) -> Result<()> {
    let uart = transport.uart("console")?;

    let invert_all: HashMap<_, _> = config
        .output
        .keys()
        .map(|mio| (mio.clone(), PinmuxPadAttr::INVERT))
        .collect();

    log::info!("Configuring pads as inverted inputs");
    PinmuxConfig::configure(
        &*uart,
        transport.top()?,
        Some(&config.input),
        None,
        Some(&invert_all),
    )?;

    log::info!("Configuring debugger GPIOs as outputs");
    // The inputs (with respect to pinmux config) correspond to the output pins on the debug board.
    for pin in config.input.values() {
        transport.gpio_pin(pin)?.set_mode(PinMode::PushPull)?;
    }

    log::info!("Disabling outputs on the DUT");
    GpioSet::set_enabled_all(&*uart, 0x0)?;

    log::info!("Testing inverted device inputs");
    read_all_verify(transport, &*uart, 0x5555_5555, !0x5555_5555, &config.input)?;
    read_all_verify(transport, &*uart, 0xAAAA_AAAA, !0xAAAA_AAAA, &config.input)?;

    for i in 0..32 {
        read_all_verify(transport, &*uart, 1 << i, !(1 << i), &config.input)?;
    }

    log::info!("Configuring pads as inverted outputs");
    PinmuxConfig::configure(
        &*uart,
        transport.top()?,
        None,
        Some(&config.output),
        Some(&invert_all),
    )?;

    log::info!("Configuring debugger GPIOs as inputs");
    // The outputs (with respect to pinmux config) correspond to the input pins on the debug board.
    for pin in config.output.keys() {
        transport.gpio_pin(pin)?.set_mode(PinMode::Input)?;
    }

    log::info!("Enabling outputs on the DUT");
    GpioSet::set_enabled_all(&*uart, 0xFFFFFFFF)?;

    log::info!("Testing inverted device outputs");
    write_all_verify(transport, &*uart, 0x5555_5555, !0x5555_5555, &config.output)?;
    write_all_verify(transport, &*uart, 0xAAAA_AAAA, !0xAAAA_AAAA, &config.output)?;

    for i in 0..32 {
        write_all_verify(transport, &*uart, 1 << i, !(1 << i), &config.output)?;
    }

    Ok(())
}

/// Test opendrain feature of pads.
///
/// Both normal and virtual opendrain can be tested by passing a different set
/// of pad filters and the correct pad attribute to apply.
fn test_opendrain(
    transport: &TransportWrapper,
    config: &Config,
    pad_filter: &[&str],
    pad_attr: PinmuxPadAttr,
) -> Result<()> {
    let uart = transport.uart("console")?;

    log::info!("Configuring debugger GPIOs as pulled-up inputs");
    // The inputs (with respect to pinmux config) correspond to the output pins on the debug board.
    let opendrain_inputs = config.input.values();
    for pin in opendrain_inputs {
        let pin = transport.gpio_pin(pin)?;
        pin.set(Some(PinMode::Input), None, Some(PullMode::PullUp), None)?;
    }

    // Filter pads to those supporting opendrain using the provided filter.
    let opendrain_outputs: HashMap<_, _> = config
        .output
        .clone()
        .into_iter()
        .filter(|(mio, _)| pad_filter.contains(&mio.as_str()))
        .collect();
    let opendrain_attrs: HashMap<_, _> = opendrain_outputs
        .keys()
        .map(|mio| (mio.clone(), pad_attr))
        .collect();

    log::info!("Configuring pads as opendrain outputs");
    PinmuxConfig::configure(
        &*uart,
        transport.top()?,
        None,
        Some(&opendrain_outputs),
        Some(&opendrain_attrs),
    )?;

    log::info!("Enabling outputs on the DUT");
    GpioSet::set_enabled_all(&*uart, 0xFFFFFFFF)?;

    log::info!("Testing opendrain device outputs");
    for pattern in [0x5555_5555, 0xAAAA_AAAA] {
        write_all_verify(transport, &*uart, pattern, pattern, &opendrain_outputs)?;
    }
    for i in 0..32 {
        write_all_verify(transport, &*uart, 1 << i, 1 << i, &opendrain_outputs)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    opts.init.init_logging();
    let transport = opts.init.init_target()?;

    let config = get_config(&transport);

    let uart = transport.uart("console")?;
    uart.set_flow_control(true)?;
    let _ = UartConsole::wait_for(&*uart, r"Running ", opts.timeout)?;

    execute_test!(test_gpio_outputs, &transport, &config);
    execute_test!(test_gpio_inputs, &transport, &config);
    execute_test!(test_pad_inversion, &transport, &config);
    // Only silicon supports OpenDrain, FPGA does not.
    if opts.init.backend_opts.interface == "teacup" {
        execute_test!(
            test_opendrain,
            &transport,
            &config,
            &SUPPORTS_OD,
            PinmuxPadAttr::OD_EN,
        );
    }
    execute_test!(
        test_opendrain,
        &transport,
        &config,
        &SUPPORTS_VIRT_OD,
        PinmuxPadAttr::VIRTUAL_OD_EN
    );

    Ok(())
}
