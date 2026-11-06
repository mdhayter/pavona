// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// Copied (and modified) from hw/top_egret/sw/dt/sim_verilator.c.

#include <stdint.h>

#include "hw/top/dt/api.h"  // Generated

// The frequencies in hw/top_wren/dv/verilator/chip_sim_tb.sv: it
// drives the main, IO and USB clocks from one 500 kHz clock, and divides that
// by 4 for the AON clock.
static const uint32_t clock_freqs[kDtClockCount] = {
    [kDtClockMain] = 500 * 1000,   [kDtClockIo] = 500 * 1000,
    [kDtClockUsb] = 500 * 1000,    [kDtClockAon] = 125 * 1000,
    [kDtClockIoDiv2] = 250 * 1000, [kDtClockIoDiv4] = 125 * 1000,
};

uint32_t dt_clock_frequency(dt_clock_t clk) {
  if (clk < kDtClockCount) {
    return clock_freqs[clk];
  }
  return 0;
}
