# Copyright zeroRISC Inc.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
load("//rules/pavona:hw.bzl", "pavona_ip")

ASCON = pavona_ip(
    name = "ascon",
    hjson = "@pavona_pavona//hw/ip/ascon/data:ascon.hjson",
    regs = "@pavona_pavona//hw/ip/ascon/rtl:regs",
)
