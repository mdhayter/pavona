// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENTITAN_SW_DEVICE_SILICON_CREATOR_ROM_BOOT_POLICY_PTRS_H_
#define OPENTITAN_SW_DEVICE_SILICON_CREATOR_ROM_BOOT_POLICY_PTRS_H_

#include "hw/top/top_memory.h"
#include "sw/device/silicon_creator/lib/manifest.h"

// On a top with flash, ROM_EXT is held there; otherwise it lives in CTN
// memory, which is governed by the soc_proxy.
#ifdef TOP_FLASH_CTRL_MEM_BASE_ADDR
#include "hw/top/dt/flash_ctrl.h"
#elif defined(TOP_SOC_PROXY_RAM_CTN_BASE_ADDR)
#include "hw/top/dt/soc_proxy.h"
#endif

typedef enum slot {
  kSlotA = 0,
  kSlotB = 1,
} slot_t;

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

#ifdef OT_PLATFORM_RV32
/**
 * Returns a pointer to the manifest of the ROM_EXT image stored in flash
 * slot A.
 *
 * @return Pointer to the manifest of the ROM_EXT image in the given slot.
 */
OT_WARN_UNUSED_RESULT
inline const manifest_t *boot_policy_manifest_get(slot_t slot) {
#ifdef TOP_FLASH_CTRL_MEM_BASE_ADDR
  uint32_t flash_base =
      dt_flash_ctrl_memory_base(kDtFlashCtrl, kDtFlashCtrlMemoryMem);
  uint32_t flash_size =
      dt_flash_ctrl_memory_size(kDtFlashCtrl, kDtFlashCtrlMemoryMem);
  return (const manifest_t *)(flash_base + (slot * (flash_size / 2)));
#elif defined(TOP_SOC_PROXY_RAM_CTN_BASE_ADDR)
  // CTN SRAM only has one slot, because we load to CTN from elsewhere on the
  // SoC.
  return (const manifest_t *)TOP_SOC_PROXY_RAM_CTN_BASE_ADDR;
#endif
}
#else
/**
 * Declarations for the function above that should be defined in tests.
 */
const manifest_t *boot_policy_manifest_get(slot_t slot);
#endif

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // OPENTITAN_SW_DEVICE_SILICON_CREATOR_ROM_BOOT_POLICY_PTRS_H_
