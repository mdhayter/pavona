# Copyright lowRISC contributors (OpenTitan project).
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
load("//rules/pavona:hw.bzl", "pavona_ip")

PWM = pavona_ip(
    name = "pwm",
    hjson = "//hw/top_wren/ip_autogen/pwm/data:pwm.hjson",
    ipconfig = "//hw/top_wren/ip_autogen/pwm/data:top_wren_pwm.ipconfig.hjson",
    regs = "//hw/top_wren/ip_autogen/pwm/rtl:regs",
)
