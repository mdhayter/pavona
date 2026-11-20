# Copyright lowRISC contributors (OpenTitan project).
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

# The following overlays are used to generate a generic OTP image with fake
# keys. This is useful for testing in dv_sim, fpga and verilator
# environments.
EGRET_OTP_SIGVERIFY_FAKE_KEYS = [
    "@pavona_pavona//sw/device/silicon_creator/rom/keys/fake/otp:json_rot_keys",
]

# This is a set of overlays to generate a generic, standard OTP image.
# Additional overlays can be applied on top to further customize the OTP.
# This set of overlays does not include any of the SECRET[0-2] partitions.
EGRET_STD_OTP_OVERLAYS_WITHOUT_SECRET_PARTITIONS = EGRET_OTP_SIGVERIFY_FAKE_KEYS + [
    "@pavona_pavona//hw/top_egret/data/otp:otp_json_creator_sw_cfg",
    "@pavona_pavona//hw/top_egret/data/otp:otp_json_owner_sw_cfg",
    "@pavona_pavona//hw/top_egret/data/otp:otp_json_alert_digest_cfg",
    "@pavona_pavona//hw/top_egret/data/otp:otp_json_hw_cfg0",
    "@pavona_pavona//hw/top_egret/data/otp:otp_json_hw_cfg1",
]

# This is a set of overlays to generate a generic, standard OTP image.
# Additional overlays can be applied on top to further customize the OTP.
EGRET_STD_OTP_OVERLAYS = EGRET_STD_OTP_OVERLAYS_WITHOUT_SECRET_PARTITIONS + [
    "@pavona_pavona//hw/top_egret/data/otp:otp_json_secret0",
    "@pavona_pavona//hw/top_egret/data/otp:otp_json_secret1",
    "@pavona_pavona//hw/top_egret/data/otp:otp_json_secret2_unlocked",
]
