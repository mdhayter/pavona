// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#include "hw/top/dt/rstmgr.h"
#include "sw/device/lib/base/abs_mmio.h"
#include "sw/device/lib/base/multibits.h"
#include "sw/device/lib/runtime/hart.h"
#include "sw/device/silicon_creator/lib/drivers/lifecycle.h"
#include "sw/device/silicon_creator/lib/drivers/rstmgr.h"

#include "hw/top/rstmgr_regs.h"

// TODO(#12905): This file includes fake definitions for the functions that are
// used by the silicon_creator bootstrap implementation and to set reset_reason
// but missing in tops with no OTP or life cycle controller due to hardware
// limitations.

#if defined(HAS_OTP_CTRL) || defined(HAS_LC_CTRL)
#error "This file is only for tops with no OTP or life cycle controller"
#endif

void lifecycle_hw_rev_get(lifecycle_hw_rev_t *hw_rev) {
  *hw_rev = (lifecycle_hw_rev_t){
      .silicon_creator_id = 0,
      .product_id = 0,
      .revision_id = 0,
  };
}

// The same as `rstmgr_reset()` in `silicon_creator/lib/drivers/rstmgr.c`,
// which cannot be linked here because the rest of that driver reads OTP.
void rstmgr_reset(void) {
  uint32_t base = dt_rstmgr_primary_reg_block(kDtRstmgrAon);
  abs_mmio_write32(base + RSTMGR_RESET_REQ_REG_OFFSET, kMultiBitBool4True);
  // Wait until the chip resets.
  while (true) {
    wait_for_interrupt();
  }
}
