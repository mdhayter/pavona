## Copyright lowRISC and contributors (COSMIC project).
## Copyright Mark Hayter.
## Licensed under the Apache License, Version 2.0, see LICENSE for details.
## SPDX-License-Identifier: Apache-2.0

## Pins for chip_wren_genesys2 on the Digilent Genesys 2.
## Copied (and modified) from Mocha's hw/top_chip/data/pins_genesys2.xdc
## (github.com/lowRISC/mocha); the other pins are from Digilent's Genesys 2
## master XDC and schematic.
##
## Pad pulls set by software do nothing on the FPGA: the pulls here are fixed.

## Clock: 200 MHz LVDS
set_property -dict { PACKAGE_PIN AD12  IOSTANDARD LVDS } [get_ports { IO_CLK_P }];
set_property -dict { PACKAGE_PIN AD11  IOSTANDARD LVDS } [get_ports { IO_CLK_N }];

## Resets
# CPU reset button, active low.
set_property -dict { PACKAGE_PIN R19   IOSTANDARD LVCMOS33 } [get_ports { POR_N }];
# FT2232H channel A ADBUS7 (PROG_D7, RI#): an FTDI input in UART mode, so pull it up.
set_property -dict { PACKAGE_PIN AA26  IOSTANDARD LVCMOS33 PULLTYPE PULLUP } [get_ports { IO_FTDI_RST_N }];

## LEDs (GPIO 0-7)
set_property -dict { PACKAGE_PIN T28   IOSTANDARD LVCMOS33 } [get_ports { IOB0 }]; # LED0
set_property -dict { PACKAGE_PIN V19   IOSTANDARD LVCMOS33 } [get_ports { IOB1 }]; # LED1
set_property -dict { PACKAGE_PIN U30   IOSTANDARD LVCMOS33 } [get_ports { IOB2 }]; # LED2
set_property -dict { PACKAGE_PIN U29   IOSTANDARD LVCMOS33 } [get_ports { IOB3 }]; # LED3
set_property -dict { PACKAGE_PIN V20   IOSTANDARD LVCMOS33 } [get_ports { IOB6 }]; # LED4
set_property -dict { PACKAGE_PIN V26   IOSTANDARD LVCMOS33 } [get_ports { IOB7 }]; # LED5
set_property -dict { PACKAGE_PIN W24   IOSTANDARD LVCMOS33 } [get_ports { IOB8 }]; # LED6
set_property -dict { PACKAGE_PIN W23   IOSTANDARD LVCMOS33 } [get_ports { IOC9 }]; # LED7

## Switches (GPIO 8-15). SW0-5 are on VADJ banks, set to 1.8 V by JP6.
set_property -dict { PACKAGE_PIN G19   IOSTANDARD LVCMOS18 } [get_ports { IOR10 }]; # SW0
set_property -dict { PACKAGE_PIN G25   IOSTANDARD LVCMOS18 } [get_ports { IOR11 }]; # SW1
set_property -dict { PACKAGE_PIN H24   IOSTANDARD LVCMOS18 } [get_ports { IOR12 }]; # SW2
set_property -dict { PACKAGE_PIN K19   IOSTANDARD LVCMOS18 } [get_ports { IOR13 }]; # SW3
set_property -dict { PACKAGE_PIN N19   IOSTANDARD LVCMOS18 } [get_ports { IOC10 }]; # SW4
set_property -dict { PACKAGE_PIN P19   IOSTANDARD LVCMOS18 } [get_ports { IOC11 }]; # SW5
set_property -dict { PACKAGE_PIN P26   IOSTANDARD LVCMOS33 } [get_ports { IOC12 }]; # SW6
set_property -dict { PACKAGE_PIN P27   IOSTANDARD LVCMOS33 } [get_ports { IOA6 }];  # SW7

## Buttons (GPIO 16-20), active high with board pull-downs, on VADJ (1.8 V).
set_property -dict { PACKAGE_PIN E18   IOSTANDARD LVCMOS18 } [get_ports { IOA7 }];  # BTNC
set_property -dict { PACKAGE_PIN B19   IOSTANDARD LVCMOS18 } [get_ports { IOA8 }];  # BTNU
set_property -dict { PACKAGE_PIN M19   IOSTANDARD LVCMOS18 } [get_ports { IOB9 }];  # BTND
set_property -dict { PACKAGE_PIN M20   IOSTANDARD LVCMOS18 } [get_ports { IOB10 }]; # BTNL
set_property -dict { PACKAGE_PIN C19   IOSTANDARD LVCMOS18 } [get_ports { IOC6 }];  # BTNR

## Spare GPIO 21 on Pmod JA7
set_property -dict { PACKAGE_PIN T22   IOSTANDARD LVCMOS33 } [get_ports { IOR6 }];  # JA7

## Boot straps (GPIO 22-24) on FT2232H channel A ADBUS4-6 (PROG_D4-D6).
## Active low at the board and inverted in the chip level. Pulled up, since
## ADBUS5-6 are FTDI inputs in UART mode.
set_property -dict { PACKAGE_PIN Y29   IOSTANDARD LVCMOS33 PULLTYPE PULLUP } [get_ports { IOC0 }];
set_property -dict { PACKAGE_PIN Y28   IOSTANDARD LVCMOS33 PULLTYPE PULLUP } [get_ports { IOC1 }];
set_property -dict { PACKAGE_PIN AA28  IOSTANDARD LVCMOS33 PULLTYPE PULLUP } [get_ports { IOC2 }];

## OLED (SSD1306): control lines on GPIO 25-28, SPI on spi_host1.
## No pulls: the board's pull-ups keep VDD and VBAT off while the pads float.
set_property -dict { PACKAGE_PIN AC17  IOSTANDARD LVCMOS18 } [get_ports { IOA0 }]; # OLED_DC
set_property -dict { PACKAGE_PIN AB17  IOSTANDARD LVCMOS18 } [get_ports { IOA1 }]; # OLED_RES
set_property -dict { PACKAGE_PIN AG17  IOSTANDARD LVCMOS18 } [get_ports { IOA2 }]; # OLED_VDD
set_property -dict { PACKAGE_PIN AB22  IOSTANDARD LVCMOS33 } [get_ports { IOA4 }]; # OLED_VBAT
set_property -dict { PACKAGE_PIN AF17  IOSTANDARD LVCMOS18 } [get_ports { IOA3 }]; # OLED_SCLK
set_property -dict { PACKAGE_PIN Y15   IOSTANDARD LVCMOS18 } [get_ports { IOA5 }]; # OLED_SDIN

## uart0: on-board USB UART (FT232R)
set_property -dict { PACKAGE_PIN Y20   IOSTANDARD LVCMOS33 } [get_ports { IOC3 }]; # RX
set_property -dict { PACKAGE_PIN Y23   IOSTANDARD LVCMOS33 } [get_ports { IOC4 }]; # TX

## uart1: PmodUSBUART on Pmod JC (pin names from the FT232RQ's side)
set_property -dict { PACKAGE_PIN AH30  IOSTANDARD LVCMOS33 } [get_ports { IOB4 }];           # RX, JC3 (Pmod TXD)
set_property -dict { PACKAGE_PIN AJ27  IOSTANDARD LVCMOS33 } [get_ports { IOB5 }];           # TX, JC2 (Pmod RXD)
set_property -dict { PACKAGE_PIN AK29  IOSTANDARD LVCMOS33 } [get_ports { IO_UART1_CTS_N }]; # JC4 (Pmod CTS#)

## i2c0 on Pmod JA 3-4, as Mocha. The FPGA pull-ups help over the long tracks.
set_property -dict { PACKAGE_PIN T26   IOSTANDARD LVCMOS33 PULLTYPE PULLUP } [get_ports { IOB11 }]; # SCL
set_property -dict { PACKAGE_PIN T27   IOSTANDARD LVCMOS33 PULLTYPE PULLUP } [get_ports { IOB12 }]; # SDA

## spi_device on FT2232H channel A (Digilent DSPI): the FT2232H is the SPI host.
set_property -dict { PACKAGE_PIN AD27  IOSTANDARD LVCMOS33 PULLTYPE PULLDOWN } [get_ports { SPI_DEV_CLK  }];
set_property -dict { PACKAGE_PIN W29   IOSTANDARD LVCMOS33 PULLTYPE PULLUP   } [get_ports { SPI_DEV_CS_L }];
set_property -dict { PACKAGE_PIN W27   IOSTANDARD LVCMOS33 PULLTYPE PULLDOWN } [get_ports { SPI_DEV_D0   }]; # host MOSI
set_property -dict { PACKAGE_PIN W28   IOSTANDARD LVCMOS33 PULLTYPE PULLDOWN } [get_ports { SPI_DEV_D1   }]; # host MISO
set_property -dict { PACKAGE_PIN AD29  IOSTANDARD LVCMOS33                   } [get_ports { IO_SPIEN     }];

## usbdev on the Dual USB PMOD (connector J1) in Pmod JB 7-10
set_property -dict { PACKAGE_PIN T25   IOSTANDARD LVCMOS33 } [get_ports { USB_P }];            # JB7, D+
set_property -dict { PACKAGE_PIN U25   IOSTANDARD LVCMOS33 } [get_ports { USB_N }];            # JB8, D-
set_property -dict { PACKAGE_PIN U22   IOSTANDARD LVCMOS33 } [get_ports { IO_USB_DPPULLUP0 }]; # JB9, D+ pull-up
set_property -dict { PACKAGE_PIN U23   IOSTANDARD LVCMOS33 } [get_ports { IOC7 }];             # JB10, VBUS sense

## rv_dm JTAG on Pmod JD, as Mocha
set_property -dict { PACKAGE_PIN V27   IOSTANDARD LVCMOS33 PULLTYPE PULLUP } [get_ports { IOR3 }]; # JD1, TCK
set_property -dict { PACKAGE_PIN Y30   IOSTANDARD LVCMOS33 PULLTYPE PULLUP } [get_ports { IOR0 }]; # JD2, TMS
set_property -dict { PACKAGE_PIN V24   IOSTANDARD LVCMOS33 PULLTYPE PULLUP } [get_ports { IOR2 }]; # JD3, TDI
set_property -dict { PACKAGE_PIN W22   IOSTANDARD LVCMOS33 PULLTYPE PULLUP } [get_ports { IOR4 }]; # JD4, TRST_N
set_property -dict { PACKAGE_PIN U24   IOSTANDARD LVCMOS33                 } [get_ports { IOR1 }]; # JD7, TDO
