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
use ot_hal::dif::{
    acc_regs, adc_ctrl_regs, aes_regs, aon_timer_regs, csrng_regs, edn_regs, entropy_src_regs,
    hmac_regs, i2c_regs, keymgr_dpe_regs, keymgr_regs, kmac_regs, pattgen_regs, rv_timer_regs,
    spi_device_regs, spi_host_regs, sysrst_ctrl_regs, uart_regs, usbdev_regs,
};

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

/// A register to test in each instance of an IP block.
#[derive(Clone, Copy)]
enum Reg {
    /// The offset of the register in a block that every top takes from hw/ip.
    Fixed(usize),
    /// The name of the register in a block generated per top, looked up in the top description.
    Top(&'static str),
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

    // IP block types and a read/write register of each. Blocks that every top takes from
    // hw/ip give the register's offset from their hw/ip description; blocks generated per top
    // name the register, whose offset comes from the top description. Every instance of each
    // block in the top is tested, wherever it is in the address map; blocks the top doesn't
    // have are skipped.
    #[rustfmt::skip]
    let devices = [
        ("acc", Reg::Fixed(acc_regs::ACC_INTR_ENABLE_REG_OFFSET)),
        ("adc_ctrl", Reg::Fixed(adc_ctrl_regs::ADC_CTRL_INTR_ENABLE_REG_OFFSET)),
        ("aes", Reg::Fixed(aes_regs::AES_IV_0_REG_OFFSET)),
        ("alert_handler", Reg::Top("INTR_ENABLE")),
        ("aon_timer", Reg::Fixed(aon_timer_regs::AON_TIMER_WKUP_CTRL_REG_OFFSET)),
        ("csrng", Reg::Fixed(csrng_regs::CSRNG_INTR_ENABLE_REG_OFFSET)),
        ("edn", Reg::Fixed(edn_regs::EDN_INTR_ENABLE_REG_OFFSET)),
        ("entropy_src", Reg::Fixed(entropy_src_regs::ENTROPY_SRC_INTR_ENABLE_REG_OFFSET)),
        ("flash_ctrl", Reg::Top("INTR_ENABLE")),
        ("gpio", Reg::Top("INTR_ENABLE")),
        ("hmac", Reg::Fixed(hmac_regs::HMAC_INTR_ENABLE_REG_OFFSET)),
        ("i2c", Reg::Fixed(i2c_regs::I2C_INTR_ENABLE_REG_OFFSET)),
        ("keymgr", Reg::Fixed(keymgr_regs::KEYMGR_INTR_ENABLE_REG_OFFSET)),
        ("keymgr_dpe", Reg::Fixed(keymgr_dpe_regs::KEYMGR_DPE_INTR_ENABLE_REG_OFFSET)),
        ("kmac", Reg::Fixed(kmac_regs::KMAC_INTR_ENABLE_REG_OFFSET)),
        ("otp_ctrl", Reg::Top("INTR_ENABLE")),
        ("pattgen", Reg::Fixed(pattgen_regs::PATTGEN_INTR_ENABLE_REG_OFFSET)),
        ("pinmux", Reg::Top("MIO_PAD_ATTR_0")),
        ("pwm", Reg::Top("INVERT")),
        ("pwrmgr", Reg::Top("INTR_ENABLE")),
        // Skip ROM_CTRL which does not have RW registers
        ("rstmgr", Reg::Top("CPU_INFO_CTRL")),
        ("rv_timer", Reg::Fixed(rv_timer_regs::RV_TIMER_INTR_ENABLE0_REG_OFFSET)),
        ("sensor_ctrl", Reg::Top("INTR_ENABLE")),
        ("spi_device", Reg::Fixed(spi_device_regs::SPI_DEVICE_INTR_ENABLE_REG_OFFSET)),
        ("spi_host", Reg::Fixed(spi_host_regs::SPI_HOST_INTR_ENABLE_REG_OFFSET)),
        ("sysrst_ctrl", Reg::Fixed(sysrst_ctrl_regs::SYSRST_CTRL_INTR_ENABLE_REG_OFFSET)),
        ("uart", Reg::Fixed(uart_regs::UART_INTR_ENABLE_REG_OFFSET)),
        ("usbdev", Reg::Fixed(usbdev_regs::USBDEV_INTR_ENABLE_REG_OFFSET)),
    ];

    let top = transport.top()?;
    let mut tests = Vec::new();
    for (ip, reg) in devices {
        let instances: Vec<_> = top.devices_of(ip)?.collect();
        if instances.is_empty() {
            log::info!("Skipping {ip}: not in top {}", top.name);
            continue;
        }
        let offset = match reg {
            Reg::Fixed(offset) => offset as u32,
            Reg::Top(name) => top.register_offset(ip, name)?,
        };
        for (name, device) in instances {
            tests.push((name, device.range.base_addr, offset));
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
