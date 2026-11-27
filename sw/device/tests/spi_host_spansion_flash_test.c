// Copyright lowRISC contributors (OpenTitan project).
// Copyright Mark Hayter.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
//
// Copied (and modified) from spi_host_winbond_flash_test.c.
//
// Tests spi_host0 against a Spansion S25FL-S flash (S25FL256S on the Digilent
// Genesys 2, the FPGA's configuration flash). The part has no SFDP and no
// RSTEN/RST, 4-byte-mode or 32 KB erase commands, so this test leaves out the
// Winbond test's steps that need them, and uses the part's own commands for
// software reset, quad enable and 4-byte addresses. The test erases and
// programs the bottom of the flash, where an FPGA bitstream usually lives.
#include <assert.h>

#include "hw/top/dt/spi_host.h"
#include "sw/device/lib/arch/device.h"
#include "sw/device/lib/base/bitfield.h"
#include "sw/device/lib/base/macros.h"
#include "sw/device/lib/base/memory.h"
#include "sw/device/lib/dif/dif_spi_host.h"
#include "sw/device/lib/runtime/log.h"
#include "sw/device/lib/testing/spi_device_testutils.h"
#include "sw/device/lib/testing/spi_flash_testutils.h"
#include "sw/device/lib/testing/test_framework/check.h"
#include "sw/device/lib/testing/test_framework/ottf_main.h"
#include "sw/device/tests/spi_host_flash_test_impl.h"

static_assert(__BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__,
              "This test assumes the target platform is little endian.");

OTTF_DEFINE_TEST_CONFIG();

enum SpansionVendorSpecific {
  kManufactureId = 0x01,
  kOpSoftwareReset = 0xf0,
  kOpReadConfig = 0x35,
  kOpPageProgramQuad = 0x32,
  kOpRead4b = 0x13,
  kOpPageProgram4b = 0x12,
  kOpSectorErase64k4b = 0xdc,
  // Quad-enable mechanism (JESD216 numbering): QE is CR1[1], read with 0x35
  // and written with a two-byte WRR (SR1, CR1). The other two-byte
  // mechanisms read the second byte with 0x05, which returns SR1 again, and
  // would write SR1's bits into CR1's one-time programmable bits.
  kQuadEnableMechanism = 5,
};

// Configuration register 1 (CR1).
static const bitfield_field32_t kCr1LatencyCode = {.mask = 0x3, .index = 6};
static const uint32_t kCr1OtpBits = 0x3c;  // TBPROT, DNU, BPNV, TBPARM
static const uint32_t kCr1Quad = 1u << 1;

// Status register 1 (SR1) error bits.
static const uint32_t kSr1Errors = (1u << 6) | (1u << 5);  // P_ERR, E_ERR

static status_t test_software_reset_spansion(dif_spi_host_t *spi) {
  dif_spi_host_segment_t op = {.type = kDifSpiHostSegmentTypeOpcode,
                               .opcode = {.opcode = kOpSoftwareReset,
                                          .width = kDifSpiHostWidthStandard}};
  TRY(dif_spi_host_transaction(spi, /*cs_id=*/0, &op, 1));
  return OK_STATUS();
}

static status_t test_enable_quad_mode_spansion(dif_spi_host_t *spi) {
  uint32_t cr1 = (uint32_t)TRY(
      spi_flash_testutils_read_status(spi, kOpReadConfig, /*length=*/1));
  LOG_INFO("CR1 is 0x%02x", cr1);
  // Latency code 00: 8 dummy cycles for fast, dual and quad output reads,
  // which the reads in spi_host_flash_test_impl.c use.
  TRY_CHECK(bitfield_field32_read(cr1, kCr1LatencyCode) == 0,
            "Latency code %d, expected 0", cr1 >> kCr1LatencyCode.index);
  if ((cr1 & kCr1Quad) == 0) {
    // A non-volatile write: only when the bit is clear.
    TRY(spi_flash_testutils_quad_enable(spi, kQuadEnableMechanism, true));
  }
  uint32_t new_cr1 = (uint32_t)TRY(
      spi_flash_testutils_read_status(spi, kOpReadConfig, /*length=*/1));
  TRY_CHECK(new_cr1 & kCr1Quad, "CR1 0x%02x: QUAD is not set", new_cr1);
  TRY_CHECK((new_cr1 & kCr1OtpBits) == (cr1 & kCr1OtpBits),
            "CR1 OTP bits changed: 0x%02x -> 0x%02x", cr1, new_cr1);
  return OK_STATUS();
}

// 4-byte addresses through the 4-byte opcodes, above the first 16 MiB.
static status_t test_4bytes_address_spansion(dif_spi_host_t *spi,
                                             const uint8_t *pattern,
                                             size_t length) {
  enum { kAddress = 0x01000100 };
  TRY(spi_flash_testutils_erase_op(spi, kOpSectorErase64k4b, kAddress,
                                   /*addr_is_4b=*/true));
  TRY(spi_flash_testutils_program_op(spi, kOpPageProgram4b, pattern, length,
                                     kAddress, /*addr_is_4b=*/true,
                                     kTransactionWidthMode111));
  uint8_t buf[256];
  TRY(spi_flash_testutils_read_op(spi, kOpRead4b, buf, length, kAddress,
                                  /*addr_is_4b=*/true,
                                  /*width=*/1,
                                  /*dummy=*/0));
  TRY_CHECK_ARRAYS_EQ(buf, pattern, length);
  return OK_STATUS();
}

static status_t test_no_errors(dif_spi_host_t *spi) {
  uint32_t sr1 = (uint32_t)TRY(spi_flash_testutils_read_status(
      spi, kSpiDeviceFlashOpReadStatus1, /*length=*/1));
  TRY_CHECK((sr1 & kSr1Errors) == 0, "SR1 0x%02x: program or erase error", sr1);
  return OK_STATUS();
}

bool test_main(void) {
  dif_spi_host_t spi_host;
  CHECK_DIF_OK(dif_spi_host_init_from_dt((dt_spi_host_t)0, &spi_host));

  CHECK(kClockFreqHiSpeedPeripheralHz <= UINT32_MAX,
        "kClockFreqHiSpeedPeripheralHz must fit in uint32_t");
  uint32_t spi_speed = 10000000;  // 10MHz
  if (spi_speed > kClockFreqHiSpeedPeripheralHz / 2) {
    spi_speed = (uint32_t)(kClockFreqHiSpeedPeripheralHz / 2);
  }
  CHECK_DIF_OK(
      dif_spi_host_configure(&spi_host,
                             (dif_spi_host_config_t){
                                 .spi_clock = spi_speed,
                                 .peripheral_clock_freq_hz =
                                     (uint32_t)kClockFreqHiSpeedPeripheralHz,
                                 .chip_select =
                                     {
                                         .idle = 1,
                                     },
                             }),
      "SPI_HOST config failed!");
  CHECK_DIF_OK(dif_spi_host_output_set_enabled(&spi_host, true));

  // On a 7-series FPGA, the configuration flash's clock is CCLK, driven
  // through STARTUPE2, which drops the first clock edges after
  // configuration. In case the tested flash is the board configuration flash
  // (e.g. on the Digilent Genesys 2), do and ignore a status read.
  CHECK_STATUS_OK(spi_flash_testutils_read_status(
      &spi_host, kSpiDeviceFlashOpReadStatus1, /*length=*/1));

  status_t result = OK_STATUS();
  EXECUTE_TEST(result, test_software_reset_spansion, &spi_host);
  EXECUTE_TEST(result, test_read_jedec, &spi_host, kManufactureId);
  EXECUTE_TEST(result, test_enable_quad_mode_spansion, &spi_host);
  // 4 KB sector erases work only in the part's parameter sectors, which are
  // at the bottom of the flash as shipped (CR1 TBPARM = 0).
  EXECUTE_TEST(result, test_sector_erase, &spi_host);
  EXECUTE_TEST(result, test_page_program, &spi_host);
  EXECUTE_TEST(result, test_fast_read, &spi_host);
  EXECUTE_TEST(result, test_dual_read, &spi_host);
  EXECUTE_TEST(result, test_quad_read, &spi_host);
  // The Spansion `QPP` opcode operates in 1-1-4 mode.
  EXECUTE_TEST(result, test_page_program_quad, &spi_host, kOpPageProgramQuad,
               kTransactionWidthMode114);
  EXECUTE_TEST(result, test_erase_64k_block, &spi_host);
  {
    uint8_t pattern[256];
    for (size_t i = 0; i < sizeof(pattern); ++i) {
      pattern[i] = (uint8_t)(i * 7 + 3);
    }
    EXECUTE_TEST(result, test_4bytes_address_spansion, &spi_host, pattern,
                 sizeof(pattern));
  }
  EXECUTE_TEST(result, test_no_errors, &spi_host);

  return status_ok(result);
}
