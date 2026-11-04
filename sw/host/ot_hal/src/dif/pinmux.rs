// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use bitflags::bitflags;

use ot_bindgen_dif as dif;

bitflags! {
    /// Pad attributes as the device's pinmux DIF takes them (`dif_pinmux_pad_attr_flags_t`
    /// in `dif_pinmux.h`), which is how `test_utils::pinmux_config` sends them.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct PinmuxPadAttr: u32 {
        const OD_EN = dif::dif_pinmux_pad_attr_flags_kDifPinmuxPadAttrOpenDrain;
        const SCHMITT_EN = dif::dif_pinmux_pad_attr_flags_kDifPinmuxPadAttrSchmittTrigger;
        const KEEPER_EN = dif::dif_pinmux_pad_attr_flags_kDifPinmuxPadAttrKeeper;
        const PULL_SELECT = dif::dif_pinmux_pad_attr_flags_kDifPinmuxPadAttrPullResistorUp;
        const PULL_EN = dif::dif_pinmux_pad_attr_flags_kDifPinmuxPadAttrPullResistorEnable;
        const VIRTUAL_OD_EN = dif::dif_pinmux_pad_attr_flags_kDifPinmuxPadAttrVirtualOpenDrain;
        const INVERT = dif::dif_pinmux_pad_attr_flags_kDifPinmuxPadAttrInvertLevel;
    }
}
