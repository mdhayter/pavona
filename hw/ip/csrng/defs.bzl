# Copyright lowRISC contributors (OpenTitan project).
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
load("//rules/pavona:hw.bzl", "pavona_ip")

CSRNG = pavona_ip(
    name = "csrng",
    hjson = "@pavona_pavona//hw/ip/csrng/data:csrng.hjson",
    regs = "@pavona_pavona//hw/ip/csrng/rtl:regs",
)
