# Copyright lowRISC contributors (OpenTitan project).
# Copyright zeroRISC Inc.
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

package(default_visibility = ["//visibility:public"])

load("//hw/top_dragonfly/data/autogen:defs.bzl", "DRAGONFLY_IPS")
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
        "//hw/top_dragonfly/data/otp:otp_ctrl_mmap.hjson",
        "//hw/top_dragonfly/data/racl:all_files",
        "//hw/top_dragonfly/ip:block_descriptions",
    ],
    seed = ":top_dragonfly_seed.testing.hjson",
    top = ":top_dragonfly.hjson",
)

copy_files(
    name = "gen_completecfg",
    srcs = [":completecfg"],
    dest = "hw/top_dragonfly/data/autogen",
    tags = ["manual"],
)

# Hjson descriptions of the top's IP blocks, as gen_top_host.py expects them.
filegroup(
    name = "host_desc_inputs",
    srcs = [ip.attrs["hjson"] for ip in DRAGONFLY_IPS] + [
        "//hw/top_dragonfly/ip:block_descriptions",
    ],
)

pavona_top_host_desc(
    name = "host_desc",
    completecfg = ":completecfg",
    ip_descriptions = [":host_desc_inputs"],
    jtag_id_pkg = "//hw/top_dragonfly:rtl/jtag_id_pkg.sv",
)

copy_files(
    name = "gen_host_desc",
    srcs = [":host_desc"],
    dest = "hw/top_dragonfly/data/autogen",
    tags = ["manual"],
)
