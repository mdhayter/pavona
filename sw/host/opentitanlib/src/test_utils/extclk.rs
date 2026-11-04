// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! Utilities for enabling or disabling external clock injection through JTAG to the clock manager.

use std::time::Duration;

use thiserror::Error;

use ot_hal::top::TopDescription;
use ot_hal::util::bitfield::BitField;
use ot_hal::util::multibits::MultiBitBool4;

use crate::io::jtag::Jtag;
use crate::test_utils::poll;

/// Available speeds for the external clock.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockSpeed {
    /// Approximately 1/2 of nominal frequency.
    Low,
    /// Approximately nominal frequency.
    High,
}

/// Controls for enabling or disabling external clock injection.
pub struct ExternalClock;

/// The clock manager's external clock registers, from the top description.
struct ExtclkRegs {
    /// Address of EXTCLK_CTRL_REGWEN.
    ctrl_regwen: u32,
    /// Address of EXTCLK_CTRL.
    ctrl: u32,
    /// Address of EXTCLK_STATUS.
    status: u32,
    regwen_en: BitField,
    ctrl_sel: BitField,
    ctrl_hi_speed_sel: BitField,
    status_ack: BitField,
}

impl ExtclkRegs {
    /// Returns the external clock registers of the top's clock manager, if it supports external
    /// clock bypass.
    fn new(top: &TopDescription) -> Result<Self, ExternalClockError> {
        if !top.clkmgr.ext_clk_bypass {
            return Err(ExternalClockError::Unsupported {
                top: top.name.clone(),
            });
        }
        let regs = || -> anyhow::Result<Self> {
            let base = top.base_addr("clkmgr_aon")? as u32;
            let reg =
                |name| -> anyhow::Result<u32> { Ok(base + top.register_offset("clkmgr", name)?) };
            Ok(ExtclkRegs {
                ctrl_regwen: reg("EXTCLK_CTRL_REGWEN")?,
                ctrl: reg("EXTCLK_CTRL")?,
                status: reg("EXTCLK_STATUS")?,
                regwen_en: top.field("clkmgr", "EXTCLK_CTRL_REGWEN", "EN")?,
                ctrl_sel: top.field("clkmgr", "EXTCLK_CTRL", "SEL")?,
                ctrl_hi_speed_sel: top.field("clkmgr", "EXTCLK_CTRL", "HI_SPEED_SEL")?,
                status_ack: top.field("clkmgr", "EXTCLK_STATUS", "ACK")?,
            })
        };
        regs().map_err(|source| ExternalClockError::Top { source })
    }
}

impl ExternalClock {
    /// Durations for polling of the STATUS register.
    const POLL_TIMEOUT: Duration = Duration::from_millis(500);
    const POLL_DELAY: Duration = Duration::from_millis(5);

    /// Enable the external clock at the given speed.
    pub fn enable(
        jtag: &mut dyn Jtag,
        top: &TopDescription,
        speed: ClockSpeed,
    ) -> Result<(), ExternalClockError> {
        let regs = ExtclkRegs::new(top)?;
        if !Self::write_enabled(jtag, &regs)? {
            return Err(ExternalClockError::WriteDisabled);
        }

        // Fields and their values for the EXTCLK_CTRL register.
        let hi_speed = match speed {
            ClockSpeed::Low => MultiBitBool4::False,
            ClockSpeed::High => MultiBitBool4::True,
        };
        let fields = [
            (regs.ctrl_sel, MultiBitBool4::True),
            (regs.ctrl_hi_speed_sel, hi_speed),
        ];

        // OR the fields together to get the register value.
        let extclk_ctrl = fields.into_iter().fold(0, |acc, (field, value)| {
            acc | field.emplace(u8::from(value) as u32)
        });

        // Set the EXTCLK_CTRL register.
        jtag.write_memory32(regs.ctrl, &[extclk_ctrl])
            .map_err(|source| ExternalClockError::Jtag { source })?;

        Self::wait_for_ack(jtag, &regs)?;

        Ok(())
    }

    /// Disable the external clock.
    pub fn disable(jtag: &mut dyn Jtag, top: &TopDescription) -> Result<(), ExternalClockError> {
        let regs = ExtclkRegs::new(top)?;
        if !Self::write_enabled(jtag, &regs)? {
            return Err(ExternalClockError::WriteDisabled);
        }

        // Set the EXTCLK_CTRL register.
        let extclk_ctrl = regs.ctrl_sel.emplace(u8::from(MultiBitBool4::False) as u32);
        jtag.write_memory32(regs.ctrl, &[extclk_ctrl])
            .map_err(|source| ExternalClockError::Jtag { source })?;

        Self::wait_for_ack(jtag, &regs)?;

        Ok(())
    }

    /// Returns the CLKMGR_EXTCLK_REGWEN setting which shows whether the
    /// `EXTCLK_CTRL` register can be written.
    fn write_enabled(jtag: &mut dyn Jtag, regs: &ExtclkRegs) -> Result<bool, ExternalClockError> {
        let mut buf = [0];
        jtag.read_memory32(regs.ctrl_regwen, &mut buf)
            .map_err(|source| ExternalClockError::Jtag { source })?;

        Ok(regs.regwen_en.extract(buf[0]) == 1)
    }

    /// Wait until the clock manager reports that the clock has changed to the external source.
    fn wait_for_ack(jtag: &mut dyn Jtag, regs: &ExtclkRegs) -> Result<(), ExternalClockError> {
        poll::poll_until(Self::POLL_TIMEOUT, Self::POLL_DELAY, || {
            // Check the status register.
            let mut status_buf = [0];
            jtag.read_memory32(regs.status, &mut status_buf)
                .map_err(|source| ExternalClockError::Jtag { source })?;

            Ok(regs.status_ack.extract(status_buf[0]) == u8::from(MultiBitBool4::True) as u32)
        })
        .map_err(|source| ExternalClockError::Nack { source })
    }
}

/// Failures when setting the external clock.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ExternalClockError {
    #[error("failed to find the clkmgr external clock registers in the top description")]
    Top { source: anyhow::Error },

    #[error("the clock manager of top {top} does not support external clock bypass")]
    Unsupported { top: String },

    #[error("failed to communicate over JTAG")]
    Jtag { source: anyhow::Error },

    #[error("did not read ACK for external clock change")]
    Nack { source: anyhow::Error },

    #[error("writing to external clock registers is disabled")]
    WriteDisabled,
}
