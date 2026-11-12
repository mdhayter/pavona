# Copyright lowRISC contributors (OpenTitan project).
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
#

load("//rules/pavona:hw.bzl", "pavona_top")
load("//hw/top_wren/data/autogen:defs.bzl", "WREN_IPS")

WREN = pavona_top(
    name = "wren",
    hjson = "//hw/top_wren/data/autogen:top_wren.gen.hjson",
    top_lib = "//hw/top_wren/sw/autogen:top_wren",
    # Host description, embedded in the host tools (//sw/host/ot_hal).
    host_desc = "//hw/top_wren/data:host_desc",
    top_rtl = "//hw/top_wren:rtl_files",
    top_verilator_core = ["lowrisc:dv:top_wren_chip_verilator_sim"],
    top_verilator_binary = {"binary": ["lowrisc_dv_top_wren_chip_verilator_sim_0.1/sim-verilator/Vchip_sim_tb"]},
    top_ld = "//hw/top_wren/sw/autogen:top_wren_memory",
    ips = WREN_IPS,
    secret_cfgs = {
        "testing": "//hw/top_wren/data/autogen:top_wren.secrets.testing.gen.hjson",
    },
)
