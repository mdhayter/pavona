// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! Configures a Xilinx 7-series FPGA through an FTDI channel wired as a JTAG
//! cable (TCK, TDI, TDO and TMS on xDBUS0-3), following the single-device JTAG
//! configuration flow of UG470. The FPGA must be the only device on the chain.

use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail, ensure};
use ftdi_mpsse::{
    ClockBits, ClockBitsOut, ClockData, ClockDataOut, ClockTMS, ClockTMSOut, MpsseCmdBuilder,
    MpsseCmdExecutor, MpsseSettings,
};

use opentitanlib::transport::ProgressIndicator;

/// An FTDI channel used as a JTAG cable, and the state of its other pins.
pub struct JtagCable {
    pub interface: ftdi::Interface,
    /// Value and direction (1 = output) of xDBUS0-7. TCK, TDI and TMS must be
    /// outputs, TMS high.
    pub low_value: u8,
    pub low_direction: u8,
    /// Value and direction of xCBUS0-7.
    pub high_value: u8,
    pub high_direction: u8,
    /// TCK frequency in Hz.
    pub frequency: u32,
}

// 7-series JTAG instructions (UG470, table 6-3).
const IR_LEN: u8 = 6;
const CFG_IN: u8 = 0x05;
const JPROGRAM: u8 = 0x0b;
const JSTART: u8 = 0x0c;
const BYPASS: u8 = 0x3f;

// Bits of the value captured in the instruction register.
const IR_CAPTURE_INIT_COMPLETE: u8 = 1 << 4;
const IR_CAPTURE_DONE: u8 = 1 << 5;

// The manufacturer field and fixed bit 0 of a Xilinx IDCODE.
const IDCODE_XILINX: u32 = 0x093;

// Longest data run of one MPSSE command.
const MPSSE_MAX_CHUNK: usize = 65536;

// TCK cycles in Run-Test/Idle after JSTART for the startup sequence.
const STARTUP_CLOCKS: usize = 2048;

// How long to wait for the configuration memory to clear after JPROGRAM.
const CLEAR_TIMEOUT: Duration = Duration::from_secs(1);

pub struct Jtag {
    device: ftdi::Device,
}

impl Jtag {
    /// Opens the cable's FTDI channel and leaves the TAP in Run-Test/Idle.
    pub fn open(vendor_id: u16, product_id: u16, cable: &JtagCable) -> Result<Self> {
        let mut device = ftdi::find_by_vid_pid(vendor_id, product_id)
            .interface(cable.interface)
            .open()
            .context("cannot open the FPGA's JTAG channel")?;
        device.init(&MpsseSettings {
            mask: cable.low_direction,
            clock_frequency: Some(cable.frequency),
            ..Default::default()
        })?;
        let cmd = MpsseCmdBuilder::new()
            .disable_loopback()
            .disable_3phase_data_clocking()
            .disable_adaptive_data_clocking()
            .set_gpio_lower(cable.low_value, cable.low_direction)
            .set_gpio_upper(cable.high_value, cable.high_direction);
        device.send(cmd.as_slice())?;
        let mut jtag = Jtag { device };
        jtag.reset_to_idle()?;
        Ok(jtag)
    }

    /// Test-Logic-Reset, then Run-Test/Idle.
    fn reset_to_idle(&mut self) -> Result<()> {
        let cmd = MpsseCmdBuilder::new().clock_tms_out(ClockTMSOut::NegEdge, 0b01_1111, false, 6);
        self.device.send(cmd.as_slice())?;
        Ok(())
    }

    /// Shifts `ir` into the instruction register, from and back to
    /// Run-Test/Idle. Returns the value captured in the register.
    fn shift_ir(&mut self, ir: u8) -> Result<u8> {
        let last = IR_LEN - 1;
        let cmd = MpsseCmdBuilder::new()
            // Select-DR, Select-IR, Capture-IR, Shift-IR.
            .clock_tms_out(ClockTMSOut::NegEdge, 0b0011, false, 4)
            .clock_bits(ClockBits::LsbPosIn, ir, last)
            // The last bit on the way to Exit1-IR, then Update-IR, Run-Test/Idle.
            .clock_tms(ClockTMS::NegTMSPosTDO, 0b011, ir & (1 << last) != 0, 3)
            .send_immediate();
        let mut captured = [0u8; 2];
        self.device.xfer(cmd.as_slice(), &mut captured)?;
        // Bits read least significant first arrive at the top of each byte.
        Ok((captured[0] >> (8 - last)) | (((captured[1] >> 5) & 1) << last))
    }

    /// Reads the IDCODE, which Test-Logic-Reset selects as the data register.
    fn read_idcode(&mut self) -> Result<u32> {
        self.reset_to_idle()?;
        let cmd = MpsseCmdBuilder::new()
            // Select-DR, Capture-DR, Shift-DR.
            .clock_tms_out(ClockTMSOut::NegEdge, 0b001, false, 3)
            .clock_data(ClockData::LsbPosIn, &[0; 4])
            // Exit1-DR, Update-DR, Run-Test/Idle.
            .clock_tms_out(ClockTMSOut::NegEdge, 0b011, false, 3)
            .send_immediate();
        let mut idcode = [0u8; 4];
        self.device.xfer(cmd.as_slice(), &mut idcode)?;
        Ok(u32::from_le_bytes(idcode))
    }

    /// Shifts `data` into the selected data register, most significant bit
    /// of each byte first, from and back to Run-Test/Idle.
    fn shift_dr_msb_first(&mut self, data: &[u8], progress: &dyn ProgressIndicator) -> Result<()> {
        let (last, data) = data.split_last().context("no data to shift")?;
        let cmd = MpsseCmdBuilder::new().clock_tms_out(ClockTMSOut::NegEdge, 0b001, false, 3);
        self.device.send(cmd.as_slice())?;
        let mut sent = 0;
        for chunk in data.chunks(MPSSE_MAX_CHUNK) {
            let cmd = MpsseCmdBuilder::new().clock_data_out(ClockDataOut::MsbNeg, chunk);
            self.device.send(cmd.as_slice())?;
            sent += chunk.len();
            progress.progress(sent);
        }
        let cmd = MpsseCmdBuilder::new()
            .clock_bits_out(ClockBitsOut::MsbNeg, *last, 7)
            .clock_tms_out(ClockTMSOut::NegEdge, 0b011, last & 1 != 0, 3);
        self.device.send(cmd.as_slice())?;
        progress.progress(sent + 1);
        Ok(())
    }

    /// Clocks TCK `count` times (rounded up to a multiple of 8) in
    /// Run-Test/Idle.
    fn idle_clocks(&mut self, count: usize) -> Result<()> {
        let zeros = vec![0u8; count.div_ceil(8)];
        for chunk in zeros.chunks(MPSSE_MAX_CHUNK) {
            let cmd = MpsseCmdBuilder::new().clock_data_out(ClockDataOut::MsbNeg, chunk);
            self.device.send(cmd.as_slice())?;
        }
        Ok(())
    }

    /// Checks that the chain holds a Xilinx device.
    fn check_idcode(&mut self) -> Result<()> {
        let idcode = self.read_idcode()?;
        ensure!(
            idcode & 0xfff == IDCODE_XILINX,
            "no Xilinx FPGA on the JTAG chain (IDCODE {idcode:#010x})"
        );
        log::info!("FPGA IDCODE {idcode:#010x}");
        Ok(())
    }

    /// Clears the configuration memory (JPROGRAM) and waits until the FPGA
    /// is ready for new configuration data.
    fn clear(&mut self) -> Result<()> {
        self.shift_ir(JPROGRAM)?;
        let start = Instant::now();
        loop {
            let status = self.shift_ir(BYPASS)?;
            if status & IR_CAPTURE_INIT_COMPLETE != 0 && status & IR_CAPTURE_DONE == 0 {
                return Ok(());
            }
            if start.elapsed() > CLEAR_TIMEOUT {
                bail!("FPGA did not clear its configuration (IR capture {status:#04x})");
            }
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    /// Loads a bitstream (a `.bit` file or raw configuration data).
    pub fn program(&mut self, bitstream: &[u8], progress: &dyn ProgressIndicator) -> Result<()> {
        let data = bit_file_data(bitstream)?;
        self.check_idcode()?;
        self.clear()?;
        self.shift_ir(CFG_IN)?;
        progress.new_stage("", data.len());
        self.shift_dr_msb_first(data, progress)?;
        self.shift_ir(JSTART)?;
        self.idle_clocks(STARTUP_CLOCKS)?;
        self.reset_to_idle()?;
        let status = self.shift_ir(BYPASS)?;
        ensure!(
            status & IR_CAPTURE_DONE != 0,
            "FPGA not configured: DONE is low (IR capture {status:#04x})"
        );
        Ok(())
    }

    /// Clears the FPGA's configuration.
    pub fn clear_bitstream(&mut self) -> Result<()> {
        self.check_idcode()?;
        self.clear()
    }
}

/// Returns the configuration data of a Xilinx `.bit` file: the contents of
/// its `e` field. Data without the `.bit` header is returned unchanged.
pub fn bit_file_data(bit: &[u8]) -> Result<&[u8]> {
    // A 9-byte first field, then the 1-byte field length before the `a` key.
    const MAGIC: [u8; 13] = [
        0x00, 0x09, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x01,
    ];
    if !bit.starts_with(&MAGIC) {
        return Ok(bit);
    }
    let mut pos = MAGIC.len();
    loop {
        let key = *bit.get(pos).context("truncated .bit header")?;
        pos += 1;
        if key == b'e' {
            let len = bit.get(pos..pos + 4).context("truncated .bit header")?;
            let len = u32::from_be_bytes(len.try_into()?) as usize;
            pos += 4;
            return bit.get(pos..pos + len).context("truncated .bit data");
        }
        ensure!(
            (b'a'..=b'd').contains(&key),
            "unexpected .bit header field {key:#04x}"
        );
        let len = bit.get(pos..pos + 2).context("truncated .bit header")?;
        pos += 2 + u16::from_be_bytes(len.try_into()?) as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field(key: u8, value: &[u8]) -> Vec<u8> {
        let mut f = vec![key];
        f.extend_from_slice(&(value.len() as u16).to_be_bytes());
        f.extend_from_slice(value);
        f
    }

    #[test]
    fn bit_file_header_is_stripped() {
        let data = [0xff, 0xff, 0xff, 0xff, 0xaa, 0x99, 0x55, 0x66];
        let mut bit = vec![
            0x00, 0x09, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x01,
        ];
        bit.extend(field(b'a', b"design\0"));
        bit.extend(field(b'b', b"7k325tffg900\0"));
        bit.extend(field(b'c', b"2026/11/13\0"));
        bit.extend(field(b'd', b"12:00:00\0"));
        bit.push(b'e');
        bit.extend_from_slice(&(data.len() as u32).to_be_bytes());
        bit.extend_from_slice(&data);
        assert_eq!(bit_file_data(&bit).unwrap(), &data);
    }

    #[test]
    fn raw_data_is_unchanged() {
        let data = [0xff, 0xff, 0xaa, 0x99, 0x55, 0x66];
        assert_eq!(bit_file_data(&data).unwrap(), &data);
    }

    #[test]
    fn truncated_bit_file_is_an_error() {
        let bit = [
            0x00, 0x09, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x01, b'e', 0,
            0, 0, 8, 0xff,
        ];
        assert!(bit_file_data(&bit).is_err());
    }
}
