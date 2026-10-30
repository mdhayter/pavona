// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Result, bail};

use ot_hal::dif::rstmgr::{RstmgrAlertInfoCtrl, RstmgrCpuInfoCtrl, RstmgrCpuRegwen, RstmgrReg};

use crate::app::TransportWrapper;
use crate::io::jtag::{JtagParams, JtagTap};

/// Reads out CPU crashdump info over JTAG.
///
/// Assumes we are already in a TEST_UNLOCKED* state, and TAP straps are cleared.
pub fn read_cpu_crashdump_data(
    transport: &TransportWrapper,
    jtag_params: &JtagParams,
) -> Result<()> {
    let rstmgr = transport.top()?.base_addr("rstmgr_aon")? as u32;
    transport.pin_strapping("PINMUX_TAP_RISCV")?.apply()?;
    let mut jtag = jtag_params.create(transport)?.connect(JtagTap::RiscvTap)?;

    // Read out CPU crashdump info.
    let mut buf = [0u32];

    // Make sure that CPU_INFO is writable.
    jtag.read_memory32(rstmgr + RstmgrReg::CpuRegwen as u32, &mut buf)?;
    if buf[0] & RstmgrCpuRegwen::EN == 0 {
        bail!("CPU_REGWEN.EN is 0, cannot read CPU crash dump.");
    }

    let mut cpu_crashdump_info = [0u32; 16];
    jtag.read_memory32(rstmgr + RstmgrReg::CpuInfoAttr as u32, &mut buf)?;
    let len = buf[0];
    log::info!("CPU Crash Dump Info ({:?} words):", len);
    for i in 0..len {
        jtag.write_memory32(
            rstmgr + RstmgrReg::CpuInfoCtrl as u32,
            &[RstmgrCpuInfoCtrl::INDEX.emplace(i) | RstmgrCpuInfoCtrl::EN],
        )?;
        jtag.read_memory32(rstmgr + RstmgrReg::CpuInfo as u32, &mut buf)?;
        cpu_crashdump_info[i as usize] = buf[0];
    }
    log::info!("  Current MTVAL         = 0x{:x?}", cpu_crashdump_info[0]);
    log::info!("  Current MPEC          = 0x{:x?}", cpu_crashdump_info[1]);
    log::info!("  Last Data Access Addr = 0x{:x?}", cpu_crashdump_info[2]);
    log::info!("  Next PC               = 0x{:x?}", cpu_crashdump_info[3]);
    log::info!("  PC                    = 0x{:x?}", cpu_crashdump_info[4]);
    log::info!("  Previous MTVAL        = 0x{:x?}", cpu_crashdump_info[5]);
    log::info!("  Previous MPEC         = 0x{:x?}", cpu_crashdump_info[6]);
    log::info!("  Double Fault          = 0x{:x?}", cpu_crashdump_info[7]);

    transport.pin_strapping("PINMUX_TAP_RISCV")?.remove()?;
    jtag.disconnect()?;

    Ok(())
}

/// Reads out alert crashdump info over JTAG.
///
/// Assumes we are already in a TEST_UNLOCKED* state, and TAP straps are cleared.
pub fn read_alert_crashdump_data(
    transport: &TransportWrapper,
    jtag_params: &JtagParams,
) -> Result<()> {
    let rstmgr = transport.top()?.base_addr("rstmgr_aon")? as u32;
    transport.pin_strapping("PINMUX_TAP_RISCV")?.apply()?;
    let mut jtag = jtag_params.create(transport)?.connect(JtagTap::RiscvTap)?;

    let mut buf = [0u32];
    jtag.read_memory32(rstmgr + RstmgrReg::AlertInfoAttr as u32, &mut buf)?;
    let len = buf[0];
    log::info!("Alert Crash Dump Info ({:?} words):", len);
    for i in 0..len {
        jtag.write_memory32(
            rstmgr + RstmgrReg::AlertInfoCtrl as u32,
            &[RstmgrAlertInfoCtrl::INDEX.emplace(i) | RstmgrAlertInfoCtrl::EN],
        )?;
        jtag.read_memory32(rstmgr + RstmgrReg::AlertInfo as u32, &mut buf)?;
        log::info!("  Word {:?} = 0x{:x?}", i, buf[0]);
    }

    transport.pin_strapping("PINMUX_TAP_RISCV")?.remove()?;
    jtag.disconnect()?;

    Ok(())
}

/// Reads out reset reason over JTAG.
///
/// Assumes we are already in a TEST_UNLOCKED* state, and TAP straps are cleared.
pub fn read_reset_reason(transport: &TransportWrapper, jtag_params: &JtagParams) -> Result<()> {
    let top = transport.top()?;
    let rstmgr = top.base_addr("rstmgr_aon")? as u32;
    transport.pin_strapping("PINMUX_TAP_RISCV")?.apply()?;
    let mut jtag = jtag_params.create(transport)?.connect(JtagTap::RiscvTap)?;

    let mut buf = [0u32];
    jtag.read_memory32(rstmgr + RstmgrReg::ResetInfo as u32, &mut buf)?;
    let reset_reason = buf[0];

    // The reset causes of the top, in RESET_INFO bit order.
    let mut causes: Vec<_> = top.rstmgr.reset_info.iter().collect();
    causes.sort_by_key(|&(_, bit)| bit);
    log::info!("Reset Reasons:");
    for (name, bit) in causes {
        log::info!("  {name:<26} = {:?}", (reset_reason & (1 << bit)) != 0);
    }

    transport.pin_strapping("PINMUX_TAP_RISCV")?.remove()?;
    jtag.disconnect()?;

    Ok(())
}
