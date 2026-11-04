# Copyright lowRISC contributors (OpenTitan project).
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

load("@rules_rust//rust:defs.bzl", "rust_library", "rust_library_group")
load("@rules_rust_bindgen//:defs.bzl", "rust_bindgen", "rust_bindgen_library")
load("//rules/pavona:top_explicit_for_tests.bzl", "pavona_for_top", "pavona_require_tops")
load("//sw/host/ot_hal/bindgen:defs.bzl", "CROSS_CHECK_TOPS")

package(default_visibility = ["//visibility:public"])

_RUSTC_FLAGS = [
    "--allow=non_snake_case",
    "--allow=non_upper_case_globals",
    "--allow=non_camel_case_types",
]

# The crates below are built from device headers that do not depend on the top, so they are
# the same for every top.

# ot_bindgen_dif: the life cycle state and token types, and the pinmux pad attribute flags.
cc_library(
    name = "difs",
    hdrs = ["difs.h"],
    deps = [
        "//sw/device/lib/dif:lc_ctrl_types",
        "//sw/device/lib/dif:pinmux_types",
    ],
)

rust_bindgen_library(
    name = "dif",
    bindgen_flags = [
        "--allowlist-type=dif_lc_ctrl_state",
        "--allowlist-type=dif_lc_ctrl_token",
        "--allowlist-type=dif_pinmux_pad_attr_flags",
    ],
    cc_lib = ":difs",
    crate_name = "ot_bindgen_dif",
    header = "difs.h",
    rustc_flags = _RUSTC_FLAGS,
)

# ot_bindgen_alert: the alert class, enable and escalation types of the ROM's alert driver.
rust_bindgen_library(
    name = "alert",
    bindgen_flags = [
        "--allowlist-type=AlertClass",
        "--allowlist-type=AlertEnable",
        "--allowlist-type=AlertEscalate",
    ],
    cc_lib = "//sw/device/silicon_creator/lib/drivers:alert_headers",
    crate_name = "ot_bindgen_alert",
    header = "//sw/device/silicon_creator/lib/drivers:alert.h",
    rustc_flags = _RUSTC_FLAGS,
)

# ot_bindgen_multibits: the multibit boolean values of sw/device/lib/base.
rust_bindgen_library(
    name = "multibits",
    bindgen_flags = [
        "--allowlist-type=multi_bit_bool",
    ],
    cc_lib = "//sw/device/lib/base:multibits",
    crate_name = "ot_bindgen_multibits",
    header = "//sw/device/lib/base:multibits.h",
    rustc_flags = _RUSTC_FLAGS,
)

rust_library_group(
    name = "bindgen",
    deps = [
        ":alert",
        ":dif",
        ":multibits",
    ],
)

# ot_bindgen_top_dif, built for each top in CROSS_CHECK_TOPS: the clkmgr, otp_ctrl and rstmgr
# definitions that that top's `//sw/host/ot_hal:ot_hal_<top>_test` checks its description
# against.
cc_library(
    name = "top_difs",
    hdrs = ["top_difs.h"],
    target_compatible_with = pavona_require_tops(CROSS_CHECK_TOPS),
    deps = [
        "//hw/top:clkmgr_c_regs",
        "//sw/device/lib/dif:otp_ctrl",
        "//sw/device/lib/dif:rstmgr",
    ],
)

rust_bindgen(
    name = "top_dif_bindgen",
    bindgen_flags = [
        "--allowlist-type=dif_rstmgr_reset_info",
        "--allowlist-var=CLKMGR_EXTCLK_.*",
        "--allowlist-var=OTP_CTRL_.*_BIT",
        "--allowlist-var=OTP_CTRL_.*_OFFSET",
        "--allowlist-var=OTP_CTRL_.*_SIZE",
        "--allowlist-var=RSTMGR_.*_BIT",
        "--allowlist-var=RSTMGR_.*_MASK",
        "--allowlist-var=RSTMGR_.*_OFFSET",
    ],
    cc_lib = ":top_difs",
    header = "top_difs.h",
    target_compatible_with = pavona_require_tops(CROSS_CHECK_TOPS),
)

[
    pavona_for_top(
        name = "top_dif_bindgen_{}".format(top),
        dep = ":top_dif_bindgen",
        top = top,
    )
    for top in CROSS_CHECK_TOPS
]

[
    rust_library(
        name = "top_dif_{}".format(top),
        srcs = [":top_dif_bindgen_{}".format(top)],
        crate_name = "ot_bindgen_top_dif",
        rustc_flags = _RUSTC_FLAGS,
    )
    for top in CROSS_CHECK_TOPS
]
