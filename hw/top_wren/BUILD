# Copyright lowRISC contributors (OpenTitan project).
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

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
