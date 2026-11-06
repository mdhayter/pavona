// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#include <algorithm>
#include <cstdint>
#include <iostream>
#include <vector>

#include "verilated_toplevel.h"
#include "verilator_memutil.h"
#include "verilator_sim_ctrl.h"

int main(int argc, char **argv) {
  chip_sim_tb top;
  VerilatorMemUtil memutil;
  VerilatorSimCtrl &simctrl = VerilatorSimCtrl::GetInstance();
  simctrl.SetTop(&top, &top.clk_i, &top.rst_ni,
                 VerilatorSimCtrlFlags::ResetPolarityNegative);

  std::string top_scope("TOP.chip_sim_tb.top_wren");

  MemArea rom0(top_scope +
                   ".u_rom_ctrl.gen_rom_scramble_disabled.u_rom."
                   "u_prim_rom",
               0x8000 / 4, 4);
  MemArea flash0(top_scope +
                     ".u_flash_macro_wrapper.gen_flash_banks[0]."
                     "u_flash_macro_bank.u_mem",
                 0x8000 / 8, 8);
  MemArea flash1(top_scope +
                     ".u_flash_macro_wrapper.gen_flash_banks[1]."
                     "u_flash_macro_bank.u_mem",
                 0x8000 / 8, 8);
  // Start with the flash region erased. Future loads can overwrite.
  std::vector<uint8_t> all_ones(flash0.GetSizeBytes());
  std::fill(all_ones.begin(), all_ones.end(), 0xffu);
  flash0.Write(/*word_offset=*/0, all_ones);
  flash1.Write(/*word_offset=*/0, all_ones);

  memutil.RegisterMemoryArea("rom0", 0x8000, &rom0);
  memutil.RegisterMemoryArea("flash0", 0x20000000u, &flash0);
  memutil.RegisterMemoryArea("flash1", 0x20008000u, &flash1);
  simctrl.RegisterExtension(&memutil);

  // see chip_egret_verilator.cc for justification and explanation
  simctrl.SetInitialResetDelay(1000);
  simctrl.SetResetDuration(10);

  std::cout << "Simulation of OpenTitan Wren" << std::endl
            << "=================================" << std::endl
            << std::endl;

  return simctrl.Exec(argc, argv).first;
}
