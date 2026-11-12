## Copyright lowRISC contributors (OpenTitan project).
## Copyright Mark Hayter.
## Licensed under the Apache License, Version 2.0, see LICENSE for details.
## SPDX-License-Identifier: Apache-2.0

## Clock Signal
create_clock -add -name sys_clk_pin -period 5.00 -waveform {0 2.5} [get_ports IO_CLK_P]

## Name the MMCM outputs and the AST's IO clock, as Egret's clocks.xdc does:
## Wren's chip level, AST and clkgen are built the same way. Without clk_io,
## Vivado times the AST's IO clock synchronizers as the main clock, with the
## skew of the AST's clock buffer between them.
create_generated_clock -name clk_main [get_pin clkgen/pll/CLKOUT0]
create_generated_clock -name clk_usb_48 [get_pin clkgen/pll/CLKOUT1]
create_generated_clock -name clk_aon [get_pin clkgen/pll/CLKOUT4]
set clk_io_pin [get_pin u_ast/u_ast_clks_byp/u_no_scan_clk_src_io_d1ord2/u_clk_div_buf/gen_fpga_buf.gen_bufg.bufg_i/O]
create_generated_clock -name clk_io -divide_by 1 -add \
    -master_clock [get_clocks clk_main] \
    -source [get_pins clkgen/pll/CLKOUT0] \
    ${clk_io_pin}

## Clock Domain Crossings
set clks_48_unbuf [get_clocks -of_objects [get_pin clkgen/pll/CLKOUT1]]

## Divided clock
## This is not really recommended per Vivado's guidelines, but hopefully these clocks are slow enough and their
## destination flops few enough.

set u_pll clkgen/pll
set u_div2 top_*/u_clkmgr_aon/u_no_scan_io_div2_div
create_generated_clock -name clk_io_div2 -divide_by 2 -source ${clk_io_pin} [get_pin ${u_div2}/gen_div2.u_div2/q_o[0]]

set u_div4 top_*/u_clkmgr_aon/u_no_scan_io_div4_div
create_generated_clock -name clk_io_div4 -divide_by 4 -source [get_pins ${u_div4}/gen_div.clk_int_reg/C] [get_pins ${u_div4}/gen_div.clk_int_reg/Q]


# The AST's IO clock divider/bypass mux is a LUT on the clock network: time
# only the rising edge through it.
set ast_src_io u_ast/u_ast_clks_byp/u_no_scan_clk_src_io_d1ord2
set_clock_sense -positive \
  [get_pins -filter {DIRECTION == OUT && IS_LEAF} -of_objects \
    [get_nets -segments -of_objects \
      [get_pins ${ast_src_io}/u_clk_div_buf/gen_fpga_buf.gen_bufg.bufg_i/I] \
    ] \
  ]

# the step-down mux is implemented with a LUT right now and the mux switches on the falling edge.
# therefore, Vivado propagates both clock edges down the clock network.
# this implementation is not ideal - but we can at least tell Vivado to only honour the rising edge for
# timing analysis.
set_clock_sense -positive \
  [get_pins -filter {DIRECTION == OUT && IS_LEAF} -of_objects \
    [get_nets -segments -of_objects \
      [get_pins top_*/u_clkmgr_aon/u_no_scan_io_div2_div/u_clk_div_buf/gen_fpga_buf.gen_bufg.bufg_i/I] \
    ] \
  ]

## SPI clocks
create_clock -add -name clk_spi  -period 100.00 -waveform {0 50} [get_ports SPI_DEV_CLK]
set_input_delay -clock clk_spi 5 [get_ports SPI_DEV_D0]
set_input_delay -clock clk_spi 5 [get_ports SPI_DEV_D1]
set_output_delay -clock clk_spi 5 [get_ports SPI_DEV_D0] -add_delay
set_output_delay -clock clk_spi 5 [get_ports SPI_DEV_D1] -add_delay

# set clock sense on the input to spi buffers to help the tool understand the clocks are shifted versions of each other
# This can also be accomplished through create_generated_clock.

## set clock sense approach
##set_clock_sense -negative \
##  [get_pins -filter {DIRECTION == OUT && IS_LEAF} -of_objects \
##    [get_nets -segments -of_objects \
##      [get_pins top_*/u_spi_device/gen_fpga_buf.gen_bufr.bufr_i_i_1/I] \
##    ] \
##  ] \
##  -clocks clk_spi
##
##set_clock_sense -positive \
##  [get_pins -filter {DIRECTION == OUT && IS_LEAF} -of_objects \
##    [get_nets -segments -of_objects \
##      [get_pins top_*/u_spi_device/gen_fpga_buf.gen_bufr.bufr_i_i_1__0/I] \
##    ] \
##  ] \
##  -clocks clk_spi

## create_generated_clock approach
## create_generated_clock is preferred since the buffer cell used here is hand-instantiated, while the set_clock_sense point is simply a LUT
create_generated_clock -name clk_spi_in  -divide_by 1 -source [get_ports SPI_DEV_CLK] [get_pins top_*/u_spi_device/u_clk_spi_in_buf/gen_fpga_buf.gen_bufr.bufr_i/O]
create_generated_clock -name clk_spi_out -divide_by 1 -source [get_ports SPI_DEV_CLK] [get_pins top_*/u_spi_device/u_clk_spi_out_buf/gen_fpga_buf.gen_bufr.bufr_i/O] -invert

## JTAG clocks and I/O delays
# rv_dm JTAG is on IOR0-4 (Pmod JD), as in Egret's CW310; this section is copied
# from Egret's clocks.xdc. TCK is IOR3, TMS IOR0, TDI IOR2, TDO IOR1.
create_clock -add -name jtag_tck -period 100.00 -waveform {0 50} [get_ports IOR3]
create_generated_clock -name lc_jtag_tck -source [get_ports IOR3] -divide_by 1 [get_pins top_*/u_pinmux_aon/u_pinmux_strap_sampling/u_pinmux_jtag_buf_lc/prim_clock_buf_tck/gen_fpga_buf.gen_bufg.bufg_i/O]
create_generated_clock -name rv_jtag_tck -source [get_ports IOR3] -divide_by 1 [get_pins top_*/u_pinmux_aon/u_pinmux_strap_sampling/u_pinmux_jtag_buf_rv/prim_clock_buf_tck/gen_fpga_buf.gen_bufg.bufg_i/O]

set lc_jtag_tck_inv_pin  \
  [get_pins -filter {DIRECTION == OUT && IS_LEAF} -of_objects \
    [get_nets -segments -of_objects \
      [get_pins top_wren/u_pinmux_aon/u_pinmux_strap_sampling/u_pinmux_jtag_buf_lc/prim_clock_buf_tck/gen_fpga_buf.gen_bufg.bufg_i/I]]]
set rv_jtag_tck_inv_pin  \
  [get_pins -filter {DIRECTION == OUT && IS_LEAF} -of_objects \
    [get_nets -segments -of_objects \
      [get_pins top_wren/u_pinmux_aon/u_pinmux_strap_sampling/u_pinmux_jtag_buf_rv/prim_clock_buf_tck/gen_fpga_buf.gen_bufg.bufg_i/I]]]

set_clock_sense -negative ${lc_jtag_tck_inv_pin}
set_clock_sense -negative ${rv_jtag_tck_inv_pin}

# Input and output delays: TDO changes on the falling edge of TCK, and TMS/TDI
# are sampled on the rising edge, so half a TCK cycle is available.
set_output_delay -add_delay -clock_fall -clock jtag_tck -max 10.0 [get_ports IOR1]
set_output_delay -add_delay -clock_fall -clock jtag_tck -min -5.0 [get_ports IOR1]
set_input_delay  -add_delay -clock_fall -clock jtag_tck -min  0.0 [get_ports {IOR0 IOR2}]
set_input_delay  -add_delay -clock_fall -clock jtag_tck -max 15.0 [get_ports {IOR0 IOR2}]

## Set asynchronous clock groups
set_clock_groups -group clk_main -group clk_usb_48 -group clk_aon -group clk_io -group clk_io_div2 -group clk_io_div4 -group [get_clocks -include_generated_clocks jtag_tck] -group {clk_spi clk_spi_in clk_spi_out} -group sys_clk_pin -asynchronous

## The usb calibration handling inside ast is assumed to be async to the outside world
## even though its interface is also a usb clock.
set_false_path -from ${clks_48_unbuf} -to [get_pins u_ast/u_usb_clk/u_ref_pulse_sync/u_sync*/u_sync_1/q_o_reg[0]/D]
