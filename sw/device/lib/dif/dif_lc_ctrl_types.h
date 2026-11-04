// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENTITAN_SW_DEVICE_LIB_DIF_DIF_LC_CTRL_TYPES_H_
#define OPENTITAN_SW_DEVICE_LIB_DIF_DIF_LC_CTRL_TYPES_H_

/**
 * @file
 * @brief Lifecycle Controller DIF types that do not depend on the top.
 *
 * The host tools share these with the device software, so this header must
 * not include anything generated for a top.
 */

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

// Moved from `sw/device/lib/dif/dif_lc_ctrl.h`.

/**
 * A lifecycle state.
 *
 * This enum represents all states that the lifecycle state register can be in;
 * some of these do not correspond to "real" lifecycle states, and cannot be
 * transitioned to.
 */
typedef enum dif_lc_ctrl_state {
  /**
   * The initial post-manufacture state.
   *
   * All functions are disabled.
   */
  kDifLcCtrlStateRaw,
  /**
   * The zeroth test state.
   *
   * Debug functions are enabled.
   */
  kDifLcCtrlStateTestUnlocked0,
  /**
   * The zeroth locked test state.
   *
   * All functions are disabled.
   */
  kDifLcCtrlStateTestLocked0,
  /**
   * The first test state.
   *
   * Debug functions are enabled.
   */
  kDifLcCtrlStateTestUnlocked1,
  /**
   * The first locked test state.
   *
   * All functions are disabled.
   */
  kDifLcCtrlStateTestLocked1,
  /**
   * The second test state.
   *
   * Debug functions are enabled.
   */
  kDifLcCtrlStateTestUnlocked2,
  /**
   * The second locked test state.
   *
   * All functions are disabled.
   */
  kDifLcCtrlStateTestLocked2,
  /**
   * The third test state.
   *
   * Debug functions are enabled.
   */
  kDifLcCtrlStateTestUnlocked3,
  /**
   * The third locked test state.
   *
   * All functions are disabled.
   */
  kDifLcCtrlStateTestLocked3,
  /**
   * The fourth test state.
   *
   * Debug functions are enabled.
   */
  kDifLcCtrlStateTestUnlocked4,
  /**
   * The fourth locked test state.
   *
   * All functions are disabled.
   */
  kDifLcCtrlStateTestLocked4,
  /**
   * The fifth test state.
   *
   * Debug functions are enabled.
   */
  kDifLcCtrlStateTestUnlocked5,
  /**
   * The fifth locked test state.
   *
   * All functions are disabled.
   */
  kDifLcCtrlStateTestLocked5,
  /**
   * The sixth test state.
   *
   * Debug functions are enabled.
   */
  kDifLcCtrlStateTestUnlocked6,
  /**
   * The sixth locked test state.
   *
   * All functions are disabled.
   */
  kDifLcCtrlStateTestLocked6,
  /**
   * The seventh test state.
   *
   * Debug functions are enabled.
   */
  kDifLcCtrlStateTestUnlocked7,
  /**
   * The development state.
   *
   * Some debug functions are enabled.
   */
  kDifLcCtrlStateDev,
  /**
   * The main production state.
   *
   * No debug functions are enabled.
   */
  kDifLcCtrlStateProd,
  /**
   * The EOL production state.
   *
   * Same as Prod, except Rma cannot be transitioned to.
   */
  kDifLcCtrlStateProdEnd,
  /**
   * RMA manufacturer analysis state.
   */
  kDifLcCtrlStateRma,
  /**
   * The scrap EOL state.
   *
   * Cannot be transitioned from; all functions are disabled.
   */
  kDifLcCtrlStateScrap,

  /**
   * State entered immediately after a transition occurs.
   *
   * Not a real state.
   */
  kDifLcCtrlStatePostTransition,
  /**
   * State entered immediately after an alert is raised.
   *
   * Not a real state.
   */
  kDifLcCtrlStateEscalate,
  /**
   * Indicates that the encoded lifecycle is invalid.
   *
   * Not a real state.
   */
  kDifLcCtrlStateInvalid,
} dif_lc_ctrl_state_t;

/**
 * A 128-bit unlock token used for certain kinds of lifecycle transitions.
 */
typedef struct dif_lc_ctrl_token {
  uint8_t data[128 / 8];
} dif_lc_ctrl_token_t;

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // OPENTITAN_SW_DEVICE_LIB_DIF_DIF_LC_CTRL_TYPES_H_
