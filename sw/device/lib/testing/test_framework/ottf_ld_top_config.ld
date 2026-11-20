/* Copyright lowRISC contributors (OpenTitan project). */
/* Copyright Mark Hayter. */
/* Licensed under the Apache License, Version 2.0, see LICENSE for details. */
/* SPDX-License-Identifier: Apache-2.0 */

/* TODO This configuration should be split into per-top headers */
#ifdef OPENTITAN_IS_DRAGONFLY
MEMORY {
    /* There is a 1MiB RAM within the CTN at offset 16MiB to hold the
     * firmware under test in DV. */
    ctn_ram(rwx) : ORIGIN = ORIGIN(ctn) + 0x1000000, LENGTH = 0x100000
}
REGION_ALIAS("ottf_storage", ctn_ram)
#else
REGION_ALIAS("ottf_storage", eflash)
#endif

/* TODO: Consider moving scafi_deprecated to the ottf_no_nv_storage attribute
 * too, set in `pavona_top()` in hw/top_scafi_deprecated/defs.bzl, so that this
 * condition names no top. */
#if defined(OPENTITAN_IS_SCAFI_DEPRECATED) || defined(OTTF_NO_NV_STORAGE)
/* Disable non-volatile scratch region and counters if building for
 * scafi_deprecated, or for a top that sets the ottf_no_nv_storage attribute
 * (OTTF_NO_NV_STORAGE). This should appear before the linker script. */
no_ottf_nv_scratch = 1
no_ottf_nv_counter = 1
#endif
