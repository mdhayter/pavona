// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

//! Runtime description of a top, loaded from the `top_<top>.host.json` file that
//! `util/gen_top_host.py` generates (Bazel target `//hw/top_<top>/data:host_desc`).
//!
//! The descriptions of the registered tops (`host_desc` in their `pavona_top()`) are embedded
//! at build time, so a top can be selected by name. Any other top (for example one kept out of tree) can be selected by the path of its
//! description file.

use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

/// Environment variable naming the top to use when none is given on the command line.
pub const TOP_ENV_VAR: &str = "TARGET_CHIP_TOP";

/// The address space seen by the main processor.
pub const HART_ADDR_SPACE: &str = "hart";

// `EMBEDDED: &[(&str, &str)]`, the (name, JSON) descriptions of the registered tops, generated
// from their `host_desc` attributes (//sw/host/ot_hal:embedded_host_descs).
include!("embedded.rs");

/// A range of addresses.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub struct AddrRange {
    pub base_addr: usize,
    pub size_bytes: usize,
}

/// The device, memory and named subranges (such as `mmio`) of one address space.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct AddrSpace {
    /// Device register interfaces, keyed by instance name, with the interface name appended
    /// when the instance has more than one (`uart0`, `lc_ctrl_regs`).
    pub devices: BTreeMap<String, AddrRange>,
    /// Memories, keyed by instance and memory name (`sram_ctrl_main_ram`).
    pub memories: BTreeMap<String, AddrRange>,
    #[serde(default)]
    pub subranges: BTreeMap<String, AddrRange>,
}

impl AddrSpace {
    /// Looks up a device or memory by name.
    pub fn range(&self, name: &str) -> Option<&AddrRange> {
        self.devices.get(name).or_else(|| self.memories.get(name))
    }
}

/// A table of names and their values, as in a topgen enum.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct NameTable(BTreeMap<String, u32>);

impl NameTable {
    /// Returns the value of `name`.
    pub fn get(&self, name: &str) -> Option<u32> {
        self.0.get(name).copied()
    }

    /// Returns the name of `value`.
    pub fn name_of(&self, value: u32) -> Option<&str> {
        self.0
            .iter()
            .find(|(_, v)| **v == value)
            .map(|(k, _)| k.as_str())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterates over the (name, value) pairs in name order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, u32)> {
        self.0.iter().map(|(k, v)| (k.as_str(), *v))
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AlertHandler {
    pub peripherals: NameTable,
    pub alerts: NameTable,
    pub alert_to_peripheral: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Pinmux {
    pub num_mio_pads: u32,
    pub num_dio_pads: u32,
    pub peripheral_in: NameTable,
    pub insel: NameTable,
    pub mio_out: NameTable,
    pub outsel: NameTable,
    pub direct_pads: NameTable,
    pub muxed_pads: NameTable,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Pwrmgr {
    pub wakeups: NameTable,
    pub reset_requests: NameTable,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Rstmgr {
    pub sw_resets: NameTable,
    /// The bit of each reset cause in the RESET_INFO register: `Por`, `LowPowerExit`,
    /// `SwReset`, then the hardware reset requests (the names in `Pwrmgr::reset_requests`,
    /// then the power manager's own, such as `MainPwr`, `Esc` and `Ndm`).
    #[serde(default)]
    pub reset_info: NameTable,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Clkmgr {
    pub gateable_clocks: NameTable,
    pub hintable_clocks: NameTable,
    /// Whether the clock manager supports external clock bypass (has the EXTCLK registers).
    #[serde(default)]
    pub ext_clk_bypass: bool,
}

/// The host's view of a top.
#[derive(Clone, Debug, Deserialize)]
pub struct TopDescription {
    /// Top name without the `top_` prefix (`egret`).
    pub name: String,
    pub addr_spaces: BTreeMap<String, AddrSpace>,
    /// Absent on tops without an alert handler.
    #[serde(default)]
    pub alert_handler: Option<AlertHandler>,
    pub pinmux: Pinmux,
    pub pwrmgr: Pwrmgr,
    pub rstmgr: Rstmgr,
    pub clkmgr: Clkmgr,
    /// JTAG IDCODE of the debug module TAP, from the top's JTAG ID package. Absent on
    /// tops without one.
    #[serde(default)]
    pub dm_idcode: Option<u32>,
    /// JTAG IDCODE of the life cycle controller TAP; the same as `dm_idcode` on a top
    /// with one combined TAP.
    #[serde(default)]
    pub lc_idcode: Option<u32>,
}

impl TopDescription {
    /// Parses a description from its JSON text.
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    /// Loads a description from a `top_<top>.host.json` file.
    pub fn load(path: &Path) -> Result<Self> {
        let json = std::fs::read_to_string(path)
            .with_context(|| format!("Reading top description {}", path.display()))?;
        Self::from_json(&json)
            .with_context(|| format!("Parsing top description {}", path.display()))
    }

    /// The names of the tops whose descriptions are embedded in this build.
    pub fn embedded_names() -> impl Iterator<Item = &'static str> {
        EMBEDDED.iter().map(|(name, _)| *name)
    }

    /// Returns the embedded description of top `name` (`egret` or `top_egret`), if there is one.
    pub fn embedded(name: &str) -> Option<Result<Self>> {
        let name = name.strip_prefix("top_").unwrap_or(name);
        EMBEDDED.iter().find(|(n, _)| *n == name).map(|(n, json)| {
            Self::from_json(json).with_context(|| format!("Parsing embedded top {n}"))
        })
    }

    /// Resolves `top`, which is either the name of an embedded top or the path of a description
    /// file.
    pub fn from_name_or_path(top: &str) -> Result<Self> {
        if let Some(desc) = Self::embedded(top) {
            return desc;
        }
        let path = Path::new(top);
        if !path.exists() {
            bail!(
                "Unknown top {top:?}: not one of [{}] and not a file",
                Self::embedded_names().collect::<Vec<_>>().join(", ")
            );
        }
        Self::load(path)
    }

    /// Returns the address space `name`.
    pub fn addr_space(&self, name: &str) -> Result<&AddrSpace> {
        match self.addr_spaces.get(name) {
            Some(space) => Ok(space),
            None => bail!("Top {} has no address space {name:?}", self.name),
        }
    }

    /// Returns the address range of device or memory `name` in the hart address space.
    pub fn range(&self, name: &str) -> Result<&AddrRange> {
        match self.addr_space(HART_ADDR_SPACE)?.range(name) {
            Some(range) => Ok(range),
            None => bail!("Top {} has no device or memory {name:?}", self.name),
        }
    }

    /// Returns the base address of device or memory `name` (`rstmgr_aon`, `sram_ctrl_main_ram`)
    /// in the hart address space.
    pub fn base_addr(&self, name: &str) -> Result<usize> {
        Ok(self.range(name)?.base_addr)
    }

    /// Returns the size of device or memory `name` in the hart address space.
    pub fn size_bytes(&self, name: &str) -> Result<usize> {
        Ok(self.range(name)?.size_bytes)
    }

    /// Returns the mask of reset cause `name` (see `Rstmgr::reset_info`) in the reset manager's
    /// RESET_INFO register.
    pub fn reset_info_mask(&self, name: &str) -> Result<u32> {
        match self.rstmgr.reset_info.get(name) {
            Some(bit) => Ok(1 << bit),
            None => bail!("Top {} has no reset cause {name:?}", self.name),
        }
    }

    /// Returns the MMIO region of the hart address space.
    pub fn mmio(&self) -> Result<&AddrRange> {
        match self.addr_space(HART_ADDR_SPACE)?.subranges.get("mmio") {
            Some(range) => Ok(range),
            None => bail!("Top {} has no MMIO region", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_jtag_idcodes() -> Result<()> {
        let idcodes = |name| -> Result<_> {
            let desc = TopDescription::embedded(name).unwrap()?;
            Ok((desc.dm_idcode, desc.lc_idcode))
        };
        assert_eq!(idcodes("egret")?, (Some(0x10001cdf), Some(0x10002cdf)));
        assert_eq!(idcodes("dragonfly")?, (Some(0x10013cdf), Some(0x10013cdf)));
        assert_eq!(idcodes("scafi_deprecated")?, (None, None));
        Ok(())
    }

    #[test]
    fn embedded_reset_info_and_extclk() -> Result<()> {
        let egret = TopDescription::embedded("egret").unwrap()?;
        assert_eq!(egret.reset_info_mask("Por")?, 1 << 0);
        assert_eq!(egret.reset_info_mask("SwReset")?, 1 << 2);
        assert_eq!(egret.reset_info_mask("AonTimerAonAonTimerRstReq")?, 1 << 4);
        assert_eq!(egret.reset_info_mask("Ndm")?, 1 << 7);
        assert!(egret.reset_info_mask("Watchdog").is_err());
        assert!(egret.clkmgr.ext_clk_bypass);
        let dragonfly = TopDescription::embedded("dragonfly").unwrap()?;
        assert_eq!(
            dragonfly.reset_info_mask("AonTimerAonAonTimerRstReq")?,
            1 << 3
        );
        assert!(!dragonfly.clkmgr.ext_clk_bypass);
        Ok(())
    }

    /// The reset causes of the top the host tools are built for match its DIF.
    #[cfg(any(feature = "egret", feature = "dragonfly"))]
    #[test]
    fn reset_info_matches_dif() -> Result<()> {
        use ot_bindgen_dif as dif;

        let desc = TopDescription::embedded(crate::top::BUILT_FOR.unwrap()).unwrap()?;
        #[cfg(feature = "egret")]
        let hw_requests = [
            (
                "SysrstCtrlAonRstReq",
                dif::dif_rstmgr_reset_info_kDifRstmgrResetInfoSysRstCtrl,
            ),
            (
                "AonTimerAonAonTimerRstReq",
                dif::dif_rstmgr_reset_info_kDifRstmgrResetInfoWatchdog,
            ),
        ];
        #[cfg(feature = "dragonfly")]
        let hw_requests = [
            (
                "AonTimerAonAonTimerRstReq",
                dif::dif_rstmgr_reset_info_kDifRstmgrResetInfoWatchdog,
            ),
            (
                "SocProxyRstReqExternal",
                dif::dif_rstmgr_reset_info_kDifRstmgrResetInfoExternalRst,
            ),
        ];
        let causes = [
            ("Por", dif::dif_rstmgr_reset_info_kDifRstmgrResetInfoPor),
            (
                "LowPowerExit",
                dif::dif_rstmgr_reset_info_kDifRstmgrResetInfoLowPowerExit,
            ),
            ("SwReset", dif::dif_rstmgr_reset_info_kDifRstmgrResetInfoSw),
            (
                "MainPwr",
                dif::dif_rstmgr_reset_info_kDifRstmgrResetInfoPowerUnstable,
            ),
            (
                "Esc",
                dif::dif_rstmgr_reset_info_kDifRstmgrResetInfoEscalation,
            ),
            ("Ndm", dif::dif_rstmgr_reset_info_kDifRstmgrResetInfoNdm),
        ];
        for (name, mask) in causes.into_iter().chain(hw_requests) {
            assert_eq!(desc.reset_info_mask(name)?, mask, "{name}");
        }
        assert_eq!(
            desc.rstmgr.reset_info.len(),
            causes.len() + hw_requests.len()
        );
        Ok(())
    }

    #[test]
    fn embedded_tops_parse() -> Result<()> {
        for name in TopDescription::embedded_names() {
            let desc = TopDescription::embedded(name).unwrap()?;
            assert_eq!(desc.name, name);
            assert!(desc.mmio()?.size_bytes > 0);
            assert_eq!(
                desc.pinmux.muxed_pads.len(),
                desc.pinmux.num_mio_pads as usize
            );
            assert_eq!(
                desc.pinmux.direct_pads.len(),
                desc.pinmux.num_dio_pads as usize
            );
        }
        Ok(())
    }

    #[test]
    fn names_and_prefix() -> Result<()> {
        assert_eq!(
            TopDescription::from_name_or_path("top_egret")?.name,
            "egret"
        );
        assert_eq!(
            TopDescription::from_name_or_path("dragonfly")?.name,
            "dragonfly"
        );
        let err = TopDescription::from_name_or_path("no_such_top").unwrap_err();
        assert!(err.to_string().contains("egret"), "{err}");
        Ok(())
    }

    #[test]
    fn load_from_file() -> Result<()> {
        // Bazel gives each test its own TEST_TMPDIR.
        let dir = std::env::var_os("TEST_TMPDIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        let pid = std::process::id();

        // A copy of a built-in description loads the same, by `load` and by path.
        let (_, json) = EMBEDDED.iter().find(|(name, _)| *name == "egret").unwrap();
        let good = dir.join(format!("desc_test_{pid}.host.json"));
        std::fs::write(&good, json)?;
        let desc = TopDescription::load(&good)?;
        assert_eq!(desc.name, "egret");
        let by_path = TopDescription::from_name_or_path(good.to_str().unwrap())?;
        assert_eq!(by_path.name, "egret");
        assert_eq!(*by_path.mmio()?, *desc.mmio()?);
        assert_eq!(
            by_path.base_addr("rstmgr_aon")?,
            desc.base_addr("rstmgr_aon")?
        );

        // Errors name the file: one that doesn't parse, and one that doesn't exist.
        let bad = dir.join(format!("desc_test_{pid}.bad.json"));
        std::fs::write(&bad, r#"{"name": "incomplete"}"#)?;
        let err = TopDescription::from_name_or_path(bad.to_str().unwrap()).unwrap_err();
        assert!(
            format!("{err:#}").contains(bad.to_str().unwrap()),
            "{err:#}"
        );
        let missing = dir.join(format!("desc_test_{pid}.missing.json"));
        let err = TopDescription::load(&missing).unwrap_err();
        assert!(
            format!("{err:#}").contains(missing.to_str().unwrap()),
            "{err:#}"
        );

        std::fs::remove_file(&good)?;
        std::fs::remove_file(&bad)?;
        Ok(())
    }

    #[test]
    fn lookups() -> Result<()> {
        let desc = TopDescription::from_json(
            r#"{
                "SPDX-License-Identifier": "Apache-2.0",
                "name": "test",
                "addr_spaces": {"hart": {
                    "devices": {"uart0": {"base_addr": 4096, "size_bytes": 64}},
                    "memories": {"ram": {"base_addr": 8192, "size_bytes": 1024}},
                    "subranges": {"mmio": {"base_addr": 4096, "size_bytes": 4096}}
                }},
                "pinmux": {
                    "num_mio_pads": 1, "num_dio_pads": 0,
                    "peripheral_in": {}, "insel": {"ConstantZero": 0, "Ioa0": 2},
                    "mio_out": {}, "outsel": {}, "direct_pads": {}, "muxed_pads": {"Ioa0": 0}
                },
                "pwrmgr": {"wakeups": {}, "reset_requests": {}},
                "rstmgr": {"sw_resets": {}},
                "clkmgr": {"gateable_clocks": {}, "hintable_clocks": {}}
            }"#,
        )?;
        assert_eq!(desc.base_addr("uart0")?, 4096);
        assert_eq!(desc.size_bytes("ram")?, 1024);
        assert_eq!(
            *desc.mmio()?,
            AddrRange {
                base_addr: 4096,
                size_bytes: 4096
            }
        );
        assert!(desc.base_addr("uart1").is_err());
        assert!(desc.addr_space("other").is_err());
        assert!(desc.alert_handler.is_none());
        assert_eq!(desc.pinmux.insel.get("Ioa0"), Some(2));
        assert_eq!(desc.pinmux.insel.name_of(0), Some("ConstantZero"));
        assert_eq!(desc.pinmux.insel.get("Ioa1"), None);
        Ok(())
    }

    /// Checks a description table against a topgen enum, in both directions. The enum's
    /// trailing `End` is not in the description.
    #[cfg(any(feature = "egret", feature = "dragonfly"))]
    fn check_table<E: clap::ValueEnum + Copy + Into<u32>>(
        table: &NameTable,
        variants: &[&str],
        what: &str,
    ) {
        let values = E::value_variants();
        assert_eq!(variants.len(), values.len());
        let mut count = 0;
        for (name, value) in variants.iter().zip(values) {
            if *name == "End" {
                continue;
            }
            assert_eq!(table.get(name), Some((*value).into()), "{what} {name}");
            count += 1;
        }
        assert_eq!(table.len(), count, "{what}");
    }

    /// Checks the selected top's description against the generated constants and enums.
    #[cfg(any(feature = "egret", feature = "dragonfly"))]
    fn check_against_constants(desc: &TopDescription) -> Result<()> {
        use crate::top::{
            DirectPads, MuxedPads, PinmuxInsel, PinmuxMioOut, PinmuxOutsel, PinmuxPeripheralIn,
        };

        let p = &desc.pinmux;
        check_table::<PinmuxPeripheralIn>(
            &p.peripheral_in,
            PinmuxPeripheralIn::VARIANTS,
            "peripheral_in",
        );
        check_table::<PinmuxInsel>(&p.insel, PinmuxInsel::VARIANTS, "insel");
        check_table::<PinmuxMioOut>(&p.mio_out, PinmuxMioOut::VARIANTS, "mio_out");
        check_table::<PinmuxOutsel>(&p.outsel, PinmuxOutsel::VARIANTS, "outsel");
        check_table::<DirectPads>(&p.direct_pads, DirectPads::VARIANTS, "direct_pads");
        check_table::<MuxedPads>(&p.muxed_pads, MuxedPads::VARIANTS, "muxed_pads");
        Ok(())
    }

    #[cfg(feature = "egret")]
    #[test]
    fn egret_matches_constants() -> Result<()> {
        use crate::top;

        let desc = TopDescription::embedded("egret").unwrap()?;
        check_against_constants(&desc)?;
        assert_eq!(top::BUILT_FOR, Some("egret"));
        assert_eq!(
            *desc.mmio()?,
            AddrRange {
                base_addr: top::TOP_EGRET_MMIO_BASE_ADDR,
                size_bytes: top::TOP_EGRET_MMIO_SIZE_BYTES,
            }
        );
        // The constants that host code uses today.
        for (name, addr) in [
            ("acc", top::ACC_BASE_ADDR),
            ("adc_ctrl_aon", top::ADC_CTRL_AON_BASE_ADDR),
            ("aes", top::AES_BASE_ADDR),
            ("alert_handler", top::ALERT_HANDLER_BASE_ADDR),
            ("aon_timer_aon", top::AON_TIMER_AON_BASE_ADDR),
            ("clkmgr_aon", top::CLKMGR_AON_BASE_ADDR),
            ("csrng", top::CSRNG_BASE_ADDR),
            ("edn0", top::EDN0_BASE_ADDR),
            ("edn1", top::EDN1_BASE_ADDR),
            ("entropy_src", top::ENTROPY_SRC_BASE_ADDR),
            ("flash_ctrl_core", top::FLASH_CTRL_CORE_BASE_ADDR),
            ("gpio", top::GPIO_BASE_ADDR),
            ("hmac", top::HMAC_BASE_ADDR),
            ("i2c0", top::I2C0_BASE_ADDR),
            ("i2c1", top::I2C1_BASE_ADDR),
            ("i2c2", top::I2C2_BASE_ADDR),
            ("keymgr", top::KEYMGR_BASE_ADDR),
            ("kmac", top::KMAC_BASE_ADDR),
            ("lc_ctrl_regs", top::LC_CTRL_REGS_BASE_ADDR),
            ("otp_ctrl_core", top::OTP_CTRL_CORE_BASE_ADDR),
            ("pattgen", top::PATTGEN_BASE_ADDR),
            ("pinmux_aon", top::PINMUX_AON_BASE_ADDR),
            ("pwm_aon", top::PWM_AON_BASE_ADDR),
            ("pwrmgr_aon", top::PWRMGR_AON_BASE_ADDR),
            ("rom_ctrl_rom", top::ROM_CTRL_ROM_BASE_ADDR),
            ("rstmgr_aon", top::RSTMGR_AON_BASE_ADDR),
            ("rv_timer", top::RV_TIMER_BASE_ADDR),
            ("sensor_ctrl_aon", top::SENSOR_CTRL_AON_BASE_ADDR),
            ("spi_device", top::SPI_DEVICE_BASE_ADDR),
            ("spi_host0", top::SPI_HOST0_BASE_ADDR),
            ("spi_host1", top::SPI_HOST1_BASE_ADDR),
            ("sram_ctrl_main_ram", top::SRAM_CTRL_MAIN_RAM_BASE_ADDR),
            ("sram_ctrl_main_regs", top::SRAM_CTRL_MAIN_REGS_BASE_ADDR),
            (
                "sram_ctrl_ret_aon_ram",
                top::SRAM_CTRL_RET_AON_RAM_BASE_ADDR,
            ),
            ("sysrst_ctrl_aon", top::SYSRST_CTRL_AON_BASE_ADDR),
            ("uart0", top::UART0_BASE_ADDR),
            ("uart1", top::UART1_BASE_ADDR),
            ("uart2", top::UART2_BASE_ADDR),
            ("uart3", top::UART3_BASE_ADDR),
            ("usbdev", top::USBDEV_BASE_ADDR),
        ] {
            assert_eq!(desc.base_addr(name)?, addr, "{name}");
        }
        for (name, size) in [
            ("rom_ctrl_rom", top::ROM_CTRL_ROM_SIZE_BYTES),
            ("sram_ctrl_main_ram", top::SRAM_CTRL_MAIN_RAM_SIZE_BYTES),
            (
                "sram_ctrl_ret_aon_ram",
                top::SRAM_CTRL_RET_AON_RAM_SIZE_BYTES,
            ),
        ] {
            assert_eq!(desc.size_bytes(name)?, size, "{name}");
        }
        Ok(())
    }

    #[cfg(feature = "dragonfly")]
    #[test]
    fn dragonfly_matches_constants() -> Result<()> {
        use crate::top;

        let desc = TopDescription::embedded("dragonfly").unwrap()?;
        check_against_constants(&desc)?;
        assert_eq!(top::BUILT_FOR, Some("dragonfly"));
        assert_eq!(
            *desc.mmio()?,
            AddrRange {
                base_addr: top::TOP_DRAGONFLY_MMIO_BASE_ADDR,
                size_bytes: top::TOP_DRAGONFLY_MMIO_SIZE_BYTES,
            }
        );
        // The constants that host code uses today.
        for (name, addr) in [
            ("acc", top::ACC_BASE_ADDR),
            ("aes", top::AES_BASE_ADDR),
            ("alert_handler", top::ALERT_HANDLER_BASE_ADDR),
            ("aon_timer_aon", top::AON_TIMER_AON_BASE_ADDR),
            ("clkmgr_aon", top::CLKMGR_AON_BASE_ADDR),
            ("csrng", top::CSRNG_BASE_ADDR),
            ("edn0", top::EDN0_BASE_ADDR),
            ("edn1", top::EDN1_BASE_ADDR),
            ("entropy_src", top::ENTROPY_SRC_BASE_ADDR),
            ("gpio", top::GPIO_BASE_ADDR),
            ("hmac", top::HMAC_BASE_ADDR),
            ("i2c0", top::I2C0_BASE_ADDR),
            ("keymgr_dpe", top::KEYMGR_DPE_BASE_ADDR),
            ("kmac", top::KMAC_BASE_ADDR),
            ("lc_ctrl_regs", top::LC_CTRL_REGS_BASE_ADDR),
            ("otp_ctrl_core", top::OTP_CTRL_CORE_BASE_ADDR),
            ("pinmux_aon", top::PINMUX_AON_BASE_ADDR),
            ("pwrmgr_aon", top::PWRMGR_AON_BASE_ADDR),
            ("rom_ctrl0_rom", top::ROM_CTRL0_ROM_BASE_ADDR),
            ("rstmgr_aon", top::RSTMGR_AON_BASE_ADDR),
            ("rv_timer", top::RV_TIMER_BASE_ADDR),
            ("spi_device", top::SPI_DEVICE_BASE_ADDR),
            ("spi_host0", top::SPI_HOST0_BASE_ADDR),
            ("sram_ctrl_main_ram", top::SRAM_CTRL_MAIN_RAM_BASE_ADDR),
            ("sram_ctrl_main_regs", top::SRAM_CTRL_MAIN_REGS_BASE_ADDR),
            (
                "sram_ctrl_ret_aon_ram",
                top::SRAM_CTRL_RET_AON_RAM_BASE_ADDR,
            ),
            ("uart0", top::UART0_BASE_ADDR),
        ] {
            assert_eq!(desc.base_addr(name)?, addr, "{name}");
        }
        for (name, size) in [
            ("rom_ctrl0_rom", top::ROM_CTRL0_ROM_SIZE_BYTES),
            ("sram_ctrl_main_ram", top::SRAM_CTRL_MAIN_RAM_SIZE_BYTES),
            (
                "sram_ctrl_ret_aon_ram",
                top::SRAM_CTRL_RET_AON_RAM_SIZE_BYTES,
            ),
        ] {
            assert_eq!(desc.size_bytes(name)?, size, "{name}");
        }
        Ok(())
    }
}
