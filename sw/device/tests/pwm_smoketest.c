// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

#include <stdint.h>

#include "hw/top/dt/api.h"     // Generated.
#include "hw/top/dt/gpio.h"    // Generated.
#include "hw/top/dt/pinmux.h"  // Generated.
#include "hw/top/dt/pwm.h"     // Generated.
#include "sw/device/lib/arch/device.h"
#include "sw/device/lib/dif/dif_gpio.h"
#include "sw/device/lib/dif/dif_pinmux.h"
#include "sw/device/lib/dif/dif_pwm.h"
#include "sw/device/lib/runtime/hart.h"
#include "sw/device/lib/runtime/log.h"
#include "sw/device/lib/testing/test_framework/check.h"
#include "sw/device/lib/testing/test_framework/ottf_main.h"
#include "sw/device/lib/testing/test_framework/ottf_utils.h"
#include "sw/device/lib/testing/test_pinout.h"

OTTF_DEFINE_TEST_CONFIG();

static const dt_pwm_t kPwmDt = 0;
static_assert(kDtPwmCount >= 1, "this test expects a pwm");
static const dt_pinmux_t kPinmuxDt = 0;
static_assert(kDtPinmuxCount == 1, "this test expects a pinmux");
static const dt_gpio_t kGpioDt = 0;
static_assert(kDtGpioCount >= 1, "this test expects a gpio");

static volatile const uint8_t kClocksHz[] = {24, 48, 96};
static volatile const uint8_t kDutyCycles[] = {11, 36, 50, 71, 91};

static dif_pwm_config_t compute_clk_config(uint32_t pwm_clk) {
  enum {
    kDutyCycleResolution = 5,
    kBeatsPerCycle = 1 << (kDutyCycleResolution + 1),  // 2 ^ (DC_RESN + 1)
  };
  return (dif_pwm_config_t){
      .beats_per_pulse_cycle = kBeatsPerCycle,
      .clock_divisor =
          ((uint32_t)kClockFreqAonHz / (kBeatsPerCycle * pwm_clk)) - 1};
}

bool test_main(void) {
  dif_pwm_t pwm;
  CHECK_DIF_OK(dif_pwm_init_from_dt(kPwmDt, &pwm));

  dif_pinmux_t pinmux;
  CHECK_DIF_OK(dif_pinmux_init_from_dt(kPinmuxDt, &pinmux));
  CHECK_DIF_OK(dif_pinmux_mio_select_output(
      &pinmux, kTestPinoutPwmTestPads.pwm,
      dt_pwm_periph_io(kPwmDt, kDtPwmPeriphIoPwm0)));

  dif_gpio_t gpio;
  CHECK_DIF_OK(dif_gpio_init_from_dt(kGpioDt, &gpio));
  CHECK_DIF_OK(dif_gpio_output_set_enabled_all(&gpio, 0x1));

  // The host drives the sample start pad to let the device know when the
  // sampling started.
  CHECK_DIF_OK(dif_pinmux_mio_select_input(
      &pinmux, dt_gpio_periph_io(kGpioDt, kDtGpioPeriphIoGpio0),
      kTestPinoutPwmTestPads.sample_start));

  for (size_t i = 0; i < ARRAYSIZE(kClocksHz); ++i) {
    dif_pwm_config_t pwm_config = compute_clk_config(kClocksHz[i]);
    for (size_t j = 0; j < ARRAYSIZE(kDutyCycles); ++j) {
      dif_pwm_channel_config_t channel_config = {
          .duty_cycle_a = (uint16_t)(pwm_config.beats_per_pulse_cycle *
                                     kDutyCycles[j] / 100),
          .duty_cycle_b = 0,
          .phase_delay = 0,
          .mode = kDifPwmModeFirmware,
          .polarity = kDifPwmPolarityActiveHigh,
          .blink_parameter_x = 0,
          .blink_parameter_y = 0,
      };

      CHECK_DIF_OK(dif_pwm_configure(&pwm, pwm_config));
      CHECK_DIF_OK(
          dif_pwm_configure_channel(&pwm, /* channel = */ 0, channel_config));

      // The sample start pad goes low when the host is sampling.
      bool not_sampling = true;
      do {
        CHECK_DIF_OK(dif_gpio_read(&gpio, 0, &not_sampling));
        if (!not_sampling) {  // Debounce
          busy_spin_micros(50);
          CHECK_DIF_OK(dif_gpio_read(&gpio, 0, &not_sampling));
        }
      } while (not_sampling);

      CHECK_DIF_OK(dif_pwm_phase_cntr_set_enabled(&pwm, kDifToggleEnabled));
      CHECK_DIF_OK(dif_pwm_channels_set_enabled(&pwm, /* channels = */ 1 << 0,
                                                kDifToggleEnabled));

      // The goes high when the host stop sampling.
      do {
        CHECK_DIF_OK(dif_gpio_read(&gpio, 0, &not_sampling));
      } while (!not_sampling);

      CHECK_DIF_OK(dif_pwm_channels_set_enabled(&pwm, /* channels = */ 1 << 0,
                                                kDifToggleDisabled));
      CHECK_DIF_OK(dif_pwm_phase_cntr_set_enabled(&pwm, kDifToggleDisabled));
    }
  }
  return true;
}
