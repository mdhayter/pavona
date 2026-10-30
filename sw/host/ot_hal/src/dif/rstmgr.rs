// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use crate::util::bitfield::BitField;

// The registers whose offsets are the same on every top. The reset causes in RESET_INFO
// differ between tops: see `TopDescription::reset_info_mask()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
#[non_exhaustive]
pub enum RstmgrReg {
    AlertTest = ot_bindgen_dif::RSTMGR_ALERT_TEST_REG_OFFSET,
    ResetReq = ot_bindgen_dif::RSTMGR_RESET_REQ_REG_OFFSET,
    ResetInfo = ot_bindgen_dif::RSTMGR_RESET_INFO_REG_OFFSET,
    AlertRegwen = ot_bindgen_dif::RSTMGR_ALERT_REGWEN_REG_OFFSET,
    AlertInfoCtrl = ot_bindgen_dif::RSTMGR_ALERT_INFO_CTRL_REG_OFFSET,
    AlertInfoAttr = ot_bindgen_dif::RSTMGR_ALERT_INFO_ATTR_REG_OFFSET,
    AlertInfo = ot_bindgen_dif::RSTMGR_ALERT_INFO_REG_OFFSET,
    CpuRegwen = ot_bindgen_dif::RSTMGR_CPU_REGWEN_REG_OFFSET,
    CpuInfoCtrl = ot_bindgen_dif::RSTMGR_CPU_INFO_CTRL_REG_OFFSET,
    CpuInfoAttr = ot_bindgen_dif::RSTMGR_CPU_INFO_ATTR_REG_OFFSET,
    CpuInfo = ot_bindgen_dif::RSTMGR_CPU_INFO_REG_OFFSET,
    SwRstRegwen0 = ot_bindgen_dif::RSTMGR_SW_RST_REGWEN_0_REG_OFFSET,
    SwRstRegwen1 = ot_bindgen_dif::RSTMGR_SW_RST_REGWEN_1_REG_OFFSET,
}

/// BitFields for the CPU_INFO_CTRL register.
pub struct RstmgrCpuRegwen;

impl RstmgrCpuRegwen {
    pub const EN: u32 = 0b1 << ot_bindgen_dif::RSTMGR_CPU_REGWEN_EN_BIT;
}

/// BitFields for the CPU_INFO_CTRL register.
pub struct RstmgrCpuInfoCtrl;

impl RstmgrCpuInfoCtrl {
    pub const EN: u32 = 0b1 << ot_bindgen_dif::RSTMGR_CPU_INFO_CTRL_EN_BIT;

    pub const INDEX: BitField = BitField {
        offset: ot_bindgen_dif::RSTMGR_CPU_INFO_CTRL_INDEX_OFFSET,
        // Relies on mask being continuous
        size: ot_bindgen_dif::RSTMGR_CPU_INFO_CTRL_INDEX_MASK.count_ones(),
    };
}

/// BitFields for the ALERT_INFO_CTRL register.
pub struct RstmgrAlertInfoCtrl;

impl RstmgrAlertInfoCtrl {
    pub const EN: u32 = 0b1 << ot_bindgen_dif::RSTMGR_ALERT_INFO_CTRL_EN_BIT;

    pub const INDEX: BitField = BitField {
        offset: ot_bindgen_dif::RSTMGR_ALERT_INFO_CTRL_INDEX_OFFSET,
        // Relies on mask being continuous
        size: ot_bindgen_dif::RSTMGR_ALERT_INFO_CTRL_INDEX_MASK.count_ones(),
    };
}
