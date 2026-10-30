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

use ot_bindgen_dif as dif;

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

    // Device names in the top description, and a read/write register of each.
    // The register offsets come from the DIFs of the top the test was built
    // for, which only cover that top's IPs; hence the remaining cfgs.
    let devices = [
        ("aes", dif::AES_IV_0_REG_OFFSET),
        #[cfg(feature = "egret")]
        ("adc_ctrl_aon", dif::ADC_CTRL_INTR_ENABLE_REG_OFFSET),
        ("alert_handler", dif::ALERT_HANDLER_INTR_ENABLE_REG_OFFSET),
        ("aon_timer_aon", dif::AON_TIMER_WKUP_CTRL_REG_OFFSET),
        ("csrng", dif::CSRNG_INTR_ENABLE_REG_OFFSET),
        ("edn0", dif::EDN_INTR_ENABLE_REG_OFFSET),
        ("edn1", dif::EDN_INTR_ENABLE_REG_OFFSET),
        ("entropy_src", dif::ENTROPY_SRC_INTR_ENABLE_REG_OFFSET),
        #[cfg(feature = "egret")]
        ("flash_ctrl_core", dif::FLASH_CTRL_INTR_ENABLE_REG_OFFSET),
        ("gpio", dif::GPIO_INTR_ENABLE_REG_OFFSET),
        ("hmac", dif::HMAC_INTR_ENABLE_REG_OFFSET),
        ("i2c0", dif::I2C_INTR_ENABLE_REG_OFFSET),
        ("i2c1", dif::I2C_INTR_ENABLE_REG_OFFSET),
        ("i2c2", dif::I2C_INTR_ENABLE_REG_OFFSET),
        #[cfg(feature = "egret")]
        ("keymgr", dif::KEYMGR_INTR_ENABLE_REG_OFFSET),
        #[cfg(feature = "dragonfly")]
        ("keymgr_dpe", dif::KEYMGR_DPE_INTR_ENABLE_REG_OFFSET),
        ("kmac", dif::KMAC_INTR_ENABLE_REG_OFFSET),
        ("acc", dif::ACC_INTR_ENABLE_REG_OFFSET),
        ("otp_ctrl_core", dif::OTP_CTRL_INTR_ENABLE_REG_OFFSET),
        #[cfg(feature = "egret")]
        ("pattgen", dif::PATTGEN_INTR_ENABLE_REG_OFFSET),
        ("pinmux_aon", dif::PINMUX_MIO_PAD_ATTR_0_REG_OFFSET),
        #[cfg(feature = "egret")]
        ("pwm_aon", dif::PWM_INVERT_REG_OFFSET),
        ("pwrmgr_aon", dif::PWRMGR_INTR_ENABLE_REG_OFFSET),
        // Skip ROM_CTRL which does not have RW registers
        ("rstmgr_aon", dif::RSTMGR_CPU_INFO_CTRL_REG_OFFSET),
        ("rv_timer", dif::RV_TIMER_INTR_ENABLE0_REG_OFFSET),
        #[cfg(feature = "egret")]
        ("sensor_ctrl_aon", dif::SENSOR_CTRL_INTR_ENABLE_REG_OFFSET),
        ("spi_device", dif::SPI_DEVICE_INTR_ENABLE_REG_OFFSET),
        ("spi_host0", dif::SPI_HOST_INTR_ENABLE_REG_OFFSET),
        ("spi_host1", dif::SPI_HOST_INTR_ENABLE_REG_OFFSET),
        #[cfg(feature = "egret")]
        ("sysrst_ctrl_aon", dif::SYSRST_CTRL_INTR_ENABLE_REG_OFFSET),
        ("uart0", dif::UART_INTR_ENABLE_REG_OFFSET),
        ("uart1", dif::UART_INTR_ENABLE_REG_OFFSET),
        ("uart2", dif::UART_INTR_ENABLE_REG_OFFSET),
        ("uart3", dif::UART_INTR_ENABLE_REG_OFFSET),
        #[cfg(feature = "egret")]
        ("usbdev", dif::USBDEV_INTR_ENABLE_REG_OFFSET),
    ];

    // Test the devices that the top has.
    let top = transport.top()?;
    let mut tests = Vec::new();
    for (name, offset) in devices {
        match top.range(name) {
            Ok(range) => tests.push((name, range.base_addr, offset)),
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
