# Copyright lowRISC contributors (OpenTitan project).
# Copyright zeroRISC Inc.
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

# Copied and modified from hw/top_egret/data/BUILD.

package(default_visibility = ["//visibility:public"])

load("//rules:autogen.bzl", "pavona_completecfg")
load("//rules:files.bzl", "copy_files")

filegroup(
    name = "all_files",
    srcs = glob(["**"]),
)

filegroup(
    name = "doc_files",
    srcs = glob(["**/*.hjson"]),
)

filegroup(
    name = "top_xbar_hjsons",
    srcs = glob(["xbar*.hjson"]),
)

pavona_completecfg(
    "completecfg",
    data = [
        ":top_xbar_hjsons",
        "//hw/top_scafi_deprecated/ip:block_descriptions",
    ],
    seed = ":top_scafi_deprecated_seed.testing.hjson",
    top = ":top_scafi_deprecated.hjson",
)

copy_files(
    name = "gen_completecfg",
    srcs = [":completecfg"],
    dest = "hw/top_scafi_deprecated/data/autogen",
    tags = ["manual"],
)
