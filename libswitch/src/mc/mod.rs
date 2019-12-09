//! Tegra210 Memory Controller implementation
//!
//! # Description
//!
//! Tegra X1 devices feature two Memory Controllers and two External
//! Memory Controllers, one set per channel.
//!
//! The Tegra X1 memory controller (MC) handles memory requests from
//! internal clients and arbitrates among them to allocate memory
//! bandwidth for DDR3L, LPDDR3, and LPDDR4 SDRAMs. The external
//! memory controller (EMC) communicates with external DDR3L,
//! LPDDR3, and LPDDR4 devices.

use mirage_mmio::{Mmio, VolatileStorage};

use crate::{clock::Car, timer::usleep};

/// Base address for the MC registers.
pub(crate) const MC_BASE: u32 = 0x7001_9000;

pub(crate) const IRAM_BOM: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x65C) as *const _) };

pub(crate) const IRAM_TOM: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x660) as *const _) };

pub(crate) const SEC_CARVEOUT_BOM: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x670) as *const _) };

pub(crate) const SEC_CARVEOUT_SIZE_MB: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x674) as *const _) };

pub(crate) const SEC_CARVEOUT_REG_CTRL: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x678) as *const _) };

pub(crate) const VIDEO_PROTECT_GPU_OVERRIDE_0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x984) as *const _) };

pub(crate) const VIDEO_PROTECT_GPU_OVERRIDE_1: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x988) as *const _) };

pub(crate) const VIDEO_PROTECT_BOM: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x648) as *const _) };

pub(crate) const VIDEO_PROTECT_SIZE_MB: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x64C) as *const _) };

pub(crate) const VIDEO_PROTECT_REG_CTRL: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x650) as *const _) };

pub(crate) const MTS_CARVEOUT_BOM: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x9A0) as *const _) };

pub(crate) const MTS_CARVEOUT_SIZE_MB: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x9A4) as *const _) };

pub(crate) const MTS_CARVEOUT_ADR_HI: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x9A8) as *const _) };

pub(crate) const MTS_CARVEOUT_REG_CTRL: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0x9ac) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_BOM: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC0C) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_BOM_HI: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC10) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_SIZE_128KB: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC14) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_CLIENT_ACCESS0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC18) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_CLIENT_ACCESS1: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC1C) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_CLIENT_ACCESS2: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC20) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_CLIENT_ACCESS3: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC24) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_CLIENT_ACCESS4: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC28) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_CLIENT_FORCE_INTERNAL_ACCESS0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC2C) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_CLIENT_FORCE_INTERNAL_ACCESS1: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC30) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_CLIENT_FORCE_INTERNAL_ACCESS2: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC34) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_CLIENT_FORCE_INTERNAL_ACCESS3: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC38) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_CLIENT_FORCE_INTERNAL_ACCESS4: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC3C) as *const _) };

pub(crate) const SECURITY_CARVEOUT1_CFG0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC08) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_BOM: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC5C) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_BOM_HI: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC60) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_SIZE_128KB: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC64) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_CLIENT_ACCESS0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC68) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_CLIENT_ACCESS1: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC6C) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_CLIENT_ACCESS2: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC70) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_CLIENT_ACCESS3: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC74) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_CLIENT_ACCESS4: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC78) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_CLIENT_FORCE_INTERNAL_ACCESS0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC7C) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_CLIENT_FORCE_INTERNAL_ACCESS1: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC80) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_CLIENT_FORCE_INTERNAL_ACCESS2: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC84) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_CLIENT_FORCE_INTERNAL_ACCESS3: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC88) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_CLIENT_FORCE_INTERNAL_ACCESS4: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC8C) as *const _) };

pub(crate) const SECURITY_CARVEOUT2_CFG0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xC58) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_BOM: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCAC) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_BOM_HI: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCB0) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_SIZE_128KB: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCB4) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_CLIENT_ACCESS0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCB8) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_CLIENT_ACCESS1: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCBC) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_CLIENT_ACCESS2: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCC0) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_CLIENT_ACCESS3: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCC4) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_CLIENT_ACCESS4: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCC8) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_CLIENT_FORCE_INTERNAL_ACCESS0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCCC) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_CLIENT_FORCE_INTERNAL_ACCESS1: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCD0) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_CLIENT_FORCE_INTERNAL_ACCESS2: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCD4) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_CLIENT_FORCE_INTERNAL_ACCESS3: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCD8) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_CLIENT_FORCE_INTERNAL_ACCESS4: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCDC) as *const _) };

pub(crate) const SECURITY_CARVEOUT3_CFG0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCA8) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_BOM: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCFC) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_BOM_HI: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD00) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_SIZE_128KB: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD04) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_CLIENT_ACCESS0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD08) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_CLIENT_ACCESS1: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD0C) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_CLIENT_ACCESS2: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD10) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_CLIENT_ACCESS3: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD14) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_CLIENT_ACCESS4: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD18) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_CLIENT_FORCE_INTERNAL_ACCESS0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD1C) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_CLIENT_FORCE_INTERNAL_ACCESS1: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD20) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_CLIENT_FORCE_INTERNAL_ACCESS2: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD24) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_CLIENT_FORCE_INTERNAL_ACCESS3: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD28) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_CLIENT_FORCE_INTERNAL_ACCESS4: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD2C) as *const _) };

pub(crate) const SECURITY_CARVEOUT4_CFG0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xCF8) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_BOM: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD4C) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_BOM_HI: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD50) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_SIZE_128KB: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD54) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_CLIENT_ACCESS0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD58) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_CLIENT_ACCESS1: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD5C) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_CLIENT_ACCESS2: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD60) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_CLIENT_ACCESS3: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD64) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_CLIENT_ACCESS4: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD68) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_CLIENT_FORCE_INTERNAL_ACCESS0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD6C) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_CLIENT_FORCE_INTERNAL_ACCESS1: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD70) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_CLIENT_FORCE_INTERNAL_ACCESS2: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD74) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_CLIENT_FORCE_INTERNAL_ACCESS3: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD78) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_CLIENT_FORCE_INTERNAL_ACCESS4: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD7C) as *const _) };

pub(crate) const SECURITY_CARVEOUT5_CFG0: Mmio<u32> =
    unsafe { Mmio::new((MC_BASE + 0xD48) as *const _) };

pub fn config_tsec_carveout(bom: u32, size_mb: u32, lock: bool) {
    SEC_CARVEOUT_BOM.write(bom);
    SEC_CARVEOUT_SIZE_MB.write(size_mb);

    if lock {
        SEC_CARVEOUT_REG_CTRL.write(1);
    }
}

pub fn config_carveout() {
    unsafe {
        Mmio::new(0x8005_FFFC as *const u32).write(0xC0ED_BBCC);
    }

    VIDEO_PROTECT_GPU_OVERRIDE_0.write(1);
    VIDEO_PROTECT_GPU_OVERRIDE_1.write(0);
    VIDEO_PROTECT_BOM.write(0);
    VIDEO_PROTECT_SIZE_MB.write(0);
    VIDEO_PROTECT_REG_CTRL.write(1);

    config_tsec_carveout(0, 0, true);

    MTS_CARVEOUT_BOM.write(0);
    MTS_CARVEOUT_SIZE_MB.write(0);
    MTS_CARVEOUT_ADR_HI.write(0);
    MTS_CARVEOUT_REG_CTRL.write(1);

    SECURITY_CARVEOUT1_BOM.write(0);
    SECURITY_CARVEOUT1_BOM_HI.write(0);
    SECURITY_CARVEOUT1_SIZE_128KB.write(0);
    SECURITY_CARVEOUT1_CLIENT_ACCESS0.write(0);
    SECURITY_CARVEOUT1_CLIENT_ACCESS1.write(0);
    SECURITY_CARVEOUT1_CLIENT_ACCESS2.write(0);
    SECURITY_CARVEOUT1_CLIENT_ACCESS3.write(0);
    SECURITY_CARVEOUT1_CLIENT_ACCESS4.write(0);
    SECURITY_CARVEOUT1_CLIENT_FORCE_INTERNAL_ACCESS0.write(0);
    SECURITY_CARVEOUT1_CLIENT_FORCE_INTERNAL_ACCESS1.write(0);
    SECURITY_CARVEOUT1_CLIENT_FORCE_INTERNAL_ACCESS2.write(0);
    SECURITY_CARVEOUT1_CLIENT_FORCE_INTERNAL_ACCESS3.write(0);
    SECURITY_CARVEOUT1_CLIENT_FORCE_INTERNAL_ACCESS4.write(0);
    SECURITY_CARVEOUT1_CFG0.write(0x4000006);

    SECURITY_CARVEOUT3_BOM.write(0);
    SECURITY_CARVEOUT3_BOM_HI.write(0);
    SECURITY_CARVEOUT3_SIZE_128KB.write(0);
    SECURITY_CARVEOUT3_CLIENT_ACCESS0.write(0);
    SECURITY_CARVEOUT3_CLIENT_ACCESS1.write(0);
    SECURITY_CARVEOUT3_CLIENT_ACCESS2.write(0x3000000);
    SECURITY_CARVEOUT3_CLIENT_ACCESS3.write(0);
    SECURITY_CARVEOUT3_CLIENT_ACCESS4.write(0x300);
    SECURITY_CARVEOUT3_CLIENT_FORCE_INTERNAL_ACCESS0.write(0);
    SECURITY_CARVEOUT3_CLIENT_FORCE_INTERNAL_ACCESS1.write(0);
    SECURITY_CARVEOUT3_CLIENT_FORCE_INTERNAL_ACCESS2.write(0);
    SECURITY_CARVEOUT3_CLIENT_FORCE_INTERNAL_ACCESS3.write(0);
    SECURITY_CARVEOUT3_CLIENT_FORCE_INTERNAL_ACCESS4.write(0);
    SECURITY_CARVEOUT3_CFG0.write(0x4401E7E);

    SECURITY_CARVEOUT4_BOM.write(0);
    SECURITY_CARVEOUT4_BOM_HI.write(0);
    SECURITY_CARVEOUT4_SIZE_128KB.write(0);
    SECURITY_CARVEOUT4_CLIENT_ACCESS0.write(0);
    SECURITY_CARVEOUT4_CLIENT_ACCESS1.write(0);
    SECURITY_CARVEOUT4_CLIENT_ACCESS2.write(0);
    SECURITY_CARVEOUT4_CLIENT_ACCESS3.write(0);
    SECURITY_CARVEOUT4_CLIENT_ACCESS4.write(0);
    SECURITY_CARVEOUT4_CLIENT_FORCE_INTERNAL_ACCESS0.write(0);
    SECURITY_CARVEOUT4_CLIENT_FORCE_INTERNAL_ACCESS1.write(0);
    SECURITY_CARVEOUT4_CLIENT_FORCE_INTERNAL_ACCESS2.write(0);
    SECURITY_CARVEOUT4_CLIENT_FORCE_INTERNAL_ACCESS3.write(0);
    SECURITY_CARVEOUT4_CLIENT_FORCE_INTERNAL_ACCESS4.write(0);
    SECURITY_CARVEOUT4_CFG0.write(0x8F);

    SECURITY_CARVEOUT5_BOM.write(0);
    SECURITY_CARVEOUT5_BOM_HI.write(0);
    SECURITY_CARVEOUT5_SIZE_128KB.write(0);
    SECURITY_CARVEOUT5_CLIENT_ACCESS0.write(0);
    SECURITY_CARVEOUT5_CLIENT_ACCESS1.write(0);
    SECURITY_CARVEOUT5_CLIENT_ACCESS2.write(0);
    SECURITY_CARVEOUT5_CLIENT_ACCESS3.write(0);
    SECURITY_CARVEOUT5_CLIENT_ACCESS4.write(0);
    SECURITY_CARVEOUT5_CLIENT_FORCE_INTERNAL_ACCESS0.write(0);
    SECURITY_CARVEOUT5_CLIENT_FORCE_INTERNAL_ACCESS1.write(0);
    SECURITY_CARVEOUT5_CLIENT_FORCE_INTERNAL_ACCESS2.write(0);
    SECURITY_CARVEOUT5_CLIENT_FORCE_INTERNAL_ACCESS3.write(0);
    SECURITY_CARVEOUT5_CLIENT_FORCE_INTERNAL_ACCESS4.write(0);
    SECURITY_CARVEOUT5_CFG0.write(0x8F);
}

pub fn config_carveout_finalize() {
    SECURITY_CARVEOUT2_BOM.write(0x8002_0000);
    SECURITY_CARVEOUT2_BOM_HI.write(0);
    SECURITY_CARVEOUT2_SIZE_128KB.write(2);
    SECURITY_CARVEOUT2_CLIENT_ACCESS0.write(0);
    SECURITY_CARVEOUT2_CLIENT_ACCESS1.write(0);
    SECURITY_CARVEOUT2_CLIENT_ACCESS2.write(0x3000000);
    SECURITY_CARVEOUT2_CLIENT_ACCESS3.write(0);
    SECURITY_CARVEOUT2_CLIENT_ACCESS4.write(0x300);
    SECURITY_CARVEOUT2_CLIENT_FORCE_INTERNAL_ACCESS0.write(0);
    SECURITY_CARVEOUT2_CLIENT_FORCE_INTERNAL_ACCESS1.write(0);
    SECURITY_CARVEOUT2_CLIENT_FORCE_INTERNAL_ACCESS2.write(0);
    SECURITY_CARVEOUT2_CLIENT_FORCE_INTERNAL_ACCESS3.write(0);
    SECURITY_CARVEOUT2_CLIENT_FORCE_INTERNAL_ACCESS4.write(0);
    SECURITY_CARVEOUT2_CFG0.write(0x440167E);
}

pub fn enable_ahb_redirect() {
    let car = unsafe { Car::get() };

    car.lvl2_clk_gate_ovrd.write((car.lvl2_clk_gate_ovrd.read() & 0xFFF7_FFFF) | 0x80000);

    IRAM_BOM.write(0x4000_0000);
    IRAM_TOM.write(0x4003_F000);
}

pub fn disable_ahb_redirect() {
    let car = unsafe { Car::get() };

    IRAM_BOM.write(0xFFFF_F000);
    IRAM_TOM.write(0);

    car.lvl2_clk_gate_ovrd.write(car.lvl2_clk_gate_ovrd.read() & 0xFFF7_FFFF);
}

pub fn enable_mc() {
    let car = unsafe { Car::get() };

    // Set EMC clock source.
    car.clk_source_emc.write((car.clk_source_emc.read() & 0x1FFF_FFFF) | 0x4000_0000);

    // Enable MIPI CAL clock.
    car.clk_enb_h_set.write((car.clk_enb_h_set.read() & 0xFDFF_FFFF) | 0x2000000);

    // Enable MC clock.
    car.clk_enb_h_set.write((car.clk_enb_h_set.read() & 0xFFFF_FFFE) | 1);

    // Enable EMC DLL clock.
    car.clk_enb_x_set.write((car.clk_enb_x_set.read() & 0xFFFF_BFFF) | 0x4000);

    // Clear EMC and MC reset.
    car.rst_dev_h_clr.write(0x2000001);
    usleep(5);

    disable_ahb_redirect();
}
