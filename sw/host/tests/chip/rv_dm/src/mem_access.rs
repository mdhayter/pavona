// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result, ensure};
use clap::Parser;
use rand::prelude::*;

use opentitanlib::app::{TransportWrapper, UartRx};
use opentitanlib::execute_test;
use opentitanlib::io::jtag::JtagTap;
use opentitanlib::test_utils::init::InitializeTest;
use opentitanlib::uart::console::UartConsole;
use opentitanlib::util::parse_int::ParseInt;

use ot_bindgen_dif as dif;

#[derive(Debug, Parser)]
struct Opts {
    #[command(flatten)]
    init: InitializeTest,

    /// Path to the ROM binary.
    #[arg(long)]
    rom: PathBuf,

    #[arg(long)]
    rv_dm_delayed_enable: bool,

    /// Seed for random number generator.
    #[arg(long, value_parser = u64::from_str)]
    seed: Option<u64>,

    /// Console receive timeout.
    #[arg(long, value_parser = humantime::parse_duration, default_value = "100s")]
    timeout: Duration,
}

const NUM_ACCESSES_PER_REGION: usize = 32;

/// Names of the boot ROM in the top descriptions, in the order to try: Egret
/// has one ROM, Dragonfly boots from the first of two.
const ROM_NAMES: [&str; 2] = ["rom_ctrl_rom", "rom_ctrl0_rom"];
/// The last 32 bytes of ROM (ROM digest) are not accessible.
const ROM_DIGEST_BYTES: usize = 32;

fn test_mem_access(opts: &Opts, transport: &TransportWrapper) -> Result<()> {
    let seed = opts.seed.unwrap_or_else(|| thread_rng().r#gen());
    log::info!("Random number generator seed is {:x}", seed);
    let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(seed);

    transport.pin_strapping("PINMUX_TAP_RISCV")?.apply()?;
    transport.reset(UartRx::Clear)?;
    let uart = &*transport.uart("console")?;
    uart.set_flow_control(true)?;

    log::info!("rv_dm_delayed_enable: {}", opts.rv_dm_delayed_enable);
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

    let top = transport.top()?;
    let rom = ROM_NAMES
        .iter()
        .find_map(|name| top.range(name).ok())
        .with_context(|| format!("Top {} has none of the ROMs {ROM_NAMES:?}", top.name))?;
    let rom_base_addr = rom.base_addr as u32;
    let rom_accessible_bytes = rom.size_bytes - ROM_DIGEST_BYTES;

    let rom_data = std::fs::read(&opts.rom)?;
    ensure!(rom_data.len() % 4 == 0);

    let rw_regions = [
        (
            "ram_ret",
            top.base_addr("sram_ctrl_ret_aon_ram")? as u32,
            top.size_bytes("sram_ctrl_ret_aon_ram")? as u32,
        ),
        (
            "ram_main",
            top.base_addr("sram_ctrl_main_ram")? as u32,
            top.size_bytes("sram_ctrl_main_ram")? as u32,
        ),
        (
            "acc_imem",
            top.base_addr("acc")? as u32 + dif::ACC_IMEM_REG_OFFSET,
            dif::ACC_IMEM_SIZE_BYTES,
        ),
        (
            "acc_dmem",
            top.base_addr("acc")? as u32 + dif::ACC_DMEM_REG_OFFSET,
            dif::ACC_DMEM_SIZE_BYTES,
        ),
    ];

    // Randomly generate accesses.
    let mut accesses = vec![];
    for (name, base, size) in rw_regions.iter().copied() {
        accesses.extend(
            (0..size)
                .step_by(4)
                .choose_multiple(&mut rng, NUM_ACCESSES_PER_REGION)
                .into_iter()
                .map(|offset| (name, base, offset, rng.r#gen::<u32>())),
        );
    }

    // Shuffle the accesses and perform write.
    accesses.shuffle(&mut rng);
    for (name, base, offset, value) in accesses.iter().copied() {
        log::info!("Writing to {name} (base {base:#x}) offset {offset:#x} with value {value:#x}");
        let addr = base + offset;
        jtag.write_memory32(addr, &[value])?;
    }

    // For ROM contents, instead of using random values, use the known value from ROM binary
    for offset in (0..rom_accessible_bytes as u32)
        .step_by(4)
        .choose_multiple(&mut rng, NUM_ACCESSES_PER_REGION)
    {
        accesses.push((
            "rom",
            rom_base_addr,
            offset,
            if offset as usize <= rom_data.len() {
                u32::from_le_bytes(rom_data[offset as usize..][..4].try_into().unwrap())
            } else {
                0
            },
        ));
    }

    // Shuffle the access again and confirm readback.
    accesses.shuffle(&mut rng);
    for (name, base, offset, value) in accesses {
        let addr = base + offset;
        let mut readback = 0;
        jtag.read_memory32(addr, std::slice::from_mut(&mut readback))?;
        log::info!(
            "Reading from {name} (base {base:#x}) offset {offset:#x} with value {readback:#x} (expecting {value:#x})"
        );
        ensure!(value == readback);
    }

    Ok(())
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    opts.init.init_logging();
    let transport = opts.init.init_target()?;

    execute_test!(test_mem_access, &opts, &transport);

    Ok(())
}
