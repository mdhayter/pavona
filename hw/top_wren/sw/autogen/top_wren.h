// Copyright lowRISC contributors (OpenTitan project).
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
//
// ------------------- W A R N I N G: A U T O - G E N E R A T E D   C O D E !! -------------------//
// PLEASE DO NOT HAND-EDIT THIS FILE. IT HAS BEEN AUTO-GENERATED WITH THE FOLLOWING COMMAND:
// util/gen_top_sw.py -t hw/top_wren/data/autogen/top_wren.gen.hjson

#ifndef OPENTITAN_HW_TOP_WREN_SW_AUTOGEN_TOP_WREN_H_
#define OPENTITAN_HW_TOP_WREN_SW_AUTOGEN_TOP_WREN_H_

/**
 * @file
 * @brief Top-specific Definitions
 *
 * This file contains preprocessor and type definitions for use within the
 * device C/C++ codebase.
 *
 * These definitions are for information that depends on the top-specific chip
 * configuration, which includes:
 * - Device Memory Information (for Peripherals and Memory)
 * - PLIC Interrupt ID Names and Source Mappings
 * - Pinmux Pin/Select Names
 * - Power Manager Wakeups
 */

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Peripheral base address for uart0 in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_UART0_BASE_ADDR 0x40000000u

/**
 * Peripheral size for uart0 in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_UART0_BASE_ADDR and
 * `TOP_WREN_UART0_BASE_ADDR + TOP_WREN_UART0_SIZE_BYTES`.
 */
#define TOP_WREN_UART0_SIZE_BYTES 0x40u

/**
 * Peripheral base address for uart1 in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_UART1_BASE_ADDR 0x40010000u

/**
 * Peripheral size for uart1 in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_UART1_BASE_ADDR and
 * `TOP_WREN_UART1_BASE_ADDR + TOP_WREN_UART1_SIZE_BYTES`.
 */
#define TOP_WREN_UART1_SIZE_BYTES 0x40u

/**
 * Peripheral base address for gpio in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_GPIO_BASE_ADDR 0x40040000u

/**
 * Peripheral size for gpio in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_GPIO_BASE_ADDR and
 * `TOP_WREN_GPIO_BASE_ADDR + TOP_WREN_GPIO_SIZE_BYTES`.
 */
#define TOP_WREN_GPIO_SIZE_BYTES 0x80u

/**
 * Peripheral base address for spi_device in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_SPI_DEVICE_BASE_ADDR 0x40050000u

/**
 * Peripheral size for spi_device in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_SPI_DEVICE_BASE_ADDR and
 * `TOP_WREN_SPI_DEVICE_BASE_ADDR + TOP_WREN_SPI_DEVICE_SIZE_BYTES`.
 */
#define TOP_WREN_SPI_DEVICE_SIZE_BYTES 0x2000u

/**
 * Peripheral base address for spi_host0 in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_SPI_HOST0_BASE_ADDR 0x40060000u

/**
 * Peripheral size for spi_host0 in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_SPI_HOST0_BASE_ADDR and
 * `TOP_WREN_SPI_HOST0_BASE_ADDR + TOP_WREN_SPI_HOST0_SIZE_BYTES`.
 */
#define TOP_WREN_SPI_HOST0_SIZE_BYTES 0x40u

/**
 * Peripheral base address for rv_timer in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_RV_TIMER_BASE_ADDR 0x40100000u

/**
 * Peripheral size for rv_timer in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_RV_TIMER_BASE_ADDR and
 * `TOP_WREN_RV_TIMER_BASE_ADDR + TOP_WREN_RV_TIMER_SIZE_BYTES`.
 */
#define TOP_WREN_RV_TIMER_SIZE_BYTES 0x200u

/**
 * Peripheral base address for usbdev in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_USBDEV_BASE_ADDR 0x40320000u

/**
 * Peripheral size for usbdev in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_USBDEV_BASE_ADDR and
 * `TOP_WREN_USBDEV_BASE_ADDR + TOP_WREN_USBDEV_SIZE_BYTES`.
 */
#define TOP_WREN_USBDEV_SIZE_BYTES 0x1000u

/**
 * Peripheral base address for pwrmgr_aon in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_PWRMGR_AON_BASE_ADDR 0x40400000u

/**
 * Peripheral size for pwrmgr_aon in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_PWRMGR_AON_BASE_ADDR and
 * `TOP_WREN_PWRMGR_AON_BASE_ADDR + TOP_WREN_PWRMGR_AON_SIZE_BYTES`.
 */
#define TOP_WREN_PWRMGR_AON_SIZE_BYTES 0x80u

/**
 * Peripheral base address for rstmgr_aon in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_RSTMGR_AON_BASE_ADDR 0x40410000u

/**
 * Peripheral size for rstmgr_aon in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_RSTMGR_AON_BASE_ADDR and
 * `TOP_WREN_RSTMGR_AON_BASE_ADDR + TOP_WREN_RSTMGR_AON_SIZE_BYTES`.
 */
#define TOP_WREN_RSTMGR_AON_SIZE_BYTES 0x40u

/**
 * Peripheral base address for clkmgr_aon in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_CLKMGR_AON_BASE_ADDR 0x40420000u

/**
 * Peripheral size for clkmgr_aon in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_CLKMGR_AON_BASE_ADDR and
 * `TOP_WREN_CLKMGR_AON_BASE_ADDR + TOP_WREN_CLKMGR_AON_SIZE_BYTES`.
 */
#define TOP_WREN_CLKMGR_AON_SIZE_BYTES 0x80u

/**
 * Peripheral base address for pinmux_aon in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_PINMUX_AON_BASE_ADDR 0x40460000u

/**
 * Peripheral size for pinmux_aon in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_PINMUX_AON_BASE_ADDR and
 * `TOP_WREN_PINMUX_AON_BASE_ADDR + TOP_WREN_PINMUX_AON_SIZE_BYTES`.
 */
#define TOP_WREN_PINMUX_AON_SIZE_BYTES 0x1000u

/**
 * Peripheral base address for aon_timer_aon in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_AON_TIMER_AON_BASE_ADDR 0x40470000u

/**
 * Peripheral size for aon_timer_aon in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_AON_TIMER_AON_BASE_ADDR and
 * `TOP_WREN_AON_TIMER_AON_BASE_ADDR + TOP_WREN_AON_TIMER_AON_SIZE_BYTES`.
 */
#define TOP_WREN_AON_TIMER_AON_SIZE_BYTES 0x40u

/**
 * Peripheral base address for ast in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_AST_BASE_ADDR 0x40480000u

/**
 * Peripheral size for ast in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_AST_BASE_ADDR and
 * `TOP_WREN_AST_BASE_ADDR + TOP_WREN_AST_SIZE_BYTES`.
 */
#define TOP_WREN_AST_SIZE_BYTES 0x400u

/**
 * Peripheral base address for core device on flash_ctrl in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_FLASH_CTRL_CORE_BASE_ADDR 0x41000000u

/**
 * Peripheral size for core device on flash_ctrl in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_FLASH_CTRL_CORE_BASE_ADDR and
 * `TOP_WREN_FLASH_CTRL_CORE_BASE_ADDR + TOP_WREN_FLASH_CTRL_CORE_SIZE_BYTES`.
 */
#define TOP_WREN_FLASH_CTRL_CORE_SIZE_BYTES 0x200u

/**
 * Peripheral base address for flash_macro_wrapper in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_FLASH_MACRO_WRAPPER_BASE_ADDR 0x41008000u

/**
 * Peripheral size for flash_macro_wrapper in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_FLASH_MACRO_WRAPPER_BASE_ADDR and
 * `TOP_WREN_FLASH_MACRO_WRAPPER_BASE_ADDR + TOP_WREN_FLASH_MACRO_WRAPPER_SIZE_BYTES`.
 */
#define TOP_WREN_FLASH_MACRO_WRAPPER_SIZE_BYTES 0x80u

/**
 * Peripheral base address for rv_plic in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_RV_PLIC_BASE_ADDR 0x48000000u

/**
 * Peripheral size for rv_plic in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_RV_PLIC_BASE_ADDR and
 * `TOP_WREN_RV_PLIC_BASE_ADDR + TOP_WREN_RV_PLIC_SIZE_BYTES`.
 */
#define TOP_WREN_RV_PLIC_SIZE_BYTES 0x8000000u

/**
 * Peripheral base address for aes in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_AES_BASE_ADDR 0x41100000u

/**
 * Peripheral size for aes in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_AES_BASE_ADDR and
 * `TOP_WREN_AES_BASE_ADDR + TOP_WREN_AES_SIZE_BYTES`.
 */
#define TOP_WREN_AES_SIZE_BYTES 0x100u

/**
 * Peripheral base address for regs device on sram_ctrl_main in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_SRAM_CTRL_MAIN_REGS_BASE_ADDR 0x411C0000u

/**
 * Peripheral size for regs device on sram_ctrl_main in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_SRAM_CTRL_MAIN_REGS_BASE_ADDR and
 * `TOP_WREN_SRAM_CTRL_MAIN_REGS_BASE_ADDR + TOP_WREN_SRAM_CTRL_MAIN_REGS_SIZE_BYTES`.
 */
#define TOP_WREN_SRAM_CTRL_MAIN_REGS_SIZE_BYTES 0x40u

/**
 * Peripheral base address for regs device on rom_ctrl in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_ROM_CTRL_REGS_BASE_ADDR 0x411E0000u

/**
 * Peripheral size for regs device on rom_ctrl in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_ROM_CTRL_REGS_BASE_ADDR and
 * `TOP_WREN_ROM_CTRL_REGS_BASE_ADDR + TOP_WREN_ROM_CTRL_REGS_SIZE_BYTES`.
 */
#define TOP_WREN_ROM_CTRL_REGS_SIZE_BYTES 0x80u

/**
 * Peripheral base address for cfg device on rv_core_ibex in top wren.
 *
 * This should be used with #mmio_region_from_addr to access the memory-mapped
 * registers associated with the peripheral (usually via a DIF).
 */
#define TOP_WREN_RV_CORE_IBEX_CFG_BASE_ADDR 0x411F0000u

/**
 * Peripheral size for cfg device on rv_core_ibex in top wren.
 *
 * This is the size (in bytes) of the peripheral's reserved memory area. All
 * memory-mapped registers associated with this peripheral should have an
 * address between #TOP_WREN_RV_CORE_IBEX_CFG_BASE_ADDR and
 * `TOP_WREN_RV_CORE_IBEX_CFG_BASE_ADDR + TOP_WREN_RV_CORE_IBEX_CFG_SIZE_BYTES`.
 */
#define TOP_WREN_RV_CORE_IBEX_CFG_SIZE_BYTES 0x100u


/**
 * Memory base address for mem memory on flash_ctrl in top wren.
 */
#define TOP_WREN_FLASH_CTRL_MEM_BASE_ADDR 0x20000000u

/**
 * Memory size for mem memory on flash_ctrl in top wren.
 */
#define TOP_WREN_FLASH_CTRL_MEM_SIZE_BYTES 0x10000u

/**
 * Memory base address for ram memory on sram_ctrl_main in top wren.
 */
#define TOP_WREN_SRAM_CTRL_MAIN_RAM_BASE_ADDR 0x10000000u

/**
 * Memory size for ram memory on sram_ctrl_main in top wren.
 */
#define TOP_WREN_SRAM_CTRL_MAIN_RAM_SIZE_BYTES 0x20000u

/**
 * Memory base address for rom memory on rom_ctrl in top wren.
 */
#define TOP_WREN_ROM_CTRL_ROM_BASE_ADDR 0x8000u

/**
 * Memory size for rom memory on rom_ctrl in top wren.
 */
#define TOP_WREN_ROM_CTRL_ROM_SIZE_BYTES 0x8000u


/**
 * PLIC Interrupt Source Peripheral.
 *
 * Enumeration used to determine which peripheral asserted the corresponding
 * interrupt.
 */
typedef enum top_wren_plic_peripheral {
  kTopWrenPlicPeripheralUnknown = 0, /**< Unknown Peripheral */
  kTopWrenPlicPeripheralUart0 = 1, /**< uart0 */
  kTopWrenPlicPeripheralUart1 = 2, /**< uart1 */
  kTopWrenPlicPeripheralGpio = 3, /**< gpio */
  kTopWrenPlicPeripheralSpiDevice = 4, /**< spi_device */
  kTopWrenPlicPeripheralSpiHost0 = 5, /**< spi_host0 */
  kTopWrenPlicPeripheralUsbdev = 6, /**< usbdev */
  kTopWrenPlicPeripheralPwrmgrAon = 7, /**< pwrmgr_aon */
  kTopWrenPlicPeripheralAonTimerAon = 8, /**< aon_timer_aon */
  kTopWrenPlicPeripheralFlashCtrl = 9, /**< flash_ctrl */
  kTopWrenPlicPeripheralLast = 9, /**< \internal Final PLIC peripheral */
} top_wren_plic_peripheral_t;

/**
 * PLIC Interrupt Source.
 *
 * Enumeration of all PLIC interrupt sources. The interrupt sources belonging to
 * the same peripheral are guaranteed to be consecutive.
 */
typedef enum top_wren_plic_irq_id {
  kTopWrenPlicIrqIdNone = 0, /**< No Interrupt */
  kTopWrenPlicIrqIdUart0TxWatermark = 1, /**< uart0_tx_watermark */
  kTopWrenPlicIrqIdUart0RxWatermark = 2, /**< uart0_rx_watermark */
  kTopWrenPlicIrqIdUart0TxDone = 3, /**< uart0_tx_done */
  kTopWrenPlicIrqIdUart0RxOverflow = 4, /**< uart0_rx_overflow */
  kTopWrenPlicIrqIdUart0RxFrameErr = 5, /**< uart0_rx_frame_err */
  kTopWrenPlicIrqIdUart0RxBreakErr = 6, /**< uart0_rx_break_err */
  kTopWrenPlicIrqIdUart0RxTimeout = 7, /**< uart0_rx_timeout */
  kTopWrenPlicIrqIdUart0RxParityErr = 8, /**< uart0_rx_parity_err */
  kTopWrenPlicIrqIdUart0TxEmpty = 9, /**< uart0_tx_empty */
  kTopWrenPlicIrqIdUart1TxWatermark = 10, /**< uart1_tx_watermark */
  kTopWrenPlicIrqIdUart1RxWatermark = 11, /**< uart1_rx_watermark */
  kTopWrenPlicIrqIdUart1TxDone = 12, /**< uart1_tx_done */
  kTopWrenPlicIrqIdUart1RxOverflow = 13, /**< uart1_rx_overflow */
  kTopWrenPlicIrqIdUart1RxFrameErr = 14, /**< uart1_rx_frame_err */
  kTopWrenPlicIrqIdUart1RxBreakErr = 15, /**< uart1_rx_break_err */
  kTopWrenPlicIrqIdUart1RxTimeout = 16, /**< uart1_rx_timeout */
  kTopWrenPlicIrqIdUart1RxParityErr = 17, /**< uart1_rx_parity_err */
  kTopWrenPlicIrqIdUart1TxEmpty = 18, /**< uart1_tx_empty */
  kTopWrenPlicIrqIdGpioGpio0 = 19, /**< gpio_gpio 0 */
  kTopWrenPlicIrqIdGpioGpio1 = 20, /**< gpio_gpio 1 */
  kTopWrenPlicIrqIdGpioGpio2 = 21, /**< gpio_gpio 2 */
  kTopWrenPlicIrqIdGpioGpio3 = 22, /**< gpio_gpio 3 */
  kTopWrenPlicIrqIdGpioGpio4 = 23, /**< gpio_gpio 4 */
  kTopWrenPlicIrqIdGpioGpio5 = 24, /**< gpio_gpio 5 */
  kTopWrenPlicIrqIdGpioGpio6 = 25, /**< gpio_gpio 6 */
  kTopWrenPlicIrqIdGpioGpio7 = 26, /**< gpio_gpio 7 */
  kTopWrenPlicIrqIdGpioGpio8 = 27, /**< gpio_gpio 8 */
  kTopWrenPlicIrqIdGpioGpio9 = 28, /**< gpio_gpio 9 */
  kTopWrenPlicIrqIdGpioGpio10 = 29, /**< gpio_gpio 10 */
  kTopWrenPlicIrqIdGpioGpio11 = 30, /**< gpio_gpio 11 */
  kTopWrenPlicIrqIdGpioGpio12 = 31, /**< gpio_gpio 12 */
  kTopWrenPlicIrqIdGpioGpio13 = 32, /**< gpio_gpio 13 */
  kTopWrenPlicIrqIdGpioGpio14 = 33, /**< gpio_gpio 14 */
  kTopWrenPlicIrqIdGpioGpio15 = 34, /**< gpio_gpio 15 */
  kTopWrenPlicIrqIdGpioGpio16 = 35, /**< gpio_gpio 16 */
  kTopWrenPlicIrqIdGpioGpio17 = 36, /**< gpio_gpio 17 */
  kTopWrenPlicIrqIdGpioGpio18 = 37, /**< gpio_gpio 18 */
  kTopWrenPlicIrqIdGpioGpio19 = 38, /**< gpio_gpio 19 */
  kTopWrenPlicIrqIdGpioGpio20 = 39, /**< gpio_gpio 20 */
  kTopWrenPlicIrqIdGpioGpio21 = 40, /**< gpio_gpio 21 */
  kTopWrenPlicIrqIdGpioGpio22 = 41, /**< gpio_gpio 22 */
  kTopWrenPlicIrqIdGpioGpio23 = 42, /**< gpio_gpio 23 */
  kTopWrenPlicIrqIdGpioGpio24 = 43, /**< gpio_gpio 24 */
  kTopWrenPlicIrqIdGpioGpio25 = 44, /**< gpio_gpio 25 */
  kTopWrenPlicIrqIdGpioGpio26 = 45, /**< gpio_gpio 26 */
  kTopWrenPlicIrqIdGpioGpio27 = 46, /**< gpio_gpio 27 */
  kTopWrenPlicIrqIdGpioGpio28 = 47, /**< gpio_gpio 28 */
  kTopWrenPlicIrqIdGpioGpio29 = 48, /**< gpio_gpio 29 */
  kTopWrenPlicIrqIdGpioGpio30 = 49, /**< gpio_gpio 30 */
  kTopWrenPlicIrqIdGpioGpio31 = 50, /**< gpio_gpio 31 */
  kTopWrenPlicIrqIdSpiDeviceUploadCmdfifoNotEmpty = 51, /**< spi_device_upload_cmdfifo_not_empty */
  kTopWrenPlicIrqIdSpiDeviceUploadPayloadNotEmpty = 52, /**< spi_device_upload_payload_not_empty */
  kTopWrenPlicIrqIdSpiDeviceUploadPayloadOverflow = 53, /**< spi_device_upload_payload_overflow */
  kTopWrenPlicIrqIdSpiDeviceReadbufWatermark = 54, /**< spi_device_readbuf_watermark */
  kTopWrenPlicIrqIdSpiDeviceReadbufFlip = 55, /**< spi_device_readbuf_flip */
  kTopWrenPlicIrqIdSpiDeviceTpmHeaderNotEmpty = 56, /**< spi_device_tpm_header_not_empty */
  kTopWrenPlicIrqIdSpiDeviceTpmRdfifoCmdEnd = 57, /**< spi_device_tpm_rdfifo_cmd_end */
  kTopWrenPlicIrqIdSpiDeviceTpmRdfifoDrop = 58, /**< spi_device_tpm_rdfifo_drop */
  kTopWrenPlicIrqIdSpiHost0Error = 59, /**< spi_host0_error */
  kTopWrenPlicIrqIdSpiHost0SpiEvent = 60, /**< spi_host0_spi_event */
  kTopWrenPlicIrqIdUsbdevPktReceived = 61, /**< usbdev_pkt_received */
  kTopWrenPlicIrqIdUsbdevPktSent = 62, /**< usbdev_pkt_sent */
  kTopWrenPlicIrqIdUsbdevDisconnected = 63, /**< usbdev_disconnected */
  kTopWrenPlicIrqIdUsbdevHostLost = 64, /**< usbdev_host_lost */
  kTopWrenPlicIrqIdUsbdevLinkReset = 65, /**< usbdev_link_reset */
  kTopWrenPlicIrqIdUsbdevLinkSuspend = 66, /**< usbdev_link_suspend */
  kTopWrenPlicIrqIdUsbdevLinkResume = 67, /**< usbdev_link_resume */
  kTopWrenPlicIrqIdUsbdevAvOutEmpty = 68, /**< usbdev_av_out_empty */
  kTopWrenPlicIrqIdUsbdevRxFull = 69, /**< usbdev_rx_full */
  kTopWrenPlicIrqIdUsbdevAvOverflow = 70, /**< usbdev_av_overflow */
  kTopWrenPlicIrqIdUsbdevLinkInErr = 71, /**< usbdev_link_in_err */
  kTopWrenPlicIrqIdUsbdevRxCrcErr = 72, /**< usbdev_rx_crc_err */
  kTopWrenPlicIrqIdUsbdevRxPidErr = 73, /**< usbdev_rx_pid_err */
  kTopWrenPlicIrqIdUsbdevRxBitstuffErr = 74, /**< usbdev_rx_bitstuff_err */
  kTopWrenPlicIrqIdUsbdevFrame = 75, /**< usbdev_frame */
  kTopWrenPlicIrqIdUsbdevPowered = 76, /**< usbdev_powered */
  kTopWrenPlicIrqIdUsbdevLinkOutErr = 77, /**< usbdev_link_out_err */
  kTopWrenPlicIrqIdUsbdevAvSetupEmpty = 78, /**< usbdev_av_setup_empty */
  kTopWrenPlicIrqIdPwrmgrAonWakeup = 79, /**< pwrmgr_aon_wakeup */
  kTopWrenPlicIrqIdAonTimerAonWkupTimerExpired = 80, /**< aon_timer_aon_wkup_timer_expired */
  kTopWrenPlicIrqIdAonTimerAonWdogTimerBark = 81, /**< aon_timer_aon_wdog_timer_bark */
  kTopWrenPlicIrqIdFlashCtrlProgEmpty = 82, /**< flash_ctrl_prog_empty */
  kTopWrenPlicIrqIdFlashCtrlProgLvl = 83, /**< flash_ctrl_prog_lvl */
  kTopWrenPlicIrqIdFlashCtrlRdFull = 84, /**< flash_ctrl_rd_full */
  kTopWrenPlicIrqIdFlashCtrlRdLvl = 85, /**< flash_ctrl_rd_lvl */
  kTopWrenPlicIrqIdFlashCtrlOpDone = 86, /**< flash_ctrl_op_done */
  kTopWrenPlicIrqIdFlashCtrlCorrErr = 87, /**< flash_ctrl_corr_err */
  kTopWrenPlicIrqIdLast = 87, /**< \internal The Last Valid Interrupt ID. */
} top_wren_plic_irq_id_t;

/**
 * PLIC Interrupt Source to Peripheral Map
 *
 * This array is a mapping from `top_wren_plic_irq_id_t` to
 * `top_wren_plic_peripheral_t`.
 */
extern const top_wren_plic_peripheral_t
    top_wren_plic_interrupt_for_peripheral[88];

/**
 * PLIC Interrupt Target.
 *
 * Enumeration used to determine which set of IE, CC, threshold registers to
 * access for a given interrupt target.
 */
typedef enum top_wren_plic_target {
  kTopWrenPlicTargetIbex0 = 0, /**< Ibex Core 0 */
  kTopWrenPlicTargetLast = 0, /**< \internal Final PLIC target */
} top_wren_plic_target_t;


#define PINMUX_MIO_PERIPH_INSEL_IDX_OFFSET 2

// PERIPH_INSEL ranges from 0 to NUM_MIO_PADS + 2 -1}
//  0 and 1 are tied to value 0 and 1
#define NUM_MIO_PADS 47
#define NUM_DIO_PADS 14

#define PINMUX_PERIPH_OUTSEL_IDX_OFFSET 3

/**
 * Pinmux Peripheral Input.
 */
typedef enum top_wren_pinmux_peripheral_in {
  kTopWrenPinmuxPeripheralInGpioGpio0 = 0, /**< Peripheral Input 0 */
  kTopWrenPinmuxPeripheralInGpioGpio1 = 1, /**< Peripheral Input 1 */
  kTopWrenPinmuxPeripheralInGpioGpio2 = 2, /**< Peripheral Input 2 */
  kTopWrenPinmuxPeripheralInGpioGpio3 = 3, /**< Peripheral Input 3 */
  kTopWrenPinmuxPeripheralInGpioGpio4 = 4, /**< Peripheral Input 4 */
  kTopWrenPinmuxPeripheralInGpioGpio5 = 5, /**< Peripheral Input 5 */
  kTopWrenPinmuxPeripheralInGpioGpio6 = 6, /**< Peripheral Input 6 */
  kTopWrenPinmuxPeripheralInGpioGpio7 = 7, /**< Peripheral Input 7 */
  kTopWrenPinmuxPeripheralInGpioGpio8 = 8, /**< Peripheral Input 8 */
  kTopWrenPinmuxPeripheralInGpioGpio9 = 9, /**< Peripheral Input 9 */
  kTopWrenPinmuxPeripheralInGpioGpio10 = 10, /**< Peripheral Input 10 */
  kTopWrenPinmuxPeripheralInGpioGpio11 = 11, /**< Peripheral Input 11 */
  kTopWrenPinmuxPeripheralInGpioGpio12 = 12, /**< Peripheral Input 12 */
  kTopWrenPinmuxPeripheralInGpioGpio13 = 13, /**< Peripheral Input 13 */
  kTopWrenPinmuxPeripheralInGpioGpio14 = 14, /**< Peripheral Input 14 */
  kTopWrenPinmuxPeripheralInGpioGpio15 = 15, /**< Peripheral Input 15 */
  kTopWrenPinmuxPeripheralInGpioGpio16 = 16, /**< Peripheral Input 16 */
  kTopWrenPinmuxPeripheralInGpioGpio17 = 17, /**< Peripheral Input 17 */
  kTopWrenPinmuxPeripheralInGpioGpio18 = 18, /**< Peripheral Input 18 */
  kTopWrenPinmuxPeripheralInGpioGpio19 = 19, /**< Peripheral Input 19 */
  kTopWrenPinmuxPeripheralInGpioGpio20 = 20, /**< Peripheral Input 20 */
  kTopWrenPinmuxPeripheralInGpioGpio21 = 21, /**< Peripheral Input 21 */
  kTopWrenPinmuxPeripheralInGpioGpio22 = 22, /**< Peripheral Input 22 */
  kTopWrenPinmuxPeripheralInGpioGpio23 = 23, /**< Peripheral Input 23 */
  kTopWrenPinmuxPeripheralInGpioGpio24 = 24, /**< Peripheral Input 24 */
  kTopWrenPinmuxPeripheralInGpioGpio25 = 25, /**< Peripheral Input 25 */
  kTopWrenPinmuxPeripheralInGpioGpio26 = 26, /**< Peripheral Input 26 */
  kTopWrenPinmuxPeripheralInGpioGpio27 = 27, /**< Peripheral Input 27 */
  kTopWrenPinmuxPeripheralInGpioGpio28 = 28, /**< Peripheral Input 28 */
  kTopWrenPinmuxPeripheralInGpioGpio29 = 29, /**< Peripheral Input 29 */
  kTopWrenPinmuxPeripheralInGpioGpio30 = 30, /**< Peripheral Input 30 */
  kTopWrenPinmuxPeripheralInGpioGpio31 = 31, /**< Peripheral Input 31 */
  kTopWrenPinmuxPeripheralInUart0Rx = 32, /**< Peripheral Input 32 */
  kTopWrenPinmuxPeripheralInUart1Rx = 33, /**< Peripheral Input 33 */
  kTopWrenPinmuxPeripheralInUsbdevSense = 34, /**< Peripheral Input 34 */
  kTopWrenPinmuxPeripheralInLast = 34, /**< \internal Last valid peripheral input */
} top_wren_pinmux_peripheral_in_t;

/**
 * Pinmux MIO Input Selector.
 */
typedef enum top_wren_pinmux_insel {
  kTopWrenPinmuxInselConstantZero = 0, /**< Tie constantly to zero */
  kTopWrenPinmuxInselConstantOne = 1, /**< Tie constantly to one */
  kTopWrenPinmuxInselIoa0 = 2, /**< MIO Pad 0 */
  kTopWrenPinmuxInselIoa1 = 3, /**< MIO Pad 1 */
  kTopWrenPinmuxInselIoa2 = 4, /**< MIO Pad 2 */
  kTopWrenPinmuxInselIoa3 = 5, /**< MIO Pad 3 */
  kTopWrenPinmuxInselIoa4 = 6, /**< MIO Pad 4 */
  kTopWrenPinmuxInselIoa5 = 7, /**< MIO Pad 5 */
  kTopWrenPinmuxInselIoa6 = 8, /**< MIO Pad 6 */
  kTopWrenPinmuxInselIoa7 = 9, /**< MIO Pad 7 */
  kTopWrenPinmuxInselIoa8 = 10, /**< MIO Pad 8 */
  kTopWrenPinmuxInselIob0 = 11, /**< MIO Pad 9 */
  kTopWrenPinmuxInselIob1 = 12, /**< MIO Pad 10 */
  kTopWrenPinmuxInselIob2 = 13, /**< MIO Pad 11 */
  kTopWrenPinmuxInselIob3 = 14, /**< MIO Pad 12 */
  kTopWrenPinmuxInselIob4 = 15, /**< MIO Pad 13 */
  kTopWrenPinmuxInselIob5 = 16, /**< MIO Pad 14 */
  kTopWrenPinmuxInselIob6 = 17, /**< MIO Pad 15 */
  kTopWrenPinmuxInselIob7 = 18, /**< MIO Pad 16 */
  kTopWrenPinmuxInselIob8 = 19, /**< MIO Pad 17 */
  kTopWrenPinmuxInselIob9 = 20, /**< MIO Pad 18 */
  kTopWrenPinmuxInselIob10 = 21, /**< MIO Pad 19 */
  kTopWrenPinmuxInselIob11 = 22, /**< MIO Pad 20 */
  kTopWrenPinmuxInselIob12 = 23, /**< MIO Pad 21 */
  kTopWrenPinmuxInselIoc0 = 24, /**< MIO Pad 22 */
  kTopWrenPinmuxInselIoc1 = 25, /**< MIO Pad 23 */
  kTopWrenPinmuxInselIoc2 = 26, /**< MIO Pad 24 */
  kTopWrenPinmuxInselIoc3 = 27, /**< MIO Pad 25 */
  kTopWrenPinmuxInselIoc4 = 28, /**< MIO Pad 26 */
  kTopWrenPinmuxInselIoc5 = 29, /**< MIO Pad 27 */
  kTopWrenPinmuxInselIoc6 = 30, /**< MIO Pad 28 */
  kTopWrenPinmuxInselIoc7 = 31, /**< MIO Pad 29 */
  kTopWrenPinmuxInselIoc8 = 32, /**< MIO Pad 30 */
  kTopWrenPinmuxInselIoc9 = 33, /**< MIO Pad 31 */
  kTopWrenPinmuxInselIoc10 = 34, /**< MIO Pad 32 */
  kTopWrenPinmuxInselIoc11 = 35, /**< MIO Pad 33 */
  kTopWrenPinmuxInselIoc12 = 36, /**< MIO Pad 34 */
  kTopWrenPinmuxInselIor0 = 37, /**< MIO Pad 35 */
  kTopWrenPinmuxInselIor1 = 38, /**< MIO Pad 36 */
  kTopWrenPinmuxInselIor2 = 39, /**< MIO Pad 37 */
  kTopWrenPinmuxInselIor3 = 40, /**< MIO Pad 38 */
  kTopWrenPinmuxInselIor4 = 41, /**< MIO Pad 39 */
  kTopWrenPinmuxInselIor5 = 42, /**< MIO Pad 40 */
  kTopWrenPinmuxInselIor6 = 43, /**< MIO Pad 41 */
  kTopWrenPinmuxInselIor7 = 44, /**< MIO Pad 42 */
  kTopWrenPinmuxInselIor10 = 45, /**< MIO Pad 43 */
  kTopWrenPinmuxInselIor11 = 46, /**< MIO Pad 44 */
  kTopWrenPinmuxInselIor12 = 47, /**< MIO Pad 45 */
  kTopWrenPinmuxInselIor13 = 48, /**< MIO Pad 46 */
  kTopWrenPinmuxInselLast = 48, /**< \internal Last valid insel value */
} top_wren_pinmux_insel_t;

/**
 * Pinmux MIO Output.
 */
typedef enum top_wren_pinmux_mio_out {
  kTopWrenPinmuxMioOutIoa0 = 0, /**< MIO Pad 0 */
  kTopWrenPinmuxMioOutIoa1 = 1, /**< MIO Pad 1 */
  kTopWrenPinmuxMioOutIoa2 = 2, /**< MIO Pad 2 */
  kTopWrenPinmuxMioOutIoa3 = 3, /**< MIO Pad 3 */
  kTopWrenPinmuxMioOutIoa4 = 4, /**< MIO Pad 4 */
  kTopWrenPinmuxMioOutIoa5 = 5, /**< MIO Pad 5 */
  kTopWrenPinmuxMioOutIoa6 = 6, /**< MIO Pad 6 */
  kTopWrenPinmuxMioOutIoa7 = 7, /**< MIO Pad 7 */
  kTopWrenPinmuxMioOutIoa8 = 8, /**< MIO Pad 8 */
  kTopWrenPinmuxMioOutIob0 = 9, /**< MIO Pad 9 */
  kTopWrenPinmuxMioOutIob1 = 10, /**< MIO Pad 10 */
  kTopWrenPinmuxMioOutIob2 = 11, /**< MIO Pad 11 */
  kTopWrenPinmuxMioOutIob3 = 12, /**< MIO Pad 12 */
  kTopWrenPinmuxMioOutIob4 = 13, /**< MIO Pad 13 */
  kTopWrenPinmuxMioOutIob5 = 14, /**< MIO Pad 14 */
  kTopWrenPinmuxMioOutIob6 = 15, /**< MIO Pad 15 */
  kTopWrenPinmuxMioOutIob7 = 16, /**< MIO Pad 16 */
  kTopWrenPinmuxMioOutIob8 = 17, /**< MIO Pad 17 */
  kTopWrenPinmuxMioOutIob9 = 18, /**< MIO Pad 18 */
  kTopWrenPinmuxMioOutIob10 = 19, /**< MIO Pad 19 */
  kTopWrenPinmuxMioOutIob11 = 20, /**< MIO Pad 20 */
  kTopWrenPinmuxMioOutIob12 = 21, /**< MIO Pad 21 */
  kTopWrenPinmuxMioOutIoc0 = 22, /**< MIO Pad 22 */
  kTopWrenPinmuxMioOutIoc1 = 23, /**< MIO Pad 23 */
  kTopWrenPinmuxMioOutIoc2 = 24, /**< MIO Pad 24 */
  kTopWrenPinmuxMioOutIoc3 = 25, /**< MIO Pad 25 */
  kTopWrenPinmuxMioOutIoc4 = 26, /**< MIO Pad 26 */
  kTopWrenPinmuxMioOutIoc5 = 27, /**< MIO Pad 27 */
  kTopWrenPinmuxMioOutIoc6 = 28, /**< MIO Pad 28 */
  kTopWrenPinmuxMioOutIoc7 = 29, /**< MIO Pad 29 */
  kTopWrenPinmuxMioOutIoc8 = 30, /**< MIO Pad 30 */
  kTopWrenPinmuxMioOutIoc9 = 31, /**< MIO Pad 31 */
  kTopWrenPinmuxMioOutIoc10 = 32, /**< MIO Pad 32 */
  kTopWrenPinmuxMioOutIoc11 = 33, /**< MIO Pad 33 */
  kTopWrenPinmuxMioOutIoc12 = 34, /**< MIO Pad 34 */
  kTopWrenPinmuxMioOutIor0 = 35, /**< MIO Pad 35 */
  kTopWrenPinmuxMioOutIor1 = 36, /**< MIO Pad 36 */
  kTopWrenPinmuxMioOutIor2 = 37, /**< MIO Pad 37 */
  kTopWrenPinmuxMioOutIor3 = 38, /**< MIO Pad 38 */
  kTopWrenPinmuxMioOutIor4 = 39, /**< MIO Pad 39 */
  kTopWrenPinmuxMioOutIor5 = 40, /**< MIO Pad 40 */
  kTopWrenPinmuxMioOutIor6 = 41, /**< MIO Pad 41 */
  kTopWrenPinmuxMioOutIor7 = 42, /**< MIO Pad 42 */
  kTopWrenPinmuxMioOutIor10 = 43, /**< MIO Pad 43 */
  kTopWrenPinmuxMioOutIor11 = 44, /**< MIO Pad 44 */
  kTopWrenPinmuxMioOutIor12 = 45, /**< MIO Pad 45 */
  kTopWrenPinmuxMioOutIor13 = 46, /**< MIO Pad 46 */
  kTopWrenPinmuxMioOutLast = 46, /**< \internal Last valid mio output */
} top_wren_pinmux_mio_out_t;

/**
 * Pinmux Peripheral Output Selector.
 */
typedef enum top_wren_pinmux_outsel {
  kTopWrenPinmuxOutselConstantZero = 0, /**< Tie constantly to zero */
  kTopWrenPinmuxOutselConstantOne = 1, /**< Tie constantly to one */
  kTopWrenPinmuxOutselConstantHighZ = 2, /**< Tie constantly to high-Z */
  kTopWrenPinmuxOutselGpioGpio0 = 3, /**< Peripheral Output 0 */
  kTopWrenPinmuxOutselGpioGpio1 = 4, /**< Peripheral Output 1 */
  kTopWrenPinmuxOutselGpioGpio2 = 5, /**< Peripheral Output 2 */
  kTopWrenPinmuxOutselGpioGpio3 = 6, /**< Peripheral Output 3 */
  kTopWrenPinmuxOutselGpioGpio4 = 7, /**< Peripheral Output 4 */
  kTopWrenPinmuxOutselGpioGpio5 = 8, /**< Peripheral Output 5 */
  kTopWrenPinmuxOutselGpioGpio6 = 9, /**< Peripheral Output 6 */
  kTopWrenPinmuxOutselGpioGpio7 = 10, /**< Peripheral Output 7 */
  kTopWrenPinmuxOutselGpioGpio8 = 11, /**< Peripheral Output 8 */
  kTopWrenPinmuxOutselGpioGpio9 = 12, /**< Peripheral Output 9 */
  kTopWrenPinmuxOutselGpioGpio10 = 13, /**< Peripheral Output 10 */
  kTopWrenPinmuxOutselGpioGpio11 = 14, /**< Peripheral Output 11 */
  kTopWrenPinmuxOutselGpioGpio12 = 15, /**< Peripheral Output 12 */
  kTopWrenPinmuxOutselGpioGpio13 = 16, /**< Peripheral Output 13 */
  kTopWrenPinmuxOutselGpioGpio14 = 17, /**< Peripheral Output 14 */
  kTopWrenPinmuxOutselGpioGpio15 = 18, /**< Peripheral Output 15 */
  kTopWrenPinmuxOutselGpioGpio16 = 19, /**< Peripheral Output 16 */
  kTopWrenPinmuxOutselGpioGpio17 = 20, /**< Peripheral Output 17 */
  kTopWrenPinmuxOutselGpioGpio18 = 21, /**< Peripheral Output 18 */
  kTopWrenPinmuxOutselGpioGpio19 = 22, /**< Peripheral Output 19 */
  kTopWrenPinmuxOutselGpioGpio20 = 23, /**< Peripheral Output 20 */
  kTopWrenPinmuxOutselGpioGpio21 = 24, /**< Peripheral Output 21 */
  kTopWrenPinmuxOutselGpioGpio22 = 25, /**< Peripheral Output 22 */
  kTopWrenPinmuxOutselGpioGpio23 = 26, /**< Peripheral Output 23 */
  kTopWrenPinmuxOutselGpioGpio24 = 27, /**< Peripheral Output 24 */
  kTopWrenPinmuxOutselGpioGpio25 = 28, /**< Peripheral Output 25 */
  kTopWrenPinmuxOutselGpioGpio26 = 29, /**< Peripheral Output 26 */
  kTopWrenPinmuxOutselGpioGpio27 = 30, /**< Peripheral Output 27 */
  kTopWrenPinmuxOutselGpioGpio28 = 31, /**< Peripheral Output 28 */
  kTopWrenPinmuxOutselGpioGpio29 = 32, /**< Peripheral Output 29 */
  kTopWrenPinmuxOutselGpioGpio30 = 33, /**< Peripheral Output 30 */
  kTopWrenPinmuxOutselGpioGpio31 = 34, /**< Peripheral Output 31 */
  kTopWrenPinmuxOutselUart0Tx = 35, /**< Peripheral Output 32 */
  kTopWrenPinmuxOutselUart1Tx = 36, /**< Peripheral Output 33 */
  kTopWrenPinmuxOutselLast = 36, /**< \internal Last valid outsel value */
} top_wren_pinmux_outsel_t;

/**
 * Dedicated Pad Selects
 */
typedef enum top_wren_direct_pads {
  kTopWrenDirectPadsSpiHost0Sd0 = 0, /**<  */
  kTopWrenDirectPadsSpiHost0Sd1 = 1, /**<  */
  kTopWrenDirectPadsSpiHost0Sd2 = 2, /**<  */
  kTopWrenDirectPadsSpiHost0Sd3 = 3, /**<  */
  kTopWrenDirectPadsSpiDeviceSd0 = 4, /**<  */
  kTopWrenDirectPadsSpiDeviceSd1 = 5, /**<  */
  kTopWrenDirectPadsSpiDeviceSd2 = 6, /**<  */
  kTopWrenDirectPadsSpiDeviceSd3 = 7, /**<  */
  kTopWrenDirectPadsUsbdevUsbDp = 8, /**<  */
  kTopWrenDirectPadsUsbdevUsbDn = 9, /**<  */
  kTopWrenDirectPadsSpiDeviceSck = 10, /**<  */
  kTopWrenDirectPadsSpiDeviceCsb = 11, /**<  */
  kTopWrenDirectPadsSpiHost0Sck = 12, /**<  */
  kTopWrenDirectPadsSpiHost0Csb = 13, /**<  */
  kTopWrenDirectPadsLast = 13, /**< \internal Last valid direct pad */
} top_wren_direct_pads_t;

/**
 * Muxed Pad Selects
 */
typedef enum top_wren_muxed_pads {
  kTopWrenMuxedPadsIoa0 = 0, /**<  */
  kTopWrenMuxedPadsIoa1 = 1, /**<  */
  kTopWrenMuxedPadsIoa2 = 2, /**<  */
  kTopWrenMuxedPadsIoa3 = 3, /**<  */
  kTopWrenMuxedPadsIoa4 = 4, /**<  */
  kTopWrenMuxedPadsIoa5 = 5, /**<  */
  kTopWrenMuxedPadsIoa6 = 6, /**<  */
  kTopWrenMuxedPadsIoa7 = 7, /**<  */
  kTopWrenMuxedPadsIoa8 = 8, /**<  */
  kTopWrenMuxedPadsIob0 = 9, /**<  */
  kTopWrenMuxedPadsIob1 = 10, /**<  */
  kTopWrenMuxedPadsIob2 = 11, /**<  */
  kTopWrenMuxedPadsIob3 = 12, /**<  */
  kTopWrenMuxedPadsIob4 = 13, /**<  */
  kTopWrenMuxedPadsIob5 = 14, /**<  */
  kTopWrenMuxedPadsIob6 = 15, /**<  */
  kTopWrenMuxedPadsIob7 = 16, /**<  */
  kTopWrenMuxedPadsIob8 = 17, /**<  */
  kTopWrenMuxedPadsIob9 = 18, /**<  */
  kTopWrenMuxedPadsIob10 = 19, /**<  */
  kTopWrenMuxedPadsIob11 = 20, /**<  */
  kTopWrenMuxedPadsIob12 = 21, /**<  */
  kTopWrenMuxedPadsIoc0 = 22, /**<  */
  kTopWrenMuxedPadsIoc1 = 23, /**<  */
  kTopWrenMuxedPadsIoc2 = 24, /**<  */
  kTopWrenMuxedPadsIoc3 = 25, /**<  */
  kTopWrenMuxedPadsIoc4 = 26, /**<  */
  kTopWrenMuxedPadsIoc5 = 27, /**<  */
  kTopWrenMuxedPadsIoc6 = 28, /**<  */
  kTopWrenMuxedPadsIoc7 = 29, /**<  */
  kTopWrenMuxedPadsIoc8 = 30, /**<  */
  kTopWrenMuxedPadsIoc9 = 31, /**<  */
  kTopWrenMuxedPadsIoc10 = 32, /**<  */
  kTopWrenMuxedPadsIoc11 = 33, /**<  */
  kTopWrenMuxedPadsIoc12 = 34, /**<  */
  kTopWrenMuxedPadsIor0 = 35, /**<  */
  kTopWrenMuxedPadsIor1 = 36, /**<  */
  kTopWrenMuxedPadsIor2 = 37, /**<  */
  kTopWrenMuxedPadsIor3 = 38, /**<  */
  kTopWrenMuxedPadsIor4 = 39, /**<  */
  kTopWrenMuxedPadsIor5 = 40, /**<  */
  kTopWrenMuxedPadsIor6 = 41, /**<  */
  kTopWrenMuxedPadsIor7 = 42, /**<  */
  kTopWrenMuxedPadsIor10 = 43, /**<  */
  kTopWrenMuxedPadsIor11 = 44, /**<  */
  kTopWrenMuxedPadsIor12 = 45, /**<  */
  kTopWrenMuxedPadsIor13 = 46, /**<  */
  kTopWrenMuxedPadsLast = 46, /**< \internal Last valid muxed pad */
} top_wren_muxed_pads_t;

/**
 * Power Manager Wakeup Signals
 */
typedef enum top_wren_power_manager_wake_ups {
  kTopWrenPowerManagerWakeUpsPinmuxAonPinWkupReq = 0, /**<  */
  kTopWrenPowerManagerWakeUpsPinmuxAonUsbWkupReq = 1, /**<  */
  kTopWrenPowerManagerWakeUpsAonTimerAonWkupReq = 2, /**<  */
  kTopWrenPowerManagerWakeUpsLast = 2, /**< \internal Last valid pwrmgr wakeup signal */
} top_wren_power_manager_wake_ups_t;

/**
 * Reset Manager Software Controlled Resets
 */
typedef enum top_wren_reset_manager_sw_resets {
  kTopWrenResetManagerSwResetsSpiDevice = 0, /**<  */
  kTopWrenResetManagerSwResetsSpiHost0 = 1, /**<  */
  kTopWrenResetManagerSwResetsUsb = 2, /**<  */
  kTopWrenResetManagerSwResetsLast = 2, /**< \internal Last valid rstmgr software reset request */
} top_wren_reset_manager_sw_resets_t;

/**
 * Power Manager Reset Request Signals
 */
typedef enum top_wren_power_manager_reset_requests {
  kTopWrenPowerManagerResetRequestsAonTimerAonAonTimerRstReq = 0, /**<  */
  kTopWrenPowerManagerResetRequestsLast = 0, /**< \internal Last valid pwrmgr reset_request signal */
} top_wren_power_manager_reset_requests_t;

/**
 * Clock Manager Software-Controlled ("Gated") Clocks.
 *
 * The Software has full control over these clocks.
 */
typedef enum top_wren_gateable_clocks {
  kTopWrenGateableClocksIoDiv4Peri = 0, /**< Clock clk_io_div4_peri in group peri */
  kTopWrenGateableClocksIoDiv2Peri = 1, /**< Clock clk_io_div2_peri in group peri */
  kTopWrenGateableClocksIoPeri = 2, /**< Clock clk_io_peri in group peri */
  kTopWrenGateableClocksUsbPeri = 3, /**< Clock clk_usb_peri in group peri */
  kTopWrenGateableClocksLast = 3, /**< \internal Last Valid Gateable Clock */
} top_wren_gateable_clocks_t;

/**
 * Clock Manager Software-Hinted Clocks.
 *
 * The Software has partial control over these clocks. It can ask them to stop,
 * but the clock manager is in control of whether the clock actually is stopped.
 */
typedef enum top_wren_hintable_clocks {
  kTopWrenHintableClocksMainAes = 0, /**< Clock clk_main_aes in group trans */
  kTopWrenHintableClocksLast = 0, /**< \internal Last Valid Hintable Clock */
} top_wren_hintable_clocks_t;

/**
 * MMIO Region
 *
 * MMIO region excludes any memory that is separate from the module
 * configuration space, i.e. ROM, main SRAM, and flash are excluded but
 * retention SRAM, spi_device memory, or usbdev memory are included.
 */
#define TOP_WREN_MMIO_BASE_ADDR 0x40000000u
#define TOP_WREN_MMIO_SIZE_BYTES 0x10000000u

// Header Extern Guard
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // OPENTITAN_HW_TOP_WREN_SW_AUTOGEN_TOP_WREN_H_
