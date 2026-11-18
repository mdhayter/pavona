// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
${gencmd}
<%
import topgen.lib as lib

addr_space_obj = lib.get_addr_space(top, addr_space)
addr_space_suffix = lib.get_addr_space_suffix(addr_space_obj)
top_file = "top_" + top["name"] + addr_space_suffix
top_prefix = "TOP_" + (top["name"] + addr_space_suffix).upper() + "_"

def generic(define):
    assert define.startswith(top_prefix), define
    return "TOP_" + define[len(top_prefix):]

def defines(region):
    return [region.base_addr_name().as_c_define(), region.size_bytes_name().as_c_define()]

names = []
for (inst_name, if_name), region in helper.memories(addr_space):
    names += defines(region)
    # toplevel.h.tpl also defines the CTN SRAM inside soc_proxy's ctn window.
    if inst_name == "soc_proxy" and if_name == "ctn":
        names += [d.replace("CTN", "RAM_CTN") for d in defines(region)]
for _, region in helper.devices(addr_space):
    names += defines(region)
for (_, _, subspace_range) in helper.subranges[addr_space]:
    names += defines(subspace_range)
%>\

#ifndef OPENTITAN_HW_TOP_TOP_MEMORY_H_
#define OPENTITAN_HW_TOP_TOP_MEMORY_H_

/**
 * @file
 * @brief Memory and peripheral addresses of the top being built, without its name.
 *
 * Included as "hw/top/top_memory.h" through //hw/top:top_memory, so that code
 * shared by several tops can use one set of names. Each name is an alias for
 * the same name with the top's prefix in ${top_file}.h (C) or
 * ${top_file}_memory.h (assembler): TOP_<NAME> for ${top_prefix}<NAME>.
 * `#ifdef` on a name tells whether this top has that memory or peripheral.
 */

#ifdef __ASSEMBLER__
#include "hw/top_${top["name"]}/sw/autogen/${top_file}_memory.h"
#else
#include "hw/top_${top["name"]}/sw/autogen/${top_file}.h"
#endif

% for name in names:
#define ${generic(name)} ${name}
% endfor

#endif  // OPENTITAN_HW_TOP_TOP_MEMORY_H_
