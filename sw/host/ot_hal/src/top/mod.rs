// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

mod autogen;
pub mod desc;

pub use desc::TopDescription;

/// The top these host tools were built for, if any. It is the default when no top is selected,
/// until the host tools are no longer built for a single top.
pub const BUILT_FOR: Option<&str> = if cfg!(feature = "egret") {
    Some("egret")
} else if cfg!(feature = "dragonfly") {
    Some("dragonfly")
} else if cfg!(feature = "scafi_deprecated") {
    Some("scafi_deprecated")
} else {
    None
};

#[cfg(feature = "egret")]
pub use autogen::egret::{
    DirectPads, MuxedPads, PinmuxInsel, PinmuxMioOut, PinmuxOutsel, PinmuxPeripheralIn,
};
#[cfg(feature = "egret")]
pub use top_egret::top_egret::*;

#[cfg(feature = "dragonfly")]
pub use autogen::dragonfly::{
    DirectPads, MuxedPads, PinmuxInsel, PinmuxMioOut, PinmuxOutsel, PinmuxPeripheralIn,
};
#[cfg(feature = "dragonfly")]
pub use top_dragonfly::top_dragonfly::*;
