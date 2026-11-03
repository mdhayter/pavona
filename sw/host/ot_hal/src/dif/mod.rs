// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// The register definitions of the `FIXED_IPS` in BUILD, which regtool generates from their hw/ip
// descriptions. These blocks are not generated per top, so the definitions are the same on
// every top.
#[rustfmt::skip]
pub mod acc_regs;
#[rustfmt::skip]
pub mod adc_ctrl_regs;
#[rustfmt::skip]
pub mod aes_regs;
#[rustfmt::skip]
pub mod aon_timer_regs;
#[rustfmt::skip]
pub mod csrng_regs;
#[rustfmt::skip]
pub mod edn_regs;
#[rustfmt::skip]
pub mod entropy_src_regs;
#[rustfmt::skip]
pub mod hmac_regs;
#[rustfmt::skip]
pub mod i2c_regs;
#[rustfmt::skip]
pub mod keymgr_regs;
#[rustfmt::skip]
pub mod keymgr_dpe_regs;
#[rustfmt::skip]
pub mod kmac_regs;
#[rustfmt::skip]
pub mod pattgen_regs;
#[rustfmt::skip]
pub mod rv_timer_regs;
#[rustfmt::skip]
pub mod spi_device_regs;
#[rustfmt::skip]
pub mod spi_host_regs;
#[rustfmt::skip]
pub mod sysrst_ctrl_regs;
#[rustfmt::skip]
pub mod uart_regs;
#[rustfmt::skip]
pub mod usbdev_regs;

pub mod aon_timer;
// The "Scafi_Deprecated" variant of the chip doesn't have the same
// set of CSRs as the "egret" chip.
#[cfg(not(feature = "scafi_deprecated"))]
pub mod clkmgr;
pub mod lc_ctrl;
pub mod otp_ctrl;
pub mod rstmgr;
pub mod uart;

#[cfg(not(feature = "scafi_deprecated"))]
pub mod pinmux;
