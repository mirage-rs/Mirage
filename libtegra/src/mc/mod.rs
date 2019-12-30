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

pub fn config_tsec_carveout(bom: u32, size_mb: u32, lock: bool) {
    let sec_carveout_bom = unsafe { &*((MC_BASE + 0x670) as *const Mmio<u32>) };

    let sec_carveout_size_mb = unsafe { &*((MC_BASE + 0x674) as *const Mmio<u32>) };

    let sec_carveout_reg_ctrl = unsafe { &*((MC_BASE + 0x678) as *const Mmio<u32>) };

    sec_carveout_bom.write(bom);
    sec_carveout_size_mb.write(size_mb);

    if lock {
        sec_carveout_reg_ctrl.write(1);
    }
}

pub fn config_carveout() {
    let video_protect_gpu_override_0 =
        unsafe { &*((MC_BASE + 0x984) as *const Mmio<u32>) };

    let video_protect_gpu_override_1 =
        unsafe { &*((MC_BASE + 0x988) as *const Mmio<u32>) };

    let video_protect_bom =
        unsafe { &*((MC_BASE + 0x648) as *const Mmio<u32>) };

    let video_protect_size_mb =
        unsafe { &*((MC_BASE + 0x64C) as *const Mmio<u32>) };

    let video_protect_reg_ctrl =
        unsafe { &*((MC_BASE + 0x650) as *const Mmio<u32>) };

    let mts_carveout_bom =
        unsafe { &*((MC_BASE + 0x9A0) as *const Mmio<u32>) };

    let mts_carveout_size_mb =
        unsafe { &*((MC_BASE + 0x9A4) as *const Mmio<u32>) };

    let mts_carveout_adr_hi =
        unsafe { &*((MC_BASE + 0x9A8) as *const Mmio<u32>) };

    let mts_carveout_reg_ctrl =
        unsafe { &*((MC_BASE + 0x9ac) as *const Mmio<u32>) };

    let security_carveout1_bom =
        unsafe { &*((MC_BASE + 0xC0C) as *const Mmio<u32>) };

    let security_carveout1_bom_hi =
        unsafe { &*((MC_BASE + 0xC10) as *const Mmio<u32>) };

    let security_carveout1_size_128kb =
        unsafe { &*((MC_BASE + 0xC14) as *const Mmio<u32>) };

    let security_carveout1_client_access0 =
        unsafe { &*((MC_BASE + 0xC18) as *const Mmio<u32>) };

    let security_carveout1_client_access1 =
        unsafe { &*((MC_BASE + 0xC1C) as *const Mmio<u32>) };

    let security_carveout1_client_access2 =
        unsafe { &*((MC_BASE + 0xC20) as *const Mmio<u32>) };

    let security_carveout1_client_access3 =
        unsafe { &*((MC_BASE + 0xC24) as *const Mmio<u32>) };

    let security_carveout1_client_access4 =
        unsafe { &*((MC_BASE + 0xC28) as *const Mmio<u32>) };

    let security_carveout1_client_force_internal_access0 =
        unsafe { &*((MC_BASE + 0xC2C) as *const Mmio<u32>) };

    let security_carveout1_client_force_internal_access1 =
        unsafe { &*((MC_BASE + 0xC30) as *const Mmio<u32>) };

    let security_carveout1_client_force_internal_access2 =
        unsafe { &*((MC_BASE + 0xC34) as *const Mmio<u32>) };

    let security_carveout1_client_force_internal_access3 =
        unsafe { &*((MC_BASE + 0xC38) as *const Mmio<u32>) };

    let security_carveout1_client_force_internal_access4 =
        unsafe { &*((MC_BASE + 0xC3C) as *const Mmio<u32>) };

    let security_carveout1_cfg0 =
        unsafe { &*((MC_BASE + 0xC08) as *const Mmio<u32>) };

    let security_carveout3_bom =
        unsafe { &*((MC_BASE + 0xCAC) as *const Mmio<u32>) };

    let security_carveout3_bom_hi =
        unsafe { &*((MC_BASE + 0xCB0) as *const Mmio<u32>) };

    let security_carveout3_size_128kb =
        unsafe { &*((MC_BASE + 0xCB4) as *const Mmio<u32>) };

    let security_carveout3_client_access0 =
        unsafe { &*((MC_BASE + 0xCB8) as *const Mmio<u32>) };

    let security_carveout3_client_access1 =
        unsafe { &*((MC_BASE + 0xCBC) as *const Mmio<u32>) };

    let security_carveout3_client_access2 =
        unsafe { &*((MC_BASE + 0xCC0) as *const Mmio<u32>) };

    let security_carveout3_client_access3 =
        unsafe { &*((MC_BASE + 0xCC4) as *const Mmio<u32>) };

    let security_carveout3_client_access4 =
        unsafe { &*((MC_BASE + 0xCC8) as *const Mmio<u32>) };

    let security_carveout3_client_force_internal_access0 =
        unsafe { &*((MC_BASE + 0xCCC) as *const Mmio<u32>) };

    let security_carveout3_client_force_internal_access1 =
        unsafe { &*((MC_BASE + 0xCD0) as *const Mmio<u32>) };

    let security_carveout3_client_force_internal_access2 =
        unsafe { &*((MC_BASE + 0xCD4) as *const Mmio<u32>) };

    let security_carveout3_client_force_internal_access3 =
        unsafe { &*((MC_BASE + 0xCD8) as *const Mmio<u32>) };

    let security_carveout3_client_force_internal_access4 =
        unsafe { &*((MC_BASE + 0xCDC) as *const Mmio<u32>) };

    let security_carveout3_cfg0 =
        unsafe { &*((MC_BASE + 0xCA8) as *const Mmio<u32>) };

    let security_carveout4_bom = unsafe { &*((MC_BASE + 0xCFC) as *const Mmio<u32>) };

    let security_carveout4_bom_hi =
        unsafe { &*((MC_BASE + 0xD00) as *const Mmio<u32>) };

    let security_carveout4_size_128kb =
        unsafe { &*((MC_BASE + 0xD04) as *const Mmio<u32>) };

    let security_carveout4_client_access0 =
        unsafe { &*((MC_BASE + 0xD08) as *const Mmio<u32>) };

    let security_carveout4_client_access1 =
        unsafe { &*((MC_BASE + 0xD0C) as *const Mmio<u32>) };

    let security_carveout4_client_access2 =
        unsafe { &*((MC_BASE + 0xD10) as *const Mmio<u32>) };

    let security_carveout4_client_access3 =
        unsafe { &*((MC_BASE + 0xD14) as *const Mmio<u32>) };

    let security_carveout4_client_access4 =
        unsafe { &*((MC_BASE + 0xD18) as *const Mmio<u32>) };

    let security_carveout4_client_force_internal_access0 =
        unsafe { &*((MC_BASE + 0xD1C) as *const Mmio<u32>) };

    let security_carveout4_client_force_internal_access1 =
        unsafe { &*((MC_BASE + 0xD20) as *const Mmio<u32>) };

    let security_carveout4_client_force_internal_access2 =
        unsafe { &*((MC_BASE + 0xD24) as *const Mmio<u32>) };

    let security_carveout4_client_force_internal_access3 =
        unsafe { &*((MC_BASE + 0xD28) as *const Mmio<u32>) };

    let security_carveout4_client_force_internal_access4 =
        unsafe { &*((MC_BASE + 0xD2C) as *const Mmio<u32>) };

    let security_carveout4_cfg0 =
        unsafe { &*((MC_BASE + 0xCF8) as *const Mmio<u32>) };

    let security_carveout5_bom = unsafe { &*((MC_BASE + 0xD4C) as *const Mmio<u32>) };

    let security_carveout5_bom_hi =
        unsafe { &*((MC_BASE + 0xD50) as *const Mmio<u32>) };

    let security_carveout5_size_128kb =
        unsafe { &*((MC_BASE + 0xD54) as *const Mmio<u32>) };

    let security_carveout5_client_access0 =
        unsafe { &*((MC_BASE + 0xD58) as *const Mmio<u32>) };

    let security_carveout5_client_access1 =
        unsafe { &*((MC_BASE + 0xD5C) as *const Mmio<u32>) };

    let security_carveout5_client_access2 =
        unsafe { &*((MC_BASE + 0xD60) as *const Mmio<u32>) };

    let security_carveout5_client_access3 =
        unsafe { &*((MC_BASE + 0xD64) as *const Mmio<u32>) };

    let security_carveout5_client_access4 =
        unsafe { &*((MC_BASE + 0xD68) as *const Mmio<u32>) };

    let security_carveout5_client_force_internal_access0 =
        unsafe { &*((MC_BASE + 0xD6C) as *const Mmio<u32>) };

    let security_carveout5_client_force_internal_access1 =
        unsafe { &*((MC_BASE + 0xD70) as *const Mmio<u32>) };

    let security_carveout5_client_force_internal_access2 =
        unsafe { &*((MC_BASE + 0xD74) as *const Mmio<u32>) };

    let security_carveout5_client_force_internal_access3 =
        unsafe { &*((MC_BASE + 0xD78) as *const Mmio<u32>) };

    let security_carveout5_client_force_internal_access4 =
        unsafe { &*((MC_BASE + 0xD7C) as *const Mmio<u32>) };

    let security_carveout5_cfg0 =
        unsafe { &*((MC_BASE + 0xD48) as *const Mmio<u32>) };

    unsafe {
        (*(0x8005_FFFC as *const Mmio<u32>)).write(0xC0ED_BBCC);
    }

    video_protect_gpu_override_0.write(1);
    video_protect_gpu_override_1.write(0);
    video_protect_bom.write(0);
    video_protect_size_mb.write(0);
    video_protect_reg_ctrl.write(1);

    config_tsec_carveout(0, 0, true);

    mts_carveout_bom.write(0);
    mts_carveout_size_mb.write(0);
    mts_carveout_adr_hi.write(0);
    mts_carveout_reg_ctrl.write(1);

    security_carveout1_bom.write(0);
    security_carveout1_bom_hi.write(0);
    security_carveout1_size_128kb.write(0);
    security_carveout1_client_access0.write(0);
    security_carveout1_client_access1.write(0);
    security_carveout1_client_access2.write(0);
    security_carveout1_client_access3.write(0);
    security_carveout1_client_access4.write(0);
    security_carveout1_client_force_internal_access0.write(0);
    security_carveout1_client_force_internal_access1.write(0);
    security_carveout1_client_force_internal_access2.write(0);
    security_carveout1_client_force_internal_access3.write(0);
    security_carveout1_client_force_internal_access4.write(0);
    security_carveout1_cfg0.write(0x4000006);

    security_carveout3_bom.write(0);
    security_carveout3_bom_hi.write(0);
    security_carveout3_size_128kb.write(0);
    security_carveout3_client_access0.write(0);
    security_carveout3_client_access1.write(0);
    security_carveout3_client_access2.write(0x3000000);
    security_carveout3_client_access3.write(0);
    security_carveout3_client_access4.write(0x300);
    security_carveout3_client_force_internal_access0.write(0);
    security_carveout3_client_force_internal_access1.write(0);
    security_carveout3_client_force_internal_access2.write(0);
    security_carveout3_client_force_internal_access3.write(0);
    security_carveout3_client_force_internal_access4.write(0);
    security_carveout3_cfg0.write(0x4401E7E);

    security_carveout4_bom.write(0);
    security_carveout4_bom_hi.write(0);
    security_carveout4_size_128kb.write(0);
    security_carveout4_client_access0.write(0);
    security_carveout4_client_access1.write(0);
    security_carveout4_client_access2.write(0);
    security_carveout4_client_access3.write(0);
    security_carveout4_client_access4.write(0);
    security_carveout4_client_force_internal_access0.write(0);
    security_carveout4_client_force_internal_access1.write(0);
    security_carveout4_client_force_internal_access2.write(0);
    security_carveout4_client_force_internal_access3.write(0);
    security_carveout4_client_force_internal_access4.write(0);
    security_carveout4_cfg0.write(0x8F);

    security_carveout5_bom.write(0);
    security_carveout5_bom_hi.write(0);
    security_carveout5_size_128kb.write(0);
    security_carveout5_client_access0.write(0);
    security_carveout5_client_access1.write(0);
    security_carveout5_client_access2.write(0);
    security_carveout5_client_access3.write(0);
    security_carveout5_client_access4.write(0);
    security_carveout5_client_force_internal_access0.write(0);
    security_carveout5_client_force_internal_access1.write(0);
    security_carveout5_client_force_internal_access2.write(0);
    security_carveout5_client_force_internal_access3.write(0);
    security_carveout5_client_force_internal_access4.write(0);
    security_carveout5_cfg0.write(0x8F);
}

pub fn config_carveout_finalize() {
    let security_carveout2_bom = unsafe { &*((MC_BASE + 0xC5C) as *const Mmio<u32>) };

    let security_carveout2_bom_hi =
        unsafe { &*((MC_BASE + 0xC60) as *const Mmio<u32>) };

    let security_carveout2_size_128kb =
        unsafe { &*((MC_BASE + 0xC64) as *const Mmio<u32>) };

    let security_carveout2_client_access0 =
        unsafe { &*((MC_BASE + 0xC68) as *const Mmio<u32>) };

    let security_carveout2_client_access1 =
        unsafe { &*((MC_BASE + 0xC6C) as *const Mmio<u32>) };

    let security_carveout2_client_access2 =
        unsafe { &*((MC_BASE + 0xC70) as *const Mmio<u32>) };

    let security_carveout2_client_access3 =
        unsafe { &*((MC_BASE + 0xC74) as *const Mmio<u32>) };

    let security_carveout2_client_access4 =
        unsafe { &*((MC_BASE + 0xC78) as *const Mmio<u32>) };

    let security_carveout2_client_force_internal_access0 =
        unsafe { &*((MC_BASE + 0xC7C) as *const Mmio<u32>) };

    let security_carveout2_client_force_internal_access1 =
        unsafe { &*((MC_BASE + 0xC80) as *const Mmio<u32>) };

    let security_carveout2_client_force_internal_access2 =
        unsafe { &*((MC_BASE + 0xC84) as *const Mmio<u32>) };

    let security_carveout2_client_force_internal_access3 =
        unsafe { &*((MC_BASE + 0xC88) as *const Mmio<u32>) };

    let security_carveout2_client_force_internal_access4 =
        unsafe { &*((MC_BASE + 0xC8C) as *const Mmio<u32>) };

    let security_carveout2_cfg0 =
        unsafe { &*((MC_BASE + 0xC58) as *const Mmio<u32>) };

    security_carveout2_bom.write(0x8002_0000);
    security_carveout2_bom_hi.write(0);
    security_carveout2_size_128kb.write(2);
    security_carveout2_client_access0.write(0);
    security_carveout2_client_access1.write(0);
    security_carveout2_client_access2.write(0x3000000);
    security_carveout2_client_access3.write(0);
    security_carveout2_client_access4.write(0x300);
    security_carveout2_client_force_internal_access0.write(0);
    security_carveout2_client_force_internal_access1.write(0);
    security_carveout2_client_force_internal_access2.write(0);
    security_carveout2_client_force_internal_access3.write(0);
    security_carveout2_client_force_internal_access4.write(0);
    security_carveout2_cfg0.write(0x440167E);
}

pub fn enable_ahb_redirect() {
    let iram_bom = unsafe { &*((MC_BASE + 0x65C) as *const Mmio<u32>) };

    let iram_tom = unsafe { &*((MC_BASE + 0x660) as *const Mmio<u32>) };

    let car = unsafe { Car::get() };

    car.lvl2_clk_gate_ovrd.write((car.lvl2_clk_gate_ovrd.read() & 0xFFF7_FFFF) | 0x80000);

    iram_bom.write(0x4000_0000);
    iram_tom.write(0x4003_F000);
}

pub fn disable_ahb_redirect() {
    let iram_bom = unsafe { &*((MC_BASE + 0x65C) as *const Mmio<u32>) };

    let iram_tom = unsafe { &*((MC_BASE + 0x660) as *const Mmio<u32>) };

    let car = unsafe { Car::get() };

    iram_bom.write(0xFFFF_F000);
    iram_tom.write(0);

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
