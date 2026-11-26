// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! GPIO bit-banging with an FTDI channel's synchronous bit-bang mode.
//!
//! In synchronous bit-bang mode the chip applies each byte it is sent to the
//! channel's low GPIO byte (`xdbus0-7`), at the rate of its baud rate
//! generator, and sends back the levels of those pins sampled just before
//! each byte is applied: the same convention as `BitbangEntry::Both`. The
//! direction of each pin is fixed for a whole waveform.
//!
//! The high GPIO byte (`xcbus0-7`) does not keep its MPSSE levels while the
//! mode is on. On the FT2232H, ACBUS1 and ACBUS2 become the bit-bang write
//! and read strobes (WRSTB#, RDSTB#), outputs that pulse with each byte;
//! ACBUS4 becomes the SIWU# input; the other ACBUS pins are not driven and
//! float high on the chip's internal pull-ups (about 75 kOhm).
//!
//! The channel leaves MPSSE mode while the bit-bang sampling is active and
//! is restored to it afterwards, with the GPIO levels and directions it had
//! before.

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::rc::Rc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, anyhow, bail, ensure};
use ftdi_mpsse::{MpsseCmdBuilder, MpsseCmdExecutor, MpsseSettings};

use opentitanlib::io::gpio::{
    BitbangEntry, DacBangEntry, GpioBitbangOperation, GpioBitbanging, GpioDacBangOperation,
    GpioError, GpioPin, PinMode,
};
use opentitanlib::transport::TransportError;

use crate::gpio::Pin;
use crate::{Channels, mpsse_settings};

/// The transport's GPIO pins by name, for finding the channel, bit and mode
/// of the pins given to `start()`.
pub(crate) type Pins = Rc<RefCell<HashMap<String, Rc<Pin>>>>;

/// The most bytes in flight: sent to the chip but not yet read back. The
/// FT2232H holds 4 KiB in each direction; sync bit-bang stops clocking when
/// its transmit buffer is full, which would stretch the waveform, so the
/// samples waiting there must never fill it.
const MAX_IN_FLIGHT: usize = 3072;

/// Samples a second for each unit of the baud rate given to libftdi in
/// synchronous bit-bang mode, measured on a Genesys 2's FT2232H (libftdi
/// itself multiplies the baud rate by 4 in bit-bang modes).
const SAMPLES_PER_BAUD: f64 = 20.0;

/// An empirical correction, in seconds, taken off the sample period before
/// the baud rate is set. FTDI does not document it: on a Genesys 2's FT2232H,
/// sample periods above about 20 kHz came out about 50 ns longer than the
/// baud rate gives (100 kHz 0.5% slow, 200 kHz 1%; 10 and 20 kHz exact).
/// With it, 4 kHz to 1 MHz are within 0.1%.
const SAMPLE_OVERHEAD: f64 = 50e-9;

/// The lowest rate in samples a second: below it the chip runs faster than
/// `SAMPLES_PER_BAUD` gives (1 kHz and 2 kHz came out four times too fast).
const MIN_RATE: f64 = 4000.0;

/// How long to wait for samples beyond the waveform's own length.
const READ_SLACK: Duration = Duration::from_secs(1);

pub struct Bitbanging {
    channels: Channels,
    pins: Pins,
}

impl Bitbanging {
    pub(crate) fn new(channels: &Channels, pins: &Pins) -> Self {
        Self {
            channels: channels.clone(),
            pins: pins.clone(),
        }
    }
}

/// One of the pins given to `start()`.
struct BitbangPin {
    bit: u8,
    output: bool,
}

/// The waveform flattened into one byte per clock tick, in the bit order of
/// the pins given to `start()`, with where each `Both` entry's samples go.
struct Flat {
    bytes: Vec<u8>,
    // (entry index, offset in `bytes`, length) of each `Both`/`BothOwned`.
    sampled: Vec<(usize, usize, usize)>,
}

fn flatten(waveform: &[BitbangEntry]) -> Result<Flat> {
    let mut flat = Flat {
        bytes: Vec::new(),
        sampled: Vec::new(),
    };
    for (index, entry) in waveform.iter().enumerate() {
        match entry {
            BitbangEntry::Write(wbuf) | BitbangEntry::Both(wbuf, _) => {
                if let BitbangEntry::Both(_, rbuf) = entry {
                    ensure!(
                        rbuf.len() == wbuf.len(),
                        GpioError::MismatchedDataLength(wbuf.len(), rbuf.len())
                    );
                    flat.sampled.push((index, flat.bytes.len(), wbuf.len()));
                }
                flat.bytes.extend_from_slice(wbuf);
            }
            BitbangEntry::WriteOwned(wbuf) => flat.bytes.extend_from_slice(wbuf),
            BitbangEntry::BothOwned(buf) => {
                flat.sampled.push((index, flat.bytes.len(), buf.len()));
                flat.bytes.extend_from_slice(buf);
            }
            BitbangEntry::Delay(0) => bail!(GpioError::InvalidBitbangDelay),
            BitbangEntry::Delay(ticks) => {
                let last = *flat.bytes.last().ok_or(GpioError::InvalidBitbangDelay)?;
                flat.bytes
                    .extend(std::iter::repeat_n(last, *ticks as usize - 1));
            }
            BitbangEntry::Await { .. } => bail!(TransportError::UnsupportedOperation),
        }
    }
    Ok(flat)
}

impl GpioBitbanging for Bitbanging {
    fn start<'a>(
        &self,
        pins: &[&dyn GpioPin],
        clock_tick: Duration,
        mut waveform: Box<[BitbangEntry<'a, 'a>]>,
    ) -> Result<Box<dyn GpioBitbangOperation<'a, 'a> + 'a>> {
        ensure!(
            !pins.is_empty() && pins.len() <= 8,
            TransportError::UnsupportedOperation
        );
        let flat = flatten(&waveform)?;

        // Find each pin's bit, all on the low byte of one channel, and its
        // direction for the waveform: an open-drain pin is driven if the
        // waveform pulls it low, and sampled otherwise.
        let mut interface = None;
        let mut bb_pins = Vec::new();
        for (index, pin) in pins.iter().enumerate() {
            let name = pin
                .get_internal_pin_name()
                .ok_or(TransportError::InvalidOperation)?;
            let pin = self
                .pins
                .borrow()
                .get(name)
                .cloned()
                .ok_or_else(|| GpioError::InvalidPinName(name.to_owned()))?;
            let loc = pin.location()?;
            ensure!(
                !loc.upper && interface.is_none_or(|i| i == loc.interface),
                GpioError::Generic(format!(
                    "{name}: bit-banged pins must all be in one channel's low byte (xdbus0-7)"
                ))
            );
            ensure!(
                bb_pins.iter().all(|p: &BitbangPin| p.bit != loc.bit),
                GpioError::Generic(format!("{name}: given twice"))
            );
            interface = Some(loc.interface);
            let output = match pin.mode() {
                Some(PinMode::PushPull) => true,
                Some(PinMode::OpenDrain) => flat.bytes.iter().any(|b| b & (1 << index) == 0),
                _ => false,
            };
            bb_pins.push(BitbangPin {
                bit: loc.bit,
                output,
            });
        }
        let channel = &self.channels[&interface.unwrap()];

        let rate = 1.0 / clock_tick.as_secs_f64();
        ensure!(
            rate.is_finite() && rate >= MIN_RATE,
            GpioError::Generic(format!(
                "bit-banging below {MIN_RATE} samples a second is not supported"
            ))
        );
        let samples = run(channel, &bb_pins, rate, &flat.bytes)?;

        // Return each `Both` entry's samples in the bit order of `pins`.
        for &(index, offset, len) in &flat.sampled {
            let rbuf: &mut [u8] = match &mut waveform[index] {
                BitbangEntry::Both(_, rbuf) => rbuf,
                BitbangEntry::BothOwned(buf) => buf,
                _ => unreachable!(),
            };
            for (r, s) in rbuf.iter_mut().zip(&samples[offset..offset + len]) {
                *r = bb_pins
                    .iter()
                    .enumerate()
                    .map(|(i, p)| ((s >> p.bit) & 1) << i)
                    .fold(0, |a, b| a | b);
            }
        }
        Ok(Box::new(Done { waveform }))
    }

    fn dac_start(
        &self,
        _pins: &[&dyn GpioPin],
        _clock_tick: Duration,
        _waveform: Box<[DacBangEntry]>,
    ) -> Result<Box<dyn GpioDacBangOperation>> {
        bail!(TransportError::UnsupportedOperation)
    }
}

/// Runs `bytes` on the channel's low GPIO byte at `rate` bytes a second and
/// returns the raw samples, one per byte.
fn run(channel: &crate::Channel, pins: &[BitbangPin], rate: f64, bytes: &[u8]) -> Result<Vec<u8>> {
    let [lower_dir, upper_dir] = channel.saved_direction.get();
    let given = pins.iter().fold(0u8, |m, p| m | 1 << p.bit);
    let driven = pins
        .iter()
        .filter(|p| p.output)
        .fold(0u8, |m, p| m | 1 << p.bit);
    // Pins not given keep their direction; given pins get theirs from above.
    let mask = (lower_dir & !given) | driven;

    channel.hal.borrow_mut().with_device(|dev| {
        // The current levels: kept on the outputs not given, and restored
        // afterwards. The HAL's cached values for its outputs are these levels.
        let mut levels = [0u8; 2];
        dev.xfer(
            MpsseCmdBuilder::new()
                .gpio_lower()
                .gpio_upper()
                .send_immediate()
                .as_slice(),
            &mut levels,
        )?;
        let keep = levels[0] & !given;
        let out: Vec<u8> = bytes
            .iter()
            .map(|w| {
                pins.iter()
                    .enumerate()
                    .filter(|(i, p)| p.output && w & (1 << i) != 0)
                    .fold(keep, |b, (_, p)| b | 1 << p.bit)
            })
            .collect();

        let result = stream(dev, mask, rate, levels[0], &out);

        // Back to MPSSE, as opened, without the USB reset, then the GPIO
        // levels and directions the HAL expects.
        let restore = dev
            .init(&MpsseSettings {
                reset: false,
                ..mpsse_settings()
            })
            .and_then(|_| {
                dev.send(
                    MpsseCmdBuilder::new()
                        .set_gpio_lower(levels[0], lower_dir)
                        .set_gpio_upper(levels[1], upper_dir)
                        .send_immediate()
                        .as_slice(),
                )
            })
            .context("restoring MPSSE mode after bit-banging");
        let samples = result?;
        restore?;
        Ok(samples)
    })
}

/// Streams `out` through synchronous bit-bang mode and reads the samples.
fn stream(dev: &mut ftdi::Device, mask: u8, rate: f64, levels: u8, out: &[u8]) -> Result<Vec<u8>> {
    // The pins drive the chip's bit-bang latch as soon as the mode is on,
    // and the latch keeps the last byte of the previous waveform (even one
    // run by another program). Load it with the current levels first, with
    // every pin an input, so that the outputs do not glitch.
    dev.set_bitmode(0, ftdi::BitMode::SyncBB)?;
    let baud = (1.0 / (1.0 / rate - SAMPLE_OVERHEAD) / SAMPLES_PER_BAUD).round();
    ensure!(
        baud >= 1.0 && baud <= i32::MAX as f64,
        TransportError::UnsupportedOperation
    );
    dev.set_baud_rate(baud as u32)?;
    dev.usb_purge_buffers()?;
    dev.write_all(&[levels])?;
    let deadline = Instant::now() + READ_SLACK;
    while dev.read(&mut [0u8])? == 0 {
        ensure!(Instant::now() < deadline, "bit-banging: no sample back");
    }
    dev.set_bitmode(mask, ftdi::BitMode::SyncBB)?;
    dev.usb_purge_buffers()?;

    let mut samples = vec![0u8; out.len()];
    let (mut written, mut read) = (0, 0);
    let start = Instant::now();
    let expected = Duration::from_secs_f64(out.len() as f64 / rate);
    let deadline = start + expected + READ_SLACK;
    while read < out.len() {
        let room = MAX_IN_FLIGHT - (written - read);
        if written < out.len() && room > 0 {
            let n = room.min(out.len() - written);
            dev.write_all(&out[written..written + n])?;
            written += n;
        }
        read += dev.read(&mut samples[read..])?;
        if Instant::now() > deadline {
            return Err(anyhow!(
                "bit-banging: {read} of {} samples before the deadline",
                out.len()
            ));
        }
    }
    let elapsed = start.elapsed();
    log::debug!(
        "bit-banged {} samples in {:?}: {:.0} samples/s for {rate:.0} asked",
        out.len(),
        elapsed,
        out.len() as f64 / elapsed.as_secs_f64()
    );
    // The chip stops its clock while it waits for data or for room to send
    // samples back, so a host that falls behind stretches the waveform.
    if elapsed > expected.mul_f64(1.005) + Duration::from_millis(50) {
        log::warn!(
            "bit-banging took {elapsed:?} for a {expected:?} waveform: the host fell behind and \
             the waveform is stretched"
        );
    }
    Ok(samples)
}

/// A waveform already run: `start()` runs it to the end.
struct Done<'a> {
    waveform: Box<[BitbangEntry<'a, 'a>]>,
}

impl<'a> GpioBitbangOperation<'a, 'a> for Done<'a> {
    fn query(&mut self) -> Result<bool> {
        Ok(true)
    }

    fn get_result(self: Box<Self>) -> Result<Box<[BitbangEntry<'a, 'a>]>> {
        Ok(self.waveform)
    }
}
