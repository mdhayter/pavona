# Copyright lowRISC contributors (OpenTitan project).
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
#
# ------------------- W A R N I N G: A U T O - G E N E R A T E D   C O D E !! -------------------#
# PLEASE DO NOT HAND-EDIT THIS FILE. IT HAS BEEN AUTO-GENERATED WITH THE FOLLOWING COMMAND:
# util/gen_top_sw.py -t hw/top_egret/data/autogen/top_egret.gen.hjson

load("//rules:linker.bzl", "ld_library")
load("//hw/top:defs.bzl", "pavona_require_top")

package(default_visibility = ["//visibility:public"])

cc_library(
    name = "top_egret",
    srcs = [
        "top_egret.c",
    ],
    hdrs = [
        "top_egret.h",
        "top_egret_memory.h",
    ],
    defines = ["OPENTITAN_IS_EGRET"],
    target_compatible_with = pavona_require_top("egret"),
)

# The top's addresses without its name, as "hw/top/top_memory.h". Use it
# through //hw/top:top_memory.
cc_library(
    name = "top_egret_generic_memory",
    hdrs = ["generic/top_memory.h"],
    include_prefix = "hw/top",
    strip_include_prefix = "generic",
    target_compatible_with = pavona_require_top("egret"),
    deps = [":top_egret"],
)

ld_library(
    name = "top_egret_memory",
    defines = [
        "OPENTITAN_TOP_MEMORY_LD=top_egret_memory.ld",
        "OPENTITAN_IS_EGRET",
    ],
    includes = ["top_egret_memory.ld"],
    target_compatible_with = pavona_require_top("egret"),
)
