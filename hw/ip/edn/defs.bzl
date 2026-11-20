# Copyright lowRISC contributors (OpenTitan project).
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
load("//rules/pavona:hw.bzl", "pavona_ip")

EDN = pavona_ip(
    name = "edn",
    hjson = "@pavona_pavona//hw/ip/edn/data:edn.hjson",
    regs = "@pavona_pavona//hw/ip/edn/rtl:regs",
)
