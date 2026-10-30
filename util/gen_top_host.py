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
requests, reset manager software resets and clock manager clocks. For the reset manager
it gives the bit of each reset cause in RESET_INFO, and for the clock manager whether it
supports external clock bypass (the EXTCLK registers).

The register offsets of the IP blocks in REGISTER_IPS are under "registers", by block type
and register name. A multiregister of several registers is one entry giving the offset of
the first, the count and the stride (NAME_0, NAME_1, ... in the generated headers).
If the OTP controller is generated with ipgen, "otp_ctrl" gives its memory map: the
partitions and items, with their byte offsets and sizes in OTP.

If the top has a JTAG ID package (hw/top_<topname>/rtl/jtag_id_pkg.sv), the description
also gives the JTAG IDCODEs of the debug module TAP (dm_idcode) and the life cycle
controller TAP (lc_idcode); on a top with one combined TAP they are the same.
"""
import argparse
import json
import re
from pathlib import Path
from typing import Dict, List, Optional, Tuple

from basegen.typing import ConfigT
import hjson

from reggen.multi_register import MultiRegister
from topgen.lib import (RustArrayMapping, RustEnum, TopGen, find_module, get_ip_hjson_path,
                        is_ipgen, load_completecfg_and_blocks)
from topgen.typing import IpBlocksT

REPO_TOP = Path(__file__).parents[1].resolve()

# A localparam in a JTAG ID package, such as
#   localparam logic [3:0] JTAG_VERSION = 4'h1;
LOCALPARAM_RE = re.compile(
    r"localparam\s+logic\s*\[\s*(\d+)\s*:\s*(\d+)\s*\]\s*(\w+)\s*=\s*([^;]*);")
# A sized literal, such as 4'h1, 4'd12 or 7'b110_1111.
LITERAL_RE = re.compile(r"(\d+)\s*'\s*([bBoOdDhH])\s*([0-9a-fA-F_]+)")
LITERAL_BASES = {"b": 2, "o": 8, "d": 10, "h": 16}
# Bits of the TAP type field (bits 15:12 of an IDCODE) that mark a debug module TAP and a
# life cycle controller TAP.
IDCODE_TAP_TYPE_SHIFT = 12
IDCODE_TAP_TYPE_DM = 1 << 0
IDCODE_TAP_TYPE_LC = 1 << 1
# The reset causes at the bottom of the reset manager's RESET_INFO register, before the
# hardware reset requests (HW_REQ), named as the register's fields.
RESET_INFO_FIXED = ["Por", "LowPowerExit", "SwReset"]
# The IP blocks whose register offsets the description gives, by block type: those host
# code addresses by register, such as the rv_dm csr_rw test.
REGISTER_IPS = [
    "acc", "adc_ctrl", "aes", "alert_handler", "aon_timer", "csrng", "edn", "entropy_src",
    "flash_ctrl", "gpio", "hmac", "i2c", "keymgr", "keymgr_dpe", "kmac", "otp_ctrl", "pattgen",
    "pinmux", "pwm", "pwrmgr", "rstmgr", "rv_timer", "sensor_ctrl", "spi_device", "spi_host",
    "sysrst_ctrl", "uart", "usbdev",
]
# The clock manager's default for ext_clk_bypass (hw/ip_templates/clkmgr/data/
# clkmgr.tpldesc.hjson).
CLKMGR_EXT_CLK_BYPASS_DEFAULT = True


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


def split_concatenation(expr: str) -> List[str]:
    """Split the inside of a {...} concatenation at its top-level commas."""
    parts, depth, start = [], 0, 0
    for i, char in enumerate(expr):
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
        elif char == "," and depth == 0:
            parts.append(expr[start:i])
            start = i + 1
    parts.append(expr[start:])
    return [part.strip() for part in parts]


def eval_param_expr(expr: str, params: Dict[str, Tuple[int, int]]) -> Tuple[int, int]:
    """Evaluate a localparam value: a sized literal, an earlier localparam, or a
    concatenation of those. Returns (value, width in bits)."""
    expr = expr.strip()
    if expr.startswith("{") and expr.endswith("}"):
        value, width = 0, 0
        for part in split_concatenation(expr[1:-1]):
            part_value, part_width = eval_param_expr(part, params)
            value = (value << part_width) | part_value
            width += part_width
        return value, width
    match = LITERAL_RE.fullmatch(expr)
    if match:
        width = int(match.group(1))
        value = int(match.group(3).replace("_", ""), LITERAL_BASES[match.group(2).lower()])
        if value >> width:
            raise ValueError(f"{expr} does not fit in {width} bits")
        return value, width
    if expr in params:
        return params[expr]
    raise ValueError(f"Cannot evaluate {expr!r}: not a sized literal, an earlier "
                     "localparam or a concatenation")


def parse_localparams(text: str) -> Dict[str, Tuple[int, int]]:
    """Return the `localparam logic [msb:lsb]` values of a package, in declaration
    order, as {name: (value, width)}. Only the forms used by the JTAG ID packages are
    supported."""
    text = re.sub(r"//[^\n]*", "", text)
    params: Dict[str, Tuple[int, int]] = {}
    for msb, lsb, name, expr in LOCALPARAM_RE.findall(text):
        width = int(msb) - int(lsb) + 1
        value, value_width = eval_param_expr(expr, params)
        if value_width != width:
            raise ValueError(f"{name} is {width} bits wide but its value has {value_width}")
        params[name] = (value, width)
    return params


def jtag_idcodes(text: str) -> Dict[str, int]:
    """Return the debug module and life cycle controller IDCODEs of a JTAG ID package.

    Each is the first `*IDCODE` localparam whose TAP type field has the DM or the LC bit
    set. A package with no such IDCODE gives no entry.
    """
    idcodes: Dict[str, int] = {}
    for name, (value, width) in parse_localparams(text).items():
        if not name.endswith("IDCODE"):
            continue
        if width != 32:
            raise ValueError(f"{name} is {width} bits wide, not 32")
        tap_type = (value >> IDCODE_TAP_TYPE_SHIFT) & 0xf
        if tap_type & IDCODE_TAP_TYPE_DM:
            idcodes.setdefault("dm_idcode", value)
        if tap_type & IDCODE_TAP_TYPE_LC:
            idcodes.setdefault("lc_idcode", value)
    return idcodes


def reset_info_bits(completecfg: ConfigT, reset_requests: Dict[str, int]) -> Dict[str, int]:
    """Return the bit of each reset cause in the reset manager's RESET_INFO register.

    HW_REQ has a bit for each hardware reset request that the power manager passes on: the
    peripheral requests (by their names in `reset_requests`, in order), then the power
    manager's internal ones, then the debug ones (by their names in the top config, such as
    MainPwr, Esc and Ndm).
    """
    bits = {name: bit for bit, name in enumerate(RESET_INFO_FIXED)}
    hw_req = len(bits)
    for name, index in reset_requests.items():
        bits[name] = hw_req + index
    requests = completecfg.get("reset_requests", {})
    next_bit = hw_req + len(requests.get("peripheral", []))
    for kind in ("int", "debug"):
        for request in requests.get(kind, []):
            bits[request["name"]] = next_bit
            next_bit += 1
    return bits


def register_offsets(block) -> Dict[str, object]:
    """Return the offset of each register and window of an IP block with one register block.
    A multiregister of several registers gives {"offset", "count", "stride"} instead; its
    registers are NAME_0, NAME_1, ... at offset + n * stride."""
    if len(block.reg_blocks) != 1:
        raise ValueError(f"{block.name} has {len(block.reg_blocks)} register blocks, not 1")
    (reg_block, ) = block.reg_blocks.values()
    offsets: Dict[str, object] = {}
    flat = {}
    for reg in reg_block.all_regs:
        if isinstance(reg, MultiRegister) and len(reg.cregs) > 1:
            first = reg.cregs[0].offset
            stride = reg.cregs[1].offset - first
            offsets[reg.name] = {"offset": first, "count": len(reg.cregs), "stride": stride}
            flat.update({f"{reg.name}_{n}": first + n * stride for n in range(len(reg.cregs))})
        else:
            # A multiregister of one register is named like a plain register.
            name, offset = (reg.cregs[0].name, reg.cregs[0].offset) \
                if isinstance(reg, MultiRegister) else (reg.name, reg.offset)
            offsets[name] = offset
            flat[name] = offset
    # The entries must describe exactly the registers in regtool's flattened list.
    if flat != {reg.name: reg.offset for reg in reg_block.flat_regs}:
        raise ValueError(f"{block.name}: registers don't match the flattened register list")
    offsets.update({window.name: window.offset for window in reg_block.windows})
    return offsets


def load_otp_mmap(completecfg: ConfigT, repo_top: Path) -> Optional[Dict[str, object]]:
    """Load the OTP memory map of the top's OTP controller from its ipgen configuration
    (hw/top_<top>/ip_autogen/otp_ctrl/data/top_<top>_otp_ctrl.ipconfig.hjson). Returns None
    if the top has no OTP controller or it is not generated with ipgen."""
    module = find_module(completecfg["module"], "otp_ctrl")
    if not module or not is_ipgen(module):
        return None
    data_dir = get_ip_hjson_path(module["type"], completecfg, repo_top).parent
    ipconfig = data_dir / f"top_{completecfg['name']}_{module['name']}.ipconfig.hjson"
    return hjson.loads(ipconfig.read_text())["param_values"]["otp_mmap"]


def otp_map(otp_mmap: Dict[str, object]) -> Dict[str, object]:
    """Return the partitions and items of an OTP memory map, by name.

    A partition's digest is its item marked as a digest, if it has one. Secret partitions
    are scrambled and accessed 64 bits at a time through the DAI.
    """
    partitions = {}
    items = {}
    for part in otp_mmap["partitions"]:
        digests = [item for item in part["items"] if item["isdigest"]]
        if len(digests) > 1:
            raise ValueError(f"OTP partition {part['name']} has more than one digest")
        partitions[part["name"]] = {
            "offset": int(part["offset"]),
            "size": int(part["size"]),
            "secret": bool(part["secret"]),
            "hw_digest": bool(part["hw_digest"]),
            "digest": {
                "offset": int(digests[0]["offset"]),
                "size": int(digests[0]["size"]),
            } if digests else None,
        }
        for item in part["items"]:
            items[item["name"]] = {
                "offset": int(item["offset"]),
                "size": int(item["size"]),
                "partition": part["name"],
            }
    return {"partitions": partitions, "items": items}


def host_description(completecfg: ConfigT, name_to_block: IpBlocksT,
                     jtag_id_pkg: Optional[str] = None,
                     otp_mmap: Optional[Dict[str, object]] = None) -> Dict[str, object]:
    """Build the host description of a top from its loaded complete configuration."""
    helper = TopGen(completecfg, name_to_block, RustEnum, RustArrayMapping)
    modules = completecfg["module"]

    desc: Dict[str, object] = {
        "SPDX-License-Identifier": "Apache-2.0",
        "generator": "util/gen_top_host.py",
        "name": completecfg["name"],
    }
    if jtag_id_pkg is not None:
        desc.update(jtag_idcodes(jtag_id_pkg))

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

    desc["registers"] = {
        ip: register_offsets(name_to_block[ip])
        for ip in REGISTER_IPS if ip in name_to_block
    }
    if otp_mmap is not None:
        desc["otp_ctrl"] = otp_map(otp_mmap)

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
            "reset_info": reset_info_bits(
                completecfg,
                enum_dict(helper.pwrmgr_reset_requests) if find_module(modules, "pwrmgr") else {}),
        }

    clkmgr = find_module(modules, "clkmgr")
    if clkmgr:
        desc["clkmgr"] = {
            "gateable_clocks": enum_dict(helper.clkmgr_gateable_clocks),
            "hintable_clocks": enum_dict(helper.clkmgr_hintable_clocks),
            "ext_clk_bypass": bool(clkmgr.get("ipgen_params", {}).get(
                "ext_clk_bypass", CLKMGR_EXT_CLK_BYPASS_DEFAULT)),
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
    parser.add_argument("--jtag-id-pkg",
                        type=Path,
                        default=None,
                        help="JTAG ID package of the top. Defaults to "
                             "hw/top_<topname>/rtl/jtag_id_pkg.sv under --repo-top, if it "
                             "exists.")
    args = parser.parse_args()

    completecfg, name_to_block = load_completecfg_and_blocks(args.completecfg, args.repo_top)
    jtag_id_pkg = args.jtag_id_pkg
    if jtag_id_pkg is None:
        default = args.repo_top / f"hw/top_{completecfg['name']}/rtl/jtag_id_pkg.sv"
        jtag_id_pkg = default if default.exists() else None
    desc = host_description(completecfg, name_to_block,
                            jtag_id_pkg.read_text() if jtag_id_pkg else None,
                            load_otp_mmap(completecfg, args.repo_top))

    outdir = args.outdir or args.completecfg.parent
    outfile = outdir / f"top_{completecfg['name']}.host.json"
    outfile.write_text(json.dumps(desc, indent=2) + "\n")


if __name__ == "__main__":
    main()
