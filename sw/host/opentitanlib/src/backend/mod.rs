// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::{Args, FromArgMatches};
use ot_hal::top::TopDescription;
use ot_hal::top::desc::TOP_ENV_VAR;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::app::config::process_config_file;
use crate::app::{TransportWrapper, TransportWrapperBuilder};
use crate::transport::{EmptyTransport, Transport};
use crate::util::parse_int::ParseInt;

pub trait Backend {
    /// Additional backend-specific arguments in addition to `BackendOpts`.
    type Opts: Args;

    /// Create a transport with the provided arguments.
    fn create_transport(common: &BackendOpts, opts: &Self::Opts) -> Result<Box<dyn Transport>>;
}

#[doc(hidden)]
pub use inventory::submit;

#[doc(hidden)]
pub struct InterfaceRegistration {
    pub name: &'static str,
    pub augment_args: fn(clap::Command) -> clap::Command,
    pub create_transport: fn(&BackendOpts, &clap::ArgMatches) -> Result<Box<dyn Transport>>,
    pub config: Option<&'static str>,
}

impl InterfaceRegistration {
    pub const fn of<T: Backend>(name: &'static str, config: Option<&'static str>) -> Self {
        Self {
            name,
            augment_args: |command| {
                let id = T::Opts::group_id();

                // Filter duplicates with id, in case backend is registered multiple times.
                if command
                    .get_groups()
                    .any(|x| Some(x.get_id()) == id.as_ref())
                {
                    return command;
                }

                T::Opts::augment_args(command)
            },
            create_transport: |common, arg_matches| {
                let opts = T::Opts::from_arg_matches(arg_matches)?;
                T::create_transport(common, &opts)
            },
            config,
        }
    }
}

inventory::collect!(InterfaceRegistration);

#[macro_export]
macro_rules! define_interface {
    ($name: literal, $backend: ty) => {
        $crate::backend::define_interface!($name, $backend, None);
    };
    ($name: literal, $backend: ty, $config: literal) => {
        $crate::backend::define_interface!($name, $backend, Some($config));
    };
    ($name: literal, $backend: ty, $config: expr) => {
        $crate::backend::submit!($crate::backend::InterfaceRegistration::of::<$backend>(
            $name, $config
        ));
    };
}
pub use crate::define_interface;

struct TransportSpecificOpts {
    matches: clap::ArgMatches,
}

impl std::fmt::Debug for TransportSpecificOpts {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("TransportSpecificOpts").finish()
    }
}

impl clap::FromArgMatches for TransportSpecificOpts {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(Self {
            matches: matches.clone(),
        })
    }

    fn update_from_arg_matches(&mut self, _matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        unreachable!()
    }
}

impl Args for TransportSpecificOpts {
    fn augment_args(mut cmd: clap::Command) -> clap::Command {
        for reg in inventory::iter::<InterfaceRegistration> {
            cmd = (reg.augment_args)(cmd);
        }
        cmd
    }

    fn augment_args_for_update(_: clap::Command) -> clap::Command {
        unreachable!()
    }
}

#[derive(Debug, Args)]
pub struct BackendOpts {
    /// Name of the debug interface.
    #[arg(long, default_value = "")]
    pub interface: String,

    /// Whether to disable DFT with a strapping config during reset. Only required in TestUnlocked*
    /// LC states activate the JTAG RV TAP. This is required since the DFT straps share the console
    /// UART pins, that hyperdebug tries to pull high. In mission mode states this should be set to
    /// false, as DFT straps are not sampled here.
    #[arg(long)]
    pub disable_dft_on_reset: bool,

    /// USB Vendor ID of the interface.
    #[arg(long, value_parser = u16::from_str)]
    pub usb_vid: Option<u16>,
    /// USB Product ID of the interface.
    #[arg(long, value_parser = u16::from_str)]
    pub usb_pid: Option<u16>,
    /// USB serial number of the interface.
    #[arg(long)]
    pub usb_serial: Option<String>,

    #[command(flatten)]
    transport_specific_opts: TransportSpecificOpts,

    /// Configuration files.
    #[arg(long, num_args = 1)]
    pub conf: Vec<PathBuf>,

    /// Path to OpenOCD JTAG adapter config file to use (usually Olimex, but could be another
    /// adapter).  If unspecified, will use native support of the backend transport.  (Currently
    /// only the HyperDebug transport has such native JTAG support, and for any other transport,
    /// this argument must be specified if using JTAG.)
    #[arg(long)]
    pub openocd_adapter_config: Option<PathBuf>,

    /// Top of the chip: the name of a built-in top (egret, dragonfly, scafi_deprecated) or the
    /// path of a `top_<top>.host.json` description. Defaults to the top these tools were built
    /// for. $TARGET_CHIP_TOP overrides a `--top` in the rcfile. Unlike other options, `--top` may
    /// be given more than once; the last one wins.
    #[arg(long, env = TOP_ENV_VAR, overrides_with = "top")]
    pub top: Option<String>,
}

/// Removes `--top` from `args`, the arguments read from an rcfile, if $TARGET_CHIP_TOP is set,
/// so that the environment variable overrides the rcfile but not the command line.
pub fn drop_rcfile_top_if_env(args: &mut Vec<OsString>) {
    if std::env::var_os(TOP_ENV_VAR).is_some_and(|v| !v.is_empty()) {
        drop_top(args);
    }
}

/// Removes `--top <value>` and `--top=<value>` from `args`.
fn drop_top(args: &mut Vec<OsString>) {
    let mut skip_value = false;
    args.retain(|arg| {
        if std::mem::take(&mut skip_value) {
            return false;
        }
        if arg == "--top" {
            skip_value = true;
            return false;
        }
        !arg.to_str().is_some_and(|a| a.starts_with("--top="))
    });
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown interface {0}")]
    UnknownInterface(String),
}

/// Creates the requested backend interface according to [`BackendOpts`].
pub fn create(args: &BackendOpts) -> Result<TransportWrapper> {
    let interface = args.interface.as_str();
    let mut env = TransportWrapperBuilder::new(interface.to_string(), args.disable_dft_on_reset);

    for conf_file in &args.conf {
        process_config_file(&mut env, conf_file.as_ref())?
    }

    let interface = env.get_interface();

    let (backend, default_conf) = 'done: {
        for reg in inventory::iter::<InterfaceRegistration> {
            if interface == reg.name {
                let transport =
                    (reg.create_transport)(args, &args.transport_specific_opts.matches)?;
                break 'done (transport, reg.config.map(Path::new));
            }
        }

        Err(Error::UnknownInterface(interface.to_string()))?
    };

    if args.conf.is_empty()
        && let Some(conf_file) = default_conf
    {
        process_config_file(&mut env, conf_file)?
    }
    env.set_openocd_adapter_config(&args.openocd_adapter_config);
    // An empty value (such as `TARGET_CHIP_TOP=`) counts as not given.
    let top = args.top.as_deref().filter(|top| !top.is_empty());
    if let Some(top) = top.or(ot_hal::top::BUILT_FOR) {
        let top = TopDescription::from_name_or_path(top)?;
        log::info!("Using top {}", top.name);
        env.set_top(top);
    }
    env.build(backend)
}

pub fn create_empty_transport() -> Result<Box<dyn Transport>> {
    Ok(Box::new(EmptyTransport))
}

struct EmptyBackend;

impl Backend for EmptyBackend {
    type Opts = ();

    fn create_transport(_: &BackendOpts, _: &()) -> Result<Box<dyn Transport>> {
        Ok(Box::new(EmptyTransport))
    }
}

define_interface!("", EmptyBackend);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_top_from_args() {
        let mut args: Vec<OsString> = [
            "tool",
            "--top",
            "egret",
            "--interface=hyper310",
            "--top=dragonfly",
            "--topology",
            "--conf",
            "x.json",
        ]
        .iter()
        .map(OsString::from)
        .collect();
        drop_top(&mut args);
        assert_eq!(
            args,
            [
                "tool",
                "--interface=hyper310",
                "--topology",
                "--conf",
                "x.json"
            ]
            .iter()
            .map(OsString::from)
            .collect::<Vec<_>>()
        );
    }
}
