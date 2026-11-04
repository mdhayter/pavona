// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use super::uart_regs as regs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum UartReg {
    IntrState = regs::UART_INTR_STATE_REG_OFFSET as u32,
    IntrEnable = regs::UART_INTR_ENABLE_REG_OFFSET as u32,
    IntrTest = regs::UART_INTR_TEST_REG_OFFSET as u32,
    AlertTest = regs::UART_ALERT_TEST_REG_OFFSET as u32,
    Ctrl = regs::UART_CTRL_REG_OFFSET as u32,
    Status = regs::UART_STATUS_REG_OFFSET as u32,
    Rdata = regs::UART_RDATA_REG_OFFSET as u32,
    Wdata = regs::UART_WDATA_REG_OFFSET as u32,
    FifoCtrl = regs::UART_FIFO_CTRL_REG_OFFSET as u32,
    FifoStatus = regs::UART_FIFO_STATUS_REG_OFFSET as u32,
    Ovrd = regs::UART_OVRD_REG_OFFSET as u32,
    Val = regs::UART_VAL_REG_OFFSET as u32,
    TimeoutCtrl = regs::UART_TIMEOUT_CTRL_REG_OFFSET as u32,
}
