// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENTITAN_SW_DEVICE_LIB_TESTING_TEST_PINOUT_H_
#define OPENTITAN_SW_DEVICE_LIB_TESTING_TEST_PINOUT_H_

/**
 * @file
 * @brief Which pads test software connects peripherals to.
 *
 * Which pad a test uses for a UART, an I2C bus, the straps or the GPIO test
 * pins is not part of the top description: the top leaves its muxed pads
 * unassigned, and the choice belongs to the DV testbench and the boards. Each
 * top therefore provides one implementation of this interface, named by the
 * `test_pinout` attribute of its description, and the testutils libraries
 * reach it through //hw/top:test_pinout. They contain no top names.
 */

#include <stdbool.h>

#include "hw/top/dt/api.h"   // Generated.
#include "hw/top/dt/uart.h"  // Generated.
#include "sw/device/lib/base/macros.h"
#include "sw/device/lib/base/status.h"
#include "sw/device/lib/dif/dif_gpio.h"
#include "sw/device/lib/dif/dif_pinmux.h"

// HAS_I2C: defined on a top with an I2C block (see test_pinout_intf).
#ifdef HAS_I2C
#include "hw/top/dt/i2c.h"  // Generated.
#endif

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

/**
 * Define the available external channels that a UART could be connected to.
 */
typedef enum uart_pinmux_channel {
  kUartPinmuxChannelConsole,
  kUartPinmuxChannelDut,
  kUartPinmuxChannelCount,
} uart_pinmux_channel_t;

/**
 * Define the available platforms which i2c is mapped
 */
typedef enum i2c_pinmux_platform_id {
  I2cPinmuxPlatformIdHyper310 = 0,
  I2cPinmuxPlatformIdDvsim,
  I2cPinmuxPlatformIdCw310Pmod,
  I2cPinmuxPlatformIdCount,
} i2c_pinmux_platform_id_t;

typedef enum spi_pinmux_platform_id {
  kSpiPinmuxPlatformIdCw310 = 0,
  kSpiPinmuxPlatformIdCw340,
  kSpiPinmuxPlatformIdTeacup,
  kSpiPinmuxPlatformIdCount,
} spi_pinmux_platform_id_t;

/**
 * The pads that `pinmux_testutils_init()` connects.
 */
typedef struct test_pinout_init {
  /** UART0 pads. */
  dt_pad_t uart0_rx;
  dt_pad_t uart0_tx;
  /** Whether to connect a second UART, and if so which one and its pads. */
  bool has_uart1;
  dt_uart_t uart1;
  dt_pad_t uart1_rx;
  dt_pad_t uart1_tx;
  /** Pads of the software straps, connected to GPIOs 22 to 24. */
  dt_pad_t straps[3];
} test_pinout_init_t;

/**
 * The pads that `pinmux_testutils_init()` connects.
 */
extern const test_pinout_init_t kTestPinoutInit;

/**
 * Anything else the top needs at the end of `pinmux_testutils_init()`, such as
 * pad attributes. Can be empty.
 *
 * @param pinmux A pinmux handle.
 */
void test_pinout_init_extra(dif_pinmux_t *pinmux);

/**
 * Maps the chip IOs to the GPIO peripheral in input and output directions.
 */
extern const dt_pad_t kPinmuxTestutilsGpioPads[kDifGpioNumPins];

/**
 * The GPIOs that tests can drive and read back on the platform the test runs
 * on, as a mask: which of the pads in `kPinmuxTestutilsGpioPads` the board
 * leaves free.
 *
 * @return A mask with a bit set for each testable GPIO.
 */
uint32_t test_pinout_testable_gpios_mask(void);

/**
 * Get the pads for a UART on a channel, for the platform the test runs on.
 *
 * @param uart_dt UART instance.
 * @param channel The channel to connect the UART to.
 * @param[out] rx_pad RX pad.
 * @param[out] tx_pad TX pad.
 * @return `kInvalidArgument` if that UART has no pads for this channel,
 * `kUnimplemented` if the top has no such pads.
 */
OT_WARN_UNUSED_RESULT
status_t test_pinout_uart_pads(dt_uart_t uart_dt, uart_pinmux_channel_t channel,
                               dt_pad_t *rx_pad, dt_pad_t *tx_pad);

#ifdef HAS_I2C
/**
 * Get the pads for an I2C bus on a platform. Only on a top with I2C.
 *
 * @param i2c_dt I2C instance.
 * @param platform The platform to connect the I2C to.
 * @param[out] sda_pad SDA pad.
 * @param[out] scl_pad SCL pad.
 * @return `kInvalidArgument` if that I2C has no pads on this platform,
 * `kUnimplemented` if the top has no such pads.
 */
OT_WARN_UNUSED_RESULT
status_t test_pinout_i2c_pads(dt_i2c_t i2c_dt,
                              i2c_pinmux_platform_id_t platform,
                              dt_pad_t *sda_pad, dt_pad_t *scl_pad);

/**
 * The I2C instance that tests use for a device on the board's I2C Pmod
 * (`I2cPinmuxPlatformIdCw310Pmod` pads). Only on a top with I2C.
 */
extern const dt_i2c_t kTestPinoutPmodI2c;
#endif  // HAS_I2C

// HAS_USBDEV: defined on a top with a USB device (see test_pinout_intf).
#ifdef HAS_USBDEV
/**
 * The pad that tests connect usbdev's VBUS sense input to. Only on a top with
 * a USB device.
 */
extern const dt_pad_t kTestPinoutUsbdevSense;

/**
 * Whether the chip level connects a differential receiver to usbdev's
 * `usb_rx_d` input, so that tests can set `use_diff_rcvr` in usbdev's
 * PHY_CONFIG. Without one, usbdev must use its single-ended D+/D- inputs:
 * with `use_diff_rcvr` set it would read a constant as receive data. This is
 * a property of the chip level (which PHY it connects to usbdev), not of the
 * board alone. Only on a top with a USB device.
 */
extern const bool kTestPinoutUsbdevHasDiffRcvr;
#endif  // HAS_USBDEV

// HAS_PWM: defined on a top with a PWM block (see test_pinout_intf).
#ifdef HAS_PWM
/**
 * The pads of the PWM test (`pwm_smoketest`), which the host samples with
 * GPIO bit-banging. Only on a top with a PWM block.
 */
typedef struct test_pinout_pwm_test_pads {
  /** The pad that PWM channel 0 drives, which the host samples. */
  dt_pad_t pwm;
  /** The pad that the host drives low while it samples, read by GPIO 0. */
  dt_pad_t sample_start;
} test_pinout_pwm_test_pads_t;

extern const test_pinout_pwm_test_pads_t kTestPinoutPwmTestPads;
#endif  // HAS_PWM

/**
 * Pads of SPI host 1 when connected to the BoB.
 */
typedef struct test_pinout_spi_host1_pads {
  dt_pad_t clk;
  dt_pad_t sd0;
  dt_pad_t sd1;
  dt_pad_t sd2;
  dt_pad_t sd3;
} test_pinout_spi_host1_pads_t;

/**
 * Get the pads of SPI host 1 when connected to the BoB, on a platform.
 *
 * @param platform_id The platform the test runs on.
 * @param[out] pads The pads.
 * @return `kUnimplemented` if the top has no such pads.
 */
OT_WARN_UNUSED_RESULT
status_t test_pinout_spi_host1_pads(spi_pinmux_platform_id_t platform_id,
                                    test_pinout_spi_host1_pads_t *pads);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // OPENTITAN_SW_DEVICE_LIB_TESTING_TEST_PINOUT_H_
