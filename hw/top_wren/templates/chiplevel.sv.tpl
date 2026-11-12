// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
${gencmd}
## Copied (and modified) from hw/top_egret/templates/chiplevel.sv.tpl, which
## top_scafi_deprecated shares. Wren has its own, since the chip level is
## specific to the board: this template only supports the Digilent Genesys 2
## (target genesys2).
<%
import re
import topgen.lib as lib
from copy import deepcopy

assert target["name"] == "genesys2", "Wren's chip template only supports the genesys2 target"

# Provide shortcuts for some commonly used variables
pinmux = top['pinmux']
pinout = top['pinout']

def get_dio_sig(pinmux: {}, pad: {}):
  '''Get DIO signal associated with this pad or return None'''
  for sig in pinmux["ios"]:
    if sig["connection"] == "direct" and pad["name"] == sig["pad"]:
      return sig
  else:
    return None

# Modify the pad lists on the fly, based on target config
muxed_pads = []
dedicated_pads = []
k = 0
for pad in pinout["pads"]:
  if pad["connection"] == "muxed":
    if pad["name"] not in target["pinout"]["remove_pads"]:
      muxed_pads.append(pad)
  else:
    k = pad["idx"]
    if pad["name"] not in target["pinout"]["remove_pads"]:
      dedicated_pads.append(pad)

for pad in target["pinout"]["add_pads"]:
  # Since these additional pads have not been elaborated in the merge phase,
  # we need to add their global index here.
  amended_pad = deepcopy(pad)
  amended_pad.update({"idx" : k})
  dedicated_pads.append(pad)
  k += 1

# The boot straps are active low at the board and inverted here, so that
# software and the test ROM see them as active high.
inverted_pads = ["IOC0", "IOC1", "IOC2"]

# The TAP strap pads are removed; tie the strap to select the rv_dm TAP.
tap1_pad = [s["pad"] for s in target["pinmux"]["special_signals"] if s["name"] == "tap1"][0]
%>\

module chip_${top["name"]}_${target["name"]} #(
  // Path to a VMEM file containing the contents of the boot ROM, which will be
  // baked into the FPGA bitstream.
  parameter BootRomInitFile = ""
) (
  // 200 MHz differential system clock. It goes straight to an IBUFDS, not
  // through the padring.
  input IO_CLK_P,
  input IO_CLK_N,

<%
  removed_port_names = []
%>\
  // Dedicated Pads
% for pad in dedicated_pads:
<%
  sig = get_dio_sig(pinmux, pad)
  if pad["name"] in target["pinout"]["remove_ports"]:
    port_comment = "// Removed port: "
    removed_port_names.append(pad["name"])
  else:
    port_comment = ""
  if sig is not None:
    comment = "// Dedicated Pad for {}".format(sig["name"])
  else:
    comment = "// Manual Pad"
%>\
  ${port_comment}${pad["port_type"]} ${pad["name"]}, ${comment}
% endfor

  // Muxed Pads
% for pad in muxed_pads:
<%
  if pad["name"] in target["pinout"]["remove_ports"]:
    port_comment = "// Removed port: "
    removed_port_names.append(pad["name"])
  else:
    port_comment = ""
%>\
  ${port_comment}${pad["port_type"]} ${pad["name"]}${" " if loop.last else ","} // MIO Pad ${pad["idx"]}
% endfor
);

  import top_${top["name"]}_pkg::*;
  import prim_pad_wrapper_pkg::*;

% if target["pinmux"]["special_signals"]:
  ////////////////////////////
  // Special Signal Indices //
  ////////////////////////////

  % for entry in target["pinmux"]["special_signals"]:
<% param_name = (lib.Name.from_snake_case(entry["name"]) +
                 lib.Name(["pad", "idx"])).as_camel_case()
%>\
  localparam int ${param_name} = ${entry["idx"]};
  % endfor
% endif

  // DFT and Debug signal positions in the pinout.
  localparam pinmux_pkg::target_cfg_t PinmuxTargetCfg = '{
    tck_idx:           TckPadIdx,
    tms_idx:           TmsPadIdx,
    trst_idx:          TrstNPadIdx,
    tdi_idx:           TdiPadIdx,
    tdo_idx:           TdoPadIdx,
    tap_strap0_idx:    Tap0PadIdx,
    tap_strap1_idx:    Tap1PadIdx,
    dft_strap0_idx:    Dft0PadIdx,
    dft_strap1_idx:    Dft1PadIdx,
    // TODO: check whether there is a better way to pass these USB-specific params
    usb_dp_idx:        DioUsbdevUsbDp,
    usb_dn_idx:        DioUsbdevUsbDn,
    usb_sense_idx:     MioInUsbdevSense,
    // Pad types for attribute WARL behavior
    dio_pad_type: {
<%
  pad_attr = []
  for sig in list(reversed(top["pinmux"]["ios"])):
    if sig["connection"] != "muxed":
      pad_attr.append((sig['name'], sig["attr"]))
%>\
% for name, attr in pad_attr:
      ${attr}${" " if loop.last else ","} // DIO ${name}
% endfor
    },
    mio_pad_type: {
<%
  pad_attr = []
  for pad in list(reversed(pinout["pads"])):
    if pad["connection"] == "muxed":
      pad_attr.append(pad["type"])
%>\
% for attr in pad_attr:
      ${attr}${" " if loop.last else ","} // MIO Pad ${len(pad_attr) - loop.index - 1}
% endfor
    },
    // Pad scan roles: there is no scan on the FPGA.
    dio_scan_role: {pinmux_reg_pkg::NDioPads{NoScan}},
    mio_scan_role: {pinmux_reg_pkg::NMioPads{NoScan}}
  };

  ////////////////////////
  // Signal definitions //
  ////////////////////////

  % if removed_port_names:
  // Net definitions for removed ports
  % endif
  % for port in removed_port_names:
  wire ${port};
  % endfor

  logic [3:0] mux_iob_sel;

  pad_attr_t [pinmux_reg_pkg::NMioPads-1:0] mio_attr;
  pad_attr_t [pinmux_reg_pkg::NDioPads-1:0] dio_attr;
  logic [pinmux_reg_pkg::NMioPads-1:0] mio_out;
  logic [pinmux_reg_pkg::NMioPads-1:0] mio_oe;
  logic [pinmux_reg_pkg::NMioPads-1:0] mio_in;
  logic [pinmux_reg_pkg::NMioPads-1:0] mio_in_raw;
  logic [${len(dedicated_pads)}-1:0]                       dio_in_raw;
  logic [pinmux_reg_pkg::NDioPads-1:0] dio_out;
  logic [pinmux_reg_pkg::NDioPads-1:0] dio_oe;
  logic [pinmux_reg_pkg::NDioPads-1:0] dio_in;

  // Inputs from the padring's muxed pads, in padring order.
  logic [${len(muxed_pads)}-1:0] mio_in_pad;
  logic [${len(muxed_pads)}-1:0] mio_in_raw_pad;

  logic unused_mio_in_raw;
  logic unused_dio_in_raw;
  assign unused_mio_in_raw = ^mio_in_raw;
  assign unused_dio_in_raw = ^dio_in_raw;

  // Manual pads
% for pad in dedicated_pads:
<%
  pad_prefix = pad["name"].lower()
%>\
% if not get_dio_sig(pinmux, pad):
  logic manual_in_${pad_prefix}, manual_out_${pad_prefix}, manual_oe_${pad_prefix};
% endif
% endfor

% for pad in dedicated_pads:
<%
  pad_prefix = pad["name"].lower()
%>\
% if not get_dio_sig(pinmux, pad):
  pad_attr_t manual_attr_${pad_prefix};
% endif
% endfor

  ////////////////////////////////
  // Muxed pad inputs and straps //
  ////////////////////////////////

% for pad in muxed_pads:
  % if pad["name"] in inverted_pads:
  // ${pad["name"]} is a boot strap, active low at the board.
  assign mio_in[${pad["idx"]}] = ~mio_in_pad[${loop.index}];
  assign mio_in_raw[${pad["idx"]}] = ~mio_in_raw_pad[${loop.index}];
  % else:
  assign mio_in[${pad["idx"]}] = mio_in_pad[${loop.index}];
  assign mio_in_raw[${pad["idx"]}] = mio_in_raw_pad[${loop.index}];
  % endif
% endfor

% if target["pinout"]["remove_pads"]:
  /////////////////////////
  // Stubbed pad tie-off //
  /////////////////////////

  // Only signals going to non-custom pads need to be tied off.
  logic [${len(pinout["pads"])-1}:0] unused_sig;
% for pad in pinout["pads"]:
  % if pad["connection"] == 'muxed':
    % if pad["name"] in target["pinout"]["remove_pads"]:
      % if pad["name"] == tap1_pad:
  // TAP strap 1 reads 1 and TAP strap 0 reads 0, which selects the rv_dm TAP.
  assign mio_in[${pad["idx"]}] = 1'b1;
      % else:
  assign mio_in[${pad["idx"]}] = 1'b0;
      % endif
  assign mio_in_raw[${pad["idx"]}] = 1'b0;
  assign unused_sig[${loop.index}] = mio_out[${pad["idx"]}] ^ mio_oe[${pad["idx"]}] ^
                                    (^mio_attr[${pad["idx"]}]);
    % endif
  % else:
    % if pad["name"] in target["pinout"]["remove_pads"]:
<%
    ## Only need to tie off if this is not a custom pad.
    sig = get_dio_sig(pinmux, pad)
    if sig is not None:
      sig_index = lib.get_io_enum_literal(sig, 'dio')
%>\
      % if sig is not None:
  assign dio_in[${lib.get_io_enum_literal(sig, 'dio')}] = 1'b0;
  assign unused_sig[${loop.index}] = dio_out[${sig_index}] ^ dio_oe[${sig_index}] ^
                                    (^dio_attr[${sig_index}]);
      % endif
    % endif
  % endif
% endfor
%endif

  //////////////////////
  // Padring Instance //
  //////////////////////

  ast_pkg::ast_clks_t ast_base_clks;

  padring #(
    // Padring specific counts may differ from pinmux config due
    // to custom, stubbed or added pads.
    .NDioPads(${len(dedicated_pads)}),
    .NMioPads(${len(muxed_pads)}),
    .DioPadType ({
% for pad in list(reversed(dedicated_pads)):
      ${pad["type"]}${" " if loop.last else ","} // ${pad['name']}
% endfor
    }),
    .MioPadType ({
% for pad in list(reversed(muxed_pads)):
      ${pad["type"]}${" " if loop.last else ","} // ${pad['name']}
% endfor
    })
  ) u_padring (
  // This is only used for scan and DFT purposes
    .clk_scan_i   ( 1'b0                  ),
    .scanmode_i   ( prim_mubi_pkg::MuBi4False ),
    .mux_iob_sel_i ( mux_iob_sel ),
    .dio_in_raw_o ( dio_in_raw ),
    // Chip IOs
    .dio_pad_io ({
% for pad in list(reversed(dedicated_pads)):
      ${pad["name"]}${"" if loop.last else ","}
% endfor
    }),

    .mio_pad_io ({
% for pad in list(reversed(muxed_pads)):
      ${pad["name"]}${"" if loop.last else ","}
% endfor
    }),

    // Core-facing
% for port in ["in_o", "out_i", "oe_i", "attr_i"]:
    .dio_${port} ({
  % for pad in list(reversed(dedicated_pads)):
  <%
    sig = get_dio_sig(pinmux, pad)
  %>\
    % if sig is None:
      manual_${port[:-2]}_${pad["name"].lower()}${"" if loop.last else ","}
    % else:
      dio_${port[:-2]}[${lib.get_io_enum_literal(sig, 'dio')}]${"" if loop.last else ","}
    % endif
  % endfor
      }),
% endfor

    .mio_in_o (mio_in_pad),
    .mio_in_raw_o (mio_in_raw_pad),
% for port in ["out_i", "oe_i", "attr_i"]:
<%
    sig_name = 'mio_' + port[:-2]
    indices = list(reversed(list(pad['idx'] for pad in muxed_pads)))
%>\
    .mio_${port} (${lib.make_bit_concatenation(sig_name, indices, 6)})${"" if loop.last else ","}
% endfor
  );

  ////////////////////////////////////////
  // USB on the Dual USB PMOD (JB 7-10) //
  ////////////////////////////////////////

  logic usb_dp_pullup_en;
  logic usb_dn_pullup_en;
  logic usb_rx_d;
  logic usb_tx_d;
  logic usb_tx_se0;
  logic usb_tx_use_d_se0;
  logic usb_rx_enable;

  // Connect the DP pad
  assign dio_in[DioUsbdevUsbDp] = manual_in_usb_p;
  assign manual_out_usb_p = dio_out[DioUsbdevUsbDp];
  assign manual_oe_usb_p = dio_oe[DioUsbdevUsbDp];
  assign manual_attr_usb_p = dio_attr[DioUsbdevUsbDp];

  // Connect the DN pad
  assign dio_in[DioUsbdevUsbDn] = manual_in_usb_n;
  assign manual_out_usb_n = dio_out[DioUsbdevUsbDn];
  assign manual_oe_usb_n = dio_oe[DioUsbdevUsbDn];
  assign manual_attr_usb_n = dio_attr[DioUsbdevUsbDn];

  // Connect the DP pull-up. The Pmod has no DN pull-up.
  assign manual_out_io_usb_dppullup0 = usb_dp_pullup_en;
  assign manual_oe_io_usb_dppullup0 = 1'b1;
  assign manual_attr_io_usb_dppullup0 = '0;

  // There is no differential receiver: usbdev uses the single-ended inputs.
  assign usb_rx_d = 1'b0;

  logic unused_usb_sigs;
  assign unused_usb_sigs = ^{
    usb_dn_pullup_en,
    usb_tx_d,
    usb_tx_se0,
    usb_tx_use_d_se0,
    usb_rx_enable,
    manual_in_io_usb_dppullup0
  };

  ////////////////////////////
  // Other board connections //
  ////////////////////////////

  // FT2232H channel A SPI enable (PROG_SPIEN): drive high, as Mocha does.
  assign manual_out_io_spien = 1'b1;
  assign manual_oe_io_spien = 1'b1;
  assign manual_attr_io_spien = '0;

  // PmodUSBUART CTS#: driven low so the host may always send.
  assign manual_out_io_uart1_cts_n = 1'b0;
  assign manual_oe_io_uart1_cts_n = 1'b1;
  assign manual_attr_io_uart1_cts_n = '0;

  // Reset inputs
  assign manual_out_por_n = 1'b0;
  assign manual_oe_por_n = 1'b0;
  assign manual_attr_por_n = '0;
  assign manual_out_io_ftdi_rst_n = 1'b0;
  assign manual_oe_io_ftdi_rst_n = 1'b0;
  assign manual_attr_io_ftdi_rst_n = '0;

  logic unused_manual_in;
  assign unused_manual_in = ^{
    manual_in_io_spien,
    manual_in_io_uart1_cts_n
  };

  //////////////////////////////////
  // AST - Common for all targets //
  //////////////////////////////////

  // pwrmgr interface
  pwrmgr_pkg::pwr_ast_req_t base_ast_pwr;
  pwrmgr_pkg::pwr_ast_rsp_t ast_base_pwr;

  // assorted ast status
  ast_pkg::ast_pwst_t ast_pwst;
  ast_pkg::ast_pwst_t ast_pwst_h;
  prim_mubi_pkg::mubi4_t ast_init_done;

  // TLUL interface
  tlul_pkg::tl_h2d_t base_ast_bus;
  tlul_pkg::tl_d2h_t ast_base_bus;

  // synchronization clocks / rests
  clkmgr_pkg::clkmgr_out_t clkmgr_aon_clocks;
  rstmgr_pkg::rstmgr_out_t rstmgr_aon_resets;

  // external clock
  logic ext_clk;

  // monitored clock
  logic sck_monitor;

  // observe interface
  logic [7:0] fla_obs;
  ast_pkg::ast_obs_ctrl_t obs_ctrl;

  logic usb_ref_pulse;
  logic usb_ref_val;

  // entropy distribution network
  edn_pkg::edn_req_t ast_edn_edn_req;
  edn_pkg::edn_rsp_t ast_edn_edn_rsp;

  // Flash connections
  prim_mubi_pkg::mubi4_t flash_bist_enable;
  logic flash_power_down_h;
  logic flash_power_ready_h;

  // clock bypass req/ack
  prim_mubi_pkg::mubi4_t io_clk_byp_req;
  prim_mubi_pkg::mubi4_t io_clk_byp_ack;
  prim_mubi_pkg::mubi4_t all_clk_byp_req;
  prim_mubi_pkg::mubi4_t all_clk_byp_ack;
  prim_mubi_pkg::mubi4_t hi_speed_sel;
  prim_mubi_pkg::mubi4_t div_step_down_req;

  // DFT connections
  logic scan_en;
  logic scan_rst_n;
  prim_mubi_pkg::mubi4_t scanmode;
  lc_ctrl_pkg::lc_tx_t dft_en;
  pinmux_pkg::dft_strap_test_req_t dft_strap_test;

  // Debug connections
  logic [ast_pkg::Ast2PadOutWidth-1:0] ast2pinmux;
  logic [ast_pkg::Pad2AstInWidth-1:0] pad2ast;

  // Jitter enable
  prim_mubi_pkg::mubi4_t jen;

  // Memory configuration connections
  ast_pkg::spm_rm_t ast_ram_1p_cfg;
  ast_pkg::spm_rm_t ast_rf_cfg;
  ast_pkg::spm_rm_t ast_rom_cfg;
  ast_pkg::dpm_rm_t ast_ram_2p_fcfg;
  ast_pkg::dpm_rm_t ast_ram_2p_lcfg;

  // The FPGA memories take no configuration, there is no flash macro to power,
  // and only the AON and main power-good signals are used.
  logic unused_ast_sigs;
  assign unused_ast_sigs = ^{
    base_ast_pwr,
    ast_pwst,
    ast_init_done,
    flash_power_down_h,
    flash_power_ready_h,
    ast_ram_1p_cfg,
    ast_rf_cfg,
    ast_rom_cfg,
    ast_ram_2p_fcfg,
    ast_ram_2p_lcfg,
    ast_pwst_h,
    ast2pinmux,
    scan_en,
    scan_rst_n,
    scanmode
  };

  //////////////////////////////////
  // AST - Custom for targets     //
  //////////////////////////////////

<%
  ast = [m for m in top["module"] if m["name"] == "ast"]
  assert(len(ast) == 1)
  ast = ast[0]
%>\

  assign ast_base_pwr.main_pok = ast_pwst.main_pok;

  logic [rstmgr_pkg::PowerDomains-1:0] por_n;
  assign por_n = {ast_pwst.main_pok, ast_pwst.aon_pok};

  assign ext_clk = '0;
  assign pad2ast = '0;

  // 200 MHz differential board clock.
  logic clk_200m;
  IBUFDS u_ibufds_clk (
    .I  (IO_CLK_P),
    .IB (IO_CLK_N),
    .O  (clk_200m)
  );

  // The reset button (POR_N) and the FT2232H reset (IO_FTDI_RST_N) both reset
  // the chip.
  logic clk_main, clk_usb_48mhz, clk_aon, rst_n;
  clkgen_xil7series # (
    .AddClkBuf(0)
  ) clkgen (
    .clk_i(clk_200m),
    .rst_ni(manual_in_por_n),
    .srst_ni(manual_in_io_ftdi_rst_n),
    .clk_main_o(clk_main),
    .clk_48MHz_o(clk_usb_48mhz),
    .clk_aon_o(clk_aon),
    .rst_no(rst_n)
  );

  logic [31:0] fpga_info;
  usr_access_xil7series u_info (
    .info_o(fpga_info)
  );

  ast_pkg::clks_osc_byp_t clks_osc_byp;
  assign clks_osc_byp = '{
    usb: clk_usb_48mhz,
    sys: clk_main,
    io:  clk_main,
    aon: clk_aon
  };

  ast u_ast (
    // external POR
    .por_ni                ( rst_n ),

    // USB IO Pull-up Calibration Setting
    .usb_io_pu_cal_o       ( ),

    // clocks' oscillator bypass for FPGA
    .clk_osc_byp_i         ( clks_osc_byp ),

    // adc
    .adc_a0_ai             ( '0 ),
    .adc_a1_ai             ( '0 ),

    // Direct short to PAD
    .ast2pad_t0_ao         (  ),
    .ast2pad_t1_ao         (  ),

    // clocks and resets supplied for detection
    .sns_clks_i            ( clkmgr_aon_clocks    ),
    .sns_rsts_i            ( rstmgr_aon_resets    ),
    .sns_spi_ext_clk_i     ( sck_monitor          ),
    // tlul
    .tl_i                  ( base_ast_bus ),
    .tl_o                  ( ast_base_bus ),
    // init done indication
    .ast_init_done_o       ( ast_init_done ),
    // buffered clocks & resets
    % for port, clk in ast["clock_connections"].items():
    .${port} (${clk}),
    % endfor
    % for port, reset in ast["reset_connections"].items():
    .${port} (${lib.get_reset_path(top, reset)}),
    % endfor
    .clk_ast_ext_i         ( ext_clk ),

    // pok test for FPGA
    .vcc_supp_i            ( 1'b1 ),
    .vcaon_supp_i          ( 1'b1 ),
    .vcmain_supp_i         ( 1'b1 ),
    .vioa_supp_i           ( 1'b1 ),
    .viob_supp_i           ( 1'b1 ),
    // pok
    .ast_pwst_o            ( ast_pwst ),
    .ast_pwst_h_o          ( ast_pwst_h ),
    // main regulator
    .main_env_iso_en_i     ( base_ast_pwr.pwr_clamp_env ),
    .main_pd_ni            ( base_ast_pwr.main_pd_n ),
    // pdm control (flash)/otp
    .flash_power_down_h_o  ( flash_power_down_h ),
    .flash_power_ready_h_o ( flash_power_ready_h ),
    .otp_power_seq_i       ( '0 ),
    .otp_power_seq_h_o     ( ),
    // system source clock
    .clk_src_sys_en_i      ( base_ast_pwr.core_clk_en ),
    // need to add function in clkmgr
    .clk_src_sys_jen_i     ( jen ),
    .clk_src_sys_o         ( ast_base_clks.clk_sys  ),
    .clk_src_sys_val_o     ( ast_base_pwr.core_clk_val ),
    // aon source clock
    .clk_src_aon_o         ( ast_base_clks.clk_aon ),
    .clk_src_aon_val_o     ( ast_base_pwr.slow_clk_val ),
    // io source clock
    .clk_src_io_en_i       ( base_ast_pwr.io_clk_en ),
    .clk_src_io_o          ( ast_base_clks.clk_io ),
    .clk_src_io_val_o      ( ast_base_pwr.io_clk_val ),
    .clk_src_io_48m_o      ( div_step_down_req ),
    // usb source clock
    .usb_ref_pulse_i       ( usb_ref_pulse ),
    .usb_ref_val_i         ( usb_ref_val ),
    .clk_src_usb_en_i      ( base_ast_pwr.usb_clk_en ),
    .clk_src_usb_o         ( ast_base_clks.clk_usb ),
    .clk_src_usb_val_o     ( ast_base_pwr.usb_clk_val ),
    // adc: Wren has no adc_ctrl
    .adc_pd_i              ( 1'b1 ),
    .adc_chnsel_i          ( '0 ),
    .adc_d_o               ( ),
    .adc_d_val_o           ( ),
    // rng: Wren has no entropy_src
    .rng_en_i              ( 1'b0 ),
    .rng_fips_i            ( 1'b0 ),
    .rng_val_o             ( ),
    .rng_b_o               ( ),
    // entropy
    .entropy_rsp_i         ( ast_edn_edn_rsp ),
    .entropy_req_o         ( ast_edn_edn_req ),
    // alerts: Wren has no sensor_ctrl
    .alert_rsp_i           ( '0 ),
    .alert_req_o           ( ),
    // dft
    .dft_strap_test_i      ( dft_strap_test   ),
    .lc_dft_en_i           ( dft_en           ),
    .fla_obs_i             ( fla_obs ),
    .otp_obs_i             ( '0 ),
    .otm_obs_i             ( '0 ),
    .usb_obs_i             ( 1'b0 ),
    .obs_ctrl_o            ( obs_ctrl ),
    // pinmux related
    .padmux2ast_i          ( pad2ast    ),
    .ast2padmux_o          ( ast2pinmux ),
    .mux_iob_sel_o         ( mux_iob_sel ),
    .ext_freq_is_96m_i     ( hi_speed_sel ),
    .all_clk_byp_req_i     ( all_clk_byp_req  ),
    .all_clk_byp_ack_o     ( all_clk_byp_ack  ),
    .io_clk_byp_req_i      ( io_clk_byp_req   ),
    .io_clk_byp_ack_o      ( io_clk_byp_ack   ),
    .flash_bist_en_o       ( flash_bist_enable ),
    // Memory configuration connections
    .dpram_rmf_o           ( ast_ram_2p_fcfg ),
    .dpram_rml_o           ( ast_ram_2p_lcfg ),
    .spram_rm_o            ( ast_ram_1p_cfg  ),
    .sprgf_rm_o            ( ast_rf_cfg      ),
    .sprom_rm_o            ( ast_rom_cfg     ),
    // scan
    .dft_scan_md_o         ( scanmode ),
    .scan_shift_en_o       ( scan_en ),
    .scan_reset_no         ( scan_rst_n )
  );

  //////////////////////
  // Top-level design //
  //////////////////////

  // Wren has no alert handler: its alerts go out of the top, and nothing
  // receives them here.
  prim_alert_pkg::alert_tx_t [NOutgoingAlertsWren-1:0] outgoing_alert_tx;
  prim_mubi_pkg::mubi4_t [NOutgoingLpgsWren-1:0] outgoing_lpg_cg_en, outgoing_lpg_rst_en;
  logic unused_outgoing_alerts;
  assign unused_outgoing_alerts = ^{outgoing_alert_tx, outgoing_lpg_cg_en, outgoing_lpg_rst_en};

  top_${top["name"]} #(
    .RomCtrlBootRomInitFile(BootRomInitFile),
    .RvCoreIbexRegFile(ibex_pkg::RegFileFPGA),
    .RvCoreIbexPipeLine(1),
    .SramCtrlMainInstrExec(1),
    .PinmuxAonTargetCfg(PinmuxTargetCfg)
  ) top_${top["name"]} (
    .por_n_i                      ( por_n                 ),
    .clk_main_i                   ( ast_base_clks.clk_sys ),
    .clk_io_i                     ( ast_base_clks.clk_io  ),
    .clk_usb_i                    ( ast_base_clks.clk_usb ),
    .clk_aon_i                    ( ast_base_clks.clk_aon ),
    .clks_ast_o                   ( clkmgr_aon_clocks     ),
    .clk_main_jitter_en_o         ( jen                   ),
    .rsts_ast_o                   ( rstmgr_aon_resets     ),
    .sck_monitor_o                ( sck_monitor           ),
    .pwrmgr_ast_req_o             ( base_ast_pwr          ),
    .pwrmgr_ast_rsp_i             ( ast_base_pwr          ),
    .usb_dp_pullup_en_o           ( usb_dp_pullup_en      ),
    .usb_dn_pullup_en_o           ( usb_dn_pullup_en      ),
    .usbdev_usb_rx_d_i            ( usb_rx_d              ),
    .usbdev_usb_tx_d_o            ( usb_tx_d              ),
    .usbdev_usb_tx_se0_o          ( usb_tx_se0            ),
    .usbdev_usb_tx_use_d_se0_o    ( usb_tx_use_d_se0      ),
    .usbdev_usb_rx_enable_o       ( usb_rx_enable         ),
    .usbdev_usb_ref_val_o         ( usb_ref_val           ),
    .usbdev_usb_ref_pulse_o       ( usb_ref_pulse         ),
    .ast_tl_req_o                 ( base_ast_bus          ),
    .ast_tl_rsp_i                 ( ast_base_bus          ),
    .ast_edn_req_i                ( ast_edn_edn_req       ),
    .ast_edn_rsp_o                ( ast_edn_edn_rsp       ),
    .ast_lc_dft_en_o              ( dft_en                ),
    .obs_ctrl_i                   ( obs_ctrl              ),
    .flash_bist_enable_i          ( flash_bist_enable     ),
    .flash_power_down_h_i         ( 1'b0                  ),
    .flash_power_ready_h_i        ( 1'b1                  ),
    .flash_obs_o                  ( fla_obs               ),
    .io_clk_byp_req_o             ( io_clk_byp_req        ),
    .io_clk_byp_ack_i             ( io_clk_byp_ack        ),
    .all_clk_byp_req_o            ( all_clk_byp_req       ),
    .all_clk_byp_ack_i            ( all_clk_byp_ack       ),
    .hi_speed_sel_o               ( hi_speed_sel          ),
    .div_step_down_req_i          ( div_step_down_req     ),
    .fpga_info_i                  ( fpga_info             ),
    .dft_strap_test_o             ( dft_strap_test        ),

    // Outgoing alerts
    .outgoing_alert_wren_tx_o     ( outgoing_alert_tx     ),
    .outgoing_alert_wren_rx_i     ( {NOutgoingAlertsWren{prim_alert_pkg::ALERT_RX_DEFAULT}} ),
    .outgoing_lpg_cg_en_wren_o    ( outgoing_lpg_cg_en    ),
    .outgoing_lpg_rst_en_wren_o   ( outgoing_lpg_rst_en   ),

    // Multiplexed I/O
    .mio_in_i        ( mio_in   ),
    .mio_out_o       ( mio_out  ),
    .mio_oe_o        ( mio_oe   ),

    // Dedicated I/O
    .dio_in_i        ( dio_in   ),
    .dio_out_o       ( dio_out  ),
    .dio_oe_o        ( dio_oe   ),

    // Pad attributes
    .mio_attr_o      ( mio_attr      ),
    .dio_attr_o      ( dio_attr      ),

    // Memory attributes
    .ram_1p_cfg_i    ( '0 ),
    .spi_ram_2p_cfg_i( '0 ),
    .usb_ram_1p_cfg_i( '0 ),
    .rom_cfg_i       ( '0 ),

    // DFT signals
    .dft_hold_tap_sel_i ( '0               ),
    .scan_rst_ni        ( 1'b1             ),
    .scan_en_i          ( 1'b0             ),
    .scanmode_i         ( prim_mubi_pkg::MuBi4False )
  );

endmodule : chip_${top["name"]}_${target["name"]}
