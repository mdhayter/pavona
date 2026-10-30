// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::sync::LazyLock;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;

use opentitanlib::app::TransportWrapper;
use opentitanlib::io::gpio::PinMode;
use opentitanlib::io::uart::Uart;
use opentitanlib::test_utils::gpio::{GpioGet, GpioSet};
use opentitanlib::test_utils::init::InitializeTest;
use opentitanlib::test_utils::pinmux_config::PinmuxConfig;
use opentitanlib::uart::console::UartConsole;
use opentitanlib::{collection, execute_test};

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

/// Returns a map of pinmux names from `pairs`.
fn names(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Returns the index of GPIO `name` (`GpioGpio<index>`).
fn gpio_index(name: &str) -> u32 {
    name.strip_prefix("GpioGpio")
        .and_then(|index| index.parse().ok())
        .unwrap_or_else(|| panic!("{name} is not a GPIO"))
}

static CONFIG: LazyLock<HashMap<&'static str, Config>> = LazyLock::new(|| {
    collection! {
         // from:https://github.com/lowRISC/opentitan/
         // blob/master/hw/top_egret/data/pins_cw310_hyperdebug.xdc
        "hyper310" => Config {
            input: names(&[
                // The commented lines represent multi-fuction pins.  These will
                // be added back in when the hyperdebug firmware can set these
                // multifunction pins into GPIO mode.
                // ioc0~2: sw strap0~2
                // ioc5: tap strap1
                // ioc6: pwm ext)_clk
                ("GpioGpio17", "Ioc10"),
                ("GpioGpio18", "Ioc11"),
                ("GpioGpio19", "Ioc12"),

                ("GpioGpio20", "Ior5"),
                ("GpioGpio21", "Ior6"),
                ("GpioGpio22", "Ior7"),
                // IOR8-9 aren't MIOs.
                ("GpioGpio25", "Ior10"),
                ("GpioGpio26", "Ior11"),
                ("GpioGpio27", "Ior12"),
                ("GpioGpio28", "Ior13"),

            ]),
            output: names(&[
                // The commented lines represent multi-fuction pins.  These will
                // be added back in when the hyperdebug firmware can set these
                // multifunction pins into GPIO mode.
                // ioc0~2: sw strap0~2
                // ioc5: tap strap1
                // ioc6: pwm ext)_clk
                ("Ioc10", "GpioGpio17"),
                ("Ioc11", "GpioGpio18"),
                ("Ioc12", "GpioGpio19"),

                ("Ior5", "GpioGpio20"),
                ("Ior6", "GpioGpio21"),
                ("Ior7", "GpioGpio22"),
                // IOR8-9 aren't MIOs.
                ("Ior10", "GpioGpio25"),
                ("Ior11", "GpioGpio26"),
                ("Ior12", "GpioGpio27"),
                ("Ior13", "GpioGpio28"),
            ]),
        },
        // Replication of the hyper310 config.
        "teacup" => Config {
            input: names(&[
                ("GpioGpio17", "Ioc10"),
                ("GpioGpio18", "Ioc11"),
                ("GpioGpio19", "Ioc12"),

                ("GpioGpio20", "Ior5"),
                ("GpioGpio21", "Ior6"),
                ("GpioGpio22", "Ior7"),

                ("GpioGpio25", "Ior10"),
                ("GpioGpio26", "Ior11"),
                ("GpioGpio27", "Ior12"),
                ("GpioGpio28", "Ior13"),
            ]),
            output: names(&[
                ("Ioc10", "GpioGpio17"),
                ("Ioc11", "GpioGpio18"),
                ("Ioc12", "GpioGpio19"),

                ("Ior5", "GpioGpio20"),
                ("Ior6", "GpioGpio21"),
                ("Ior7", "GpioGpio22"),

                ("Ior10", "GpioGpio25"),
                ("Ior11", "GpioGpio26"),
                ("Ior12", "GpioGpio27"),
                ("Ior13", "GpioGpio28"),
            ]),
        },
    }
});

fn gpio_write(
    transport: &TransportWrapper,
    uart: &dyn Uart,
    value: u32,
    config: &HashMap<String, String>,
    invert: u8,
) -> Result<()> {
    let mask = config.values().fold(0, |acc, v| {
        let i = gpio_index(v);
        acc | 1u32 << i
    });
    let masked_value = value & mask;

    log::info!(
        "write & verify pattern: {:08x} & {:08x} -> {:08x}",
        value,
        mask,
        masked_value
    );
    if masked_value == 0 {
        log::info!(
            "skipping: {:08x} has no bits in common with the pinmux configuration",
            value
        );
    } else {
        // invert == 1: use for walking 0
        // invert == 0: use for walking 1
        let write_val = if invert == 1 {
            !masked_value
        } else {
            masked_value
        };
        GpioSet::write_all(uart, write_val)?;
        let mut result = 0u32;
        for (k, v) in config.iter() {
            let n = u32::from(transport.gpio_pin(k)?.read()?);
            let i = gpio_index(v);
            result |= n << i;
        }
        if invert == 1 {
            assert_eq!(write_val & mask, result);
        } else {
            assert_eq!(write_val, result);
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}

fn gpio_read(
    transport: &TransportWrapper,
    uart: &dyn Uart,
    value: u32,
    config: &HashMap<String, String>,
    invert: u32,
) -> Result<()> {
    let mask = if invert == 1 {
        // all 1 for target pins.
        GpioGet::read_all(uart)?
    } else {
        config.keys().fold(0, |acc, k| {
            let i = gpio_index(k);
            acc | 1u32 << i
        })
    };
    let masked_value = value & mask;
    log::info!(
        "read & verify pattern: {:08x} & {:08x} -> {:08x}",
        value,
        mask,
        masked_value
    );
    if masked_value == 0 {
        log::info!(
            "skipping: {:08x} has no bits in common with the pinmux configuration",
            value
        );
    } else {
        for (k, v) in config.iter() {
            let i = gpio_index(k);
            transport
                .gpio_pin(v)?
                .write(((masked_value >> i) & 1) != invert)?;
        }
        // issue dummy read to make sure write complete.
        let _ = GpioGet::read_all(uart)?;
        std::thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}

fn test_gpio_outputs(opts: &Opts, transport: &TransportWrapper) -> Result<()> {
    let uart = transport.uart("console")?;
    log::info!(
        "Configuring pinmux for {:?}",
        opts.init.backend_opts.interface
    );
    let config = CONFIG
        .get(opts.init.backend_opts.interface.as_str())
        .expect("interface");
    PinmuxConfig::configure(
        &*uart,
        transport.top()?,
        Some(&config.input),
        Some(&config.output),
        None,
    )?;

    log::info!("Configuring debugger GPIOs as inputs");
    // The outputs (with respect to pinmux config) correspond to the input pins on the debug board.
    for pin in config.output.keys() {
        transport.gpio_pin(pin)?.set_mode(PinMode::Input)?;
    }

    log::info!("Enabling outputs on the DUT");
    GpioSet::set_input_noise_filter(&*uart, u32::MAX, 1)?;
    GpioSet::set_enabled_all(&*uart, u32::MAX)?;

    // Output level high test
    GpioSet::irq_set_trigger(&*uart, u32::MAX, "high".into())?;
    GpioSet::irq_acknowledge_all(&*uart)?;
    GpioSet::irq_restore_all(&*uart, u32::MAX)?;
    log::info!("test: output level high test");

    // Walking 1
    for i in 16..30 {
        gpio_write(transport, &*uart, 1 << i, &config.output, 0)?;
    }
    GpioSet::irq_disable_all(&*uart, u32::MAX)?;

    // Output Rising Edge test
    GpioSet::irq_set_trigger(&*uart, u32::MAX, "rising".into())?;
    GpioSet::irq_acknowledge_all(&*uart)?;
    GpioSet::irq_restore_all(&*uart, u32::MAX)?;

    log::info!("test: output rising edge test");

    // Walking 1
    for i in 16..30 {
        gpio_write(transport, &*uart, 1 << i, &config.output, 0)?;
    }
    GpioSet::irq_disable_all(&*uart, u32::MAX)?;

    // Output level low test
    log::info!("test: output level low test");
    GpioSet::write_all(&*uart, u32::MAX)?;

    GpioSet::irq_set_trigger(&*uart, u32::MAX, "low".into())?;
    GpioSet::irq_acknowledge_all(&*uart)?;
    GpioSet::irq_restore_all(&*uart, 0x1E7E_0000)?;
    // Walking 0
    for i in 0..32 {
        gpio_write(transport, &*uart, 1 << i, &config.output, 1)?;
    }
    GpioSet::irq_disable_all(&*uart, u32::MAX)?;

    // Output Falling Edge test
    log::info!("test: output falling edge test");
    GpioSet::irq_set_trigger(&*uart, u32::MAX, "falling".into())?;
    GpioSet::irq_acknowledge_all(&*uart)?;
    GpioSet::irq_restore_all(&*uart, u32::MAX)?;
    GpioSet::write_all(&*uart, u32::MAX)?;

    // Walking 0
    for i in 0..32 {
        gpio_write(transport, &*uart, 1 << i, &config.output, 1)?;
    }
    Ok(())
}

fn test_gpio_inputs(opts: &Opts, transport: &TransportWrapper) -> Result<()> {
    let uart = transport.uart("console")?;
    log::info!(
        "Configuring pinmux for {:?}",
        opts.init.backend_opts.interface
    );
    let config = CONFIG
        .get(opts.init.backend_opts.interface.as_str())
        .expect("interface");
    PinmuxConfig::configure(&*uart, transport.top()?, Some(&config.input), None, None)?;

    log::info!("Configuring debugger GPIOs as outputs");
    // The inputs (with respect to pinmux config) correspond to the output pins on the debug board.
    for pin in config.input.values() {
        transport.gpio_pin(pin)?.set_mode(PinMode::PushPull)?;
    }

    log::info!("Disabling outputs on the DUT");
    GpioSet::set_enabled_all(&*uart, 0x0)?;
    GpioSet::set_input_noise_filter(&*uart, u32::MAX, 1)?;

    // Initialize pins to 0
    for pin in config.input.values() {
        transport.gpio_pin(pin)?.write(false)?;
    }

    // Input level high test
    log::info!("test: input level high test");
    GpioSet::irq_set_trigger(&*uart, u32::MAX, "high".into())?;
    GpioSet::irq_acknowledge_all(&*uart)?;
    GpioSet::irq_restore_all(&*uart, u32::MAX)?;
    // Walking 1
    for i in 16..30 {
        gpio_read(transport, &*uart, 1 << i, &config.input, 0)?;
    }
    GpioSet::irq_disable_all(&*uart, u32::MAX)?;

    // Initialize pins to 0
    for pin in config.input.values() {
        transport.gpio_pin(pin)?.write(false)?;
    }
    // Input Rising Edge test
    log::info!("test: input rising edge test");
    GpioSet::irq_set_trigger(&*uart, u32::MAX, "rising".into())?;
    GpioSet::irq_acknowledge_all(&*uart)?;
    GpioSet::irq_restore_all(&*uart, u32::MAX)?;
    // Walking 1
    for i in 16..30 {
        gpio_read(transport, &*uart, 1 << i, &config.input, 0)?;
    }
    GpioSet::irq_disable_all(&*uart, u32::MAX)?;

    // Initialize pins to 1
    for pin in config.input.values() {
        transport.gpio_pin(pin)?.write(true)?;
    }

    // Input level low test
    log::info!("test: input level low test");
    GpioSet::irq_set_trigger(&*uart, u32::MAX, "low".into())?;
    GpioSet::irq_acknowledge_all(&*uart)?;
    GpioSet::irq_restore_all(&*uart, 0x1E7E_0000)?;
    // Walking 0
    for i in 16..30 {
        gpio_read(transport, &*uart, 1 << i, &config.input, 1)?;
    }
    GpioSet::irq_disable_all(&*uart, u32::MAX)?;

    // Initialize pins to 1
    for pin in config.input.values() {
        transport.gpio_pin(pin)?.write(true)?;
    }
    // Input Falling Edge test
    log::info!("test: input falling edge test");
    GpioSet::irq_set_trigger(&*uart, u32::MAX, "falling".into())?;
    GpioSet::irq_acknowledge_all(&*uart)?;
    GpioSet::irq_restore_all(&*uart, u32::MAX)?;

    // Walking 0
    for i in 16..30 {
        gpio_read(transport, &*uart, 1 << i, &config.input, 1)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    opts.init.init_logging();
    let transport = opts.init.init_target()?;

    let uart = transport.uart("console")?;
    uart.set_flow_control(true)?;
    let _ = UartConsole::wait_for(&*uart, r"gpio_intr_test ", opts.timeout)?;

    execute_test!(test_gpio_inputs, &opts, &transport);
    execute_test!(test_gpio_outputs, &opts, &transport);
    Ok(())
}
