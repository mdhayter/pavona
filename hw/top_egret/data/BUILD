# Copyright lowRISC contributors (OpenTitan project).
# Copyright zeroRISC Inc.
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

package(default_visibility = ["//visibility:public"])

load("//hw/top_egret/data/autogen:defs.bzl", "EGRET_IPS")
load("//rules:autogen.bzl", "pavona_completecfg", "pavona_top_host_desc")
load("//rules:files.bzl", "copy_files")

filegroup(
    name = "all_files",
    srcs = glob(["**"]),
)

filegroup(
    name = "doc_files",
    srcs = glob(["**/*_testplan.hjson"]),
)

filegroup(
    name = "top_xbar_hjsons",
    srcs = glob(["xbar*.hjson"]),
)

pavona_completecfg(
    "completecfg",
    data = [
        ":top_xbar_hjsons",
        "//hw/top_egret/data/otp:otp_ctrl_mmap.hjson",
        "//hw/top_egret/ip:block_descriptions",
    ],
    seed = ":top_egret_seed.testing.hjson",
    top = ":top_egret.hjson",
)

copy_files(
    name = "gen_completecfg",
    srcs = [":completecfg"],
    dest = "hw/top_egret/data/autogen",
    tags = ["manual"],
)

# Hjson descriptions of the top's IP blocks and its OTP memory map, as gen_top_host.py expects
# them.
filegroup(
    name = "host_desc_inputs",
    srcs = [ip.attrs["hjson"] for ip in EGRET_IPS] + [
        "//hw/top_egret/ip:block_descriptions",
        # The OTP memory map.
        "//hw/top_egret/ip_autogen/otp_ctrl/data:top_egret_otp_ctrl.ipconfig.hjson",
    ],
)

pavona_top_host_desc(
    name = "host_desc",
    completecfg = ":completecfg",
    ip_descriptions = [":host_desc_inputs"],
    jtag_id_pkg = "//hw/top_egret:rtl/jtag_id_pkg.sv",
)

copy_files(
    name = "gen_host_desc",
    srcs = [":host_desc"],
    dest = "hw/top_egret/data/autogen",
    tags = ["manual"],
)
