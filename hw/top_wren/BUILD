# Copyright lowRISC contributors (OpenTitan project).
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

load(
    "//rules/pavona:defs.bzl",
    "DEFAULT_TEST_FAILURE_MSG",
    "DEFAULT_TEST_SUCCESS_MSG",
    "sim_verilator",
)

package(default_visibility = ["//visibility:public"])

filegroup(
    name = "rtl_files",
    srcs = glob(
        ["**"],
    ) + [
        "//hw/top_egret:rtl_files",  # Add Egret RTL files for alert_handler_pkg and top_pkg
        "//hw/top_wren/data:all_files",
        "//hw/top_wren/dv/verilator:all_files",
        "//hw/top_wren/ip:rtl_files",
    ],
    visibility = ["//visibility:public"],
)

filegroup(
    name = "doc_files",
    srcs = glob(
        [
            "**/*.md",
            "**/*.hjson",
            "**/*.html",
        ],
        allow_empty = True,
        exclude_directories = 0,
    ) + [
        "//hw/top_wren/data:doc_files",
    ],
    visibility = ["//visibility:public"],
)

###########################################################################
# Sim Verilator Environments
#
# The sim_verilator_base target is only meant to be used for building ROMs
# and other items without `testonly=True`.
#
# Copied (and modified) from hw/top_egret/BUILD. Wren has no OTP, and its
# Verilator model loads an unscrambled ROM image (`rom32`). Its test
# programs boot from the test ROM with no ROM_EXT, so there is no slot
# layout.
###########################################################################
sim_verilator(
    name = "sim_verilator_base",
    design = "wren",
    exec_env = "sim_verilator",
    libs = [
        "//sw/device/lib/arch:boot_stage_rom_ext",
        "//sw/device/lib/arch:sim_verilator",
        "//hw/top_wren/sw/dt:sim_verilator",
    ],
    linker_script = "//sw/device/lib/testing/test_framework:ottf_ld_silicon_creator_slot_a",
    test_cmd = "testing-not-supported",
)

sim_verilator(
    name = "sim_verilator",
    testonly = True,
    args = [
        "--rcfile=",
        "--logging=info",
        "--top=wren",
        "--interface=verilator",
        "--verilator-bin=$(rootpath //hw:verilator)",
        "--verilator-rom={rom:rom32}",
        "--verilator-flash={firmware}",
    ],
    base = ":sim_verilator_base",
    data = [
        "//hw:fusesoc_ignore",
        "//hw:verilator",
    ],
    exec_env = "sim_verilator",
    param = {
        "exit_success": DEFAULT_TEST_SUCCESS_MSG,
        "exit_failure": DEFAULT_TEST_FAILURE_MSG,
    },
    rom = "//sw/device/lib/testing/test_rom:test_rom",
    test_cmd = """
        --exec="console --non-interactive --exit-success='{exit_success}' --exit-failure='{exit_failure}'"
        no-op
    """,
)
