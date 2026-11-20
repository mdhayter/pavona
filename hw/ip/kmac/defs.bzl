# Copyright lowRISC contributors (OpenTitan project).
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
load("//rules/pavona:hw.bzl", "pavona_ip")

KMAC = pavona_ip(
    name = "kmac",
    hjson = "@pavona_pavona//hw/ip/kmac/data:kmac.hjson",
    regs = "@pavona_pavona//hw/ip/kmac/rtl:regs",
)
