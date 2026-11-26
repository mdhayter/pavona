// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// Test pinout of top_egret: see sw/device/lib/testing/test_pinout.h.

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

// VBUS sense on IOC7.
const dt_pad_t kTestPinoutUsbdevSense = kDtPadIoc7;

// The chip level connects a USB differential receiver to `usb_rx_d`.
const bool kTestPinoutUsbdevHasDiffRcvr = true;

// I2C2 on the Pmod pads.
const dt_i2c_t kTestPinoutPmodI2c = kDtI2c2;

void test_pinout_init_extra(dif_pinmux_t *pinmux) {
  // Enable pull-ups on UART0 RX
  // Pull-ups are available only on certain platforms.
  if (kDeviceType == kDeviceSimDV) {
    dif_pinmux_pad_attr_t out_attr;
    dif_pinmux_pad_attr_t in_attr = {
        .slew_rate = 0,
        .drive_strength = 0,
        .flags = kDifPinmuxPadAttrPullResistorEnable |
                 kDifPinmuxPadAttrPullResistorUp};

    CHECK_DIF_OK(dif_pinmux_pad_write_attrs_dt(pinmux, kTestPinoutInit.uart0_rx,
                                               in_attr, &out_attr));
  };

  // Configure a higher drive strength for the USB_P and USB_N pads because
  // the pad drivers must be capable of overpowering the 'pull' signal
  // strength of the internal pull ups in the differential receiver.
  //
  // 'pull' strength is required because at the host end of the USB, there
  // are 'weak' pull downs, allowing it to detect device presence when it
  // applies its pull up.
  //    strong PAD driver > internal pull up > weak pull down at host
  //
  // Normally the pull up on USB_P will be asserted, but we may be employing
  // 'pin flipping' and instead choose to apply the _N pull up.
  if (kDeviceType == kDeviceSimDV) {
    dif_pinmux_pad_attr_t out_attr;
    dif_pinmux_pad_attr_t in_attr = {
        .slew_rate = 0, .drive_strength = 1, .flags = 0};

    CHECK_DIF_OK(dif_pinmux_pad_write_attrs_dt(pinmux, kDtPadUsbdevUsbDp,
                                               in_attr, &out_attr));
    CHECK_DIF_OK(dif_pinmux_pad_write_attrs_dt(pinmux, kDtPadUsbdevUsbDn,
                                               in_attr, &out_attr));
  }

  // Configure USBDEV SENSE outputs to be high-Z (IOC7)
  CHECK_DIF_OK(dif_pinmux_mio_select_output(pinmux, kDtPadIoc7,
                                            kDtPeriphIoConstantHighZ));
}

// Mapping of Chip IOs to the GPIO peripheral.
//
// Depending on the simulation platform, there may be a limitation to how chip
// IOs are allocated to the GPIO peripheral, even for testing. The DV testbench
// does not have this limitation, and is able to allocate as many chip IOs as
// the number of GPIOs supported by the peripheral. At this time, these pin
// assignments matches DV (see `hw/top_egret/dv/env/chip_if.sv`).
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

uint32_t test_pinout_testable_gpios_mask(void) {
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

status_t test_pinout_i2c_pads(dt_i2c_t i2c_dt,
                              i2c_pinmux_platform_id_t platform,
                              dt_pad_t *sda_pad, dt_pad_t *scl_pad) {
  switch (platform) {
    case I2cPinmuxPlatformIdHyper310:  // CW310 HyperDebug
      *sda_pad = kDtPadIoa7;
      *scl_pad = kDtPadIoa8;
      break;
    case I2cPinmuxPlatformIdDvsim:  // DV
      // In DV, there's one agent for each I2C instance, with a fixed set of
      // muxed pins.
      switch (i2c_dt) {
        case kDtI2c0:  // I2C0 uses the same pins as CW310 HyperDebug
          *sda_pad = kDtPadIoa7;
          *scl_pad = kDtPadIoa8;
          break;
        case kDtI2c1:
          *sda_pad = kDtPadIob10;
          *scl_pad = kDtPadIob9;
          break;
        case kDtI2c2:  // I2C2 uses the same pins as CW310 PMOD
          *sda_pad = kDtPadIob12;
          *scl_pad = kDtPadIob11;
          break;
        default:
          return INVALID_ARGUMENT();
      }
      break;
    case I2cPinmuxPlatformIdCw310Pmod:  // CW310 PMOD
      *sda_pad = kDtPadIob12;
      *scl_pad = kDtPadIob11;
      break;
    default:
      return INVALID_ARGUMENT();
  }

  return OK_STATUS();
}

/**
 * This table stores spi host 1 pin mappings of different platforms.
 * This is used to connect spi host 1 to mio pins based on the platform.
 */
static const test_pinout_spi_host1_pads_t kSpiHost1PinmuxMap[] = {
    [kSpiPinmuxPlatformIdCw310] =
        {
            .clk = kDtPadIoa3,
            .sd0 = kDtPadIoa5,
            .sd1 = kDtPadIoa4,
            .sd2 = kDtPadIoa8,
            .sd3 = kDtPadIoa7,
        },
    [kSpiPinmuxPlatformIdCw340] =
        {
            .clk = kDtPadIoa3,
            .sd0 = kDtPadIoa5,
            .sd1 = kDtPadIoa4,
            .sd2 = kDtPadIoa8,
            .sd3 = kDtPadIoa7,
        },
    [kSpiPinmuxPlatformIdTeacup] =
        {
            .clk = kDtPadIoa3,
            .sd0 = kDtPadIoa4,
            .sd1 = kDtPadIoa5,
            .sd2 = kDtPadIoa8,
            .sd3 = kDtPadIoa7,
        },
};

status_t test_pinout_spi_host1_pads(spi_pinmux_platform_id_t platform_id,
                                    test_pinout_spi_host1_pads_t *pads) {
  TRY_CHECK(platform_id < kSpiPinmuxPlatformIdCount);
  *pads = kSpiHost1PinmuxMap[platform_id];
  return OK_STATUS();
}
