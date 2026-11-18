// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// Test pinout of top_dragonfly: see sw/device/lib/testing/test_pinout.h.

#include "sw/device/lib/testing/test_pinout.h"

#include "sw/device/lib/testing/test_framework/check.h"

#define MODULE_ID MAKE_MODULE_ID('t', 'p', 'o')

const test_pinout_init_t kTestPinoutInit = {
    .uart0_rx = kDtPadUart0Rx,
    .uart0_tx = kDtPadUart0Tx,
    // No UART1.
    .has_uart1 = false,
    .straps = {kDtPadGpioGpio22, kDtPadGpioGpio23, kDtPadGpioGpio24},
};

void test_pinout_init_extra(dif_pinmux_t *pinmux) {}

const dt_pad_t kPinmuxTestutilsGpioPads[kDifGpioNumPins] = {
    kDtPadGpioGpio0,  kDtPadGpioGpio1,  kDtPadGpioGpio2,  kDtPadGpioGpio3,
    kDtPadGpioGpio4,  kDtPadGpioGpio5,  kDtPadGpioGpio6,  kDtPadGpioGpio7,
    kDtPadGpioGpio8,  kDtPadGpioGpio9,  kDtPadGpioGpio10, kDtPadGpioGpio11,
    kDtPadGpioGpio12, kDtPadGpioGpio13, kDtPadGpioGpio14, kDtPadGpioGpio15,
    kDtPadGpioGpio16, kDtPadGpioGpio17, kDtPadGpioGpio18, kDtPadGpioGpio19,
    kDtPadGpioGpio20, kDtPadGpioGpio21, kDtPadGpioGpio22, kDtPadGpioGpio23,
    kDtPadGpioGpio24, kDtPadGpioGpio25, kDtPadGpioGpio26, kDtPadGpioGpio27,
    kDtPadGpioGpio28, kDtPadGpioGpio29, kDtPadGpioGpio30, kDtPadGpioGpio31,
};

status_t test_pinout_uart_pads(dt_uart_t uart_dt, uart_pinmux_channel_t channel,
                               dt_pad_t *rx_pad, dt_pad_t *tx_pad) {
  // Dragonfly only has UART0 and uses dedicated pads
  if (uart_dt != kDtUart0) {
    return INVALID_ARGUMENT();
  }
  *rx_pad = kDtPadUart0Rx;
  *tx_pad = kDtPadUart0Tx;

  return OK_STATUS();
}

status_t test_pinout_i2c_pads(dt_i2c_t i2c_dt,
                              i2c_pinmux_platform_id_t platform,
                              dt_pad_t *sda_pad, dt_pad_t *scl_pad) {
  // Dragonfly only has I2C0 and uses dedicated pads
  if (i2c_dt != kDtI2c0) {
    return INVALID_ARGUMENT();
  }
  *sda_pad = kDtPadI2c0Sda;
  *scl_pad = kDtPadI2c0Scl;

  return OK_STATUS();
}

status_t test_pinout_spi_host1_pads(spi_pinmux_platform_id_t platform_id,
                                    test_pinout_spi_host1_pads_t *pads) {
  return UNIMPLEMENTED();
}
