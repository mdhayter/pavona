#!/usr/bin/env python3
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
"""Generate a host-side description of a top level from its complete configuration.

Host tools such as opentitantool are currently built for one top level and get its
addresses and pinmux tables at compile time. This generator writes the same information
to a JSON file that host tools can load at run time instead:

    <outdir>/top_<topname>.host.json

The description contains, for each address space, the base address and size of every
device register block, memory and subrange (such as "mmio"). It also contains the
top's enumerations, using the same names as the generated Rust code: alert handler
peripherals and alerts, pinmux selections and pads, power manager wakeups and reset
requests, reset manager software resets and clock manager clocks.
"""
import argparse
import json
from pathlib import Path
from typing import Dict

from basegen.typing import ConfigT
from topgen.lib import (RustArrayMapping, RustEnum, TopGen, find_module,
                        load_completecfg_and_blocks)
from topgen.typing import IpBlocksT

REPO_TOP = Path(__file__).parents[1].resolve()


def enum_dict(enum: RustEnum) -> Dict[str, int]:
    """Return the variants of a generated Rust enum as a name -> value dict."""
    return {name.as_rust_enum(): value for name, value, _ in enum.constants}


def mapping_dict(mapping: RustArrayMapping) -> Dict[str, str]:
    """Return a generated Rust array mapping as a variant name -> variant name dict."""
    return {
        in_name.as_rust_enum(): out_name.as_rust_enum()
        for in_name, out_name in mapping.mapping.items()
    }


def region_dict(region) -> Dict[str, int]:
    return {"base_addr": int(region.base_addr), "size_bytes": int(region.size_bytes)}


def host_description(completecfg: ConfigT, name_to_block: IpBlocksT) -> Dict[str, object]:
    """Build the host description of a top from its loaded complete configuration."""
    helper = TopGen(completecfg, name_to_block, RustEnum, RustArrayMapping)
    modules = completecfg["module"]

    desc: Dict[str, object] = {
        "SPDX-License-Identifier": "Apache-2.0",
        "generator": "util/gen_top_host.py",
        "name": completecfg["name"],
    }

    addr_spaces = {}
    for addr_space in completecfg["addr_spaces"]:
        name = addr_space["name"]
        addr_spaces[name] = {
            "devices": {
                region.short_name.as_snake_case(): region_dict(region)
                for _, region in helper.devices(name)
            },
            "memories": {
                region.short_name.as_snake_case(): region_dict(region)
                for _, region in helper.memories(name)
            },
            "subranges": {
                subspace_name: region_dict(region)
                for subspace_name, _, region in helper.subranges[name]
            },
        }
    desc["addr_spaces"] = addr_spaces

    if find_module(modules, "alert_handler"):
        desc["alert_handler"] = {
            "peripherals": enum_dict(helper.alert_sources),
            "alerts": enum_dict(helper.alert_alerts),
            "alert_to_peripheral": mapping_dict(helper.alert_mapping),
        }

    if find_module(modules, "pinmux"):
        io_counts = completecfg["pinmux"]["io_counts"]
        dedicated = io_counts["dedicated"]
        desc["pinmux"] = {
            "num_mio_pads": int(io_counts["muxed"]["pads"]),
            "num_dio_pads": int(dedicated["inouts"] + dedicated["inputs"] +
                                dedicated["outputs"]),
            "peripheral_in": enum_dict(helper.pinmux_peripheral_in),
            "insel": enum_dict(helper.pinmux_insel),
            "mio_out": enum_dict(helper.pinmux_mio_out),
            "outsel": enum_dict(helper.pinmux_outsel),
            "direct_pads": enum_dict(helper.direct_pads),
            "muxed_pads": enum_dict(helper.muxed_pads),
        }

    if find_module(modules, "pwrmgr"):
        desc["pwrmgr"] = {
            "wakeups": enum_dict(helper.pwrmgr_wakeups),
            "reset_requests": enum_dict(helper.pwrmgr_reset_requests),
        }

    if find_module(modules, "rstmgr"):
        desc["rstmgr"] = {
            "sw_resets": enum_dict(helper.rstmgr_sw_rsts),
        }

    if find_module(modules, "clkmgr"):
        desc["clkmgr"] = {
            "gateable_clocks": enum_dict(helper.clkmgr_gateable_clocks),
            "hintable_clocks": enum_dict(helper.clkmgr_hintable_clocks),
        }

    return desc


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--completecfg", "-t",
                        type=Path,
                        required=True,
                        help="Complete top level configuration Hjson.")
    parser.add_argument("--outdir", "-o",
                        type=Path,
                        default=None,
                        help="Output directory. Defaults to the directory of the complete "
                             "configuration.")
    parser.add_argument("--repo-top",
                        type=Path,
                        default=REPO_TOP,
                        help="Directory under which the IP block descriptions are found. "
                             "Defaults to the repository containing this script.")
    args = parser.parse_args()

    completecfg, name_to_block = load_completecfg_and_blocks(args.completecfg, args.repo_top)
    desc = host_description(completecfg, name_to_block)

    outdir = args.outdir or args.completecfg.parent
    outfile = outdir / f"top_{completecfg['name']}.host.json"
    outfile.write_text(json.dumps(desc, indent=2) + "\n")


if __name__ == "__main__":
    main()
