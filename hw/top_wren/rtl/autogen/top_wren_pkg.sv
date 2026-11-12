// Copyright lowRISC contributors (OpenTitan project).
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
//
// ------------------- W A R N I N G: A U T O - G E N E R A T E D   C O D E !! -------------------//
// PLEASE DO NOT HAND-EDIT THIS FILE. IT HAS BEEN AUTO-GENERATED WITH THE FOLLOWING COMMAND:
//
// util/gen_top_sv.py --completecfg hw/top_wren/data/autogen/top_wren.gen.hjson
//                    --seedcfg hw/top_wren/data/autogen/top_wren.secrets.testing.gen.hjson

package top_wren_pkg;
  /**
   * Peripheral base address for uart0 in top wren.
   */
  parameter int unsigned TOP_WREN_UART0_BASE_ADDR = 32'h40000000;

  /**
   * Peripheral size in bytes for uart0 in top wren.
   */
  parameter int unsigned TOP_WREN_UART0_SIZE_BYTES = 32'h40;

  /**
   * Peripheral base address for uart1 in top wren.
   */
  parameter int unsigned TOP_WREN_UART1_BASE_ADDR = 32'h40010000;

  /**
   * Peripheral size in bytes for uart1 in top wren.
   */
  parameter int unsigned TOP_WREN_UART1_SIZE_BYTES = 32'h40;

  /**
   * Peripheral base address for gpio in top wren.
   */
  parameter int unsigned TOP_WREN_GPIO_BASE_ADDR = 32'h40040000;

  /**
   * Peripheral size in bytes for gpio in top wren.
   */
  parameter int unsigned TOP_WREN_GPIO_SIZE_BYTES = 32'h80;

  /**
   * Peripheral base address for spi_device in top wren.
   */
  parameter int unsigned TOP_WREN_SPI_DEVICE_BASE_ADDR = 32'h40050000;

  /**
   * Peripheral size in bytes for spi_device in top wren.
   */
  parameter int unsigned TOP_WREN_SPI_DEVICE_SIZE_BYTES = 32'h2000;

  /**
   * Peripheral base address for i2c0 in top wren.
   */
  parameter int unsigned TOP_WREN_I2C0_BASE_ADDR = 32'h40080000;

  /**
   * Peripheral size in bytes for i2c0 in top wren.
   */
  parameter int unsigned TOP_WREN_I2C0_SIZE_BYTES = 32'h80;

  /**
   * Peripheral base address for spi_host0 in top wren.
   */
  parameter int unsigned TOP_WREN_SPI_HOST0_BASE_ADDR = 32'h40060000;

  /**
   * Peripheral size in bytes for spi_host0 in top wren.
   */
  parameter int unsigned TOP_WREN_SPI_HOST0_SIZE_BYTES = 32'h40;

  /**
   * Peripheral base address for rv_timer in top wren.
   */
  parameter int unsigned TOP_WREN_RV_TIMER_BASE_ADDR = 32'h40100000;

  /**
   * Peripheral size in bytes for rv_timer in top wren.
   */
  parameter int unsigned TOP_WREN_RV_TIMER_SIZE_BYTES = 32'h200;

  /**
   * Peripheral base address for usbdev in top wren.
   */
  parameter int unsigned TOP_WREN_USBDEV_BASE_ADDR = 32'h40320000;

  /**
   * Peripheral size in bytes for usbdev in top wren.
   */
  parameter int unsigned TOP_WREN_USBDEV_SIZE_BYTES = 32'h1000;

  /**
   * Peripheral base address for pwrmgr_aon in top wren.
   */
  parameter int unsigned TOP_WREN_PWRMGR_AON_BASE_ADDR = 32'h40400000;

  /**
   * Peripheral size in bytes for pwrmgr_aon in top wren.
   */
  parameter int unsigned TOP_WREN_PWRMGR_AON_SIZE_BYTES = 32'h80;

  /**
   * Peripheral base address for spi_host1 in top wren.
   */
  parameter int unsigned TOP_WREN_SPI_HOST1_BASE_ADDR = 32'h40310000;

  /**
   * Peripheral size in bytes for spi_host1 in top wren.
   */
  parameter int unsigned TOP_WREN_SPI_HOST1_SIZE_BYTES = 32'h40;

  /**
   * Peripheral base address for pwm_aon in top wren.
   */
  parameter int unsigned TOP_WREN_PWM_AON_BASE_ADDR = 32'h40450000;

  /**
   * Peripheral size in bytes for pwm_aon in top wren.
   */
  parameter int unsigned TOP_WREN_PWM_AON_SIZE_BYTES = 32'h80;

  /**
   * Peripheral base address for rstmgr_aon in top wren.
   */
  parameter int unsigned TOP_WREN_RSTMGR_AON_BASE_ADDR = 32'h40410000;

  /**
   * Peripheral size in bytes for rstmgr_aon in top wren.
   */
  parameter int unsigned TOP_WREN_RSTMGR_AON_SIZE_BYTES = 32'h80;

  /**
   * Peripheral base address for clkmgr_aon in top wren.
   */
  parameter int unsigned TOP_WREN_CLKMGR_AON_BASE_ADDR = 32'h40420000;

  /**
   * Peripheral size in bytes for clkmgr_aon in top wren.
   */
  parameter int unsigned TOP_WREN_CLKMGR_AON_SIZE_BYTES = 32'h80;

  /**
   * Peripheral base address for pinmux_aon in top wren.
   */
  parameter int unsigned TOP_WREN_PINMUX_AON_BASE_ADDR = 32'h40460000;

  /**
   * Peripheral size in bytes for pinmux_aon in top wren.
   */
  parameter int unsigned TOP_WREN_PINMUX_AON_SIZE_BYTES = 32'h1000;

  /**
   * Peripheral base address for aon_timer_aon in top wren.
   */
  parameter int unsigned TOP_WREN_AON_TIMER_AON_BASE_ADDR = 32'h40470000;

  /**
   * Peripheral size in bytes for aon_timer_aon in top wren.
   */
  parameter int unsigned TOP_WREN_AON_TIMER_AON_SIZE_BYTES = 32'h40;

  /**
   * Peripheral base address for ast in top wren.
   */
  parameter int unsigned TOP_WREN_AST_BASE_ADDR = 32'h40480000;

  /**
   * Peripheral size in bytes for ast in top wren.
   */
  parameter int unsigned TOP_WREN_AST_SIZE_BYTES = 32'h400;

  /**
   * Peripheral base address for core device on flash_ctrl in top wren.
   */
  parameter int unsigned TOP_WREN_FLASH_CTRL_CORE_BASE_ADDR = 32'h41000000;

  /**
   * Peripheral size in bytes for core device on flash_ctrl in top wren.
   */
  parameter int unsigned TOP_WREN_FLASH_CTRL_CORE_SIZE_BYTES = 32'h200;

  /**
   * Peripheral base address for flash_macro_wrapper in top wren.
   */
  parameter int unsigned TOP_WREN_FLASH_MACRO_WRAPPER_BASE_ADDR = 32'h41008000;

  /**
   * Peripheral size in bytes for flash_macro_wrapper in top wren.
   */
  parameter int unsigned TOP_WREN_FLASH_MACRO_WRAPPER_SIZE_BYTES = 32'h80;

  /**
   * Peripheral base address for rv_plic in top wren.
   */
  parameter int unsigned TOP_WREN_RV_PLIC_BASE_ADDR = 32'h48000000;

  /**
   * Peripheral size in bytes for rv_plic in top wren.
   */
  parameter int unsigned TOP_WREN_RV_PLIC_SIZE_BYTES = 32'h8000000;

  /**
   * Peripheral base address for regs device on sram_ctrl_main in top wren.
   */
  parameter int unsigned TOP_WREN_SRAM_CTRL_MAIN_REGS_BASE_ADDR = 32'h411C0000;

  /**
   * Peripheral size in bytes for regs device on sram_ctrl_main in top wren.
   */
  parameter int unsigned TOP_WREN_SRAM_CTRL_MAIN_REGS_SIZE_BYTES = 32'h40;

  /**
   * Peripheral base address for regs device on rom_ctrl in top wren.
   */
  parameter int unsigned TOP_WREN_ROM_CTRL_REGS_BASE_ADDR = 32'h411E0000;

  /**
   * Peripheral size in bytes for regs device on rom_ctrl in top wren.
   */
  parameter int unsigned TOP_WREN_ROM_CTRL_REGS_SIZE_BYTES = 32'h80;

  /**
   * Peripheral base address for regs device on rv_dm in top wren.
   */
  parameter int unsigned TOP_WREN_RV_DM_REGS_BASE_ADDR = 32'h41200000;

  /**
   * Peripheral size in bytes for regs device on rv_dm in top wren.
   */
  parameter int unsigned TOP_WREN_RV_DM_REGS_SIZE_BYTES = 32'h10;

  /**
   * Peripheral base address for mem device on rv_dm in top wren.
   */
  parameter int unsigned TOP_WREN_RV_DM_MEM_BASE_ADDR = 32'h10000;

  /**
   * Peripheral size in bytes for mem device on rv_dm in top wren.
   */
  parameter int unsigned TOP_WREN_RV_DM_MEM_SIZE_BYTES = 32'h1000;

  /**
   * Peripheral base address for dbg device on rv_dm in top wren.
   */
  parameter int unsigned TOP_WREN_RV_DM_DBG_BASE_ADDR = 32'h1000;

  /**
   * Peripheral size in bytes for dbg device on rv_dm in top wren.
   */
  parameter int unsigned TOP_WREN_RV_DM_DBG_SIZE_BYTES = 32'h200;

  /**
   * Peripheral base address for cfg device on rv_core_ibex in top wren.
   */
  parameter int unsigned TOP_WREN_RV_CORE_IBEX_CFG_BASE_ADDR = 32'h411F0000;

  /**
   * Peripheral size in bytes for cfg device on rv_core_ibex in top wren.
   */
  parameter int unsigned TOP_WREN_RV_CORE_IBEX_CFG_SIZE_BYTES = 32'h100;

  /**
   * Memory base address for mem memory on flash_ctrl in top wren.
   */
  parameter int unsigned TOP_WREN_FLASH_CTRL_MEM_BASE_ADDR = 32'h20000000;

  /**
   * Memory size for mem memory on flash_ctrl in top wren.
   */
  parameter int unsigned TOP_WREN_FLASH_CTRL_MEM_SIZE_BYTES = 32'h10000;

  /**
   * Memory base address for ram memory on sram_ctrl_main in top wren.
   */
  parameter int unsigned TOP_WREN_SRAM_CTRL_MAIN_RAM_BASE_ADDR = 32'h10000000;

  /**
   * Memory size for ram memory on sram_ctrl_main in top wren.
   */
  parameter int unsigned TOP_WREN_SRAM_CTRL_MAIN_RAM_SIZE_BYTES = 32'h20000;

  /**
   * Memory base address for rom memory on rom_ctrl in top wren.
   */
  parameter int unsigned TOP_WREN_ROM_CTRL_ROM_BASE_ADDR = 32'h8000;

  /**
   * Memory size for rom memory on rom_ctrl in top wren.
   */
  parameter int unsigned TOP_WREN_ROM_CTRL_ROM_SIZE_BYTES = 32'h8000;


  // Number of wren outgoing alerts
  parameter int unsigned NOutgoingAlertsWren = 30;

  // Number of LPGs for outgoing alert group wren
  parameter int unsigned NOutgoingLpgsWren = 15;

  // Enumeration of wren outgoing alert modules
  typedef enum int unsigned {
    TopWrenAlertPeripheralUart0 = 0,
    TopWrenAlertPeripheralUart1 = 1,
    TopWrenAlertPeripheralGpio = 2,
    TopWrenAlertPeripheralSpiDevice = 3,
    TopWrenAlertPeripheralI2c0 = 4,
    TopWrenAlertPeripheralSpiHost0 = 5,
    TopWrenAlertPeripheralRvTimer = 6,
    TopWrenAlertPeripheralUsbdev = 7,
    TopWrenAlertPeripheralPwrmgrAon = 8,
    TopWrenAlertPeripheralSpiHost1 = 9,
    TopWrenAlertPeripheralPwmAon = 10,
    TopWrenAlertPeripheralRstmgrAon = 11,
    TopWrenAlertPeripheralClkmgrAon = 12,
    TopWrenAlertPeripheralPinmuxAon = 13,
    TopWrenAlertPeripheralAonTimerAon = 14,
    TopWrenAlertPeripheralFlashCtrl = 15,
    TopWrenAlertPeripheralRvPlic = 16,
    TopWrenAlertPeripheralSramCtrlMain = 17,
    TopWrenAlertPeripheralRomCtrl = 18,
    TopWrenAlertPeripheralRvDm = 19,
    TopWrenAlertPeripheralRvCoreIbex = 20,
    TopWrenOutgoingAlertWrenPeripheralCount
  } outgoing_alert_wren_peripheral_e;

  // Enumeration of wren outgoing alerts
  typedef enum int unsigned {
    TopWrenAlertIdUart0FatalFault = 0,
    TopWrenAlertIdUart1FatalFault = 1,
    TopWrenAlertIdGpioFatalFault = 2,
    TopWrenAlertIdSpiDeviceFatalFault = 3,
    TopWrenAlertIdI2c0FatalFault = 4,
    TopWrenAlertIdSpiHost0FatalFault = 5,
    TopWrenAlertIdRvTimerFatalFault = 6,
    TopWrenAlertIdUsbdevFatalFault = 7,
    TopWrenAlertIdPwrmgrAonFatalFault = 8,
    TopWrenAlertIdSpiHost1FatalFault = 9,
    TopWrenAlertIdPwmAonFatalFault = 10,
    TopWrenAlertIdRstmgrAonFatalFault = 11,
    TopWrenAlertIdRstmgrAonFatalCnstyFault = 12,
    TopWrenAlertIdClkmgrAonRecovFault = 13,
    TopWrenAlertIdClkmgrAonFatalFault = 14,
    TopWrenAlertIdPinmuxAonFatalFault = 15,
    TopWrenAlertIdAonTimerAonFatalFault = 16,
    TopWrenAlertIdFlashCtrlRecovErr = 17,
    TopWrenAlertIdFlashCtrlFatalStdErr = 18,
    TopWrenAlertIdFlashCtrlFatalErr = 19,
    TopWrenAlertIdFlashCtrlFatalPrimFlashAlert = 20,
    TopWrenAlertIdFlashCtrlRecovPrimFlashAlert = 21,
    TopWrenAlertIdRvPlicFatalFault = 22,
    TopWrenAlertIdSramCtrlMainFatalError = 23,
    TopWrenAlertIdRomCtrlFatal = 24,
    TopWrenAlertIdRvDmFatalFault = 25,
    TopWrenAlertIdRvCoreIbexFatalSwErr = 26,
    TopWrenAlertIdRvCoreIbexRecovSwErr = 27,
    TopWrenAlertIdRvCoreIbexFatalHwErr = 28,
    TopWrenAlertIdRvCoreIbexRecovHwErr = 29,
    TopWrenOutgoingAlertWrenIdCount
  } outgoing_alert_wren_id_e;

  // Enumeration of wren outgoing alerts AsyncOn configuration
  parameter logic [NOutgoingAlertsWren-1:0] AsyncOnOutgoingAlertWren = {
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1,
    1'b1
  };

  // Enumeration of interrupts
  typedef enum int unsigned {
    TopWrenPlicIrqIdNone = 0,
    TopWrenPlicIrqIdUart0TxWatermark = 1,
    TopWrenPlicIrqIdUart0RxWatermark = 2,
    TopWrenPlicIrqIdUart0TxDone = 3,
    TopWrenPlicIrqIdUart0RxOverflow = 4,
    TopWrenPlicIrqIdUart0RxFrameErr = 5,
    TopWrenPlicIrqIdUart0RxBreakErr = 6,
    TopWrenPlicIrqIdUart0RxTimeout = 7,
    TopWrenPlicIrqIdUart0RxParityErr = 8,
    TopWrenPlicIrqIdUart0TxEmpty = 9,
    TopWrenPlicIrqIdUart1TxWatermark = 10,
    TopWrenPlicIrqIdUart1RxWatermark = 11,
    TopWrenPlicIrqIdUart1TxDone = 12,
    TopWrenPlicIrqIdUart1RxOverflow = 13,
    TopWrenPlicIrqIdUart1RxFrameErr = 14,
    TopWrenPlicIrqIdUart1RxBreakErr = 15,
    TopWrenPlicIrqIdUart1RxTimeout = 16,
    TopWrenPlicIrqIdUart1RxParityErr = 17,
    TopWrenPlicIrqIdUart1TxEmpty = 18,
    TopWrenPlicIrqIdGpioGpio0 = 19,
    TopWrenPlicIrqIdGpioGpio1 = 20,
    TopWrenPlicIrqIdGpioGpio2 = 21,
    TopWrenPlicIrqIdGpioGpio3 = 22,
    TopWrenPlicIrqIdGpioGpio4 = 23,
    TopWrenPlicIrqIdGpioGpio5 = 24,
    TopWrenPlicIrqIdGpioGpio6 = 25,
    TopWrenPlicIrqIdGpioGpio7 = 26,
    TopWrenPlicIrqIdGpioGpio8 = 27,
    TopWrenPlicIrqIdGpioGpio9 = 28,
    TopWrenPlicIrqIdGpioGpio10 = 29,
    TopWrenPlicIrqIdGpioGpio11 = 30,
    TopWrenPlicIrqIdGpioGpio12 = 31,
    TopWrenPlicIrqIdGpioGpio13 = 32,
    TopWrenPlicIrqIdGpioGpio14 = 33,
    TopWrenPlicIrqIdGpioGpio15 = 34,
    TopWrenPlicIrqIdGpioGpio16 = 35,
    TopWrenPlicIrqIdGpioGpio17 = 36,
    TopWrenPlicIrqIdGpioGpio18 = 37,
    TopWrenPlicIrqIdGpioGpio19 = 38,
    TopWrenPlicIrqIdGpioGpio20 = 39,
    TopWrenPlicIrqIdGpioGpio21 = 40,
    TopWrenPlicIrqIdGpioGpio22 = 41,
    TopWrenPlicIrqIdGpioGpio23 = 42,
    TopWrenPlicIrqIdGpioGpio24 = 43,
    TopWrenPlicIrqIdGpioGpio25 = 44,
    TopWrenPlicIrqIdGpioGpio26 = 45,
    TopWrenPlicIrqIdGpioGpio27 = 46,
    TopWrenPlicIrqIdGpioGpio28 = 47,
    TopWrenPlicIrqIdGpioGpio29 = 48,
    TopWrenPlicIrqIdGpioGpio30 = 49,
    TopWrenPlicIrqIdGpioGpio31 = 50,
    TopWrenPlicIrqIdSpiDeviceUploadCmdfifoNotEmpty = 51,
    TopWrenPlicIrqIdSpiDeviceUploadPayloadNotEmpty = 52,
    TopWrenPlicIrqIdSpiDeviceUploadPayloadOverflow = 53,
    TopWrenPlicIrqIdSpiDeviceReadbufWatermark = 54,
    TopWrenPlicIrqIdSpiDeviceReadbufFlip = 55,
    TopWrenPlicIrqIdSpiDeviceTpmHeaderNotEmpty = 56,
    TopWrenPlicIrqIdSpiDeviceTpmRdfifoCmdEnd = 57,
    TopWrenPlicIrqIdSpiDeviceTpmRdfifoDrop = 58,
    TopWrenPlicIrqIdI2c0FmtThreshold = 59,
    TopWrenPlicIrqIdI2c0RxThreshold = 60,
    TopWrenPlicIrqIdI2c0AcqThreshold = 61,
    TopWrenPlicIrqIdI2c0RxOverflow = 62,
    TopWrenPlicIrqIdI2c0ControllerHalt = 63,
    TopWrenPlicIrqIdI2c0SclInterference = 64,
    TopWrenPlicIrqIdI2c0SdaInterference = 65,
    TopWrenPlicIrqIdI2c0StretchTimeout = 66,
    TopWrenPlicIrqIdI2c0SdaUnstable = 67,
    TopWrenPlicIrqIdI2c0CmdComplete = 68,
    TopWrenPlicIrqIdI2c0TxStretch = 69,
    TopWrenPlicIrqIdI2c0TxThreshold = 70,
    TopWrenPlicIrqIdI2c0AcqStretch = 71,
    TopWrenPlicIrqIdI2c0UnexpStop = 72,
    TopWrenPlicIrqIdI2c0HostTimeout = 73,
    TopWrenPlicIrqIdSpiHost0Error = 74,
    TopWrenPlicIrqIdSpiHost0SpiEvent = 75,
    TopWrenPlicIrqIdSpiHost1Error = 76,
    TopWrenPlicIrqIdSpiHost1SpiEvent = 77,
    TopWrenPlicIrqIdUsbdevPktReceived = 78,
    TopWrenPlicIrqIdUsbdevPktSent = 79,
    TopWrenPlicIrqIdUsbdevDisconnected = 80,
    TopWrenPlicIrqIdUsbdevHostLost = 81,
    TopWrenPlicIrqIdUsbdevLinkReset = 82,
    TopWrenPlicIrqIdUsbdevLinkSuspend = 83,
    TopWrenPlicIrqIdUsbdevLinkResume = 84,
    TopWrenPlicIrqIdUsbdevAvOutEmpty = 85,
    TopWrenPlicIrqIdUsbdevRxFull = 86,
    TopWrenPlicIrqIdUsbdevAvOverflow = 87,
    TopWrenPlicIrqIdUsbdevLinkInErr = 88,
    TopWrenPlicIrqIdUsbdevRxCrcErr = 89,
    TopWrenPlicIrqIdUsbdevRxPidErr = 90,
    TopWrenPlicIrqIdUsbdevRxBitstuffErr = 91,
    TopWrenPlicIrqIdUsbdevFrame = 92,
    TopWrenPlicIrqIdUsbdevPowered = 93,
    TopWrenPlicIrqIdUsbdevLinkOutErr = 94,
    TopWrenPlicIrqIdUsbdevAvSetupEmpty = 95,
    TopWrenPlicIrqIdPwrmgrAonWakeup = 96,
    TopWrenPlicIrqIdAonTimerAonWkupTimerExpired = 97,
    TopWrenPlicIrqIdAonTimerAonWdogTimerBark = 98,
    TopWrenPlicIrqIdFlashCtrlProgEmpty = 99,
    TopWrenPlicIrqIdFlashCtrlProgLvl = 100,
    TopWrenPlicIrqIdFlashCtrlRdFull = 101,
    TopWrenPlicIrqIdFlashCtrlRdLvl = 102,
    TopWrenPlicIrqIdFlashCtrlOpDone = 103,
    TopWrenPlicIrqIdFlashCtrlCorrErr = 104,
    TopWrenPlicIrqIdCount
  } interrupt_rv_plic_id_e;


  // Enumeration of IO power domains.
  // Only used in ASIC target.
  typedef enum logic [2:0] {
    IoBankVcc = 0,
    IoBankAvcc = 1,
    IoBankVioa = 2,
    IoBankViob = 3,
    IoBankCount = 4
  } pwr_dom_e;

  // Enumeration for MIO signals on the top-level.
  typedef enum int unsigned {
    MioInGpioGpio0 = 0,
    MioInGpioGpio1 = 1,
    MioInGpioGpio2 = 2,
    MioInGpioGpio3 = 3,
    MioInGpioGpio4 = 4,
    MioInGpioGpio5 = 5,
    MioInGpioGpio6 = 6,
    MioInGpioGpio7 = 7,
    MioInGpioGpio8 = 8,
    MioInGpioGpio9 = 9,
    MioInGpioGpio10 = 10,
    MioInGpioGpio11 = 11,
    MioInGpioGpio12 = 12,
    MioInGpioGpio13 = 13,
    MioInGpioGpio14 = 14,
    MioInGpioGpio15 = 15,
    MioInGpioGpio16 = 16,
    MioInGpioGpio17 = 17,
    MioInGpioGpio18 = 18,
    MioInGpioGpio19 = 19,
    MioInGpioGpio20 = 20,
    MioInGpioGpio21 = 21,
    MioInGpioGpio22 = 22,
    MioInGpioGpio23 = 23,
    MioInGpioGpio24 = 24,
    MioInGpioGpio25 = 25,
    MioInGpioGpio26 = 26,
    MioInGpioGpio27 = 27,
    MioInGpioGpio28 = 28,
    MioInGpioGpio29 = 29,
    MioInGpioGpio30 = 30,
    MioInGpioGpio31 = 31,
    MioInI2c0Sda = 32,
    MioInI2c0Scl = 33,
    MioInSpiHost1Sd0 = 34,
    MioInSpiHost1Sd1 = 35,
    MioInSpiHost1Sd2 = 36,
    MioInSpiHost1Sd3 = 37,
    MioInUart0Rx = 38,
    MioInUart1Rx = 39,
    MioInUsbdevSense = 40,
    MioInCount = 41
  } mio_in_e;

  typedef enum {
    MioOutGpioGpio0 = 0,
    MioOutGpioGpio1 = 1,
    MioOutGpioGpio2 = 2,
    MioOutGpioGpio3 = 3,
    MioOutGpioGpio4 = 4,
    MioOutGpioGpio5 = 5,
    MioOutGpioGpio6 = 6,
    MioOutGpioGpio7 = 7,
    MioOutGpioGpio8 = 8,
    MioOutGpioGpio9 = 9,
    MioOutGpioGpio10 = 10,
    MioOutGpioGpio11 = 11,
    MioOutGpioGpio12 = 12,
    MioOutGpioGpio13 = 13,
    MioOutGpioGpio14 = 14,
    MioOutGpioGpio15 = 15,
    MioOutGpioGpio16 = 16,
    MioOutGpioGpio17 = 17,
    MioOutGpioGpio18 = 18,
    MioOutGpioGpio19 = 19,
    MioOutGpioGpio20 = 20,
    MioOutGpioGpio21 = 21,
    MioOutGpioGpio22 = 22,
    MioOutGpioGpio23 = 23,
    MioOutGpioGpio24 = 24,
    MioOutGpioGpio25 = 25,
    MioOutGpioGpio26 = 26,
    MioOutGpioGpio27 = 27,
    MioOutGpioGpio28 = 28,
    MioOutGpioGpio29 = 29,
    MioOutGpioGpio30 = 30,
    MioOutGpioGpio31 = 31,
    MioOutI2c0Sda = 32,
    MioOutI2c0Scl = 33,
    MioOutSpiHost1Sd0 = 34,
    MioOutSpiHost1Sd1 = 35,
    MioOutSpiHost1Sd2 = 36,
    MioOutSpiHost1Sd3 = 37,
    MioOutUart0Tx = 38,
    MioOutUart1Tx = 39,
    MioOutSpiHost1Sck = 40,
    MioOutSpiHost1Csb = 41,
    MioOutPwmAonPwm0 = 42,
    MioOutPwmAonPwm1 = 43,
    MioOutPwmAonPwm2 = 44,
    MioOutPwmAonPwm3 = 45,
    MioOutPwmAonPwm4 = 46,
    MioOutPwmAonPwm5 = 47,
    MioOutCount = 48
  } mio_out_e;

  // Enumeration for DIO signals, used on both the top and chip-levels.
  typedef enum int unsigned {
    DioSpiHost0Sd0 = 0,
    DioSpiHost0Sd1 = 1,
    DioSpiHost0Sd2 = 2,
    DioSpiHost0Sd3 = 3,
    DioSpiDeviceSd0 = 4,
    DioSpiDeviceSd1 = 5,
    DioSpiDeviceSd2 = 6,
    DioSpiDeviceSd3 = 7,
    DioUsbdevUsbDp = 8,
    DioUsbdevUsbDn = 9,
    DioSpiDeviceSck = 10,
    DioSpiDeviceCsb = 11,
    DioSpiHost0Sck = 12,
    DioSpiHost0Csb = 13,
    DioCount = 14
  } dio_e;

  // Enumeration for the types of pads.
  typedef enum {
    MioPad,
    DioPad
  } pad_type_e;

  // Raw MIO/DIO input array indices on chip-level.
  // TODO: Does not account for target specific stubbed/added pads.
  // Need to make a target-specific package for those.
  typedef enum int unsigned {
    MioPadIoa0 = 0,
    MioPadIoa1 = 1,
    MioPadIoa2 = 2,
    MioPadIoa3 = 3,
    MioPadIoa4 = 4,
    MioPadIoa5 = 5,
    MioPadIoa6 = 6,
    MioPadIoa7 = 7,
    MioPadIoa8 = 8,
    MioPadIob0 = 9,
    MioPadIob1 = 10,
    MioPadIob2 = 11,
    MioPadIob3 = 12,
    MioPadIob4 = 13,
    MioPadIob5 = 14,
    MioPadIob6 = 15,
    MioPadIob7 = 16,
    MioPadIob8 = 17,
    MioPadIob9 = 18,
    MioPadIob10 = 19,
    MioPadIob11 = 20,
    MioPadIob12 = 21,
    MioPadIoc0 = 22,
    MioPadIoc1 = 23,
    MioPadIoc2 = 24,
    MioPadIoc3 = 25,
    MioPadIoc4 = 26,
    MioPadIoc5 = 27,
    MioPadIoc6 = 28,
    MioPadIoc7 = 29,
    MioPadIoc8 = 30,
    MioPadIoc9 = 31,
    MioPadIoc10 = 32,
    MioPadIoc11 = 33,
    MioPadIoc12 = 34,
    MioPadIor0 = 35,
    MioPadIor1 = 36,
    MioPadIor2 = 37,
    MioPadIor3 = 38,
    MioPadIor4 = 39,
    MioPadIor5 = 40,
    MioPadIor6 = 41,
    MioPadIor7 = 42,
    MioPadIor10 = 43,
    MioPadIor11 = 44,
    MioPadIor12 = 45,
    MioPadIor13 = 46,
    MioPadCount
  } mio_pad_e;

  typedef enum int unsigned {
    DioPadPorN = 0,
    DioPadUsbP = 1,
    DioPadUsbN = 2,
    DioPadCc1 = 3,
    DioPadCc2 = 4,
    DioPadFlashTestVolt = 5,
    DioPadFlashTestMode0 = 6,
    DioPadFlashTestMode1 = 7,
    DioPadSpiHostD0 = 8,
    DioPadSpiHostD1 = 9,
    DioPadSpiHostD2 = 10,
    DioPadSpiHostD3 = 11,
    DioPadSpiHostClk = 12,
    DioPadSpiHostCsL = 13,
    DioPadSpiDevD0 = 14,
    DioPadSpiDevD1 = 15,
    DioPadSpiDevD2 = 16,
    DioPadSpiDevD3 = 17,
    DioPadSpiDevClk = 18,
    DioPadSpiDevCsL = 19,
    DioPadCount
  } dio_pad_e;

  // List of peripheral instantiated in this chip.
  typedef enum {
    PeripheralAonTimerAon,
    PeripheralAst,
    PeripheralClkmgrAon,
    PeripheralFlashCtrl,
    PeripheralFlashMacroWrapper,
    PeripheralGpio,
    PeripheralI2c0,
    PeripheralPinmuxAon,
    PeripheralPwmAon,
    PeripheralPwrmgrAon,
    PeripheralRomCtrl,
    PeripheralRstmgrAon,
    PeripheralRvCoreIbex,
    PeripheralRvDm,
    PeripheralRvPlic,
    PeripheralRvTimer,
    PeripheralSpiDevice,
    PeripheralSpiHost0,
    PeripheralSpiHost1,
    PeripheralSramCtrlMain,
    PeripheralUart0,
    PeripheralUart1,
    PeripheralUsbdev,
    PeripheralCount
  } peripheral_e;

  // MMIO Region
  //
  parameter int unsigned TOP_WREN_MMIO_BASE_ADDR = 32'h40000000;
  parameter int unsigned TOP_WREN_MMIO_SIZE_BYTES = 32'h10000000;

  // TODO: Enumeration for PLIC Interrupt source peripheral.

// MACROs for AST analog simulation support
`ifdef ANALOGSIM
  `define INOUT_AI input ast_pkg::awire_t
  `define INOUT_AO output ast_pkg::awire_t
`else
  `define INOUT_AI inout
  `define INOUT_AO inout
`endif

endpackage
