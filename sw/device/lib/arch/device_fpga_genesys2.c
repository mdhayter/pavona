// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// Copied (and modified) from sw/device/lib/arch/device_fpga_cw305.c.

#include <stdbool.h>

#include "sw/device/lib/arch/device.h"
#include "sw/device/lib/base/macros.h"
#include "sw/device/silicon_creator/lib/drivers/ibex.h"

/**
 * @file
 * @brief Device-specific symbol definitions for the Digilent Genesys 2 device.
 *
 * The clock frequencies are those of top_wren's Genesys 2 build
 * (`hw/top_wren/rtl/clkgen_xil7series.sv`): a 1200 MHz VCO divided by 120
 * for main and IO, 25 for USB and 40 * 120 for AON. The peripheral clock is
 * IO / 4.
 */

const device_type_t kDeviceType = kDeviceFpgaGenesys2;

const uint64_t kClockFreqCpuMhz = 10;

const uint64_t kClockFreqCpuHz = kClockFreqCpuMhz * 1000 * 1000;

uint64_t to_cpu_cycles(uint64_t usec) { return usec * kClockFreqCpuMhz; }

const uint64_t kClockFreqPeripheralHz = 25 * 100 * 1000;  // 2.5MHz

const uint64_t kClockFreqUsbHz = 48 * 1000 * 1000;  // 48MHz

const uint64_t kClockFreqAonHz = 250 * 1000;  // 250kHz

const uint64_t kUartBaudrate = 115200;

const uint32_t kUartNCOValue =
    CALCULATE_UART_NCO(kUartBaudrate, kClockFreqPeripheralHz);

const uint32_t kUartBaud115K =
    CALCULATE_UART_NCO(115200, kClockFreqPeripheralHz);
const uint32_t kUartBaud230K =
    CALCULATE_UART_NCO(115200 * 2, kClockFreqPeripheralHz);
const uint32_t kUartBaud460K =
    CALCULATE_UART_NCO(115200 * 4, kClockFreqPeripheralHz);
const uint32_t kUartBaud921K =
    CALCULATE_UART_NCO(115200 * 8, kClockFreqPeripheralHz);
const uint32_t kUartBaud1M33 =
    CALCULATE_UART_NCO(1333333, kClockFreqPeripheralHz);
const uint32_t kUartBaud1M50 =
    CALCULATE_UART_NCO(1500000, kClockFreqPeripheralHz);

const uint32_t kAstCheckPollCpuCycles =
    CALCULATE_AST_CHECK_POLL_CPU_CYCLES(kClockFreqCpuHz);

uintptr_t device_test_status_address(void) { return 0; }

uintptr_t device_log_bypass_uart_address(void) { return 0; }
