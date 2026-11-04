// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// The generated pinmux enums, only for the cross-check tests in `desc` (`ot_hal_<top>_test`,
// whose crate feature names the top).
#[cfg(any(feature = "egret", feature = "dragonfly"))]
mod autogen;
pub mod desc;

pub use desc::TopDescription;
