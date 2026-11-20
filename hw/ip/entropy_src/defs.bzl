# Copyright lowRISC contributors (OpenTitan project).
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
load("//rules/pavona:hw.bzl", "pavona_ip")

ENTROPY_SRC = pavona_ip(
    name = "entropy_src",
    hjson = "@pavona_pavona//hw/ip/entropy_src/data:entropy_src.hjson",
    regs = "@pavona_pavona//hw/ip/entropy_src/rtl:regs",
)
