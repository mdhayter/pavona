// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use std::cmp;
use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Context, Result};

use ot_hal::dif::pinmux::PinmuxPadAttr;
use ot_hal::top::TopDescription;
use ot_hal::top::desc::NameTable;

use crate::io::uart::Uart;
use crate::test_utils::e2e_command::TestCommand;
use crate::test_utils::rpc::{ConsoleRecv, ConsoleSend};
use crate::test_utils::status::Status;

// Create aliases for the C names of these types so that the ujson
// created structs can access these structures by their C names.
// The C types are per-top enums. The host sends their numeric values, taken
// from the top description, which the device accepts in place of the names.
#[allow(non_camel_case_types)]
type pinmux_peripheral_in_t = u32;
#[allow(non_camel_case_types)]
type pinmux_insel_t = u32;
#[allow(non_camel_case_types)]
type pinmux_mio_out_t = u32;
#[allow(non_camel_case_types)]
type pinmux_outsel_t = u32;

// Bring in the auto-generated sources.
include!(env!("pinmux_config"));

/// Capacity of a single configuration message.
const CONFIG_CAP: usize = 16;

/// Returns the value of pinmux name `name` from `table` of `top`.
fn lookup(top: &TopDescription, table: &NameTable, kind: &str, name: &str) -> Result<u32> {
    table
        .get(name)
        .with_context(|| format!("Top {} has no pinmux {kind} {name:?}", top.name))
}

/// Returns the end marker of `table`, which ends a list on the device.
fn end(table: &NameTable) -> u32 {
    // The values run from 0, so the end marker is the number of values.
    table.len() as u32
}

impl PinmuxConfig {
    /// Configures the pinmux of `top` through the test command interface.
    ///
    /// `inputs` maps peripheral inputs to input selectors, `outputs` maps MIOs
    /// to output selectors, and `attrs` maps MIOs to pad attributes. Inputs,
    /// selectors and MIOs are given by their names in the top description,
    /// such as `GpioGpio0` and `Ioa0`.
    pub fn configure(
        uart: &dyn Uart,
        top: &TopDescription,
        inputs: Option<&HashMap<String, String>>,
        outputs: Option<&HashMap<String, String>>,
        attrs: Option<&HashMap<String, PinmuxPadAttr>>,
    ) -> Result<()> {
        for config in Self::messages(top, inputs, outputs, attrs)? {
            TestCommand::PinmuxConfig.send(uart)?;
            config.send(uart)?;
            Status::recv(uart, Duration::from_secs(300), false, false)?;
        }

        Ok(())
    }

    /// Returns the configuration messages that `configure` sends.
    fn messages(
        top: &TopDescription,
        inputs: Option<&HashMap<String, String>>,
        outputs: Option<&HashMap<String, String>>,
        attrs: Option<&HashMap<String, PinmuxPadAttr>>,
    ) -> Result<Vec<PinmuxConfig>> {
        let pinmux = &top.pinmux;

        // The PinmuxConfig struct can carry a limited number of input
        // and output configurations.  We'll take the whole config and
        // chunk it into as many PinmuxConfig commands as necessary.

        let mut inputs = inputs
            .into_iter()
            .flat_map(HashMap::iter)
            .map(|(key, value)| {
                Ok((
                    lookup(top, &pinmux.peripheral_in, "peripheral input", key)?,
                    lookup(top, &pinmux.insel, "input selector", value)?,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let mut outputs = outputs
            .into_iter()
            .flat_map(HashMap::iter)
            .map(|(key, value)| {
                Ok((
                    lookup(top, &pinmux.mio_out, "MIO output", key)?,
                    lookup(top, &pinmux.outsel, "output selector", value)?,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let mut attrs = attrs
            .into_iter()
            .flat_map(HashMap::iter)
            .map(|(key, value)| {
                Ok((
                    lookup(top, &pinmux.mio_out, "MIO output", key)?,
                    value.bits(),
                ))
            })
            .collect::<Result<Vec<_>>>()?;

        let len = cmp::max(cmp::max(inputs.len(), outputs.len()), attrs.len())
            .next_multiple_of(CONFIG_CAP);

        // Pad with end markers, as the per-top enums' defaults did.
        inputs.resize(len, (end(&pinmux.peripheral_in), end(&pinmux.insel)));
        outputs.resize(len, (end(&pinmux.mio_out), end(&pinmux.outsel)));
        attrs.resize(len, (end(&pinmux.mio_out), 0));

        let mut messages = Vec::new();
        while !inputs.is_empty() && !outputs.is_empty() && !attrs.is_empty() {
            let (in_peripherals, in_selectors) = inputs.drain(..CONFIG_CAP).unzip();
            let (out_mios, out_selectors) = outputs.drain(..CONFIG_CAP).unzip();
            let (attr_mios, attr_flags) = attrs.drain(..CONFIG_CAP).unzip();

            let config = PinmuxConfig {
                input: PinmuxInputSelection {
                    peripheral: in_peripherals,
                    selector: in_selectors,
                },
                output: PinmuxOutputSelection {
                    mio: out_mios,
                    selector: out_selectors,
                },
                attrs: PinmuxAttrConfig {
                    mio: attr_mios,
                    flags: attr_flags,
                },
            };

            messages.push(config);
        }

        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn egret() -> TopDescription {
        TopDescription::embedded("egret").unwrap().unwrap()
    }

    fn names(entries: &[(&str, &str)]) -> HashMap<String, String> {
        entries
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    fn map<V: Clone>(entries: &[(&str, V)]) -> HashMap<String, V> {
        entries
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }

    #[test]
    fn messages_use_values_and_end_markers() -> Result<()> {
        let top = egret();
        let inputs = names(&[("GpioGpio0", "Ioa0")]);
        let outputs = names(&[("Ioa0", "GpioGpio0"), ("Ioa1", "GpioGpio1")]);
        let attrs = map(&[("Ioa1", PinmuxPadAttr::INVERT)]);
        let messages = PinmuxConfig::messages(&top, Some(&inputs), Some(&outputs), Some(&attrs))?;
        assert_eq!(messages.len(), 1);
        let config = &messages[0];

        // Egret's values, and its End values (57, 49, 47, 78) as padding.
        assert_eq!(config.input.peripheral.len(), CONFIG_CAP);
        assert_eq!(config.input.peripheral[..2], [0, 57]);
        assert_eq!(config.input.selector[..2], [2, 49]);
        let mut out: Vec<_> = config.output.mio[..2]
            .iter()
            .copied()
            .zip(config.output.selector[..2].iter().copied())
            .collect();
        out.sort();
        assert_eq!(out, [(0, 3), (1, 4)]);
        assert_eq!(config.output.mio[2], 47);
        assert_eq!(config.output.selector[2], 78);
        assert_eq!(config.attrs.mio[..2], [1, 47]);
        assert_eq!(config.attrs.flags[..2], [PinmuxPadAttr::INVERT.bits(), 0]);

        // The device receives numbers, not names.
        let json = serde_json::to_value(config)?;
        assert_eq!(json["input"]["peripheral"][0], 0);
        assert_eq!(json["input"]["selector"][0], 2);
        Ok(())
    }

    #[test]
    fn messages_are_chunked() -> Result<()> {
        let top = egret();
        let names: Vec<_> = (0..=CONFIG_CAP).map(|i| format!("GpioGpio{i}")).collect();
        let inputs: HashMap<_, _> = names
            .iter()
            .map(|name| (name.clone(), "ConstantZero".to_string()))
            .collect();
        let messages = PinmuxConfig::messages(&top, Some(&inputs), None, None)?;
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[1].input.peripheral[1], 57);
        assert!(messages.iter().all(|m| m.output.mio[0] == 47));
        Ok(())
    }

    #[test]
    fn messages_reject_unknown_names() {
        let top = egret();
        let inputs = names(&[("GpioGpio99", "Ioa0")]);
        let err = PinmuxConfig::messages(&top, Some(&inputs), None, None).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Top egret has no pinmux peripheral input \"GpioGpio99\""
        );
        let outputs = names(&[("Ioa0", "NoSuchPeripheral")]);
        assert!(PinmuxConfig::messages(&top, None, Some(&outputs), None).is_err());
    }
}
