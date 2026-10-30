# Copyright lowRISC contributors (OpenTitan project).
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
#

load("//rules/pavona:hw.bzl", "pavona_top")
load("//hw/top_dragonfly/data/autogen:defs.bzl", "DRAGONFLY_INSTANCES", "DRAGONFLY_IPS")
load("//hw/top_dragonfly/data/otp:defs.bzl", "DRAGONFLY_OTP_SIGVERIFY_FAKE_KEYS", "DRAGONFLY_STD_OTP_OVERLAYS")

DRAGONFLY = pavona_top(
    name = "dragonfly",
    hjson = "//hw/top_dragonfly/data/autogen:top_dragonfly.gen.hjson",
    top_lib = "//hw/top_dragonfly/sw/autogen:top_dragonfly",
    test_pinout = "//hw/top_dragonfly/sw:test_pinout",
    # Execution environments by role, for pavona_test_envs().
    test_envs = {
        "sim_dv": "//hw/top_dragonfly:sim_dv",
        "sim_verilator": "//hw/top_dragonfly:sim_verilator",
    },
    # Host description, embedded in the host tools (//sw/host/ot_hal).
    host_desc = "//hw/top_dragonfly/data:host_desc",
    top_rtl = "//hw/top_dragonfly:rtl_files",
    top_verilator_core = ["lowrisc:dv:top_dragonfly_chip_verilator_sim"],
    top_verilator_binary = {"binary": ["lowrisc_dv_top_dragonfly_chip_verilator_sim_0.1/sim-verilator/Vchip_sim_tb"]},
    top_ld = "//hw/top_dragonfly/sw/autogen:top_dragonfly_memory",
    otp_map = "//hw/top_dragonfly/data/otp:otp_ctrl_mmap.hjson",
    std_otp_overlay = DRAGONFLY_STD_OTP_OVERLAYS,
    otp_sigverify_fake_keys = DRAGONFLY_OTP_SIGVERIFY_FAKE_KEYS,
    # Bitstream splicing: 10240-word OTP memory map.
    bitstream_splice = {"otp_size": 10240, "rom_instance": "rom0"},
    ips = DRAGONFLY_IPS,
    instances = DRAGONFLY_INSTANCES,
    # If ast_init_sized is present (whatever its value: remove it rather than
    # setting it to False), the test ROM reads from OTP the size of the AST
    # configuration to copy (CREATOR_SW_CFG_AST_INIT_SIZE, in words).
    # Otherwise it checks CREATOR_SW_CFG_AST_INIT_EN and, if that is true,
    # copies the AST registers up to and including REGAL.
    ast_init_sized = True,
    secret_cfgs = {
        "testing": "//hw/top_dragonfly/data/autogen:top_dragonfly.secrets.testing.gen.hjson",
    },
)

DRAGONFLY_SLOTS = {
    "sram_main": "0x10000000",
    "sram_main_size": "0x10000",
    "sram_ctn": "0x41000000",
    "rom_ext_sram_ctn": "0x0",
    "sram_ctn_size": "0x100000",
}
