// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// Copied (and modified) from hw/top_egret/sw/dt/fpga_cw310.c.

#include <stdint.h>

#include "hw/top/dt/api.h"  // Generated

// The clocks of Wren's Genesys 2 build (`hw/top_wren/rtl/clkgen_xil7series.sv`
// and the chip template): IO runs from the main clock.
static const uint32_t clock_freqs[kDtClockCount] = {
    [kDtClockMain] = 10 * 1000 * 1000,  [kDtClockIo] = 10 * 1000 * 1000,
    [kDtClockUsb] = 48 * 1000 * 1000,   [kDtClockAon] = 250 * 1000,
    [kDtClockIoDiv2] = 5 * 1000 * 1000, [kDtClockIoDiv4] = 2500 * 1000,
};

uint32_t dt_clock_frequency(dt_clock_t clk) {
  if (clk < kDtClockCount) {
    return clock_freqs[clk];
  }
  return 0;
}
