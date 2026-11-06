// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#include "sw/device/silicon_creator/lib/drivers/lifecycle.h"
#include "sw/device/silicon_creator/lib/drivers/rstmgr.h"

// TODO(#12905): This file includes fake definitions for the functions that are
// used by the silicon_creator bootstrap implementation and to set reset_reason
// but missing in tops with no OTP or life cycle controller due to hardware
// limitations.

#ifndef OPENTITAN_IS_SCAFI_DEPRECATED
#error "This file should be compiled only for the Scafi_Deprecated top level"
#endif

void lifecycle_hw_rev_get(lifecycle_hw_rev_t *hw_rev) {
  *hw_rev = (lifecycle_hw_rev_t){
      .silicon_creator_id = 0,
      .product_id = 0,
      .revision_id = 0,
  };
}

void rstmgr_reset(void) {
  while (true) {
  }
}
