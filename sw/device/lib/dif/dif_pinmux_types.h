// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENTITAN_SW_DEVICE_LIB_DIF_DIF_PINMUX_TYPES_H_
#define OPENTITAN_SW_DEVICE_LIB_DIF_DIF_PINMUX_TYPES_H_

/**
 * @file
 * @brief Pin Multiplexer DIF types that do not depend on the top.
 *
 * The host tools share these with the device software, so this header must
 * not include anything generated for a top.
 */

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

// Moved from `sw/device/lib/dif/dif_pinmux.h`.

/**
 * Pad attribute flag values
 *
 * This `enum` is a bitfield, meaning that it is allowed to pass combined value
 * of multiple individual attributes.
 *
 * Some attributes have defined meanings, others are implementation defined. Not
 * all pads will support all attributes, or attribute combinations. The Pin
 * Multiplexer DIF checks for conflicting or unsupported attributes provided by
 * the Write-Any Read-Legal (WARL) semantics of the underlying hardware. If the
 * combination of attributes is not supported - some/all of these attributes
 * will still be disabled (relevant bit unset in the attribute field).
 *
 * IMPORTANT:
 * - Functions are used to toggle these attributes, must signal an error if at
 *   least one of the attributes in the bitfield could not be toggled.
 */
typedef enum dif_pinmux_pad_attr_flags {
  kDifPinmuxPadAttrInvertLevel = 1 << 0,
  kDifPinmuxPadAttrVirtualOpenDrain = 1 << 1,
  kDifPinmuxPadAttrPullResistorEnable = 1 << 2,
  kDifPinmuxPadAttrPullResistorUp = 1 << 3,
  kDifPinmuxPadAttrKeeper = 1 << 4,
  kDifPinmuxPadAttrSchmittTrigger = 1 << 5,
  kDifPinmuxPadAttrOpenDrain = 1 << 6,
  kDifPinmuxPadAttrInputDisable = 1 << 7,
} dif_pinmux_pad_attr_flags_t;

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // OPENTITAN_SW_DEVICE_LIB_DIF_DIF_PINMUX_TYPES_H_
