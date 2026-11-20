# Copyright lowRISC contributors (OpenTitan project).
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
load("//rules/pavona:hw.bzl", "pavona_ip")

RV_DM = pavona_ip(
    name = "rv_dm",
    hjson = "@pavona_pavona//hw/ip/rv_dm/data:rv_dm.hjson",
    regs = "@pavona_pavona//hw/ip/rv_dm/rtl:regs",
)
