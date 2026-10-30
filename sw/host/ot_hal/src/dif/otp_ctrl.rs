// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! The OTP controller's register fields that are the same on every top. The register offsets
//! and the OTP memory map differ between tops and come from the top description
//! (`TopDescription::register_offset("otp_ctrl", ..)`, `otp_partition()`, `otp_item()`).

use bitflags::bitflags;

use ot_bindgen_dif as dif;

use crate::top::desc::OtpPartition;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct DirectAccessCmd: u32 {
        const DIGEST = 0b1 << dif::OTP_CTRL_DIRECT_ACCESS_CMD_DIGEST_BIT;
        const RD     = 0b1 << dif::OTP_CTRL_DIRECT_ACCESS_CMD_RD_BIT;
        const WR     = 0b1 << dif::OTP_CTRL_DIRECT_ACCESS_CMD_WR_BIT;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct OtpCtrlStatus: u32 {
        const BUS_INTEG_ERROR       = 0b1 << dif::OTP_CTRL_STATUS_BUS_INTEG_ERROR_BIT;
        const CHECK_PENDING         = 0b1 << dif::OTP_CTRL_STATUS_CHECK_PENDING_BIT;
        const DAI_ERROR             = 0b1 << dif::OTP_CTRL_STATUS_DAI_ERROR_BIT;
        const DAI_IDLE              = 0b1 << dif::OTP_CTRL_STATUS_DAI_IDLE_BIT;
        const KEY_DERIV_FSM_ERROR   = 0b1 << dif::OTP_CTRL_STATUS_KEY_DERIV_FSM_ERROR_BIT;
        const LCI_ERROR             = 0b1 << dif::OTP_CTRL_STATUS_LCI_ERROR_BIT;
        const LFSR_FSM_ERROR        = 0b1 << dif::OTP_CTRL_STATUS_LFSR_FSM_ERROR_BIT;
        const PARTITION_ERROR       = 0b1 << dif::OTP_CTRL_STATUS_PARTITION_ERROR_BIT;
        const SCRAMBLING_FSM_ERROR  = 0b1 << dif::OTP_CTRL_STATUS_SCRAMBLING_FSM_ERROR_BIT;
        const TIMEOUT_ERROR         = 0b1 << dif::OTP_CTRL_STATUS_TIMEOUT_ERROR_BIT;

        const ERRORS =
            Self::BUS_INTEG_ERROR.bits() |
            Self::DAI_ERROR.bits() |
            Self::KEY_DERIV_FSM_ERROR.bits() |
            Self::LCI_ERROR.bits() |
            Self::LFSR_FSM_ERROR.bits() |
            Self::PARTITION_ERROR.bits() |
            Self::SCRAMBLING_FSM_ERROR.bits() |
            Self::TIMEOUT_ERROR.bits();
    }
}

/// Granularities of memory accesses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Granularity {
    /// 32-bit.
    B32,
    /// 64-bit.
    B64,
}

impl OtpPartition {
    /// Returns the granularity of DAI accesses to this partition: 64-bit for secret
    /// partitions, 32-bit otherwise. Digests are always accessed with 64-bit granularity.
    pub fn access_granule(&self) -> Granularity {
        if self.secret {
            Granularity::B64
        } else {
            Granularity::B32
        }
    }
}
