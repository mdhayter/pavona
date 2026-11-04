// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use super::aon_timer_regs as regs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum AonTimerReg {
    AlertTest = regs::AON_TIMER_ALERT_TEST_REG_OFFSET as u32,
    WkupCtrl = regs::AON_TIMER_WKUP_CTRL_REG_OFFSET as u32,
    WkupTholdLo = regs::AON_TIMER_WKUP_THOLD_LO_REG_OFFSET as u32,
    WkupTholdHi = regs::AON_TIMER_WKUP_THOLD_HI_REG_OFFSET as u32,
    WkupCountLo = regs::AON_TIMER_WKUP_COUNT_LO_REG_OFFSET as u32,
    WkupCountHi = regs::AON_TIMER_WKUP_COUNT_HI_REG_OFFSET as u32,
    WdogRegwen = regs::AON_TIMER_WDOG_REGWEN_REG_OFFSET as u32,
    WdogCtrl = regs::AON_TIMER_WDOG_CTRL_REG_OFFSET as u32,
    WdogBarkThold = regs::AON_TIMER_WDOG_BARK_THOLD_REG_OFFSET as u32,
    WdogBiteThold = regs::AON_TIMER_WDOG_BITE_THOLD_REG_OFFSET as u32,
    WdogCount = regs::AON_TIMER_WDOG_COUNT_REG_OFFSET as u32,
    IntrState = regs::AON_TIMER_INTR_STATE_REG_OFFSET as u32,
    IntrTest = regs::AON_TIMER_INTR_TEST_REG_OFFSET as u32,
    WkupCause = regs::AON_TIMER_WKUP_CAUSE_REG_OFFSET as u32,
}
