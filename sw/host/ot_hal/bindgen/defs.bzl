# Copyright Mark Hayter.
# Licensed under the Apache License, Version 2.0, see LICENSE for details.
# SPDX-License-Identifier: Apache-2.0

# The tops whose descriptions the `//sw/host/ot_hal:ot_hal_<top>_test`s check against their DIFs
# and generated constants. Any other top is checked by the top-independent tests only.
CROSS_CHECK_TOPS = [
    "dragonfly",
    "egret",
]
