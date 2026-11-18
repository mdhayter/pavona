# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

load("//rules/pavona:defs.bzl", "PAVONA_CPU")

package(default_visibility = ["//visibility:public"])

# Which pads test software uses: see sw/device/lib/testing/test_pinout.h.
cc_library(
    name = "test_pinout",
    srcs = ["test_pinout.c"],
    target_compatible_with = [PAVONA_CPU],
    deps = [
        "//hw/top/dt",
        "//sw/device/lib/arch:device",
        "//sw/device/lib/dif:pinmux",
        "//sw/device/lib/testing:test_pinout_intf",
        "//sw/device/lib/testing/test_framework:check",
    ],
)
