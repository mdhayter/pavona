// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
//
// ------------------- W A R N I N G: A U T O - G E N E R A T E D   C O D E !! -------------------//
// PLEASE DO NOT HAND-EDIT THIS FILE. IT HAS BEEN AUTO-GENERATED WITH THE FOLLOWING COMMAND:
//
// util/gen_top_sv.py --completecfg hw/top_wren/data/autogen/top_wren.gen.hjson
//                    --seedcfg hw/top_wren/data/autogen/top_wren.secrets.testing.gen.hjson


module chip_wren_genesys2 #(
  // Path to a VMEM file containing the contents of the boot ROM, which will be
  // baked into the FPGA bitstream.
  parameter BootRomInitFile = ""
) (
  // 200 MHz differential system clock. It goes straight to an IBUFDS, not
  // through the padring.
  input IO_CLK_P,
  input IO_CLK_N,

  // Dedicated Pads
  inout POR_N, // Manual Pad
  inout USB_P, // Manual Pad
  inout USB_N, // Manual Pad
  inout SPI_DEV_D0, // Dedicated Pad for spi_device_sd
  inout SPI_DEV_D1, // Dedicated Pad for spi_device_sd
  inout SPI_DEV_CLK, // Dedicated Pad for spi_device_sck
  inout SPI_DEV_CS_L, // Dedicated Pad for spi_device_csb
  inout IO_FTDI_RST_N, // Manual Pad
  inout IO_SPIEN, // Manual Pad
  inout IO_UART1_CTS_N, // Manual Pad
  inout IO_USB_DPPULLUP0, // Manual Pad

  // Muxed Pads
  inout IOA0, // MIO Pad 0
  inout IOA1, // MIO Pad 1
  inout IOA2, // MIO Pad 2
  inout IOA3, // MIO Pad 3
  inout IOA4, // MIO Pad 4
  inout IOA5, // MIO Pad 5
  inout IOA6, // MIO Pad 6
  inout IOA7, // MIO Pad 7
  inout IOA8, // MIO Pad 8
  inout IOB0, // MIO Pad 9
  inout IOB1, // MIO Pad 10
  inout IOB2, // MIO Pad 11
  inout IOB3, // MIO Pad 12
  inout IOB4, // MIO Pad 13
  inout IOB5, // MIO Pad 14
  inout IOB6, // MIO Pad 15
  inout IOB7, // MIO Pad 16
  inout IOB8, // MIO Pad 17
  inout IOB9, // MIO Pad 18
  inout IOB10, // MIO Pad 19
  inout IOB11, // MIO Pad 20
  inout IOB12, // MIO Pad 21
  inout IOC0, // MIO Pad 22
  inout IOC1, // MIO Pad 23
  inout IOC2, // MIO Pad 24
  inout IOC3, // MIO Pad 25
  inout IOC4, // MIO Pad 26
  inout IOC6, // MIO Pad 28
  inout IOC7, // MIO Pad 29
  inout IOC9, // MIO Pad 31
  inout IOC10, // MIO Pad 32
  inout IOC11, // MIO Pad 33
  inout IOC12, // MIO Pad 34
  inout IOR0, // MIO Pad 35
  inout IOR1, // MIO Pad 36
  inout IOR2, // MIO Pad 37
  inout IOR3, // MIO Pad 38
  inout IOR4, // MIO Pad 39
  inout IOR6, // MIO Pad 41
  inout IOR10, // MIO Pad 43
  inout IOR11, // MIO Pad 44
  inout IOR12, // MIO Pad 45
  inout IOR13  // MIO Pad 46
);

  import top_wren_pkg::*;
  import prim_pad_wrapper_pkg::*;

  ////////////////////////////
  // Special Signal Indices //
  ////////////////////////////

  localparam int Tap0PadIdx = 30;
  localparam int Tap1PadIdx = 27;
  localparam int Dft0PadIdx = 40;
  localparam int Dft1PadIdx = 42;
  localparam int TckPadIdx = 38;
  localparam int TmsPadIdx = 35;
  localparam int TrstNPadIdx = 39;
  localparam int TdiPadIdx = 37;
  localparam int TdoPadIdx = 36;

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
      BidirStd, // DIO spi_host0_csb
      BidirStd, // DIO spi_host0_sck
      InputStd, // DIO spi_device_csb
      InputStd, // DIO spi_device_sck
      BidirStd, // DIO usbdev_usb_dn
      BidirStd, // DIO usbdev_usb_dp
      BidirStd, // DIO spi_device_sd
      BidirStd, // DIO spi_device_sd
      BidirStd, // DIO spi_device_sd
      BidirStd, // DIO spi_device_sd
      BidirStd, // DIO spi_host0_sd
      BidirStd, // DIO spi_host0_sd
      BidirStd, // DIO spi_host0_sd
      BidirStd  // DIO spi_host0_sd
    },
    mio_pad_type: {
      BidirOd, // MIO Pad 46
      BidirOd, // MIO Pad 45
      BidirOd, // MIO Pad 44
      BidirOd, // MIO Pad 43
      BidirStd, // MIO Pad 42
      BidirStd, // MIO Pad 41
      BidirStd, // MIO Pad 40
      BidirStd, // MIO Pad 39
      BidirStd, // MIO Pad 38
      BidirStd, // MIO Pad 37
      BidirStd, // MIO Pad 36
      BidirStd, // MIO Pad 35
      BidirOd, // MIO Pad 34
      BidirOd, // MIO Pad 33
      BidirOd, // MIO Pad 32
      BidirStd, // MIO Pad 31
      BidirStd, // MIO Pad 30
      BidirStd, // MIO Pad 29
      BidirStd, // MIO Pad 28
      BidirStd, // MIO Pad 27
      BidirStd, // MIO Pad 26
      BidirStd, // MIO Pad 25
      BidirStd, // MIO Pad 24
      BidirStd, // MIO Pad 23
      BidirStd, // MIO Pad 22
      BidirOd, // MIO Pad 21
      BidirOd, // MIO Pad 20
      BidirOd, // MIO Pad 19
      BidirOd, // MIO Pad 18
      BidirStd, // MIO Pad 17
      BidirStd, // MIO Pad 16
      BidirStd, // MIO Pad 15
      BidirStd, // MIO Pad 14
      BidirStd, // MIO Pad 13
      BidirStd, // MIO Pad 12
      BidirStd, // MIO Pad 11
      BidirStd, // MIO Pad 10
      BidirStd, // MIO Pad 9
      BidirOd, // MIO Pad 8
      BidirOd, // MIO Pad 7
      BidirOd, // MIO Pad 6
      BidirStd, // MIO Pad 5
      BidirStd, // MIO Pad 4
      BidirStd, // MIO Pad 3
      BidirStd, // MIO Pad 2
      BidirStd, // MIO Pad 1
      BidirStd  // MIO Pad 0
    },
    // Pad scan roles: there is no scan on the FPGA.
    dio_scan_role: {pinmux_reg_pkg::NDioPads{NoScan}},
    mio_scan_role: {pinmux_reg_pkg::NMioPads{NoScan}}
  };

  ////////////////////////
  // Signal definitions //
  ////////////////////////


  logic [3:0] mux_iob_sel;

  pad_attr_t [pinmux_reg_pkg::NMioPads-1:0] mio_attr;
  pad_attr_t [pinmux_reg_pkg::NDioPads-1:0] dio_attr;
  logic [pinmux_reg_pkg::NMioPads-1:0] mio_out;
  logic [pinmux_reg_pkg::NMioPads-1:0] mio_oe;
  logic [pinmux_reg_pkg::NMioPads-1:0] mio_in;
  logic [pinmux_reg_pkg::NMioPads-1:0] mio_in_raw;
  logic [11-1:0]                       dio_in_raw;
  logic [pinmux_reg_pkg::NDioPads-1:0] dio_out;
  logic [pinmux_reg_pkg::NDioPads-1:0] dio_oe;
  logic [pinmux_reg_pkg::NDioPads-1:0] dio_in;

  // Inputs from the padring's muxed pads, in padring order.
  logic [43-1:0] mio_in_pad;
  logic [43-1:0] mio_in_raw_pad;

  logic unused_mio_in_raw;
  logic unused_dio_in_raw;
  assign unused_mio_in_raw = ^mio_in_raw;
  assign unused_dio_in_raw = ^dio_in_raw;

  // Manual pads
  logic manual_in_por_n, manual_out_por_n, manual_oe_por_n;
  logic manual_in_usb_p, manual_out_usb_p, manual_oe_usb_p;
  logic manual_in_usb_n, manual_out_usb_n, manual_oe_usb_n;
  logic manual_in_io_ftdi_rst_n, manual_out_io_ftdi_rst_n, manual_oe_io_ftdi_rst_n;
  logic manual_in_io_spien, manual_out_io_spien, manual_oe_io_spien;
  logic manual_in_io_uart1_cts_n, manual_out_io_uart1_cts_n, manual_oe_io_uart1_cts_n;
  logic manual_in_io_usb_dppullup0, manual_out_io_usb_dppullup0, manual_oe_io_usb_dppullup0;

  pad_attr_t manual_attr_por_n;
  pad_attr_t manual_attr_usb_p;
  pad_attr_t manual_attr_usb_n;
  pad_attr_t manual_attr_io_ftdi_rst_n;
  pad_attr_t manual_attr_io_spien;
  pad_attr_t manual_attr_io_uart1_cts_n;
  pad_attr_t manual_attr_io_usb_dppullup0;

  ////////////////////////////////
  // Muxed pad inputs and straps //
  ////////////////////////////////

  assign mio_in[0] = mio_in_pad[0];
  assign mio_in_raw[0] = mio_in_raw_pad[0];
  assign mio_in[1] = mio_in_pad[1];
  assign mio_in_raw[1] = mio_in_raw_pad[1];
  assign mio_in[2] = mio_in_pad[2];
  assign mio_in_raw[2] = mio_in_raw_pad[2];
  assign mio_in[3] = mio_in_pad[3];
  assign mio_in_raw[3] = mio_in_raw_pad[3];
  assign mio_in[4] = mio_in_pad[4];
  assign mio_in_raw[4] = mio_in_raw_pad[4];
  assign mio_in[5] = mio_in_pad[5];
  assign mio_in_raw[5] = mio_in_raw_pad[5];
  assign mio_in[6] = mio_in_pad[6];
  assign mio_in_raw[6] = mio_in_raw_pad[6];
  assign mio_in[7] = mio_in_pad[7];
  assign mio_in_raw[7] = mio_in_raw_pad[7];
  assign mio_in[8] = mio_in_pad[8];
  assign mio_in_raw[8] = mio_in_raw_pad[8];
  assign mio_in[9] = mio_in_pad[9];
  assign mio_in_raw[9] = mio_in_raw_pad[9];
  assign mio_in[10] = mio_in_pad[10];
  assign mio_in_raw[10] = mio_in_raw_pad[10];
  assign mio_in[11] = mio_in_pad[11];
  assign mio_in_raw[11] = mio_in_raw_pad[11];
  assign mio_in[12] = mio_in_pad[12];
  assign mio_in_raw[12] = mio_in_raw_pad[12];
  assign mio_in[13] = mio_in_pad[13];
  assign mio_in_raw[13] = mio_in_raw_pad[13];
  assign mio_in[14] = mio_in_pad[14];
  assign mio_in_raw[14] = mio_in_raw_pad[14];
  assign mio_in[15] = mio_in_pad[15];
  assign mio_in_raw[15] = mio_in_raw_pad[15];
  assign mio_in[16] = mio_in_pad[16];
  assign mio_in_raw[16] = mio_in_raw_pad[16];
  assign mio_in[17] = mio_in_pad[17];
  assign mio_in_raw[17] = mio_in_raw_pad[17];
  assign mio_in[18] = mio_in_pad[18];
  assign mio_in_raw[18] = mio_in_raw_pad[18];
  assign mio_in[19] = mio_in_pad[19];
  assign mio_in_raw[19] = mio_in_raw_pad[19];
  assign mio_in[20] = mio_in_pad[20];
  assign mio_in_raw[20] = mio_in_raw_pad[20];
  assign mio_in[21] = mio_in_pad[21];
  assign mio_in_raw[21] = mio_in_raw_pad[21];
  // IOC0 is a boot strap, active low at the board.
  assign mio_in[22] = ~mio_in_pad[22];
  assign mio_in_raw[22] = ~mio_in_raw_pad[22];
  // IOC1 is a boot strap, active low at the board.
  assign mio_in[23] = ~mio_in_pad[23];
  assign mio_in_raw[23] = ~mio_in_raw_pad[23];
  // IOC2 is a boot strap, active low at the board.
  assign mio_in[24] = ~mio_in_pad[24];
  assign mio_in_raw[24] = ~mio_in_raw_pad[24];
  assign mio_in[25] = mio_in_pad[25];
  assign mio_in_raw[25] = mio_in_raw_pad[25];
  assign mio_in[26] = mio_in_pad[26];
  assign mio_in_raw[26] = mio_in_raw_pad[26];
  assign mio_in[28] = mio_in_pad[27];
  assign mio_in_raw[28] = mio_in_raw_pad[27];
  assign mio_in[29] = mio_in_pad[28];
  assign mio_in_raw[29] = mio_in_raw_pad[28];
  assign mio_in[31] = mio_in_pad[29];
  assign mio_in_raw[31] = mio_in_raw_pad[29];
  assign mio_in[32] = mio_in_pad[30];
  assign mio_in_raw[32] = mio_in_raw_pad[30];
  assign mio_in[33] = mio_in_pad[31];
  assign mio_in_raw[33] = mio_in_raw_pad[31];
  assign mio_in[34] = mio_in_pad[32];
  assign mio_in_raw[34] = mio_in_raw_pad[32];
  assign mio_in[35] = mio_in_pad[33];
  assign mio_in_raw[35] = mio_in_raw_pad[33];
  assign mio_in[36] = mio_in_pad[34];
  assign mio_in_raw[36] = mio_in_raw_pad[34];
  assign mio_in[37] = mio_in_pad[35];
  assign mio_in_raw[37] = mio_in_raw_pad[35];
  assign mio_in[38] = mio_in_pad[36];
  assign mio_in_raw[38] = mio_in_raw_pad[36];
  assign mio_in[39] = mio_in_pad[37];
  assign mio_in_raw[39] = mio_in_raw_pad[37];
  assign mio_in[41] = mio_in_pad[38];
  assign mio_in_raw[41] = mio_in_raw_pad[38];
  assign mio_in[43] = mio_in_pad[39];
  assign mio_in_raw[43] = mio_in_raw_pad[39];
  assign mio_in[44] = mio_in_pad[40];
  assign mio_in_raw[44] = mio_in_raw_pad[40];
  assign mio_in[45] = mio_in_pad[41];
  assign mio_in_raw[45] = mio_in_raw_pad[41];
  assign mio_in[46] = mio_in_pad[42];
  assign mio_in_raw[46] = mio_in_raw_pad[42];

  /////////////////////////
  // Stubbed pad tie-off //
  /////////////////////////

  // Only signals going to non-custom pads need to be tied off.
  logic [66:0] unused_sig;
  assign dio_in[DioSpiHost0Sd0] = 1'b0;
  assign unused_sig[8] = dio_out[DioSpiHost0Sd0] ^ dio_oe[DioSpiHost0Sd0] ^
                                    (^dio_attr[DioSpiHost0Sd0]);
  assign dio_in[DioSpiHost0Sd1] = 1'b0;
  assign unused_sig[9] = dio_out[DioSpiHost0Sd1] ^ dio_oe[DioSpiHost0Sd1] ^
                                    (^dio_attr[DioSpiHost0Sd1]);
  assign dio_in[DioSpiHost0Sd2] = 1'b0;
  assign unused_sig[10] = dio_out[DioSpiHost0Sd2] ^ dio_oe[DioSpiHost0Sd2] ^
                                    (^dio_attr[DioSpiHost0Sd2]);
  assign dio_in[DioSpiHost0Sd3] = 1'b0;
  assign unused_sig[11] = dio_out[DioSpiHost0Sd3] ^ dio_oe[DioSpiHost0Sd3] ^
                                    (^dio_attr[DioSpiHost0Sd3]);
  assign dio_in[DioSpiHost0Sck] = 1'b0;
  assign unused_sig[12] = dio_out[DioSpiHost0Sck] ^ dio_oe[DioSpiHost0Sck] ^
                                    (^dio_attr[DioSpiHost0Sck]);
  assign dio_in[DioSpiHost0Csb] = 1'b0;
  assign unused_sig[13] = dio_out[DioSpiHost0Csb] ^ dio_oe[DioSpiHost0Csb] ^
                                    (^dio_attr[DioSpiHost0Csb]);
  assign dio_in[DioSpiDeviceSd2] = 1'b0;
  assign unused_sig[16] = dio_out[DioSpiDeviceSd2] ^ dio_oe[DioSpiDeviceSd2] ^
                                    (^dio_attr[DioSpiDeviceSd2]);
  assign dio_in[DioSpiDeviceSd3] = 1'b0;
  assign unused_sig[17] = dio_out[DioSpiDeviceSd3] ^ dio_oe[DioSpiDeviceSd3] ^
                                    (^dio_attr[DioSpiDeviceSd3]);
  // TAP strap 1 reads 1 and TAP strap 0 reads 0, which selects the rv_dm TAP.
  assign mio_in[27] = 1'b1;
  assign mio_in_raw[27] = 1'b0;
  assign unused_sig[47] = mio_out[27] ^ mio_oe[27] ^
                                    (^mio_attr[27]);
  assign mio_in[30] = 1'b0;
  assign mio_in_raw[30] = 1'b0;
  assign unused_sig[50] = mio_out[30] ^ mio_oe[30] ^
                                    (^mio_attr[30]);
  assign mio_in[40] = 1'b0;
  assign mio_in_raw[40] = 1'b0;
  assign unused_sig[60] = mio_out[40] ^ mio_oe[40] ^
                                    (^mio_attr[40]);
  assign mio_in[42] = 1'b0;
  assign mio_in_raw[42] = 1'b0;
  assign unused_sig[62] = mio_out[42] ^ mio_oe[42] ^
                                    (^mio_attr[42]);

  //////////////////////
  // Padring Instance //
  //////////////////////

  ast_pkg::ast_clks_t ast_base_clks;

  padring #(
    // Padring specific counts may differ from pinmux config due
    // to custom, stubbed or added pads.
    .NDioPads(11),
    .NMioPads(43),
    .DioPadType ({
      BidirStd, // IO_USB_DPPULLUP0
      BidirStd, // IO_UART1_CTS_N
      BidirStd, // IO_SPIEN
      InputStd, // IO_FTDI_RST_N
      InputStd, // SPI_DEV_CS_L
      InputStd, // SPI_DEV_CLK
      BidirStd, // SPI_DEV_D1
      BidirStd, // SPI_DEV_D0
      BidirTol, // USB_N
      BidirTol, // USB_P
      InputStd  // POR_N
    }),
    .MioPadType ({
      BidirOd, // IOR13
      BidirOd, // IOR12
      BidirOd, // IOR11
      BidirOd, // IOR10
      BidirStd, // IOR6
      BidirStd, // IOR4
      BidirStd, // IOR3
      BidirStd, // IOR2
      BidirStd, // IOR1
      BidirStd, // IOR0
      BidirOd, // IOC12
      BidirOd, // IOC11
      BidirOd, // IOC10
      BidirStd, // IOC9
      BidirStd, // IOC7
      BidirStd, // IOC6
      BidirStd, // IOC4
      BidirStd, // IOC3
      BidirStd, // IOC2
      BidirStd, // IOC1
      BidirStd, // IOC0
      BidirOd, // IOB12
      BidirOd, // IOB11
      BidirOd, // IOB10
      BidirOd, // IOB9
      BidirStd, // IOB8
      BidirStd, // IOB7
      BidirStd, // IOB6
      BidirStd, // IOB5
      BidirStd, // IOB4
      BidirStd, // IOB3
      BidirStd, // IOB2
      BidirStd, // IOB1
      BidirStd, // IOB0
      BidirOd, // IOA8
      BidirOd, // IOA7
      BidirOd, // IOA6
      BidirStd, // IOA5
      BidirStd, // IOA4
      BidirStd, // IOA3
      BidirStd, // IOA2
      BidirStd, // IOA1
      BidirStd  // IOA0
    })
  ) u_padring (
  // This is only used for scan and DFT purposes
    .clk_scan_i   ( 1'b0                  ),
    .scanmode_i   ( prim_mubi_pkg::MuBi4False ),
    .mux_iob_sel_i ( mux_iob_sel ),
    .dio_in_raw_o ( dio_in_raw ),
    // Chip IOs
    .dio_pad_io ({
      IO_USB_DPPULLUP0,
      IO_UART1_CTS_N,
      IO_SPIEN,
      IO_FTDI_RST_N,
      SPI_DEV_CS_L,
      SPI_DEV_CLK,
      SPI_DEV_D1,
      SPI_DEV_D0,
      USB_N,
      USB_P,
      POR_N
    }),

    .mio_pad_io ({
      IOR13,
      IOR12,
      IOR11,
      IOR10,
      IOR6,
      IOR4,
      IOR3,
      IOR2,
      IOR1,
      IOR0,
      IOC12,
      IOC11,
      IOC10,
      IOC9,
      IOC7,
      IOC6,
      IOC4,
      IOC3,
      IOC2,
      IOC1,
      IOC0,
      IOB12,
      IOB11,
      IOB10,
      IOB9,
      IOB8,
      IOB7,
      IOB6,
      IOB5,
      IOB4,
      IOB3,
      IOB2,
      IOB1,
      IOB0,
      IOA8,
      IOA7,
      IOA6,
      IOA5,
      IOA4,
      IOA3,
      IOA2,
      IOA1,
      IOA0
    }),

    // Core-facing
    .dio_in_o ({
        manual_in_io_usb_dppullup0,
        manual_in_io_uart1_cts_n,
        manual_in_io_spien,
        manual_in_io_ftdi_rst_n,
        dio_in[DioSpiDeviceCsb],
        dio_in[DioSpiDeviceSck],
        dio_in[DioSpiDeviceSd1],
        dio_in[DioSpiDeviceSd0],
        manual_in_usb_n,
        manual_in_usb_p,
        manual_in_por_n
      }),
    .dio_out_i ({
        manual_out_io_usb_dppullup0,
        manual_out_io_uart1_cts_n,
        manual_out_io_spien,
        manual_out_io_ftdi_rst_n,
        dio_out[DioSpiDeviceCsb],
        dio_out[DioSpiDeviceSck],
        dio_out[DioSpiDeviceSd1],
        dio_out[DioSpiDeviceSd0],
        manual_out_usb_n,
        manual_out_usb_p,
        manual_out_por_n
      }),
    .dio_oe_i ({
        manual_oe_io_usb_dppullup0,
        manual_oe_io_uart1_cts_n,
        manual_oe_io_spien,
        manual_oe_io_ftdi_rst_n,
        dio_oe[DioSpiDeviceCsb],
        dio_oe[DioSpiDeviceSck],
        dio_oe[DioSpiDeviceSd1],
        dio_oe[DioSpiDeviceSd0],
        manual_oe_usb_n,
        manual_oe_usb_p,
        manual_oe_por_n
      }),
    .dio_attr_i ({
        manual_attr_io_usb_dppullup0,
        manual_attr_io_uart1_cts_n,
        manual_attr_io_spien,
        manual_attr_io_ftdi_rst_n,
        dio_attr[DioSpiDeviceCsb],
        dio_attr[DioSpiDeviceSck],
        dio_attr[DioSpiDeviceSd1],
        dio_attr[DioSpiDeviceSd0],
        manual_attr_usb_n,
        manual_attr_usb_p,
        manual_attr_por_n
      }),

    .mio_in_o (mio_in_pad),
    .mio_in_raw_o (mio_in_raw_pad),
    .mio_out_i ({
        mio_out[46:43],
        mio_out[41],
        mio_out[39:31],
        mio_out[29:28],
        mio_out[26:0]
      }),
    .mio_oe_i ({
        mio_oe[46:43],
        mio_oe[41],
        mio_oe[39:31],
        mio_oe[29:28],
        mio_oe[26:0]
      }),
    .mio_attr_i ({
        mio_attr[46:43],
        mio_attr[41],
        mio_attr[39:31],
        mio_attr[29:28],
        mio_attr[26:0]
      })
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
    .clk_ast_tlul_i (clkmgr_aon_clocks.clk_io_div4_secure),
    .clk_ast_adc_i (clkmgr_aon_clocks.clk_aon_secure),
    .clk_ast_alert_i (clkmgr_aon_clocks.clk_io_div4_secure),
    .clk_ast_es_i (clkmgr_aon_clocks.clk_main_secure),
    .clk_ast_rng_i (clkmgr_aon_clocks.clk_main_secure),
    .clk_ast_usb_i (clkmgr_aon_clocks.clk_usb_peri),
    .rst_ast_tlul_ni (rstmgr_aon_resets.rst_lc_io_div4_n[rstmgr_pkg::Domain0Sel]),
    .rst_ast_adc_ni (rstmgr_aon_resets.rst_sys_aon_n[rstmgr_pkg::Domain0Sel]),
    .rst_ast_alert_ni (rstmgr_aon_resets.rst_lc_io_div4_n[rstmgr_pkg::Domain0Sel]),
    .rst_ast_es_ni (rstmgr_aon_resets.rst_sys_n[rstmgr_pkg::Domain0Sel]),
    .rst_ast_rng_ni (rstmgr_aon_resets.rst_sys_n[rstmgr_pkg::Domain0Sel]),
    .rst_ast_usb_ni (rstmgr_aon_resets.rst_usb_n[rstmgr_pkg::Domain0Sel]),
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

  top_wren #(
    .RomCtrlBootRomInitFile(BootRomInitFile),
    .RvCoreIbexRegFile(ibex_pkg::RegFileFPGA),
    .RvCoreIbexPipeLine(1),
    .SramCtrlMainInstrExec(1),
    .PinmuxAonTargetCfg(PinmuxTargetCfg)
  ) top_wren (
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

endmodule : chip_wren_genesys2
