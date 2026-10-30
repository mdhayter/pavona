// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! The clock manager's external clock (EXTCLK) registers. A clock manager has them if it
//! supports external clock bypass (`ext_clk_bypass` in the top description), and they are then
//! at the same offsets on every top, so they are taken from the Egret clock manager. The other
//! clock manager registers differ between tops.

use bitflags::bitflags;

use super::clkmgr_regs as regs;
use crate::util::bitfield::BitField;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ClkmgrReg {
    AlertTest = regs::CLKMGR_ALERT_TEST_REG_OFFSET as u32,
    ExtclkCtrlRegwen = regs::CLKMGR_EXTCLK_CTRL_REGWEN_REG_OFFSET as u32,
    ExtclkCtrl = regs::CLKMGR_EXTCLK_CTRL_REG_OFFSET as u32,
    ExtclkStatus = regs::CLKMGR_EXTCLK_STATUS_REG_OFFSET as u32,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ClkmgrExtclkCtrlRegwen: u32 {
        const EN = 0b1 << regs::CLKMGR_EXTCLK_CTRL_REGWEN_EN_BIT;
    }
}

/// BitFields for the CLKMGR_EXTCLK_CTRL register.
pub struct ClkmgrExtclkCtrl;

impl ClkmgrExtclkCtrl {
    pub const HI_SPEED_SEL: BitField = BitField {
        offset: regs::CLKMGR_EXTCLK_CTRL_HI_SPEED_SEL_OFFSET as u32,
        // Relies on mask being continuous
        size: regs::CLKMGR_EXTCLK_CTRL_HI_SPEED_SEL_MASK.count_ones(),
    };

    pub const SEL: BitField = BitField {
        offset: regs::CLKMGR_EXTCLK_CTRL_SEL_OFFSET as u32,
        // Relies on mask being continuous
        size: regs::CLKMGR_EXTCLK_CTRL_SEL_MASK.count_ones(),
    };
}
