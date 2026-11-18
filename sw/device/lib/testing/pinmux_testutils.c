// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#include "sw/device/lib/testing/pinmux_testutils.h"

#include "hw/top/dt/pinmux.h"
#include "hw/top/dt/uart.h"
#include "sw/device/lib/arch/device.h"
#include "sw/device/lib/base/macros.h"
#include "sw/device/lib/base/status.h"
#include "sw/device/lib/dif/dif_base.h"
#include "sw/device/lib/dif/dif_gpio.h"
#include "sw/device/lib/dif/dif_pinmux.h"
#include "sw/device/lib/runtime/hart.h"
#include "sw/device/lib/testing/test_framework/check.h"

static const dt_gpio_t kGpioDt = kDtGpio;
static const dt_uart_t kUart0Dt = kDtUart0;

void pinmux_testutils_init(dif_pinmux_t *pinmux) {
  // Set up SW straps on IOC0-IOC2, for GPIOs 22-24
  CHECK_STATUS_OK(pinmux_testutils_connect(
      pinmux, dt_gpio_periph_io(kGpioDt, kDtGpioPeriphIoGpio22),
      kDtPeriphIoDirIn, kTestPinoutInit.straps[0]));
  CHECK_STATUS_OK(pinmux_testutils_connect(
      pinmux, dt_gpio_periph_io(kGpioDt, kDtGpioPeriphIoGpio23),
      kDtPeriphIoDirIn, kTestPinoutInit.straps[1]));
  CHECK_STATUS_OK(pinmux_testutils_connect(
      pinmux, dt_gpio_periph_io(kGpioDt, kDtGpioPeriphIoGpio24),
      kDtPeriphIoDirIn, kTestPinoutInit.straps[2]));

  // Configure UART0 RX input.
  CHECK_STATUS_OK(pinmux_testutils_connect(
      pinmux, dt_uart_periph_io(kUart0Dt, kDtUartPeriphIoRx), kDtPeriphIoDirIn,
      kTestPinoutInit.uart0_rx));
  // Configure UART0 TX output.
  CHECK_STATUS_OK(pinmux_testutils_connect(
      pinmux, dt_uart_periph_io(kUart0Dt, kDtUartPeriphIoTx), kDtPeriphIoDirOut,
      kTestPinoutInit.uart0_tx));

  if (kTestPinoutInit.has_uart1) {
    // Configure UART1 RX input.
    CHECK_STATUS_OK(pinmux_testutils_connect(
        pinmux, dt_uart_periph_io(kTestPinoutInit.uart1, kDtUartPeriphIoRx),
        kDtPeriphIoDirIn, kTestPinoutInit.uart1_rx));
    // Configure UART1 TX output.
    CHECK_STATUS_OK(pinmux_testutils_connect(
        pinmux, dt_uart_periph_io(kTestPinoutInit.uart1, kDtUartPeriphIoTx),
        kDtPeriphIoDirOut, kTestPinoutInit.uart1_tx));
  }

  test_pinout_init_extra(pinmux);
}

status_t pinmux_testutils_connect(const dif_pinmux_t *pinmux,
                                  dt_periph_io_t periph_io,
                                  dt_periph_io_dir_t dir, dt_pad_t pad) {
  switch (dt_periph_io_type(periph_io)) {
    case kDtPeriphIoTypeMio:
      if (dt_pad_type(pad) != kDtPadTypeMio) {
        return INVALID_ARGUMENT();
      }
      // Configure input.
      if (dir == kDtPeriphIoDirIn || dir == kDtPeriphIoDirInout) {
        TRY(dif_pinmux_mio_select_input(pinmux, periph_io, pad));
      }
      // Configure output as requested...
      if (dir == kDtPeriphIoDirOut || dir == kDtPeriphIoDirInout) {
        TRY(dif_pinmux_mio_select_output(pinmux, pad, periph_io));
      }
      // ... or as high-Z.
      else if (dt_periph_io_dir(periph_io) == kDtPeriphIoDirInout) {
        TRY(dif_pinmux_mio_select_output(pinmux, pad,
                                         kDtPeriphIoConstantHighZ));
      }
      return OK_STATUS();
    case kDtPeriphIoTypeDio:
      // Nothing to do but to check that they are actually connected together.
      if (dt_pad_type(pad) != kDtPadTypeDio ||
          dt_periph_io_dio_pad(periph_io) != pad) {
        return INVALID_ARGUMENT();
      }
      // Make sure that the directions are compatible.
      dt_periph_io_dir_t io_dir = dt_periph_io_dir(periph_io);
      if ((io_dir == kDtPeriphIoDirIn || io_dir == kDtPeriphIoDirOut) &&
          dir != io_dir) {
        return INVALID_ARGUMENT();
      }
      return OK_STATUS();
    default:
      return INVALID_ARGUMENT();
  }
}

// kPinmuxTestutilsGpioPads is defined by the top's test pinout.

uint32_t pinmux_testutils_get_testable_gpios_mask(void) {
  switch (kDeviceType) {
    case kDeviceSimDV:
    case kDeviceSimVerilator:
      // All GPIOs are testable in DV.
      return 0xffffffff;
    case kDeviceFpgaCw310:
    case kDeviceFpgaCw340:
      // Only IOR6, IOR7, and IOR10 to IOR13 are available for use as GPIOs.
      return 0xfc000000;
    case kDeviceSilicon:
      // IOA3/6, IOB6, IOC9-12, IOR5-7 and IOR10-13.
      return 0xfe0f0248;
    default:
      CHECK(false);
      return 0;
  }
}

uint32_t pinmux_testutils_read_strap_pin(dif_pinmux_t *pinmux, dif_gpio_t *gpio,
                                         dif_gpio_pin_t io, dt_pad_t pad) {
  // Turn off the pull enable on the pad and read the IO.
  dif_pinmux_pad_attr_t attr = {.flags = 0};
  dif_pinmux_pad_attr_t attr_out;
  CHECK_DIF_OK(dif_pinmux_pad_write_attrs_dt(pinmux, pad, attr, &attr_out));
  // Let the change propagate.
  busy_spin_micros(100);
  bool state;
  // The value read is unmodified by the internal pull resistors and represents
  // the upper bit of the 4 possible states [Strong0, Weak0, Weak1,
  // Strong1].
  CHECK_DIF_OK(dif_gpio_read(gpio, io, &state));
  uint32_t result = state ? 2 : 0;

  // Based on the previous read, enable the opposite pull resistor.  If the
  // external signal is weak, the internal pull resistor will win; if the
  // external signal is strong, the external value will win.
  attr.flags = kDifPinmuxPadAttrPullResistorEnable |
               (state ? 0 : kDifPinmuxPadAttrPullResistorUp);
  CHECK_DIF_OK(dif_pinmux_pad_write_attrs_dt(pinmux, pad, attr, &attr_out));
  // Let the change propagate.
  busy_spin_micros(100);
  // Combine the result of the contest between the external signal in internal
  // pull resistors.  This represents the lower bit of the 4 possible states.
  CHECK_DIF_OK(dif_gpio_read(gpio, io, &state));
  result += state ? 1 : 0;
  return result;
}

uint32_t pinmux_testutils_read_straps(dif_pinmux_t *pinmux, dif_gpio_t *gpio) {
  uint32_t strap = 0;
  strap |= pinmux_testutils_read_strap_pin(pinmux, gpio, 22,
                                           kTestPinoutInit.straps[0]);
  strap |= pinmux_testutils_read_strap_pin(pinmux, gpio, 23,
                                           kTestPinoutInit.straps[1])
           << 2;
  strap |= pinmux_testutils_read_strap_pin(pinmux, gpio, 24,
                                           kTestPinoutInit.straps[2])
           << 4;
  return strap;
}

void pinmux_testutils_configure_pads(const dif_pinmux_t *pinmux,
                                     const pinmux_pad_attributes_t *attrs,
                                     size_t num_attrs) {
  for (size_t i = 0; i < num_attrs; ++i) {
    dif_pinmux_pad_attr_t desired_attr, actual_attr;
    CHECK_DIF_OK(
        dif_pinmux_pad_get_attrs_dt(pinmux, attrs[i].pad, &desired_attr));
    desired_attr.flags = attrs[i].flags;
    CHECK_DIF_OK(dif_pinmux_pad_write_attrs_dt(pinmux, attrs[i].pad,
                                               desired_attr, &actual_attr));
  }
}
