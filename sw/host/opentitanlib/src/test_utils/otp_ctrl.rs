// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! This module contains code for programming and reading OTP parameters
//! through the Direct Access Interface.
//!
//! Beware that OTP parameters can only be written once and can be corrupted if
//! written again.
//!
//! Parameters and partitions are named as in the top's OTP memory map (`DEVICE_ID`,
//! `HW_CFG0`), which comes from the top description.

use std::mem;
use std::time::Duration;

use anyhow::{Context, bail};
use thiserror::Error;

use ot_hal::dif::otp_ctrl::{DirectAccessCmd, Granularity, OtpCtrlStatus};
use ot_hal::top::TopDescription;

use crate::io::jtag::Jtag;
use crate::test_utils::poll;

/// Controls for reading and writing OTP parameters.
pub struct OtpParam;
impl OtpParam {
    /// Read parameter `param` (`DEVICE_ID`) from OTP into an output buffer.
    pub fn read_param(
        jtag: &mut dyn Jtag,
        top: &TopDescription,
        param: &str,
        out_buf: &mut [u32],
    ) -> OtpDaiResult<()> {
        let dai = OtpDai::new(top)?;
        let (byte_addr, size, access_granule) = Self::lookup(top, param)?;

        if mem::size_of_val(out_buf) < size as usize {
            let err = OtpDaiError::BufSize {
                buf_size: mem::size_of_val(out_buf),
                param_size: size,
            };
            return Err(err);
        }

        match access_granule {
            Granularity::B32 => {
                let out_iter = out_buf.iter_mut().take(size as usize);
                for (idx, out_word) in out_iter.enumerate() {
                    let addr = byte_addr + (idx * mem::size_of::<u32>()) as u32;
                    let [lower, _] = dai.read(jtag, addr, access_granule)?;
                    *out_word = lower;
                }
            }
            Granularity::B64 => {
                let out_iter = out_buf.chunks_mut(2).take(size as usize / 2);
                for (idx, out_words) in out_iter.enumerate() {
                    let addr = byte_addr + (idx * mem::size_of::<u64>()) as u32;
                    let otp_words = dai.read(jtag, addr, access_granule)?;
                    out_words[0] = otp_words[0];
                    out_words[1] = otp_words[1];
                }
            }
        }

        Ok(())
    }

    /// Write a value from a buffer to OTP parameter `param` (`DEVICE_ID`).
    pub fn write_param(
        jtag: &mut dyn Jtag,
        top: &TopDescription,
        param: &str,
        data: &[u32],
    ) -> OtpDaiResult<()> {
        let dai = OtpDai::new(top)?;
        let (byte_addr, size, access_granule) = Self::lookup(top, param)?;

        // Check if the word has been written already.
        // Note: we can only check non-secret partitions, since secret partitions are scrambled,
        // when they are backdoor loaded with 0s they will be garbage when read out. Only
        // secret partitions are accessed with 64-bit granularity.
        if access_granule == Granularity::B32 {
            let mut curr_value = [0u32].repeat((size / 4).try_into().unwrap());
            Self::read_param(jtag, top, param, curr_value.as_mut_slice())?;
            for word in curr_value.iter() {
                if *word != 0u32 {
                    return Err(OtpDaiError::WriteErrorAlreadyWritten { value: *word });
                }
            }
        }

        if mem::size_of_val(data) > size as usize {
            let err = OtpDaiError::BufSize {
                buf_size: mem::size_of_val(data),
                param_size: size,
            };
            return Err(err);
        }

        match access_granule {
            Granularity::B32 => {
                let data_iter = data.iter().take(size as usize);
                for (idx, data_word) in data_iter.enumerate() {
                    let addr = byte_addr + (idx * mem::size_of::<u32>()) as u32;
                    dai.write(jtag, addr, access_granule, [*data_word, 0x00])?;
                }
            }
            Granularity::B64 => {
                let data_iter = data.chunks(2).take(size as usize / 2);
                for (idx, data_words) in data_iter.enumerate() {
                    let addr = byte_addr + (idx * mem::size_of::<u64>()) as u32;
                    dai.write(jtag, addr, access_granule, [data_words[0], data_words[1]])?;
                }
            }
        }

        Ok(())
    }

    /// Returns the byte address, size and access granularity of parameter `param`.
    fn lookup(top: &TopDescription, param: &str) -> OtpDaiResult<(u32, u32, Granularity)> {
        let lookup = || {
            let item = top.otp_item(param)?;
            let partition = top.otp_partition(&item.partition)?;
            Ok((item.offset, item.size, partition.access_granule()))
        };
        lookup().map_err(|source| OtpDaiError::Top { source })
    }
}

/// Commands for operating on OTP partitions.
pub struct OtpPartition;
impl OtpPartition {
    /// Lock partition `partition` (`HW_CFG0`) by calculating its digest. Only partitions
    /// with a hardware digest can be locked this way.
    pub fn lock(jtag: &mut dyn Jtag, top: &TopDescription, partition: &str) -> OtpDaiResult<()> {
        let partition = top
            .otp_partition(partition)
            .map_err(|source| OtpDaiError::Top { source })?;
        if !partition.hw_digest {
            return Err(OtpDaiError::NotSupported);
        }
        OtpDai::new(top)?.lock(jtag, partition.offset)
    }

    /// Read back this partition's digest from the OTP.
    ///
    /// This goes via the DAI, but partitions are also exposed via CSRs after reset.
    pub fn read_digest(
        jtag: &mut dyn Jtag,
        top: &TopDescription,
        partition: &str,
    ) -> OtpDaiResult<[u32; 2]> {
        let digest = top
            .otp_partition(partition)
            .map_err(|source| OtpDaiError::Top { source })?
            .digest
            .ok_or(OtpDaiError::NotSupported)?;
        assert_eq!(
            digest.size, 8,
            "OTP partition digests should be 2 words in size"
        );

        OtpDai::new(top)?.read(jtag, digest.offset, Granularity::B64)
    }
}

/// Direct Access Interface to the OTP.
struct OtpDai {
    /// Addresses of the OTP controller's registers.
    status: u32,
    cmd: u32,
    address: u32,
    wdata: [u32; 2],
    rdata: [u32; 2],
}
impl OtpDai {
    fn new(top: &TopDescription) -> OtpDaiResult<Self> {
        let addrs = || -> anyhow::Result<Self> {
            let base = top.base_addr("otp_ctrl_core")? as u32;
            let reg =
                |name| -> anyhow::Result<u32> { Ok(base + top.register_offset("otp_ctrl", name)?) };
            Ok(Self {
                status: reg("STATUS")?,
                cmd: reg("DIRECT_ACCESS_CMD")?,
                address: reg("DIRECT_ACCESS_ADDRESS")?,
                wdata: [reg("DIRECT_ACCESS_WDATA_0")?, reg("DIRECT_ACCESS_WDATA_1")?],
                rdata: [reg("DIRECT_ACCESS_RDATA_0")?, reg("DIRECT_ACCESS_RDATA_1")?],
            })
        };
        addrs().map_err(|source| OtpDaiError::Top { source })
    }

    // Polling timeout and delay while waiting on statuses.
    const POLL_TIMEOUT: Duration = Duration::from_millis(500);
    const POLL_DELAY: Duration = Duration::from_millis(5);

    /// Perform a single read over the Direct Access Interface.
    ///
    /// On success, returns an array of words `[lower, upper]` where `upper` is non-zero
    /// for 64-bit granularity reads.
    pub fn read(
        &self,
        jtag: &mut dyn Jtag,
        byte_addr: u32,
        granule: Granularity,
    ) -> OtpDaiResult<[u32; 2]> {
        self.wait_for_idle(jtag)?;

        // Set the DAI address to read from.
        let dai_address_addr = self.address;
        jtag.write_memory32(dai_address_addr, &[byte_addr])
            .map_err(|source| OtpDaiError::Jtag { source })?;

        // Trigger a read command.
        let dai_cmd_addr = self.cmd;
        jtag.write_memory32(dai_cmd_addr, &[DirectAccessCmd::RD.bits()])
            .map_err(|source| OtpDaiError::Jtag { source })?;

        self.wait_for_idle(jtag)?;

        let [dai_rdata0_addr, dai_rdata1_addr] = self.rdata;

        let mut rdata = [0x00; 2];

        jtag.read_memory32(dai_rdata0_addr, &mut rdata[0..=0])
            .map_err(|source| OtpDaiError::Jtag { source })?;

        if granule == Granularity::B64 {
            jtag.read_memory32(dai_rdata1_addr, &mut rdata[1..=1])
                .map_err(|source| OtpDaiError::Jtag { source })?;
        }

        Ok(rdata)
    }

    /// Perform a single write over the Direct Access Interface.
    pub fn write(
        &self,
        jtag: &mut dyn Jtag,
        byte_addr: u32,
        granule: Granularity,
        values: [u32; 2],
    ) -> OtpDaiResult<()> {
        self.wait_for_idle(jtag)?;

        // Set the DAI address to write to.
        let dai_address_addr = self.address;
        jtag.write_memory32(dai_address_addr, &[byte_addr])
            .map_err(|source| OtpDaiError::Jtag { source })?;

        let [dai_wdata0_addr, dai_wdata1_addr] = self.wdata;

        jtag.write_memory32(dai_wdata0_addr, &[values[0]])
            .map_err(|source| OtpDaiError::Jtag { source })?;

        if granule == Granularity::B64 {
            jtag.write_memory32(dai_wdata1_addr, &[values[1]])
                .map_err(|source| OtpDaiError::Jtag { source })?;
        }

        // Trigger a write command.
        let dai_cmd_addr = self.cmd;
        jtag.write_memory32(dai_cmd_addr, &[DirectAccessCmd::WR.bits()])
            .map_err(|source| OtpDaiError::Jtag { source })?;

        self.wait_for_idle(jtag)?;

        Ok(())
    }

    /// Lock the partition starting at offset `byte_addr`.
    pub fn lock(&self, jtag: &mut dyn Jtag, byte_addr: u32) -> OtpDaiResult<()> {
        self.wait_for_idle(jtag)?;

        // Set the DAI address to the start of the partition.
        let dai_address_addr = self.address;
        jtag.write_memory32(dai_address_addr, &[byte_addr])
            .map_err(|source| OtpDaiError::Jtag { source })?;

        // Trigger a digest calculation command.
        let dai_cmd_addr = self.cmd;
        jtag.write_memory32(dai_cmd_addr, &[DirectAccessCmd::DIGEST.bits()])
            .map_err(|source| OtpDaiError::Jtag { source })?;

        self.wait_for_idle(jtag)?;

        Ok(())
    }

    /// Wait for the OTP controller's status to read `DAI_IDLE`, showing it's
    /// ready to accept commands.
    pub fn wait_for_idle(&self, jtag: &mut dyn Jtag) -> Result<(), OtpDaiError> {
        let otp_status_addr = self.status;

        poll::poll_until(Self::POLL_TIMEOUT, Self::POLL_DELAY, || {
            let mut status = [0];
            jtag.read_memory32(otp_status_addr, &mut status)
                .map_err(|source| OtpDaiError::Jtag { source })?;

            let status =
                OtpCtrlStatus::from_bits(status[0]).context("status has invalid bits set")?;

            if status.intersects(OtpCtrlStatus::ERRORS) {
                bail!("status {status:#b} has error bits set");
            }

            Ok(status.contains(OtpCtrlStatus::DAI_IDLE))
        })
        .map_err(|source| OtpDaiError::WaitForIdle { source })
    }
}

pub type OtpDaiResult<T> = Result<T, OtpDaiError>;

/// Failures to write OTP parameters through the Direct Access Interface.
#[derive(Debug, Error)]
pub enum OtpDaiError {
    #[error("provided buffer has invalid size {buf_size} for parameter of size {param_size}")]
    BufSize { buf_size: usize, param_size: u32 },

    #[error("feature not supported for current partition")]
    NotSupported,

    #[error("failed to find otp_ctrl or the OTP parameter in the top description")]
    Top { source: anyhow::Error },

    #[error("failed to communicate over JTAG")]
    Jtag { source: anyhow::Error },

    #[error("failed to wait for otp_ctrl DAI to be idle")]
    WaitForIdle { source: anyhow::Error },

    #[error("writing to otp_ctrl direct access registers is disabled")]
    WriteDisabled,

    #[error(
        "writing to OTP failed since field has already been written (with value: 0x{value:08X})"
    )]
    WriteErrorAlreadyWritten { value: u32 },
}
