# Copyright lowRISC contributors (OpenTitan project).
# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0
"""
This contains a class to access the clkmgr's configuration from its ipconfig
files.
"""
from typing import List, Dict

from ipgen.clkmgr_gen import config_clk_meas, get_all_srcs, get_rg_srcs


class ClkmgrIpConfig:
    def __init__(self, ipconfig: object):
        """
        Initialize an `IpConfig` from an already loaded and parsed `ipconfig.hjson`
        file, as well as a top configuration.
        """
        self.param_values = ipconfig["param_values"]

    def sw_clks(self) -> List[Dict]:
        """
        Return the list of software controllable clocks: each clock is described by a
        dictionary with the following fields:
        - `src`: clock source
        - `module`: the module whose clock is controlled.

        The list is ordered as in the description.
        """
        return [
            {
                "src": clk["src_name"],
                "module": clk["endpoint_ip"],
            }
            for clk in self.param_values["typed_clocks"]["sw_clks"].values()
        ]

    def hint_clks(self) -> List[Dict]:
        """
        Return the list of hintable clocks: each clock is described by a
        dictionary with the following fields:
        - `src`: clock source
        - `module`: the module whose clock is controlled.

        The list is ordered as in the description.
        """
        return [
            {
                "src": clk["src_name"],
                "module": clk["endpoint_ip"],
            }
            for clk in self.param_values["typed_clocks"].get("hint_clks", {}).values()
        ]

    def measurable_clks(self) -> List[str]:
        """
        Return the list of measurable clocks: each clock is described by its
        name.
        """
        # Currently, the clkmgr measures every root-gated clock.
        rg_srcs = get_rg_srcs(self.param_values["typed_clocks"])
        return rg_srcs

    def measurement_ref_cnt(self, clk: str) -> int:
        """
        Return the number of AON clock cycles the clkmgr measures a clock over:
        the `RefCnt` the RTL template gets from `config_clk_meas()`.
        """
        all_srcs = get_all_srcs(self.param_values["src_clks"],
                                self.param_values["derived_clks"])
        _, ref_cnt, _ = config_clk_meas(clk, all_srcs)
        return ref_cnt
