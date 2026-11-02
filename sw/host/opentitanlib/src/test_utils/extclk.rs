// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! Utilities for enabling or disabling external clock injection through JTAG to the clock manager.

use std::time::Duration;

use thiserror::Error;

use ot_hal::dif::clkmgr::{ClkmgrExtclkCtrl, ClkmgrReg};
use ot_hal::top::TopDescription;
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

impl ExternalClock {
    /// Returns the base address of the clock manager, if it supports external clock bypass.
    fn clkmgr_base_addr(top: &TopDescription) -> Result<u32, ExternalClockError> {
        if !top.clkmgr.ext_clk_bypass {
            return Err(ExternalClockError::Unsupported {
                top: top.name.clone(),
            });
        }
        top.base_addr("clkmgr_aon")
            .map(|addr| addr as u32)
            .map_err(|source| ExternalClockError::Top { source })
    }

    /// Durations for polling of the STATUS register.
    const POLL_TIMEOUT: Duration = Duration::from_millis(500);
    const POLL_DELAY: Duration = Duration::from_millis(5);

    /// Enable the external clock at the given speed.
    pub fn enable(
        jtag: &mut dyn Jtag,
        top: &TopDescription,
        speed: ClockSpeed,
    ) -> Result<(), ExternalClockError> {
        let clkmgr = Self::clkmgr_base_addr(top)?;
        if !Self::write_enabled(jtag, clkmgr)? {
            return Err(ExternalClockError::WriteDisabled);
        }

        // Fields and their values for the EXTCLK_CTRL register.
        let hi_speed = match speed {
            ClockSpeed::Low => MultiBitBool4::False,
            ClockSpeed::High => MultiBitBool4::True,
        };
        let fields = [
            (ClkmgrExtclkCtrl::SEL, MultiBitBool4::True),
            (ClkmgrExtclkCtrl::HI_SPEED_SEL, hi_speed),
        ];

        // OR the fields together to get the register value.
        let extclk_ctrl = fields.into_iter().fold(0, |acc, (field, value)| {
            acc | field.emplace(u8::from(value) as u32)
        });

        // Set the EXTCLK_CTRL register.
        jtag.write_memory32(clkmgr + ClkmgrReg::ExtclkCtrl as u32, &[extclk_ctrl])
            .map_err(|source| ExternalClockError::Jtag { source })?;

        Self::wait_for_ack(jtag, clkmgr)?;

        Ok(())
    }

    /// Disable the external clock.
    pub fn disable(jtag: &mut dyn Jtag, top: &TopDescription) -> Result<(), ExternalClockError> {
        let clkmgr = Self::clkmgr_base_addr(top)?;
        if !Self::write_enabled(jtag, clkmgr)? {
            return Err(ExternalClockError::WriteDisabled);
        }

        // Set the EXTCLK_CTRL register.
        let extclk_ctrl = ClkmgrExtclkCtrl::SEL.emplace(u8::from(MultiBitBool4::False) as u32);
        jtag.write_memory32(clkmgr + ClkmgrReg::ExtclkCtrl as u32, &[extclk_ctrl])
            .map_err(|source| ExternalClockError::Jtag { source })?;

        Self::wait_for_ack(jtag, clkmgr)?;

        Ok(())
    }

    /// Returns the CLKMGR_EXTCLK_REGWEN setting which shows whether the
    /// `EXTCLK_CTRL` register can be written.
    fn write_enabled(jtag: &mut dyn Jtag, clkmgr: u32) -> Result<bool, ExternalClockError> {
        let mut buf = [0];
        jtag.read_memory32(clkmgr + ClkmgrReg::ExtclkCtrlRegwen as u32, &mut buf)
            .map_err(|source| ExternalClockError::Jtag { source })?;

        Ok(buf[0] & 1 == 1)
    }

    /// Wait until the clock manager reports that the clock has changed to the external source.
    fn wait_for_ack(jtag: &mut dyn Jtag, clkmgr: u32) -> Result<(), ExternalClockError> {
        poll::poll_until(Self::POLL_TIMEOUT, Self::POLL_DELAY, || {
            // Check the status register.
            let mut status_buf = [0];
            jtag.read_memory32(clkmgr + ClkmgrReg::ExtclkStatus as u32, &mut status_buf)
                .map_err(|source| ExternalClockError::Jtag { source })?;

            Ok(status_buf[0] == u8::from(MultiBitBool4::True) as u32)
        })
        .map_err(|source| ExternalClockError::Nack { source })
    }
}

/// Failures when setting the external clock.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ExternalClockError {
    #[error("failed to find clkmgr in the top description")]
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
