// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use anyhow::{Context, Result, ensure};
use clap::Parser;
use rand::prelude::*;

use opentitanlib::app::{TransportWrapper, UartRx};
use opentitanlib::execute_test;
use opentitanlib::io::jtag::{Jtag, JtagTap};
use opentitanlib::test_utils::init::InitializeTest;
use opentitanlib::uart::console::UartConsole;

#[derive(Debug, Parser)]
struct Opts {
    #[command(flatten)]
    init: InitializeTest,

    #[arg(long)]
    rv_dm_delayed_enable: bool,

    /// Seed for random number generator.
    #[arg(long)]
    seed: Option<u64>,

    /// Console receive timeout.
    #[arg(long, value_parser = humantime::parse_duration, default_value = "100s")]
    timeout: Duration,
}

fn test(jtag: &mut dyn Jtag, base: usize, offset: u32) -> Result<()> {
    let addr = base as u32 + offset;

    let mut old_value = 0;
    jtag.read_memory32(addr, std::slice::from_mut(&mut old_value))?;

    let new_value = old_value ^ 1;
    jtag.write_memory32(addr, &[new_value])?;
    let mut readback = 0;
    jtag.read_memory32(addr, std::slice::from_mut(&mut readback))?;
    ensure!(new_value == readback);

    jtag.write_memory32(addr, &[old_value])?;
    jtag.read_memory32(addr, std::slice::from_mut(&mut readback))?;
    ensure!(old_value == readback);

    Ok(())
}

fn test_csr_rw(opts: &Opts, transport: &TransportWrapper) -> Result<()> {
    let seed = opts.seed.unwrap_or_else(|| thread_rng().r#gen());
    log::info!("Random number generator seed is {:x}", seed);
    let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(seed);

    transport.pin_strapping("PINMUX_TAP_RISCV")?.apply()?;
    transport.reset(UartRx::Clear)?;
    let uart = &*transport.uart("console")?;
    uart.set_flow_control(true)?;

    if opts.rv_dm_delayed_enable {
        UartConsole::wait_for(uart, r"DEBUG_MODE_ENABLED", opts.timeout)?;
    } else {
        // Avoid watchdog timeout by entering bootstrap mode.
        transport.pin_strapping("ROM_BOOTSTRAP")?.apply()?;
        transport.reset(UartRx::Clear)?;
    }

    let jtag = &mut *opts
        .init
        .bootstrap
        .options
        .jtag_params
        .create(transport)?
        .connect(JtagTap::RiscvTap)?;

    // Device names in the top description, their IP block types, and a read/write
    // register of each. The offsets come from the top description; devices the top
    // doesn't have are skipped.
    let devices = [
        ("aes", "aes", "IV_0"),
        ("adc_ctrl_aon", "adc_ctrl", "INTR_ENABLE"),
        ("alert_handler", "alert_handler", "INTR_ENABLE"),
        ("aon_timer_aon", "aon_timer", "WKUP_CTRL"),
        ("csrng", "csrng", "INTR_ENABLE"),
        ("edn0", "edn", "INTR_ENABLE"),
        ("edn1", "edn", "INTR_ENABLE"),
        ("entropy_src", "entropy_src", "INTR_ENABLE"),
        ("flash_ctrl_core", "flash_ctrl", "INTR_ENABLE"),
        ("gpio", "gpio", "INTR_ENABLE"),
        ("hmac", "hmac", "INTR_ENABLE"),
        ("i2c0", "i2c", "INTR_ENABLE"),
        ("i2c1", "i2c", "INTR_ENABLE"),
        ("i2c2", "i2c", "INTR_ENABLE"),
        ("keymgr", "keymgr", "INTR_ENABLE"),
        ("keymgr_dpe", "keymgr_dpe", "INTR_ENABLE"),
        ("kmac", "kmac", "INTR_ENABLE"),
        ("acc", "acc", "INTR_ENABLE"),
        ("otp_ctrl_core", "otp_ctrl", "INTR_ENABLE"),
        ("pattgen", "pattgen", "INTR_ENABLE"),
        ("pinmux_aon", "pinmux", "MIO_PAD_ATTR_0"),
        ("pwm_aon", "pwm", "INVERT"),
        ("pwrmgr_aon", "pwrmgr", "INTR_ENABLE"),
        // Skip ROM_CTRL which does not have RW registers
        ("rstmgr_aon", "rstmgr", "CPU_INFO_CTRL"),
        ("rv_timer", "rv_timer", "INTR_ENABLE0"),
        ("sensor_ctrl_aon", "sensor_ctrl", "INTR_ENABLE"),
        ("spi_device", "spi_device", "INTR_ENABLE"),
        ("spi_host0", "spi_host", "INTR_ENABLE"),
        ("spi_host1", "spi_host", "INTR_ENABLE"),
        ("sysrst_ctrl_aon", "sysrst_ctrl", "INTR_ENABLE"),
        ("uart0", "uart", "INTR_ENABLE"),
        ("uart1", "uart", "INTR_ENABLE"),
        ("uart2", "uart", "INTR_ENABLE"),
        ("uart3", "uart", "INTR_ENABLE"),
        ("usbdev", "usbdev", "INTR_ENABLE"),
    ];

    // Test the devices that the top has.
    let top = transport.top()?;
    let mut tests = Vec::new();
    for (name, ip, reg) in devices {
        match top.range(name) {
            Ok(range) => tests.push((name, range.base_addr, top.register_offset(ip, reg)?)),
            Err(_) => log::info!("Skipping {name}: not in top {}", top.name),
        }
    }

    tests.shuffle(&mut rng);

    for (name, base, offset) in tests.iter().copied() {
        log::info!("Testing {}", name);
        test(jtag, base, offset).with_context(|| format!("{} test failed", name))?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    opts.init.init_logging();
    let transport = opts.init.init_target()?;

    execute_test!(test_csr_rw, &opts, &transport);

    Ok(())
}
