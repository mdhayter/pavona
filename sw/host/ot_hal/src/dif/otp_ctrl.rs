// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! The granularity of Direct Access Interface accesses to OTP partitions.

use crate::top::desc::OtpPartition;

/// Granularities of memory accesses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Granularity {
    /// 32-bit.
    B32,
    /// 64-bit.
    B64,
}

impl OtpPartition {
    /// Returns the granularity of DAI accesses to this partition: 64-bit for secret
    /// partitions, 32-bit otherwise. Digests are always accessed with 64-bit granularity.
    pub fn access_granule(&self) -> Granularity {
        // TODO: (carried over) Take granularities from the `.hjson` instead of hardcoding.
        if self.secret {
            Granularity::B64
        } else {
            Granularity::B32
        }
    }
}
