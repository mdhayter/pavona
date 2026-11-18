// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// Test pinout of top_scafi_deprecated: see sw/device/lib/testing/test_pinout.h.

#include "sw/device/lib/testing/test_pinout.h"

#include "sw/device/lib/arch/device.h"
#include "sw/device/lib/testing/test_framework/check.h"

#define MODULE_ID MAKE_MODULE_ID('t', 'p', 'o')

const test_pinout_init_t kTestPinoutInit = {
    .uart0_rx = kDtPadIoc3,
    .uart0_tx = kDtPadIoc4,
    .has_uart1 = true,
    .uart1 = kDtUart1,
    .uart1_rx = kDtPadIob4,
    .uart1_tx = kDtPadIob5,
    .straps = {kDtPadIoc0, kDtPadIoc1, kDtPadIoc2},
};

void test_pinout_init_extra(dif_pinmux_t *pinmux) {}

// Mapping of Chip IOs to the GPIO peripheral.
//
// Depending on the simulation platform, there may be a limitation to how chip
// IOs are allocated to the GPIO peripheral, even for testing. The DV testbench
// does not have this limitation, and is able to allocate as many chip IOs as
// the number of GPIOs supported by the peripheral. Scafi has no DV testbench:
// these are Egret's pin assignments, which match Egret's DV (see
// `hw/top_egret/dv/env/chip_if.sv`).
//
// The pinout spreadsheet allocates fewer pins to GPIOs than what the GPIO IP
// supports. This oversubscription is intentional to maximize testing.
const dt_pad_t kPinmuxTestutilsGpioPads[kDifGpioNumPins] = {
    kDtPadIoa0,  kDtPadIoa1, kDtPadIoa2,  kDtPadIoa3,  kDtPadIoa4,
    kDtPadIoa5,  kDtPadIoa6, kDtPadIoa7,  kDtPadIoa8,  kDtPadIob6,
    kDtPadIob7,  kDtPadIob8, kDtPadIob9,  kDtPadIob10, kDtPadIob11,
    kDtPadIob12, kDtPadIoc9, kDtPadIoc10, kDtPadIoc11, kDtPadIoc12,
    kDtPadIor0,  kDtPadIor1, kDtPadIor2,  kDtPadIor3,  kDtPadIor4,
    kDtPadIor5,  kDtPadIor6, kDtPadIor7,  kDtPadIor10, kDtPadIor11,
    kDtPadIor12, kDtPadIor13};

status_t test_pinout_uart_pads(dt_uart_t uart_dt, uart_pinmux_channel_t channel,
                               dt_pad_t *rx_pad, dt_pad_t *tx_pad) {
  // For DV platform, each UART has its own specific mapping
  if (kDeviceType == kDeviceSimDV) {
    switch (uart_dt) {
      case kDtUart0:
        *rx_pad = kDtPadIoc3;
        *tx_pad = kDtPadIoc4;
        break;
      case kDtUart1:
        *rx_pad = kDtPadIob4;
        *tx_pad = kDtPadIob5;
        break;
      case kDtUart2:
        *rx_pad = kDtPadIoa4;
        *tx_pad = kDtPadIoa5;
        break;
      case kDtUart3:
        *rx_pad = kDtPadIoa0;
        *tx_pad = kDtPadIoa1;
        break;
      default:
        return INVALID_ARGUMENT();
    }
  } else {
    // For synthesized platforms, use channel-based mapping
    switch (channel) {
      case kUartPinmuxChannelConsole:
        *rx_pad = kDtPadIoc3;
        *tx_pad = kDtPadIoc4;
        break;
      case kUartPinmuxChannelDut:
        *rx_pad = kDtPadIob4;
        *tx_pad = kDtPadIob5;
        break;
      default:
        return INVALID_ARGUMENT();
    }
  }

  return OK_STATUS();
}

// No I2C.

status_t test_pinout_spi_host1_pads(spi_pinmux_platform_id_t platform_id,
                                    test_pinout_spi_host1_pads_t *pads) {
  return UNIMPLEMENTED();
}
