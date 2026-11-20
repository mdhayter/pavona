# Copyright lowRISC contributors (OpenTitan project).
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
load("//rules/pavona:hw.bzl", "pavona_ip")

PATTGEN = pavona_ip(
    name = "pattgen",
    hjson = "@pavona_pavona//hw/ip/pattgen/data:pattgen.hjson",
    regs = "@pavona_pavona//hw/ip/pattgen/rtl:regs",
)
