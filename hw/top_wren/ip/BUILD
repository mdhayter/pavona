# Copyright lowRISC contributors (OpenTitan project).
# Copyright zeroRISC Inc.
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

package(default_visibility = ["//visibility:public"])

filegroup(
    name = "rtl_files",
    srcs = glob(["**"]) + [
        "//hw/top_wren/ip/ast:rtl_files",
        "//hw/top_wren/ip/sensor_ctrl:rtl_files",
        "//hw/top_wren/ip/xbar_main:rtl_files",
        "//hw/top_wren/ip/xbar_peri:rtl_files",
        "//hw/top_wren/ip_autogen/clkmgr:rtl_files",
        "//hw/top_wren/ip_autogen/flash_ctrl:rtl_files",
        "//hw/top_wren/ip_autogen/gpio:rtl_files",
        "//hw/top_wren/ip_autogen/pwm:rtl_files",
        "//hw/top_wren/ip_autogen/pinmux:rtl_files",
        "//hw/top_wren/ip_autogen/pwrmgr:rtl_files",
        "//hw/top_wren/ip_autogen/rstmgr:rtl_files",
        "//hw/top_wren/ip_autogen/rv_core_ibex:rtl_files",
        "//hw/top_wren/ip_autogen/rv_plic:rtl_files",
    ],
)

# Copied and modified from hw/top_egret/ip/BUILD.
filegroup(
    name = "block_descriptions",
    srcs = [
        "//hw/top_wren/ip/ast/data:ast.hjson",
        "//hw/top_wren/ip/sensor_ctrl/data:sensor_ctrl.hjson",
    ],
)
