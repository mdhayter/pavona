# Copyright lowRISC contributors (OpenTitan project).
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

# Copied (and modified) from hw/top_scafi_deprecated/bitstream/BUILD.

load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("//rules:fusesoc.bzl", "fusesoc_build")
load("//rules:bitstreams.bzl", "bitstream_manifest_fragment")

_FPGA_PATH_TMPL = "lowrisc_systems_{}_0.1/synth-vivado/{}"

fusesoc_build(
    name = "fpga_genesys2",
    testonly = True,
    srcs = [
        "//hw:rtl_files",
        "//hw/top_egret:rtl_files",  # Add Egret RTL files for alert_handler_pkg and the padring
        "//hw/top_wren:rtl_files",
    ],
    cores = ["//hw:cores"],
    output_groups = {
        "bitstream": [_FPGA_PATH_TMPL.format("chip_wren_genesys2", "lowrisc_systems_chip_wren_genesys2_0.1.bit")],
        "mmi": [_FPGA_PATH_TMPL.format("chip_wren_genesys2", "memories.mmi")],
        "logs": [_FPGA_PATH_TMPL.format("chip_wren_genesys2", "lowrisc_systems_chip_wren_genesys2_0.1.runs/")],
    },
    systems = ["lowrisc:systems:chip_wren_genesys2:0.1"],
    tags = ["manual"],
    target = "synth",
)

filegroup(
    name = "genesys2_mmi",
    testonly = True,
    srcs = [":fpga_genesys2"],
    output_group = "mmi",
    tags = ["manual"],
)

bitstream_manifest_fragment(
    name = "wren_genesys2_manifest",
    testonly = True,
    srcs = [":fpga_genesys2"],
    bitstream = "bitstream",
    design = "chip_wren_genesys2",
    memories = [
        "rom",
    ],
    memory_map_file = ":genesys2_mmi",
    tags = ["manual"],
)

pkg_tar(
    name = "wren_genesys2_archive",
    testonly = True,
    srcs = [":wren_genesys2_manifest"],
    mode = "0444",
    package_dir = "build-bin/hw/top_wren/chip_wren_genesys2",
    strip_prefix = "/hw/top_wren/bitstream/wren_genesys2_manifest",
    tags = ["manual"],
)
