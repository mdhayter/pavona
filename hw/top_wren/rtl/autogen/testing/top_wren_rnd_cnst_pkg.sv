// Copyright lowRISC contributors (OpenTitan project).
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
//
// ------------------- W A R N I N G: A U T O - G E N E R A T E D   C O D E !! -------------------//
// PLEASE DO NOT HAND-EDIT THIS FILE. IT HAS BEEN AUTO-GENERATED WITH THE FOLLOWING COMMAND:
//
// util/gen_top_sv.py --completecfg hw/top_wren/data/autogen/top_wren.gen.hjson
//                    --seedcfg hw/top_wren/data/autogen/top_wren.secrets.testing.gen.hjson
//
// File is generated based on the following seed configuration:
//   hw/top_wren/data/autogen/top_wren.secrets.testing.gen.hjson


package top_wren_rnd_cnst_pkg;

  ////////////////////////////////////////////
  // flash_ctrl
  ////////////////////////////////////////////
  // Compile-time random bits for default address key
  parameter flash_ctrl_pkg::flash_key_t RndCnstFlashCtrlAddrKey = {
    128'h1FD48846_C3BA83E3_9A7186FD_D4649B21
  };

  // Compile-time random bits for default data key
  parameter flash_ctrl_pkg::flash_key_t RndCnstFlashCtrlDataKey = {
    128'h47D01819_B16C7C2D_5E1021D1_897C1938
  };

  // Compile-time random bits for default seeds
  parameter flash_ctrl_top_specific_pkg::all_seeds_t RndCnstFlashCtrlAllSeeds = {
    256'h47816D03_5BF580CE_B1ADBE5B_7226EE1E_344FEE0B_BBBA2383_B8A3DB07_A5639029,
    256'hE47D09B8_33A6520B_ED72D9C5_FC49DF86_11D88407_9E486963_1F812D85_5B1A4DFC
  };

  // Compile-time random bits for initial LFSR seed
  parameter flash_ctrl_top_specific_pkg::lfsr_seed_t RndCnstFlashCtrlLfsrSeed = {
    32'h11DFCD9C
  };

  // Compile-time random permutation for LFSR output
  parameter flash_ctrl_top_specific_pkg::lfsr_perm_t RndCnstFlashCtrlLfsrPerm = {
    160'hE915984C_A77EBEDE_522243B1_51DB7E04_969BB0C1
  };

  ////////////////////////////////////////////
  // sram_ctrl_main
  ////////////////////////////////////////////
  // Compile-time random reset value for SRAM scrambling key.
  parameter otp_ctrl_pkg::sram_key_t RndCnstSramCtrlMainSramKey = {
    128'h8ED8E581_4947E9F5_F7767858_1690CBB6
  };

  // Compile-time random reset value for SRAM scrambling nonce.
  parameter otp_ctrl_pkg::sram_nonce_t RndCnstSramCtrlMainSramNonce = {
    128'hD6F70549_93B1D678_194A9A91_BA847FC6
  };

  // Compile-time random bits for initial LFSR seed
  parameter sram_ctrl_pkg::lfsr_seed_t RndCnstSramCtrlMainLfsrSeed = {
    64'h8E77019E_CB2F3015
  };

  // Compile-time random permutation for LFSR output
  parameter sram_ctrl_pkg::lfsr_perm_t RndCnstSramCtrlMainLfsrPerm = {
    128'h8A42AFCA_80F8254E_77CD86E6_03CB41D0,
    256'hFB0BFA71_31E90193_1EE802C2_5161F5D6_487EAEDA_FD23163B_68F5A4D5_6B45C797
  };

  ////////////////////////////////////////////
  // rom_ctrl
  ////////////////////////////////////////////
  // Fixed nonce used for address / data scrambling
  parameter bit [63:0] RndCnstRomCtrlScrNonce = {
    64'h3F143518_14944255
  };

  // Randomised constant used as a scrambling key for ROM data
  parameter bit [127:0] RndCnstRomCtrlScrKey = {
    128'hCD7674E2_F195009A_136C9873_B3301ACC
  };

  ////////////////////////////////////////////
  // rv_core_ibex
  ////////////////////////////////////////////
  // Default seed of the PRNG used for random instructions.
  parameter ibex_pkg::lfsr_seed_t RndCnstRvCoreIbexLfsrSeed = {
    32'hD516A19D
  };

  // Permutation applied to the LFSR of the PRNG used for random instructions.
  parameter ibex_pkg::lfsr_perm_t RndCnstRvCoreIbexLfsrPerm = {
    160'hFB110B57_9B32927B_B5EB0789_894F5977_634104A3
  };

  // Default icache scrambling key
  parameter logic [ibex_pkg::SCRAMBLE_KEY_W-1:0] RndCnstRvCoreIbexIbexKeyDefault = {
    128'hBFFA8840_9298B990_A9A60E4B_94E3257A
  };

  // Default icache scrambling nonce
  parameter logic [ibex_pkg::SCRAMBLE_NONCE_W-1:0] RndCnstRvCoreIbexIbexNonceDefault = {
    64'h2141CAE5_78818FCD
  };

endpackage : top_wren_rnd_cnst_pkg
